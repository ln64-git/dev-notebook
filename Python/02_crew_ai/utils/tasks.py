class Task:
    def __init__(self, name, priority, deadline, description):
        self.name = name
        self.priority = priority
        self.status = "Pending"
        self.deadline = deadline
        self.description = description