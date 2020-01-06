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
func Call(message *C.char) *C.char {
	return nil
}

func main() {}
