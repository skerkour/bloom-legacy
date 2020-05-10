package objects

import (
	"gitlab.com/bloom42/bloom/core/api/model"
)

// compress and encrypt encodes to JSON, compress and encrypt with an unique key
// an object
func compressAndEncrypt(object StoredObject) (*model.ObjectInput, error) {
	return &model.ObjectInput{}, nil
}
