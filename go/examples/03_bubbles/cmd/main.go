package main

import (
	"fmt"
	"math/rand"
	"os"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

var docStyle = lipgloss.NewStyle().Margin(1, 2)

type item struct {
	title, desc string
}

func (i item) Title() string       { return i.title }
func (i item) Description() string { return i.desc }
func (i item) FilterValue() string { return i.title }

type model struct {
	list list.Model
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		if msg.String() == "ctrl+c" {
			return m, tea.Quit
		}
	case tea.WindowSizeMsg:
		h, v := docStyle.GetFrameSize()
		m.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	m.list, cmd = m.list.Update(msg)
	return m, cmd
}

func (m model) View() string {
	return docStyle.Render(m.list.View())
}

var quotes = []string{
	"\"Natura nihil agit frustra\" - Aristotle",
	"\"Deus ex machina\" - Horace",
	"\"Omnia mutantur, nihil interit\" - Ovid",
	"\"Ex nihilo nihil fit\" - Parmenides",
	"\"Homo homini deus est\" - Plautus",
}

func main() {
	items := []list.Item{
		item{title: "Input text", desc: quotes[rand.Intn(len(quotes))]},
		// item{title: "Speaker Name", desc: "It's good on toast"},
		// item{title: "Speaker Name", desc: "It's good on toast"},
		// item{title: "Speaker Gender", desc: "It's good on toast"},
		// item{title: "Azure Region", desc: "It cools you down"},
		item{title: "Speech Settings", desc: "more speech settings"}}

	m := model{list: list.New(items, list.NewDefaultDelegate(), 0, 0)}
	m.list.Title = "My Fave Things"

	p := tea.NewProgram(m, tea.WithAltScreen())

	if _, err := p.Run(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}
