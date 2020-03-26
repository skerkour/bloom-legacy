package api

import (
	"net/http"
)

func NotFoundHandler(w http.ResponseWriter, r *http.Request) {
	err := Error{
		Path:    r.URL.Path,
		Message: "route not found",
		Extensions: map[string]string{
			"code": "not_found",
		},
	}
	ResError(w, r, 404, err)
}
