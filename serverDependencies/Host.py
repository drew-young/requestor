class Host:
    def __init__(self,IP,OS,hostname,team):
        self.IP = IP
        self.OS = OS
        self.hostname = hostname
        self.team = team
        self.id = f"{self.hostname}.{self.team}"
        self.queuedCommands = list()
        self.commandCounter = 0
        self.addCommand('whoami')
    
    def __repr__(self):
        return self.ID
    
    def addCommand(self,command):
        self.queuedCommands.append(f"{str(self.commandCounter)}: {command}")
        self.commandCounter += 1