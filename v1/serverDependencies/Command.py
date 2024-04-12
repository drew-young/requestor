class Command:
    def __init__(self,id,command):
        self.id = id
        self.command = command
        self.response = ""
        self.acknowledged = False

    def __repr__(self):
        return f"{{\"cmd_id\":\"{self.id}\",\"command\": \"{self.command}\"}}"