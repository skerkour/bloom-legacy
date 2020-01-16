package api

import (
	"encoding/json"
	"net/http"
)

type helloWorld struct {
	Hello string `json:"hello"`
}

func HelloWorlHandler(w http.ResponseWriter, r *http.Request) {
	data, err := json.Marshal(helloWorld{Hello: "world"})
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("content-type", "application/json")
	w.Write(data)
}
