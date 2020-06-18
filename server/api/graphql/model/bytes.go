package model

import (
	"encoding/base64"
	"fmt"
	"io"
	"strconv"

	"github.com/99designs/gqlgen/graphql"
)

// MarshalBytes encodes a `[]byte` to JSON
func MarshalBytes(buffer []byte) graphql.Marshaler {
	return graphql.WriterFunc(func(w io.Writer) {
		b64 := base64.StdEncoding.EncodeToString(buffer)
		io.WriteString(w, strconv.Quote(b64))
	})
}

// UnmarshalBytes decodes JSON to `[]byte`
func UnmarshalBytes(value interface{}) (buffer []byte, err error) {
	switch v := value.(type) {
	case string:
		buffer, err = base64.StdEncoding.DecodeString(v)
	case *string:
		buffer, err = base64.StdEncoding.DecodeString(*v)
	case []byte:
		buffer = v
	default:
		err = fmt.Errorf("%T is not []byte", v)
	}

	return buffer, err
}

// type Bytes []byte

// func (b Bytes) MarshalGQL(w io.Writer) {
// 	encoder := base64.NewEncoder(base64.StdEncoding, w)
// 	encoder.Write(b)
// 	encoder.Close()
// }

// func (b *Bytes) UnmarshalGQL(v interface{}) error {
// 	var err error

// 	switch v := v.(type) {
// 	case string:
// 		*b, err = base64.StdEncoding.DecodeString(v)
// 	case *string:
// 		*b, err = base64.StdEncoding.DecodeString(*v)
// 	case []byte:
// 		*b = v
// 	default:
// 		err = fmt.Errorf("%T is not []byte", v)
// 	}

// 	return err
// }
