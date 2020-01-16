package api

import (
	"encoding/json"
	"gitlab.com/bloom42/libs/rz-go"
	"net/http"
)

// ResJSON write json response
func ResJSON(w http.ResponseWriter, r *http.Request, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	err := json.NewEncoder(w).Encode(payload)
	if err != nil {
		rz.FromCtx(r.Context()).Error("encoding response to JSON", rz.Err(err))
		errorMessage := "internal error, please try again later"
		res := Error{Message: errorMessage, Code: "internal"}
		w.WriteHeader(500)
		_ = json.NewEncoder(w).Encode(res)
		return
	}
}
