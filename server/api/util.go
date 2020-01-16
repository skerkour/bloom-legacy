package api

import (
	"encoding/json"
	"gitlab.com/bloom42/libs/rz-go"
	"net/http"
)

type Response struct {
	Error  *string     `json:"error"`
	Data   interface{} `json:"data"`
	Status int         `json:"status"`
}

// ResJSON write json response
func ResJSON(w http.ResponseWriter, r *http.Request, status int, payload interface{}) {
	res := Response{Data: payload, Status: status}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	err := json.NewEncoder(w).Encode(res)
	if err != nil {
		rz.FromCtx(r.Context()).Error("encoding response to JSON", rz.Err(err))
		errorMessage := "internal error, please try again later"
		res := Response{Error: &errorMessage, Status: status}
		w.WriteHeader(500)
		_ = json.NewEncoder(w).Encode(res)
		return
	}
}
