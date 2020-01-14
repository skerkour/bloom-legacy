package main

import (
	"C"
	"encoding/json"
	"gitlab.com/bloom42/bloom/core"
)

//export blmcore_call
func blmcore_call(message *C.char) *C.char {
	var messageIn core.MessageIn

	bytesMsg := C.GoString(message)
	// if bytesMsg == nil {
	// 	return nil // TODO(z0mbie42): return error
	// }

	err := json.Unmarshal([]byte(bytesMsg), &messageIn)
	if err != nil {
		return nil // TODO(z0mbie42): return error
	}

	data, err := core.HandleMessage(messageIn)
	return C.CString(string(data))
}

//export blmcore_init
func blmcore_init() C.int {
	return C.int(0)
}

func main() {}
