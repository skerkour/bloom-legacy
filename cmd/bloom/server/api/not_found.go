package api

import (
	"net/http"
)

// NotFoundHandler simply returns a JSON message indicating that the route does not exist
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
