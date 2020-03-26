package api

import (
	"net/http"
)

type Error struct {
	Path       string            `json:"path"`
	Message    string            `json:"message"`
	Extensions map[string]string `json:"extension"`
}

func ResError(w http.ResponseWriter, r *http.Request, status int, erro Error) {
	if status == 500 {
		erro.Message = "internal error, please try again later"
	}

	apiRes := ApiRes{Errors: []Error{erro}}
	ResJSON(w, r, status, apiRes)
}
