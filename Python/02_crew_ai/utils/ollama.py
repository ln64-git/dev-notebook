import anthropic

def generate_response(prompt):
    response = ollama.generate_text(prompt)
    return response

# Load the Ollama model globally
ollama = anthropic.LanguageModel.from_path("ollama")