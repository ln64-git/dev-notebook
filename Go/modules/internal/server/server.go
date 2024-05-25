package server

import (
	"fmt"
	"net"
	"net/http"
	"strconv"
	"time"

	"github.com/dev/go/modules/internal/types"
)

func StartServer(state types.AppState) {
	port := state.Port
	fmt.Printf("Starting server on port %d", port)
	http.HandleFunc("/status", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		w.WriteHeader(http.StatusOK)
		fmt.Fprint(w, "Server is running")
	})

	addr := ":" + strconv.Itoa(port)
	err := http.ListenAndServe(addr, nil)
	if err != nil {
		fmt.Printf("Failed to start server: %v", err)
	}
}

func CheckServerRunning(port int) bool {
	timeout := 2 * time.Second
	conn, err := net.DialTimeout("tcp", fmt.Sprintf(":%d", port), timeout)
	if err != nil {
		return false
	}
	conn.Close()
	return true
}

func ConnectToServer(port int) {
	fmt.Printf("Connected to the server on port %d.\n", port)
	resp, err := http.Get(fmt.Sprintf("http://localhost:%d/status", port))
	if err != nil {
		fmt.Printf("Failed to connect to the server: %v\n", err)
		return
	}
	defer resp.Body.Close()
	fmt.Printf("Server response: %s\n", resp.Status)
}
