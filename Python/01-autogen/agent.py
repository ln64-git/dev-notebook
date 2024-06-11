import os
import autogen
from autogen import AssistantAgent, UserProxyAgent

llm_config = {
    "config_list": [
        {
            "model": "llama3",
            "base_url": "http://localhost:11434/v1",
            "api_key": "ollama",
        }
    ]
}

assistant = AssistantAgent("assistant", llm_config=llm_config)
user_proxy = UserProxyAgent(
    "user_proxy",
    code_execution_config={"executor": autogen.coding.LocalCommandLineCodeExecutor(work_dir="coding")},
)

# Start the chat
user_proxy.initiate_chat(
    assistant,
    message="Plot a chart of NVDA and TESLA stock price change YTD.",
)