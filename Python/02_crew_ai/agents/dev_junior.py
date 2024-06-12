class JuniorDeveloper:
    def __init__(self):
        self.name = "Junior Developer"
        self.tasks = []

    def add_task(self, task):
        self.tasks.append(task)
        self.tasks.sort(key=lambda x: x.priority, reverse=True)
        print(f"{self.name} adds task '{task.name}' with priority {task.priority}")

    def update_task(self, task_name, status):
        for task in self.tasks:
            if task.name == task_name:
                task.status = status
                print(f"{self.name} updates task '{task_name}' to '{status}'")
                return
        print(f"{self.name} cannot find task '{task_name}' to update")

    def get_progress(self):
        completed_tasks = [task for task in self.tasks if task.status == "Completed"]
        return f"Completed {len(completed_tasks)} out of {len(self.tasks)} tasks"

    def get_tasks(self):
        return self.tasks