package types

// State struct to hold program state
type AppState struct {
	Port                 int
	Token                string
	StatusRequested      bool
	QuitRequested        bool
	VoiceGender          string
	VoiceName            string
	ServerAlreadyRunning bool
}
