package main

import (
	"encoding/json"
	"log"
	"net/http"
	"strings"

	"gitlab.com/bloom42/bloom/core"
)

func handleElectronPost(w http.ResponseWriter, r *http.Request) {
	var messageIn core.MessageIn

	setupResponse(&w, r)
	if (*r).Method == "OPTIONS" {
		return
	}

	// Try to decode the request body into the struct. If there is an error,
	// respond to the client with the error message and a 400 status code.
	err := json.NewDecoder(r.Body).Decode(&messageIn)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	parts := strings.Split(messageIn.Method, ".")
	if len(parts) != 2 {
		http.Error(w, err.Error(), http.StatusBadRequest) // TODO(z0mbie42): return error
		return
	}

	// TODO(z0mbie42): handle methods returns go error, and convert to c error here
	var messageOut core.MessageOut

	switch parts[0] {
	case "core":
		messageOut = core.HandleCoreMethod(parts[1], messageIn.Params)
	case "notes":
		messageOut = core.HandleNotesMethod(parts[1], messageIn.Params)
	case "calculator":
		messageOut = core.HandleCalculatorMehtod(parts[1], messageIn.Params)
	default:
		messageOut = core.MethodNotFoundError(parts[0], "service") // TODO: return not found error
	}

	data, err := json.Marshal(messageOut)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// Do something with the Person struct...
	w.Header().Set("content-type", "application/json")
	w.Write(data)
}

func enableCors(w *http.ResponseWriter) {
	(*w).Header().Set("Access-Control-Allow-Origin", "*")
}

func setupResponse(w *http.ResponseWriter, req *http.Request) {
	(*w).Header().Set("Access-Control-Allow-Origin", "*")
	(*w).Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
	(*w).Header().Set("Access-Control-Allow-Headers", "Accept, Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization")
}

func main() {
	http.HandleFunc("/electronCall", handleElectronPost)

	log.Fatal(http.ListenAndServe("127.0.0.1:8042", nil))
}
