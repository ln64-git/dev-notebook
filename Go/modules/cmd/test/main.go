package main

import (
	"os"
	"os/signal"
	"syscall"

	"github.com/charmbracelet/log"
	"github.com/dev/go/modules/external/ollama"
)

func main() {
	log.Info("Program started")

	model := "llama2-uncensored"
	prompt := "give me three sentences."
	port := 11434

	tokenChan, err := ollama.GetOllamaTokenResponse(model, prompt, port)
	if err != nil {
		log.Fatal(err)
	}

	go func() {
		for token := range tokenChan {
			log.Info(token)
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Info("Program exiting")
}
