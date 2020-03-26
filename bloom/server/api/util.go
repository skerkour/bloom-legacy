package api

import (
	"encoding/json"
	"net/http"

	"gitlab.com/bloom42/libs/rz-go"
)

type ApiRes struct {
	Data   interface{} `json:"data"`
	Errors []Error     `json:"errors"`
}

// ResJSON write json response
func ResJSON(w http.ResponseWriter, r *http.Request, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	err := json.NewEncoder(w).Encode(payload)
	if err != nil {
		rz.FromCtx(r.Context()).Error("encoding response to JSON", rz.Err(err))
		errorMessage := "internal error, please try again later"
		res := Error{
			Message: errorMessage,
			Extensions: map[string]string{
				"code": "internal",
			},
		}
		w.WriteHeader(500)
		_ = json.NewEncoder(w).Encode(res)
		return
	}
}
