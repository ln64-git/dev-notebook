package model

import (
	"flag"
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
)

type Model struct {
	port      int
	serve     bool
	userInput string
}

func InitialModel() Model {
	port := flag.Int("port", 8080, "Port to listen on")
	serve := flag.Bool("serve", false, "Start the server")
	userInput := flag.String("play", "", "User input for playing")

	flag.Parse()

	return Model{
		port:      *port,
		serve:     *serve,
		userInput: *userInput,
	}
}

func (m Model) Init() tea.Cmd {
	return nil
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyCtrlC:
			return m, tea.Quit
		}
	}
	return m, nil
}

func (m Model) View() string {
	s := "Voxctl\n\n"
	s += fmt.Sprintf("Port: %d\n", m.port)
	s += fmt.Sprintf("Serve: %v\n", m.serve)
	s += fmt.Sprintf("User Input: %s\n", m.userInput)
	return s
}
