class Manager:
    def __init__(self):
        self.name = "Manager"

    def assign_task(self, task, developer):
        developer.add_task(task)
        print(f"{self.name} assigns task '{task.name}' to {developer.name}")

    def review_progress(self, developer):
        progress = developer.get_progress()
        print(f"{self.name} reviews progress of {developer.name}: {progress}")

    def communicate(self, assistant, message):
        assistant.receive_message(message, self.name)