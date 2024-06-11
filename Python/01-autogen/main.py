import os
import autogen
from autogen import UserProxyAgent
from agents.assistant import create_assistant
from config.config import get_config

from agents_test.manager import create_manager_agent
from agents_test.architect import create_software_architect_agent
from agents_test.dev_senior import create_senior_developer_agent
from agents_test.dev_junior import create_junior_developer_agent

def main():
    config = get_config()

    assistant = create_assistant(config)

    # manager = create_manager_agent(config)
    # architect = create_software_architect_agent(config)
    # dev_senior = create_senior_developer_agent(config)
    # dev_junior = create_junior_developer_agent(config)

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