package core

import "encoding/json"

type MessageIn struct {
	Method string          `json:"method"`
	Params json.RawMessage `json:"params"`
}

type MessageOut struct {
	Data  interface{} `json:"data"`
	Error *ErrorData  `json:"error"`
}

type ErrorData struct {
	Code    string      `json:"code"`
	Message string      `json:"message"`
	Meta    interface{} `json:"meta"`
}

type InitRes struct {
	Success bool `json:"success"`
}
