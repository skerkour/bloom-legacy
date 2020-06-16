package model

import (
	"encoding/json"
	"fmt"
	"io"
	"strconv"

	"github.com/99designs/gqlgen/graphql"
)

// MarshalInt64 encode a `int64` to JSON
func MarshalInt64(i int64) graphql.Marshaler {
	return graphql.WriterFunc(func(w io.Writer) {
		str := strconv.FormatInt(int64(i), 10)
		w.Write([]byte(str))
	})
}

// UnmarshalInt64 decodes JSON to `int64`
func UnmarshalInt64(v interface{}) (i int64, err error) {

	switch value := v.(type) {
	case string:
		i, err = strconv.ParseInt(value, 10, 64)
		if err != nil {
			return
		}
	case *string:
		i, err = strconv.ParseInt(*value, 10, 64)
	case []byte:
		i, err = strconv.ParseInt(string(value), 10, 64)
	case int:
		i = int64(value)
	case int32:
		i = int64(value)
	case int64:
		i = value
	case json.Number:
		i, err = strconv.ParseInt(string(value), 10, 64)
	default:
		err = fmt.Errorf("%T type is not deserializable for Int64", v)
	}

	return
}

/*
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
*/
