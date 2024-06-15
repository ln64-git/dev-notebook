from crewai import Agent
from textwrap import dedent
from langchain.llms import Ollama

class CustomAgents:
    def __init__(self):
        self.Ollama = Ollama(model="llama3")

    def architect(self):
        return Agent(
            role="System Architect",
            backstory=dedent("""The Architect is responsible for designing the system architecture."""),
            goal=dedent("""Ensure that the system architecture meets all business and technical requirements."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )

    def assistant(self):
        return Agent(
            role="Administrative Assistant",
            backstory=dedent("""The Assistant manages schedules, reminders, and communications."""),
            goal=dedent("""Streamline administrative tasks and support the team efficiently."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )

    def junior_developer(self):
        return Agent(
            role="Junior Developer",
            backstory=dedent("""The Junior Developer is responsible for implementing tasks assigned by senior developers and managers."""),
            goal=dedent("""Complete assigned tasks efficiently and learn from senior team members."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )

    def senior_developer(self):
        return Agent(
            role="Senior Developer",
            backstory=dedent("""The Senior Developer oversees the development process and mentors junior developers."""),
            goal=dedent("""Ensure high-quality code and timely completion of projects."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )

    def manager(self):
        return Agent(
            role="Project Manager",
            backstory=dedent("""The Manager coordinates between team members and oversees project progress."""),
            goal=dedent("""Ensure the project meets its goals and deadlines."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )

    def qa_engineer(self):
        return Agent(
            role="QA Engineer",
            backstory=dedent("""The QA Engineer is responsible for testing the software to ensure quality and functionality."""),
            goal=dedent("""Identify and report bugs, ensure software quality, and verify that all requirements are met."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )

    def devops_engineer(self):
        return Agent(
            role="DevOps Engineer",
            backstory=dedent("""The DevOps Engineer is responsible for managing deployment and infrastructure."""),
            goal=dedent("""Ensure smooth deployment, maintain infrastructure, and improve CI/CD processes."""),
            allow_delegation=False,
            verbose=True,
            llm=self.Ollama,
        )
