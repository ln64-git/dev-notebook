package main

import (
	"os"
	"os/signal"
	"syscall"

	"github.com/dev/go/modules/internal/log"
)

func main() {
	// Initialize the logger with default configuration
	cfg := log.DefaultConfig()
	logger, err := log.InitLogger(cfg) // Pass the address of cfg
	if err != nil {
		println("Error initializing logger:", err)
		os.Exit(1)
	}
	logger.Info("Program started")

	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	logger.Info("Program Exiting")
}
