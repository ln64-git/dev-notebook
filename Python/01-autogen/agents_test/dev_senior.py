# dev_senior.py
import os
import autogen
from autogen import AssistantAgent

def create_senior_developer_agent(config):
    llm_config = {
        "config_list": [
            {
                "model": config.get("model_name", "llama3"),
                "base_url": config.get("base_url", "http://localhost:11434/v1"),
                "api_key": config.get("api_key", "ollama"),
            }
        ]
    }
    return AssistantAgent("senior_developer", llm_config=llm_config, traits=[
        "You are a senior developer responsible for writing high-quality code, mentoring junior developers, and ensuring the codebase is maintainable and scalable.",
        "Your role is to oversee the implementation of complex features, review code, and provide guidance on best practices and architectural decisions.",
        "You should have a deep understanding of software design principles, coding patterns, and be proficient in multiple programming languages and technologies.",
    ])