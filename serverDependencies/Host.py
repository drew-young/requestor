from serverDependencies.Command import Command

class Host:
    def __init__(self,ip,os,hostname,team,alive=False):
        self.ip = ip
        self.os = os
        self.hostname = hostname
        self.team = team
        self.id = f"{self.hostname}.{self.team}"
        self.commands = dict()
        self.commandCounter = 0
        self.lastCheckIn = ""
        self.alive = alive
        self.addCommand('whoami')
        self.addCommand('pwd')
    
    def __repr__(self):
        return self.id
    
    def addCommand(self,command):
        self.commands[self.commandCounter] = Command(self.commandCounter, command)
        self.commandCounter += 1
        print("Added command successfully.")

    def addResponse(self,cmd_id, resp):
        self.commands[cmd_id].response = resp
    
    def getQueuedCommands(self):
        command_count = 0
        queuedCommands = ""
        for i in range(len(self.commands)):
            if not self.commands[i].response:
                command_count += 1
                if i == len(self.commands) - 1: #if it's the last one, don't include a ,
                    queuedCommands += f'"{i}": {str(self.commands[i])}'
                else:
                    queuedCommands += f'"{i}": {str(self.commands[i])},'
        queuedCommands = "{" + f'"command_count":"{command_count}",' +queuedCommands + "}"
        return str(queuedCommands)