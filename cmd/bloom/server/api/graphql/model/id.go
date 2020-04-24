package model

import (
	"fmt"
	"io"
	"strconv"

	"gitlab.com/bloom42/lily/uuid"
)

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
