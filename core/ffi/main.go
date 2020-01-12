package main

import (
	"C"
	"encoding/json"
	"strings"
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

//export blmcore_init
func blmcore_init() C.int {
	return C.int(0)
}

func main() {}
