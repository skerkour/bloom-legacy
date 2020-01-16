package api

import (
	"encoding/json"
	"gitlab.com/bloom42/libs/rz-go"
	"net/http"
)

type Error struct {
	Code    string            `json:"code"`
	Message string            `json:"msg"`
	Meta    map[string]string `json:"meta"`
}

func ResError(w http.ResponseWriter, r *http.Request, status int, erro Error) {
	if status == 500 {
		erro.Message = "internal error, please try again later"
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	err := json.NewEncoder(w).Encode(erro)
	if err != nil {
		rz.FromCtx(r.Context()).Error("encoding error response to JSON", rz.Err(err))
	}
}
