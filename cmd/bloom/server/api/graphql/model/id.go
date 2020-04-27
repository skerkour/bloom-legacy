package model

import (
	"fmt"
	"io"
	"strconv"

	"github.com/99designs/gqlgen/graphql"
	"gitlab.com/bloom42/lily/uuid"
)

// MarshalID encode a `uuid.UUID` to JSON
func MarshalID(id uuid.UUID) graphql.Marshaler {
	return graphql.WriterFunc(func(w io.Writer) {
		io.WriteString(w, strconv.Quote(id.String()))
	})
}

// UnmarshalID decodes JSON to `uuid.UUID`
func UnmarshalID(v interface{}) (uuid.UUID, error) {
	var err error
	var ret uuid.UUID

	switch v := v.(type) {
	case string:
		ret, err = uuid.Parse(v)
	case *string:
		ret, err = uuid.Parse(*v)
	case []byte:
		ret, err = uuid.ParseBytes(v)
	default:
		err = fmt.Errorf("%T is not []byte", v)
	}

	return ret, err
}

/*
type ID uuid.UUID

func (id ID) MarshalGQL(w io.Writer) {
	io.WriteString(w, strconv.Quote(uuid.UUID(id).String()))
}

func (id *ID) UnmarshalGQL(v interface{}) error {
	var err error
	var ret uuid.UUID

	switch v := v.(type) {
	case string:
		ret, err = uuid.Parse(v)
		*id = ID(ret)
	case *string:
		ret, err = uuid.Parse(*v)
		*id = ID(ret)
	case []byte:
		ret, err = uuid.ParseBytes(v)
		*id = ID(ret)
	default:
		err = fmt.Errorf("%T is not []byte", v)
	}

	return err
}
*/
