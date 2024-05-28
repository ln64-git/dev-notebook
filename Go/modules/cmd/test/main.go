package main

import (
	"os"
	"os/signal"
	"syscall"

	"github.com/charmbracelet/log"
	"github.com/dev/go/modules/external/ollama"
	"github.com/dev/go/modules/internal/speech"
)

func main() {
	log.Info("Program started")

	model := "llama2-uncensored"
	prompt := "give me a three sentence poem."

	tokenChan, err := ollama.GetOllamaTokenResponse(model, prompt)
	if err != nil {
		log.Fatal(err)
	}

	sentenceChan := make(chan string)
	go speech.SegmentTextFromChannel(tokenChan, sentenceChan)

	go func() {
		for sentence := range sentenceChan {
			log.Info(sentence)
		}
	}()

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Info("Program exiting")
}
