package main

import (
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/dev/go/modules/internal/log"
	"github.com/dev/go/modules/internal/server"
	"github.com/dev/go/modules/internal/types"
	"github.com/sirupsen/logrus"
)

func main() {
	// Initialize the logger with default configuration
	cfg := log.DefaultConfig()
	logger, err := log.InitLogger(cfg)
	if err != nil {
		logrus.Fatalf("could not initialize logger: %v", err)
	}
	logger.Info("Logger initialized successfully with default configuration")

	flagPort := flag.Int("port", 8080, "Port number to connect or serve")
	flag.Parse()

	serverAlreadyRunning := server.CheckServerRunning(*flagPort)

	state := types.ServerState{
		Logger:               logger,
		Port:                 *flagPort,
		ServerAlreadyRunning: serverAlreadyRunning,
	}
	if !serverAlreadyRunning {
		go server.StartServer(state)
	} else {
		if server.ConnectToServer(state.Port) {
			state.Logger.Infof("Connected to the existing server on port %d.", state.Port)
		} else {
			state.Logger.Errorf("Failed to connect to the existing server on port %d.", state.Port)
		}
	}

	processRequest(state)

	if state.QuitRequested {
		state.Logger.Info("Quit flag requested, Program Exiting")
		return
	}

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	state.Logger.Info("Program Exiting")
}

func processRequest(state types.ServerState) {
	client := &http.Client{}

	switch {
	case state.StatusRequested:
		state.Logger.Info("Status requested.")
		resp, err := client.Get(fmt.Sprintf("http://localhost:%d/status", state.Port))
		if err != nil {
			state.Logger.Errorf("Failed to get status: %v\n", err)
			return
		}
		defer resp.Body.Close()
		body, _ := io.ReadAll(resp.Body)
		state.Logger.Infof("Status response: %s\n", string(body))
	}
}
