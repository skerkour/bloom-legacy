package api

// Response represents an api response sent by any of the HTTP apis
type Response struct {
	Data   interface{} `json:"data"`
	Errors []Error     `json:"errors"`
}
