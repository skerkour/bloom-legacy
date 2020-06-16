package http

import (
	"net/http"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/errors"
)

// HandlerHelloWorld simply returns the `{"hello": "world"}` JSON payload`
func (server *Server) HandlerHelloWorld(w http.ResponseWriter, r *http.Request) {
	payload := api.Response{
		Data: map[string]string{"hello": "world"},
	}
	server.SendJSON(w, r, http.StatusOK, payload)
}

// HandlerNotFound simply returns an error indicating that the route does not exist
func (server *Server) HandlerNotFound(w http.ResponseWriter, r *http.Request) {
	err := errors.NotFound("Route not found.")
	server.SendError(w, r, http.StatusNotFound, err)
}

// HandlerIndex simply redirect to `config.WebsiteUrl`
func (server *Server) HandlerIndex(w http.ResponseWriter, r *http.Request) {
	http.Redirect(w, r, server.config.WebsiteUrl, 301)
}
