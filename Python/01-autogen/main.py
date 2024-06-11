import os
import autogen
from autogen import UserProxyAgent
from config.config import get_config

from agents.manager import create_manager_agent
from agents.architect import create_software_architect_agent
from agents.dev_senior import create_senior_developer_agent
from agents.dev_junior import create_junior_developer_agent
from agents.assistant import create_assistant

def main():
    config = get_config()
    
    assistant = create_assistant(config)
    
    manager = create_manager_agent(config)
    architect = create_software_architect_agent(config)
    dev_senior = create_senior_developer_agent(config)
    dev_junior = create_junior_developer_agent(config)

    user_proxy = UserProxyAgent(
        "user_proxy",
        code_execution_config={"executor": autogen.coding.LocalCommandLineCodeExecutor(work_dir="coding")},
    )

    # Start the chat
    user_proxy.initiate_chat(
        assistant,
        message="Build me an application that does x.",
    )

    # Delegate tasks to agents
    manager.initiate_chat(
        architect,
        message="Design the overall structure and architecture for the application."
    )

    architect.initiate_chat(
        dev_senior,
        message="Create the core components and frameworks for the application based on the architecture."
    )

    dev_senior.initiate_chat(
        dev_junior,
        message="Implement the features and functionalities as per the specifications."
    )

    dev_junior.initiate_chat(
        assistant,
        message="Assist in integrating the implemented features and perform initial testing."
    )

    # Final check and coordination
    user_proxy.initiate_chat(
        manager,
        message="Ensure the application meets the client's requirements and is ready for deployment."
    )

if __name__ == "__main__":
    main()
