package model

import (
	"fmt"
	"io"
	"strconv"
)

type Int64 int64

func (i Int64) MarshalGQL(w io.Writer) {
	str := strconv.FormatInt(int64(i), 10)
	w.Write([]byte(str))
}

func (i *Int64) UnmarshalGQL(v interface{}) error {
	var err error

	switch v := v.(type) {
	case string:
		ii, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	case *string:
		ii, err := strconv.ParseInt(*v, 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	case []byte:
		ii, err := strconv.ParseInt(string(v), 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	default:
		err = fmt.Errorf("%T is not []byte", v)
	}

	return err
}
