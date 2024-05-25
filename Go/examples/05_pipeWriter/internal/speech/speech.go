package speech

import (
	"fmt"
)

type SpeechRequest struct {
	Text      string `json:"text"`
	Gender    string `json:"gender"`
	VoiceName string `json:"voiceName"`
}

func (r SpeechRequest) ToJSON() string {
	return fmt.Sprintf(`{"text":"%s","gender":"%s","voiceName":"%s"}`, r.Text, r.Gender, r.VoiceName)
}
