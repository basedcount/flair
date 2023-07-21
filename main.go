package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/grovesbs/flairs/server"
)

func main() {
	port := os.Getenv("FLAIRS_PORT")
	if len(port) == 0 {
		port = ":6969"
	}
	port = strings.TrimPrefix(port, ":") // removes ':'
	port = fmt.Sprintf(":%s", port) // adds it back for formatting ':'
	if port[0] != ':' {
		os.Stderr.WriteString(fmt.Sprintf("Variable PORT must start with a colon (':'). You provided %s", port))
		os.Exit(64)
	}

	server.Serve(port)
	
}

