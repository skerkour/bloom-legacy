package main

import (
	stdlog "log"
	"os"

	"gitlab.com/bloom42/bloom/bloom/cli"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/rz/log"
)

func main() {
	stdlog.SetOutput(os.Stderr)
	err := cli.Execute()
	if err != nil {
		log.Fatal("main", rz.Error("err", err))
	}
}
