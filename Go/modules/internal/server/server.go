package server

import (
	"fmt"
	"net"
	"net/http"
	"strconv"
	"time"

	"github.com/dev/go/modules/internal/types"
)

// StartServer starts the HTTP server on the specified port.
func StartServer(state types.ServerState) {
	port := state.Port
	state.Logger.Infof("Starting server on port %d", port)
	http.HandleFunc("/status", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		w.WriteHeader(http.StatusOK)
		state.Logger.Infof("Server is running")
	})

	addr := ":" + strconv.Itoa(port)
	err := http.ListenAndServe(addr, nil)
	if err != nil {
		state.Logger.Info("Failed to start server: %v\n", err)
	}
}

// CheckServerRunning checks if the server is already running on the specified port.
func CheckServerRunning(port int) bool {
	timeout := 2 * time.Second
	conn, err := net.DialTimeout("tcp", fmt.Sprintf(":%d", port), timeout)
	if err != nil {
		return false
	}
	conn.Close()
	return true
}

// ConnectToServer connects to the server on the specified port and returns a boolean indicating success or failure.
func ConnectToServer(port int) bool {
	resp, err := http.Get(fmt.Sprintf("http://localhost:%d/status", port))
	if err != nil {
		fmt.Printf("Failed to connect to the server: %v\n", err)
		return false
	}
	defer resp.Body.Close()
	return resp.StatusCode == http.StatusOK
}
