package async

import (
	"encoding/json"
)

type Message struct {
	Type  string      `json:"type"`
	Data  interface{} `json:"data"`
	Error *string     `json:"error"`
}

type DecodedMessage struct {
	Type  string          `json:"type"`
	Data  json.RawMessage `json:"data"`
	Error *string         `json:"error"`
}
