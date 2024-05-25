package main

import (
	"bufio"
	"bytes"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/ln64-git/voxctl/internal/config"
	"github.com/ln64-git/voxctl/internal/log"
	"github.com/ln64-git/voxctl/internal/server"
	"github.com/ln64-git/voxctl/internal/speech"
	"github.com/ln64-git/voxctl/internal/types"
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
	flagToken := flag.Bool("token", false, "Process input stream token")
	flagStatus := flag.Bool("status", false, "Request info")
	flagQuit := flag.Bool("quit", false, "Exit application after request")
	flag.Parse()

	log.Logger.Printf("Flags: token=%v, status=%v, quit=%v\n", *flagToken, *flagStatus, *flagQuit)

	cfg, err := config.GetConfig()
	if err != nil {
		log.Logger.Printf("Failed to get configuration: %v\n", err)
		return
	}

	state := types.AppState{
		Port:            *flagPort,
		StatusRequested: *flagStatus,
		QuitRequested:   *flagQuit,
		VoiceGender:     cfg.VoiceGender,
		VoiceName:       cfg.VoiceName,
	}

	if *flagToken {
		log.Logger.Println("Token input detected.")
		scanner := bufio.NewScanner(os.Stdin)
		if scanner.Scan() {
			line := scanner.Text()
			state.Token = line
			log.Logger.Printf("Token text: %s\n", state.Token)
		}
		if err := scanner.Err(); err != nil {
			log.Logger.Printf("Failed to read from stdin: %v\n", err)
			return
		}
	}

	serverAlreadyRunning := server.CheckServerRunning(*flagPort)
	state.ServerAlreadyRunning = serverAlreadyRunning

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

	case state.Token != "":
		log.Logger.Println("Token input detected.")
		tokenReq := speech.SpeechRequest{
			Text:      state.Token,
			Gender:    state.VoiceGender,
			VoiceName: state.VoiceName,
		}
		body := bytes.NewBufferString(tokenReq.ToJSON())
		resp, err := client.Post(fmt.Sprintf("http://localhost:%d/token", state.Port), "application/json", body)
		if err != nil {
			log.Logger.Printf("Failed to send token request: %v\n", err)
			return
		}
		defer resp.Body.Close()
		log.Logger.Printf("Token response: %s\n", resp.Status)
	}
}

