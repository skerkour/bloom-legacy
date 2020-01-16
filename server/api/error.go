package api

type Error struct {
	Code    string            `json:"code"`
	Message string            `json:"msg"`
	Meta    map[string]string `json:"meta"`
}
