import os
import autogen
from autogen import AssistantAgent

def create_assistant(config):
    llm_config = {
        "config_list": [
            {
                "model": config.get("model_name", "llama3"),
                "base_url": config.get("base_url", "http://localhost:11434/v1"),
                "api_key": config.get("api_key", "ollama"),
            }
        ]
    }
    return AssistantAgent("assistant", llm_config=llm_config)