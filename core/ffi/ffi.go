package main

import (
  "C"
  "encoding/json"
)

type Payload struct {
  Method string `json:"method"`
  Params json.RawMessage `json:"params"`
}

//export call
func Call(message *C.char) *C.char {
  return nil
}

func main(){}
