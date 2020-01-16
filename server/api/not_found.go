package api

import (
	"net/http"
)

func NotFoundHandler(w http.ResponseWriter, r *http.Request) {
	err := Error{
		Code:    "not_found",
		Message: "route not found",
		Meta: map[string]string{
			"path": r.URL.Path,
		},
	}
	ResError(w, r, 404, err)
}
