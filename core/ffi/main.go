package main

import (
	"C"
	"encoding/json"
	"gitlab.com/bloom42/bloom/core"
	"strings"
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

	parts := strings.Split(messageIn.Method, ".")
	if len(parts) != 2 {
		return nil // TODO(z0mbie42): return error
	}

	// TODO(z0mbie42): handle methods returns go error, and convert to c error here
	var messageOut core.MessageOut

	switch parts[0] {
	case "notes":
		messageOut = core.HandleNotesMethod(parts[1], messageIn.Params)
	case "calculator":
		messageOut = core.HandleCalculatorMehtod(parts[1], messageIn.Params)
	default:
		messageOut = core.MethodNotFoundError(parts[0], "service") // TODO: return not found error
	}

	data, err := json.Marshal(messageOut)
	if err != nil {
		return nil
	}
	return C.CString(string(data))
}

//export blmcore_init
func blmcore_init() C.int {
	return C.int(0)
}

func main() {}
