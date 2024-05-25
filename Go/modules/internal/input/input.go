package input

import (
	"encoding/json"

	"github.com/dev/go/modules/internal/log"
	"github.com/dev/go/modules/internal/speech"
)

func SanitizeInput(requestBody string) (string, error) {
	var req speech.SpeechRequest
	err := json.Unmarshal([]byte(requestBody), &req)
	if err != nil {
		return "", err
	}

	log.Logger.Printf("text: %s", req.Text)
	return req.Text, nil
}
