package core

import (
	"C"
	"fmt"
)

func MethodNotFoundError(method, service string) MessageOut {
	err := &ErrorData{
		Code:    "METHOD_NOT_FOUND",
		Message: fmt.Sprintf("method '%s' not found in service: '%s'", method, service),
		Meta:    nil,
	}
	return MessageOut{Error: err}
	// data, _ := json.Marshal(payloadOut)
	// return C.CString(string(data))
}
