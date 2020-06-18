package main

import (
	stdlog "log"
	"os"

	"gitlab.com/bloom42/bloom/server/app/cli"
	"gitlab.com/bloom42/gobox/log"
)

func main() {
	stdlog.SetOutput(os.Stderr)
	log.SetGlobalLogger(log.Clone(log.SetFormatter(log.FormatterConsole())))

	err := cli.Execute()
	if err != nil {
		log.Fatal("main", log.Err("error", err))
	}
}
