package main

import (
	stdlog "log"
	"os"

	"gitlab.com/bloom42/bloom/server/cli"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

func main() {
	stdlog.SetOutput(os.Stderr)
	err := cli.Execute()
	if err != nil {
		log.Fatal("main", rz.Error("err", err))
	}
}
