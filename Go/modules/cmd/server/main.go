package main

import (
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/charmbracelet/log"
	"github.com/dev/go/modules/internal/server"
	"github.com/dev/go/modules/internal/types"
)

func main() {

	flagPort := flag.Int("port", 8080, "Port number to connect or serve")
	flag.Parse()

	serverAlreadyRunning := server.CheckServerRunning(*flagPort)

	state := types.ServerState{
		Port:                 *flagPort,
		ServerAlreadyRunning: serverAlreadyRunning,
	}

	// Check if server is already running
	if !server.CheckServerRunning(state.Port) {
		go server.StartServer(state)
	} else {
		resp, err := server.ConnectToServer(state.Port)
		if err != nil {
			log.Errorf("Failed to connect to the existing server on port %d: %v", state.Port, err)
		} else {
			log.Infof("Connected to the existing server on port %d. Status: %s", state.Port, resp.Status)
			resp.Body.Close()
		}
	}

	processRequest(state)
	if state.QuitRequested {
		log.Info("Quit flag requested, Program Exiting")
		return
	}

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Info("Program Exiting")
}

func processRequest(state types.ServerState) {
	client := &http.Client{}

	switch {
	case state.StatusRequested:
		log.Info("Status requested.")
		resp, err := client.Get(fmt.Sprintf("http://localhost:%d/status", state.Port))
		if err != nil {
			log.Errorf("Failed to get status: %v\n", err)
			return
		}
		defer resp.Body.Close()
		body, _ := io.ReadAll(resp.Body)
		log.Infof("Status response: %s\n", string(body))
	}
}
