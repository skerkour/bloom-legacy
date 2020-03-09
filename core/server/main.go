package main

import (
	"encoding/json"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"gitlab.com/bloom42/bloom/core"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

const UNIX_SOCKET = "/tmp/com.bloom42.bloom.sock"

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

func setupResponse(w *http.ResponseWriter, req *http.Request) {
	(*w).Header().Set("Access-Control-Allow-Origin", "*")
	(*w).Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
	(*w).Header().Set("Access-Control-Allow-Headers", "Accept, Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization")
}

func main() {
	// remove Unix socket if exists
	os.Remove(UNIX_SOCKET)

	// handle SIGKILL
	signalCatcher := make(chan os.Signal, 2)
	signal.Notify(signalCatcher, os.Interrupt,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)
	go func() {
		<-signalCatcher
		err := os.Remove(UNIX_SOCKET)
		if err != nil {
			log.Fatal("error removing unix socket", rz.Err(err))
		}
		os.Exit(0)
	}()

	http.HandleFunc("/electronCall", handleElectronPost)

	unixListener, err := net.Listen("unix", UNIX_SOCKET)
	if err != nil {
		log.Fatal("error listening to unix socket", rz.Err(err))
	}
	defer unixListener.Close()

	err = http.Serve(unixListener, nil)
	if err != nil {
		log.Fatal("error running server", rz.Err(err))
	}
}
