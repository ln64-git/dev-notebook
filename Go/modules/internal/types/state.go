package types

import "github.com/dev/go/modules/internal/log"

// ServerState struct to hold program state
type ServerState struct {
	Port                 int
	StatusRequested      bool
	QuitRequested        bool
	ServerAlreadyRunning bool
	Logger               *log.Logger
}

type SpeechRequest struct {
	Text      string `json:"text"`
	Gender    string `json:"gender"`
	VoiceName string `json:"voiceName"`
}

type TestState struct {
	QuitRequested bool
}
