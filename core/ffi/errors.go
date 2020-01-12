package main

import (
	"C"
	"encoding/json"
	"fmt"
)

func methodNotFoundError(method, service string) *C.char {
	err := &CoreError{
		Code:    "METHOD_NOT_FOUND",
		Message: fmt.Sprintf("method '%s' not found in service: '%s'", method, service),
		Meta:    nil,
	}
	payloadOut := PayloadOut{Error: err}
	data, _ := json.Marshal(payloadOut)
	return C.CString(string(data))
}
