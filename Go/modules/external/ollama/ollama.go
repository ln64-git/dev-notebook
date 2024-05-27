package ollama

import (
	"bufio"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
)

// OllamaResponse represents the structure of the response received from the Ollama API.
type OllamaResponse struct {
	Model      string `json:"model"`
	CreatedAt  string `json:"created_at"`
	Response   string `json:"response"`
	Done       bool   `json:"done"`
	DoneReason string `json:"done_reason,omitempty"`
}

// GetOllamaTokenResponse generates a response token from the Ollama model based on the provided prompt.
// It returns a channel that streams the response tokens and any encountered error.
//
// Parameters:
//   - model: The name of the Ollama model to use for generation.
//   - prompt: The input prompt to generate the response.
//   - port: The port on which the Ollama API is running.
//
// Returns:
//   - A channel of strings streaming the response tokens.
//   - An error, if any occurs during the request creation or execution.
func GetOllamaTokenResponse(model string, prompt string, port int) (<-chan string, error) {
	// Set default port if not provided
	if port == 0 {
		port = 11434
	}

	// Prepare the request URL and payload
	url := fmt.Sprintf("http://localhost:%d/api/generate", port)
	payload := strings.NewReader(fmt.Sprintf(`{"model": "%s","prompt":"%s"}`, model, prompt))

	// Create a new HTTP POST request
	req, err := http.NewRequest("POST", url, payload)
	if err != nil {
		return nil, err
	}

	// Set the request headers
	req.Header.Add("Content-Type", "application/json")

	// Execute the HTTP request
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}

	// Channel to stream response tokens
	tokenChan := make(chan string)

	// Goroutine to process the response body and stream tokens
	go func() {
		defer res.Body.Close()
		scanner := bufio.NewScanner(res.Body)
		for scanner.Scan() {
			var response OllamaResponse
			if err := json.Unmarshal(scanner.Bytes(), &response); err != nil {
				close(tokenChan)
				return
			}
			tokenChan <- response.Response
			if response.Done {
				break
			}
		}
		close(tokenChan)
	}()

	return tokenChan, nil
}
