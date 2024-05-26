package types

// ServerState struct to hold program state
type ServerState struct {
	Port                 int
	StatusRequested      bool
	QuitRequested        bool
	ServerAlreadyRunning bool
}

type SpeechRequest struct {
	Text      string `json:"text"`
	Gender    string `json:"gender"`
	VoiceName string `json:"voiceName"`
}

type TestState struct {
	QuitRequested bool
}
