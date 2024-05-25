package model

type model struct {
	textInput       string
	voiceGender     string
	voiceName       string
	status          string
	err             error
	subscriptionKey string
	region          string
}

func InitialModel() model {
	return model{
		textInput:       "",
		voiceGender:     "Female",
		voiceName:       "en-US-JennyNeural",
		status:          "Ready",
		err:             nil,
		subscriptionKey: "b16408ad75964fc69037d035ac0e4db0",
		region:          "eastus",
	}
}
