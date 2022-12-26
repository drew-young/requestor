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
    return HOSTS[identifier].queuedCommands

@app.route("/hosts/<identifier>/responses", methods=["POST"])
def getResponse(identifier):
    """
    Accepts a POST request for the client to send responses to
    """
    data = request.get_json()
    cmd_id = data["cmd_id"]
    response = data["response"]
    HOSTS[identifier].addResponse(cmd_id,response)
    print(f"[SERVER] Host ({identifier}) - {cmd_id}: {response}")
    return f"{identifier} - Thanks!"

@app.route("/hosts/newHost", methods=["POST"])
def newHost():
    """
    Takes in data from a POST request and returns an identifier.
    """
    data = request.get_json()
    IP = data["IP"]
    OS = data["OS"]
    hostname,team = getInfoByIP(IP)
    expectedHostname = hostname + "." + team
    if expectedHostname in HOSTS:
        return f"{expectedHostname}"
    host = Host(IP,OS,hostname,team)
    if hostname != "unknown":
        HOSTNAMES[hostname].activeHosts.append(host)
    TEAMS[team].activeHosts.append(host)
    if hostname == "unknown" and team == "unknown":
        global UNKNOWN_COUNT
        UNKNOWN_COUNT += 1
        host.id += str(UNKNOWN_COUNT)
    HOSTS[host.id] = host
    return f"{host.id}"

def getInfoByIP(IP):
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
            print("[SERVER] Created team: " + str(i))
        TEAMS["unknown"] = Team("unknown") #unknown team for clients that don't fit config
        print("[SERVER] Created team: " + "unknown")
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
    HOSTNAMES[hostname] = Hostname(hostname,ip,os)
    for i in range(1,NUM_OF_TEAMS):
        team = str(i)
        expectedHostIP = ip.replace("x",team)
        newHost = Host(expectedHostIP, os, hostname, team)
        HOSTNAMES[hostname].hosts.append(newHost)
        TEAMS[team].hosts.append(newHost)
        HOSTS[newHost.id] = newHost
        print(f"[SERVER] Added host: {newHost} to TEAM {team}")
        print(f"[SERVER] Added host: {newHost} to HOSTNAME {hostname}")

def main():
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
        command = input("Enter a command: ")
        HOSTS["Ubuntu1.1"].addCommand(command)

if __name__ == "__main__":
    main()