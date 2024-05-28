// speech/speech.go
package speech

import (
	"fmt"
	"log"
	"strings"

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

func ProcessSpeech(logger *log.Logger, req SpeechRequest, azureSubscriptionKey, azureRegion string, audioPlayer *audio.AudioPlayer) error {
	segments := SegmentedText(req.Text)
	for _, segment := range segments {
		audioData, err := azure.SynthesizeSpeech(azureSubscriptionKey, azureRegion, segment, req.Gender, req.VoiceName)
		if err != nil {
			return err
		}
		audioPlayer.Play(audioData)
	}
	return nil
}

func SegmentTextFromChannel(tokenChan <-chan string, sentenceChan chan<- string) {
	defer close(sentenceChan)
	var builder strings.Builder

	for token := range tokenChan {
		builder.WriteString(token)
		if strings.ContainsAny(token, ",.!?") {
			sentence := builder.String()
			sentenceChan <- sentence
			builder.Reset()
		}
	}
}

func SegmentedText(text string) []string {
	var sentences []string
	var currentSentence string

	for i, char := range text {
		if char == ',' || char == '.' || char == '!' || char == '?' {
			sentences = append(sentences, currentSentence)
			currentSentence = ""
		} else {
			currentSentence += string(char)
			if i == len(text)-1 {
				sentences = append(sentences, currentSentence)
			}
		}
	}

	return sentences
}
