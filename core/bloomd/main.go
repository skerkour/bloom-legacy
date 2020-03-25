package main

import (
	"context"
	"encoding/json"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"gitlab.com/bloom42/bloom/core"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/rz/log"
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

	http.HandleFunc("/electronCall", handleElectronPost)

	unixListener, err := net.Listen("unix", UNIX_SOCKET)
	if err != nil {
		log.Fatal("error listening to unix socket", rz.Err(err))
	}
	defer unixListener.Close()

	server := http.Server{}

	go func() {
		err = server.Serve(unixListener)
		if err != nil {
			log.Fatal("error running BloomCoreServer", rz.Err(err))
		}
	}()
	log.Info("BloomCoreServer started", rz.String("socket", UNIX_SOCKET))

	signalCatcher := make(chan os.Signal, 2)

	signal.Notify(signalCatcher, os.Interrupt,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)
	sig := <-signalCatcher
	log.Info("BloomCoreServer is shutting down", rz.String("reason", sig.String()))
	err = os.Remove(UNIX_SOCKET)
	if err != nil {
		log.Fatal("error removing unix socket", rz.Err(err))
	}

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	server.SetKeepAlivesEnabled(false)
	if err := server.Shutdown(ctx); err != nil {
		log.Fatal("Could not gracefuly shutdown BloomCoreServer", rz.Err(err))
	}
	log.Info("BloomCoreServer stopped")
}
