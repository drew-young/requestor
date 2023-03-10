from serverDependencies.Command import Command
from datetime import datetime

class Host:
    def __init__(self,ip,os,hostname,team,alive=False):
        self.ip = ip
        self.os = os
        self.hostname = hostname
        self.team = team
        self.id = f"{self.hostname}.{self.team}"
        self.commands = dict()
        self.commandCounter = 0
        self.lastCheckIn = None
        self.alive = alive
        self.addCommand('checkIn') #Command 0 is always "checkIn" and has no response
    
    def __repr__(self):
        return self.id
    
    def addCommand(self,command):
        """
        Adds a command to the hosts history of commands.
        """
        self.commands[self.commandCounter] = Command(self.commandCounter, command)
        self.commandCounter += 1
        return str(self.commandCounter - 1)

    def addResponse(self,cmd_id, resp):
        """
        Adds a response to a command.
        """
        self.commands[int(cmd_id)].response = resp
    
    def getQueuedCommands(self):
        """
        Returns a JSON formatted string of the count of commands, and commands without responses.
        """
        command_count = 0
        queuedCommands = ""
        for i in range(len(self.commands)):
            if not self.commands[i].acknowledged:
                self.commands[i].acknowledged = True
                command_count += 1
                if i == len(self.commands) - 1: #if it's the last one, don't include a ,
                    queuedCommands += f'"{command_count}": {str(self.commands[i])}'
                else:
                    queuedCommands += f'"{command_count}": {str(self.commands[i])},'
        queuedCommands = queuedCommands.strip(",")
        queuedCommands = "{" + f'"command_count":"{command_count}",' +queuedCommands + "}" if queuedCommands else "{" + f'"command_count":"{command_count}"' + "}"
        return str(queuedCommands)
    
    def getResponses(self):
        return str(self.commands)
    
    def getResponse(self,cmd_id):
        return self.commands[int(cmd_id)].response

    def checkIn(self):
        """
        Updates the last check-in time to now.
        """
        self.alive = True
        self.lastCheckIn = datetime.now()

    def getTimeSinceLastCheckIn(self):
        """
        Returns the time since the last check-in.
        """
        if self.lastCheckIn == None:
            return "Never checked in."
        return (datetime.now() - self.lastCheckIn).total_seconds()