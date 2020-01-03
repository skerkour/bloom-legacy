package main

import (
  "C"
)

//export call
func call(method *C.char, data *C.char) *C.char {
  return nil
}

func main(){}
