package api

import (
	"net/http"
)

// HelloWorlHandler simply returns the `{"hello": "world"}` JSON payload`
func HelloWorlHandler(w http.ResponseWriter, r *http.Request) {
	data := map[string]string{"hello": "world"}
	ResJSON(w, r, http.StatusOK, data)
}
