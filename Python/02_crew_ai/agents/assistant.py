class Assistant:
    def __init__(self):
        self.name = "Assistant"
        self.messages = []

    def schedule_meeting(self, time):
        print(f"{self.name} schedules a meeting at {time}")

    def send_reminder(self, message, recipient):
        print(f"{self.name} sends reminder to {recipient}: {message}")

    def receive_message(self, message, sender):
        self.messages.append((message, sender))
        print(f"{self.name} receives message from {sender}: {message}")

    def compile_report(self, developers):
        report = "Progress Report:\n"
        for developer in developers:
            progress = developer.get_progress()
            report += f"- {developer.name}: {progress}\n"
        print(f"{self.name} compiles the report:\n{report}")
        return report

    def detailed_report(self, developers):
        report = "Detailed Progress Report:\n"
        for developer in developers:
            tasks = developer.get_tasks()
            report += f"- {developer.name}:\n"
            for task in tasks:
                report += f" - {task.name}: {task.status} (Priority: {task.priority}, Deadline: {task.deadline}, Description: {task.description})\n"
        print(f"{self.name} compiles the detailed report:\n{report}")
        return report