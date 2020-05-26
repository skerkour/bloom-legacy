package objects

import (
	"encoding/json"
	"errors"

	"github.com/golang/snappy"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/crypto"
)

func decryptObject(encryptedObject *model.Object, masterKey []byte) (ret *Object, err error) {
	// TODO: improve AD
	// TODO: verify data
	if encryptedObject.Algorithm != DEFAULT_ALGORITHM_STRING {
		err = errors.New("Invalid algorithm")
		return
	}
	if len(encryptedObject.EncryptedData) > MAX_OBJECT_SIZE {
		err = errors.New("Object is too large")
		return
	}
	if len(encryptedObject.Nonce) != crypto.AEADNonceSize {
		err = errors.New("Nonce is invalid")
		return
	}

	// decrypt objectKey
	objectKeyCipher, err := crypto.NewAEAD(masterKey)
	if err != nil {
		return ret, err
	}
	objectKeyNonce, err := crypto.DeriveKeyFromKey(encryptedObject.Nonce, encryptedObject.ID, crypto.AEADNonceSize)
	if err != nil {
		return
	}
	objectKey, err := objectKeyCipher.Open(nil, objectKeyNonce, encryptedObject.EncryptedKey, encryptedObject.ID)
	if err != nil {
		err = errors.New("Error decrypting object key")
		return
	}
	defer crypto.Zeroize(objectKey) // wipe objectKey from memory

	// if len(encryptedObject.EncryptedData) == 0 {
	// 	return nil, nil
	// }

	// decrypt object data
	objectDataCipher, err := crypto.NewAEAD(objectKey)
	if err != nil {
		return ret, err
	}
	compressedObjectData, err := objectDataCipher.Open(nil, encryptedObject.Nonce, encryptedObject.EncryptedData, encryptedObject.ID)
	if err != nil {
		err = errors.New("Error decrypting object")
		return
	}
	defer crypto.Zeroize(compressedObjectData) // wipe compressed object from memory

	// decompress object
	objectData, err := snappy.Decode(nil, compressedObjectData)
	if err != nil {
		err = errors.New("Error decompressing object")
		return
	}
	defer crypto.Zeroize(objectData) // wipe JSON object from memory

	ret = &Object{}
	err = json.Unmarshal(objectData, ret)
	if err != nil {
		err = errors.New("Error parsing object JSON")
		return
	}

	return ret, nil
}
