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
)

func main() {
	err := log.InitLogger()
	if err != nil {
		fmt.Printf("Failed to initialize logger: %v\n", err)
		return
	}
	defer log.Logger.Writer()
	log.Logger.Println("main - Program Started")

	flagPort := flag.Int("port", 8080, "Port number to connect or serve")
	flag.Parse()

	serverAlreadyRunning := server.CheckServerRunning(*flagPort)

	state := types.AppState{
		Port:                 *flagPort,
		ServerAlreadyRunning: serverAlreadyRunning,
	}

	if !serverAlreadyRunning {
		go server.StartServer(state)
	} else {
		log.Logger.Printf("Server is already running on port %d. Connecting to the existing server...\n", state.Port)
		server.ConnectToServer(state.Port)
	}

	processRequest(state)
	if state.QuitRequested {
		log.Logger.Println("Quit flag requested, Program Exiting")
		return
	}

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Logger.Println("Program Exiting")
}

func processRequest(state types.AppState) {
	client := &http.Client{}

	switch {
	case state.StatusRequested:
		log.Logger.Println("Status requested.")
		resp, err := client.Get(fmt.Sprintf("http://localhost:%d/status", state.Port))
		if err != nil {
			log.Logger.Printf("Failed to get status: %v\n", err)
			return
		}
		defer resp.Body.Close()
		body, _ := io.ReadAll(resp.Body)
		log.Logger.Printf("Status response: %s\n", string(body))

	}
}
