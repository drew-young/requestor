class Hostname:
    def __init__(self, hostname, ipFormat, os):
        self.ipFormat = ipFormat
        self.hostname = hostname
        self.expectedHosts = list()
        self.activeHosts = list()
        self.os = os