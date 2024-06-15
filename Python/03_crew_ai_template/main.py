import os
import time
import threading
from crewai import Crew
from decouple import config
from textwrap import dedent

from utils.agents import CustomAgents
from utils.tasks import CustomTasks

from langchain.tools import DuckDuckGoSearchRun

search_tool = DuckDuckGoSearchRun()

os.environ["OPENAI_API_KEY"] = config("OPENAI_API_KEY")
os.environ["OPENAI_ORGANIZATION"] = config("OPENAI_ORGANIZATION_ID")

class CustomCrew:
    def __init__(self, app_name, app_description):
        self.app_name = app_name
        self.app_description = app_description

    def run(self):
        try:
            agents = CustomAgents()
            tasks = CustomTasks()

            architect_agent = agents.architect()
            senior_dev_agent = agents.senior_developer()
            junior_dev_agent = agents.junior_developer()
            qa_engineer_agent = agents.qa_engineer()
            devops_engineer_agent = agents.devops_engineer()
            assistant_agent = agents.assistant()

            design_system_task = tasks.design_system(
                architect_agent,
                priority="High",
                deadline="2024-06-30",
                app_name=self.app_name,
                app_description=self.app_description
            )

            implement_feature_task = tasks.implement_feature(
                senior_dev_agent,
                priority="High",
                deadline="2024-06-25",
                feature_name="User Authentication"
            )

            test_feature_task = tasks.test_feature(
                qa_engineer_agent,
                priority="High",
                deadline="2024-06-27",
                feature_name="User Authentication"
            )

            deploy_application_task = tasks.deploy_application(
                devops_engineer_agent,
                priority="High",
                deadline="2024-07-01",
                app_name=self.app_name
            )

            crew = Crew(
                agents=[architect_agent, senior_dev_agent, junior_dev_agent, qa_engineer_agent, devops_engineer_agent, assistant_agent],
                tasks=[design_system_task, implement_feature_task, test_feature_task, deploy_application_task],
                verbose=True,
            )

            result = crew.kickoff()

            # Schedule meeting one minute after the program runs
            meeting_thread = threading.Thread(
                target=self.schedule_meeting_one_minute_later,
                args=(tasks, assistant_agent)
            )
            meeting_thread.start()

            return result

        except Exception as e:
            print(f"An error occurred: {e}")
            return None

    def schedule_meeting_one_minute_later(self, tasks, assistant_agent):
        time.sleep(60)  # Wait for 1 minute
        schedule_meeting_task = tasks.schedule_meeting(
            assistant_agent,
            priority="Medium",
            deadline="2024-06-25",
            time="10:00 AM"
        )
        print("\nMeeting has been scheduled one minute after the program started.")

if __name__ == "__main__":
    print("## Welcome to Crew AI Template")
    print("-------------------------------")
    try:
        app_name = input(dedent("""Enter the application name: """))
        app_description = input(dedent("""Enter the application description: """))

        custom_crew = CustomCrew(app_name, app_description)
        result = custom_crew.run()

        if result:
            print("\n\n########################")
            print("## Here is your custom crew run result:")
            print("########################\n")
            print(result)
        else:
            print("Failed to execute the custom crew run.")

    except Exception as e:
        print(f"An error occurred during input: {e}")
