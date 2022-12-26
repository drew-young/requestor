class Host:
    def __init__(self,IP,OS,hostname,team):
        self.IP = IP
        self.OS = OS
        self.hostname = hostname
        self.team = team
        self.id = f"{self.hostname}.{self.team}"
    
    def __repr__(self):
        return self.ID