package kernel

import (
	"database/sql/driver"
	"errors"
	"time"
)

type SQLiteTime time.Time

func (t SQLiteTime) Time() time.Time {
	return time.Time(t)
}

func (t SQLiteTime) Value() (driver.Value, error) {
	tim := time.Time(t)
	valueStr := tim.Format(time.RFC3339Nano)
	return []byte(valueStr), nil
}

func (t *SQLiteTime) Scan(src interface{}) error {
	var sourceStr string

	switch src.(type) {
	case string:
		sourceStr = src.(string)
	case []byte:
		sourceStr = string(src.([]byte))
	default:
		return errors.New("Incompatible type for SQliteTime")
	}

	tim, err := time.Parse(time.RFC3339Nano, sourceStr)
	if err != nil {
		return err
	}

	*t = SQLiteTime(tim)

	return nil
}

func (t *SQLiteTime) MarshalJSON() ([]byte, error) {
	tim := time.Time(*t)
	valueStr := tim.Format(time.RFC3339Nano)
	return []byte(valueStr), nil
}

func (t *SQLiteTime) UnmarshalJSON(data []byte) error {
	tim, err := time.Parse(time.RFC3339Nano, string(data))
	if err != nil {
		return err
	}

	*t = SQLiteTime(tim)
	return nil
}
