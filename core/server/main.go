package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/elextronPost", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello boy, you've requested: %s\n", r.URL.Path)
	})

	log.Fatal(http.ListenAndServe("127.0.0.1:8042", nil))
}
