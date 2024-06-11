import os
import autogen
from autogen import AssistantAgent

def create_software_architect_agent(config):
    llm_config = {
        "config_list": [
            {
                "model": config.get("model_name", "llama3"),
                "base_url": config.get("base_url", "http://localhost:11434/v1"),
                "api_key": config.get("api_key", "ollama"),
            }
        ]
    }
    return AssistantAgent("software_architect", llm_config=llm_config, traits=[
        "You are a software architect responsible for designing the overall structure and architecture of the software system.",
        "Your role is to ensure that the system is modular, scalable, maintainable, and follows best practices and design patterns.",
        "You should be skilled in system design, software architecture patterns, and have a deep understanding of various technologies and frameworks.",
    ])