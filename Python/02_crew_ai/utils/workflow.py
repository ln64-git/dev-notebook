from agents.manager import Manager
from agents.assistant import Assistant
from agents.dev_senior import SeniorDeveloper
from agents.dev_junior import JuniorDeveloper
from agents.architect import Architect
from utils.tasks import Task
from utils.ollama import generate_response

def run_workflow(prompt=None):
    # Create instances of each agent
    manager = Manager()
    assistant = Assistant()
    senior_dev = SeniorDeveloper()
    junior_dev = JuniorDeveloper()
    architect = Architect()

    # Define tasks with priorities, deadlines, and descriptions
    task1 = Task('Implement database schema', 1, '2024-07-01', 'Design and implement the database schema.')
    task2 = Task('Develop API endpoints', 2, '2024-07-15', 'Develop RESTful API endpoints for the backend.')

    # Manager assigns tasks to developers
    manager.assign_task(task1, senior_dev)
    manager.assign_task(task2, junior_dev)

    # Developers work on tasks
    senior_dev.update_task('Implement database schema', 'In Progress')
    junior_dev.update_task('Develop API endpoints', 'In Progress')

    # Assistant schedules meetings
    assistant.schedule_meeting("10:00 AM")

    # Architect designs system
    architect.design_system()

    # Developers complete tasks
    senior_dev.update_task('Implement database schema', 'Completed')
    junior_dev.update_task('Develop API endpoints', 'Completed')

    # Manager reviews progress
    manager.review_progress(senior_dev)
    manager.review_progress(junior_dev)

    # Communication
    manager.communicate(assistant, "Prepare a progress report")

    # Assistant compiles and sends the report
    report = assistant.compile_report([senior_dev, junior_dev])
    assistant.send_reminder(report, "Team")

    # Assistant compiles and sends a detailed report
    detailed_report = assistant.detailed_report([senior_dev, junior_dev])
    assistant.send_reminder(detailed_report, "Management")

    if prompt:
        # Use the prompt here
        print(f"Received prompt: {prompt}")
        response = generate_response(prompt)
        print(f"Ollama response: {response}")