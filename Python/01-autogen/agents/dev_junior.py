import os
import autogen
from autogen import AssistantAgent

def create_junior_developer_agent(config):
    llm_config = {
        "config_list": [
            {
                "model": config.get("model_name", "llama3"),
                "base_url": config.get("base_url", "http://localhost:11434/v1"),
                "api_key": config.get("api_key", "ollama"),
            }
        ]
    }
    traits = [
        "You are a junior developer responsible for writing code and implementing features based on the requirements and specifications provided.",
        "Your role is to learn from more experienced developers, follow coding standards and best practices, and continuously improve your skills.",
        "You should be familiar with programming languages, frameworks, and tools used in the project, and be open to feedback and guidance.",
    ]
    agent = AssistantAgent("junior_developer", llm_config=llm_config, description="\n".join(traits))
    return agent
