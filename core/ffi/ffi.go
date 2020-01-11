package main

import (
	"C"
	"encoding/json"
	"gitlab.com/bloom42/bloom/core/bloom/calculator"
	"gitlab.com/bloom42/bloom/core/bloom/notes"
	"strings"
	"fmt"
)

type PayloadIn struct {
	Method string          `json:"method"`
	Params json.RawMessage `json:"params"`
}

type PayloadOut struct {
	Data  interface{} `json:"data"`
	Error *CoreError  `json:"error"`
}

type CoreError struct {
	Code    string      `json:"code"`
	Message string      `json:"message"`
	Meta    interface{} `json:"meta"`
}

//export blmcore_call
func blmcore_call(message *C.char) *C.char {
	var payload PayloadIn

	bytesMsg := C.GoString(message)
	// if bytesMsg == nil {
	// 	return nil // TODO(z0mbie42): return error
	// }

	err := json.Unmarshal([]byte(bytesMsg), &payload)
	if err != nil {
		return nil // TODO(z0mbie42): return error
	}

	parts := strings.Split(payload.Method, ".")
	if len(parts) != 2 {
		return nil // TODO(z0mbie42): return error
	}

	// TODO(z0mbie42): handle methods returns go error, and convert to c error here
	switch parts[0] {
	case "notes":
		return handleNotesMethod(parts[1], payload.Params)
	case "calculator":
		return handleCalculatorMehtod(parts[1], payload.Params)
	default:
		return nil // TODO: return not found error
	}
}

func handleNotesMethod(method string, jsonParams json.RawMessage) *C.char {
	switch method {
	case "list_notes":
		var params notes.ListNotesParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		res, err := notes.ListNotes(params)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		payloadOut := PayloadOut{Data: res}
		data, err := json.Marshal(payloadOut)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		return C.CString(string(data))
	default:
		return methodNotFoundError(method, "notes")
	}
}

func handleCalculatorMehtod(method string, jsonParams json.RawMessage) *C.char {
	switch method {
	case "calc":
		var params calculator.CalcParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		res, err := calculator.Calc(params)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		payloadOut := PayloadOut{Data: res}
		data, err := json.Marshal(payloadOut)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		return C.CString(string(data))
	default:
		return methodNotFoundError(method, "calculator")
	}
}

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

func main() {}
