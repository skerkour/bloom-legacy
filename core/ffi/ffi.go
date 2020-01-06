package main

import (
	"C"
	"encoding/json"
)

type Payload struct {
	Method string          `json:"method"`
	Params json.RawMessage `json:"params"`
}

type Error struct {
	Code string      `json:"code"`
	Msg  string      `json:"msg"`
	Meta interface{} `json:"meta"`
}

//export call
func call(message *C.char) *C.char {
	var payload Payload

	bytesMsg := C.GoString(message)
	// if bytesMsg == nil {
	// 	return nil // TODO(z0mbie42): return error
	// }

	err := json.Unmarshal([]byte(bytesMsg), &payload)
	if err != nil {
		return nil // TODO(z0mbie42): return error
	}

	switch payload.Method {
	default:
		return nil // TODO: return not found error
	}
}

func main() {}
