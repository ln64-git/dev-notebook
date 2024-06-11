import os
import autogen
from autogen import UserProxyAgent
from agents.assistant import create_assistant
from config.config import get_config

def main():
    config = get_config()
    assistant = create_assistant(config)

    user_proxy = UserProxyAgent(
        "user_proxy",
        code_execution_config={"executor": autogen.coding.LocalCommandLineCodeExecutor(work_dir="coding")},
    )

    # Start the chat
    user_proxy.initiate_chat(
        assistant,
        message="Plot a chart of NVDA and TESLA stock price change YTD.",
    )

if __name__ == "__main__":
    main()