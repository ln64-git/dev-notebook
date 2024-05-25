package log

import (
	"os"
	"path/filepath"
	"time"

	charmlog "github.com/charmbracelet/log"
)

type Logger struct {
	*charmlog.Logger
}

// Config defines the structure for log configuration settings.
type Config struct {
	LogDir  string // Directory to save log files
	LogFile string // Log file name
}

// InitLogger initializes and returns a logger with the given configuration.
func InitLogger(cfg Config) (*Logger, error) { // Ensure the log directory exists.
	if err := os.MkdirAll(cfg.LogDir, 0755); err != nil {
		return nil, err
	}

	// Create the full path to the log file.
	logFilePath := filepath.Join(cfg.LogDir, cfg.LogFile)
	logFile, err := os.OpenFile(logFilePath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return nil, err
	}

	logger := &Logger{Logger: charmlog.NewWithOptions(logFile, charmlog.Options{
		ReportCaller:    true,
		ReportTimestamp: true,
		TimeFormat:      time.DateTime,
	})}

	return logger, nil
}

// DefaultConfig provides a default logging configuration.
func DefaultConfig() Config {
	return Config{
		LogDir:  "logs",
		LogFile: "app.log",
	}
}
