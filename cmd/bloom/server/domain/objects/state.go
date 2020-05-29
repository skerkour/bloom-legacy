package objects

import (
	"encoding/base64"
	"strconv"
)

// EncodeState encode a state to a string
func EncodeState(state int64) string {
	str := strconv.FormatInt(state, 10)
	return base64.StdEncoding.EncodeToString([]byte(STATE_INT64 + str))
}

// DecodeStateString decodes a state string to its int64 representation
func DecodeStateString(str string) (state int64, err error) {
	if str == "" {
		state = 0
		return
	}
	decoded, err := base64.StdEncoding.DecodeString(str)
	if err != nil {
		return
	}
	state, err = strconv.ParseInt(string(decoded[1:]), 10, 64)
	return
}
