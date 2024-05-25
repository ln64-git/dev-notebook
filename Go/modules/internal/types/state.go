package types

// State struct to hold program state
type AppState struct {
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
