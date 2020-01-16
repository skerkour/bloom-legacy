package api

import (
	"encoding/json"
	"net/http"
)

func NotFoundHandler(w http.ResponseWriter, r *http.Request) {
	data, err := json.Marshal(Error{
		Code:    "not_found",
		Message: "route not found",
		Meta: map[string]string{
			"path": r.URL.Path,
		},
	})
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Header().Set("content-type", "application/json")
	w.WriteHeader(http.StatusNotFound)
	w.Write(data)

}
