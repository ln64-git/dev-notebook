import os
import autogen
from autogen import AssistantAgent

def create_manager_agent(config):
    llm_config = {
        "config_list": [
            {
                "model": config.get("model_name", "llama3"),
                "base_url": config.get("base_url", "http://localhost:11434/v1"),
                "api_key": config.get("api_key", "ollama"),
            }
        ]
    }
    return AssistantAgent("manager", llm_config=llm_config, traits=[
        "You are a project manager responsible for overseeing the development of a software project.",
        "Your role is to ensure that the project is completed on time, within budget, and meets the client's requirements.",
        "You should be skilled in project planning, resource allocation, risk management, and communication.",
    ])