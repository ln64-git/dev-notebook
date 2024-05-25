// speech/speech.go
package speech

import (
	"fmt"

	"github.com/dev/go/modules/external/azure"
	"github.com/dev/go/modules/internal/audio"
)

type SpeechRequest struct {
	Text      string `json:"text"`
	Gender    string `json:"gender"`
	VoiceName string `json:"voiceName"`
}

func (r SpeechRequest) SpeechRequestToJSON() string {
	return fmt.Sprintf(`{"text":"%s","gender":"%s","voiceName":"%s"}`, r.Text, r.Gender, r.VoiceName)
}

func ProcessSpeech(req SpeechRequest, azureSubscriptionKey, azureRegion string, audioPlayer *audio.AudioPlayer) error {
	sentences := parseBreathFromText(req.Text)

	for _, sentence := range sentences {
		audioData, err := azure.SynthesizeSpeech(azureSubscriptionKey, azureRegion, sentence, req.Gender, req.VoiceName)
		if err != nil {
			fmt.Printf("Failed to synthesize speech: %v", err)
			return err
		}

		audioPlayer.Play(audioData)
	}

	fmt.Printf("Speech synthesized and added to the queue")
	return nil
}

func parseBreathFromText(text string) []string {
	var sentences []string
	var currentSentence string

	for i, char := range text {
		if char == ',' || char == '.' || char == '!' || char == '?' {
			sentences = append(sentences, currentSentence)
			currentSentence = ""
			fmt.Printf("Parsed sentence: %s", sentences[len(sentences)-1])
		} else {
			currentSentence += string(char)
			if i == len(text)-1 {
				sentences = append(sentences, currentSentence)
				fmt.Printf("Parsed sentence: %s", currentSentence)
			}
		}
	}

	return sentences
}

func SanitizeInput(input string) (string, error) {
	var req SpeechRequest

	fmt.Printf("text: %s", req.Text)
	return req.Text, nil
}
