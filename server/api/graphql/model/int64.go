package model

import (
	"encoding/json"
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

	switch value := v.(type) {
	case string:
		ii, err := strconv.ParseInt(value, 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	case *string:
		ii, err := strconv.ParseInt(*value, 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	case []byte:
		ii, err := strconv.ParseInt(string(value), 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	case int:
		*i = Int64(int64(value))
	case int32:
		*i = Int64(int64(value))
	case int64:
		*i = Int64(value)
	case json.Number:
		ii, err := strconv.ParseInt(string(value), 10, 64)
		if err != nil {
			return nil
		}
		*i = Int64(ii)
	default:
		err = fmt.Errorf("%T type is not deserializable for Int64", v)
	}

	return err
}
