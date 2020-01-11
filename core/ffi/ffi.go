package main

import (
	"C"
	"encoding/json"
	"strings"
)

type Payload struct {
	Method string          `json:"method"`
	Params json.RawMessage `json:"params"`
}

type Error struct {
	Code    string      `json:"code"`
	Message string      `json:"message"`
	Meta    interface{} `json:"meta"`
}

//export blmcore_call
func blmcore_call(message *C.char) *C.char {
	var payload Payload

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
	default:
		return nil // TODO: return not found error
	}
}

func handleNotesMethod(method string, params json.RawMessage) *C.char {
	switch method {
	case "list_notes":
		return C.CString(`{"data": { "notes": []}}`)
	default:
		return nil // TODO: return not found error
	}
}

func main() {}
