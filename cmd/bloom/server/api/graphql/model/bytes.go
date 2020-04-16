package model

import (
	"encoding/base64"
	"fmt"
	"io"
)

type Bytes []byte

func (b Bytes) MarshalGQL(w io.Writer) {
	encoder := base64.NewEncoder(base64.RawURLEncoding, w)
	encoder.Write(b)
	encoder.Close()
}

func (b *Bytes) UnmarshalGQL(v interface{}) error {
	var err error

	switch v := v.(type) {
	case string:
		*b, err = base64.RawURLEncoding.DecodeString(v)
	case *string:
		*b, err = base64.RawURLEncoding.DecodeString(*v)
	case []byte:
		*b = v
	default:
		err = fmt.Errorf("%T is not []byte", v)
	}

	return err
}
