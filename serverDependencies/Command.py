class Command:
    def __init__(self,id,command):
        self.id = id
        self.command = command
        self.response = ""

    def __repr__(self):
        return f"{{\"{self.id}\": \"{self.command}\"}}"