package core

import (
	"C"
	"fmt"
)

func methodNotFoundError(method, service string) MessageOut {
	err := &ErrorData{
		Code:    "METHOD_NOT_FOUND",
		Message: fmt.Sprintf("method '%s' not found in service: '%s'", method, service),
		Meta:    nil,
	}
	return MessageOut{Error: err}
	// data, _ := json.Marshal(payloadOut)
	// return C.CString(string(data))
}

func serviceNotFoundError(service string) MessageOut {
	err := &ErrorData{
		Code:    "Service_NOT_FOUND",
		Message: fmt.Sprintf("method '%s' not found", service),
		Meta:    nil,
	}
	return MessageOut{Error: err}
	// data, _ := json.Marshal(payloadOut)
	// return C.CString(string(data))
}

func InternalError(err error) MessageOut {
	errData := &ErrorData{
		Code:    "INTERNAL",
		Message: err.Error(),
		Meta:    nil,
	}
	return MessageOut{Error: errData}
	// data, _ := json.Marshal(payloadOut)
	// return C.CString(string(data))
}
