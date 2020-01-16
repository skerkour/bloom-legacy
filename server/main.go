package main

import (
	stdlog "log"
	"os"

	"gitlab.com/bloom42/bloom/server/cmd"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

func main() {
	stdlog.SetOutput(os.Stderr)
	err := cmd.Execute()
	if err != nil {
		log.Fatal("main", rz.Error("err", err))
	}
}
