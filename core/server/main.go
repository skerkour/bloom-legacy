package main

import (
	"encoding/json"
	"log"
	"net"
	"net/http"

	"gitlab.com/bloom42/bloom/core"
	"gitlab.com/bloom42/libs/rz-go"
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

	data, err := core.HandleMessage(messageIn)
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

	unixListener, err := net.Listen("unix", "/tmp/bloom42.sock")
	if err != nil {
		log.Fatal("listening to unix socket", rz.Err(err))
	}
	defer unixListener.Close()

	log.Fatal(http.Serve(unixListener, nil))
}
