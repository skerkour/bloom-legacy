package http

import (
	"encoding/json"
	"net/http"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// SendJSON sends back a json response
func (server *Server) SendJSON(w http.ResponseWriter, r *http.Request, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	err := json.NewEncoder(w).Encode(payload)
	if err != nil {
		errorMessage := "server.ResJSON: encoding payload to JSON"
		log.FromCtx(r.Context()).Error(errorMessage, log.Err("error", err))
		res := api.Response{
			Errors: []api.Error{api.NewError(errors.Internal(errorMessage, err))},
		}
		w.WriteHeader(500)
		_ = json.NewEncoder(w).Encode(res)
		return
	}
}

// SendError sends back a json error response
func (server *Server) SendError(w http.ResponseWriter, r *http.Request, status int, err error) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	payload := api.Response{
		Errors: []api.Error{api.NewError(err)},
	}
	server.SendJSON(w, r, status, payload)
}
