class Hostname:
    def __init__(self, hostname, ipFormat, os):
        self.ipFormat = ipFormat
        self.hostname = hostname
        self.hosts = list()
        self.os = os