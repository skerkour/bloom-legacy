package api

import (
	"net/http"
)

func HelloWorlHandler(w http.ResponseWriter, r *http.Request) {
	res := map[string]string{"hello": "world"}
	ResJSON(w, r, 200, res)
}
