import requests
import time
import threading

#to ignore warnings about self-signed certificate
from requests.packages.urllib3.exceptions import InsecureRequestWarning
requests.packages.urllib3.disable_warnings(InsecureRequestWarning)

def addCommand(host, command):
    """
    Adds command to server for a given host and returns command ID
    """
    if not command:
        return None
    try:
        res = requests.post(f'{SERVER_IP}/hosts/{host}/addCommand', json={"command":command},verify=False)
        if res.ok:
            return res.text
        else:
            raise Exception
    except:
        print("Server does not appear to be online.\nServer IP: " + SERVER_IP)
        return None

def getResponse(host, cmd_id):
    """
    Gets response from server for a given command ID
    """
    if not cmd_id:
        return None
    try:
        res = requests.post(f'{SERVER_IP}/hosts/{host}/responses', json={"cmd_id":cmd_id},verify=False)
        if res.ok:
            return res.text
        else:
            raise Exception
    except:
        print("Server does not appear to be online.\nServer IP: " + SERVER_IP)
        return None

def sendAndReceive(host, command):
    """
    Sends command to server, waits 5 seconds, then gets response
    """
    cmd_id = addCommand(host, command)
    if not cmd_id:
        return None
    time.sleep(15)
    print(f"\nHost: {host} \n\tCommand: {command} \n\tResponse: {getResponse(host, cmd_id)}")

def getCheckInTimes():
    try:
        res = requests.post(f'{SERVER_IP}/api/getCheckInTimes', json={}, verify=False)
        if res.ok:
            return res.text
        else:
            raise Exception
    except:
        print("Server does not appear to be online.\nServer IP: " + SERVER_IP)
        return None

def init():
    """
    Sets variables for server IP, list of hosts, and number of teams
    """
    global SERVER_IP
    SERVER_IP = "https://129.21.21.74:443"
    server_info = requests.post(f'{SERVER_IP}/api/getServerInfo', json={}, verify=False).json()
    global TEAMS
    TEAMS = int(server_info['teams']) #number of teams
    global HOSTS
    HOSTS = list(server_info['hostnames']) #list of hostnames
    for i in range(TEAMS):
        pass

#todo function that checks to see if all hosts are active and prints out which hosts are down

def selectHostByTeam():
    """
    Allows user to select a host by team number and returns the selected host
    """
    for i in range(TEAMS):
        print(f"Team {i}") #todo add active hosts number
    print("Team unknown")
    while True:
        team = input("Enter team number: ")
        if team.isdigit(): #if the input is a number
            team = int(team)
            if team >= 0 and team < TEAMS: #if the number is in the range of teams, break the loop
                break
        elif team == "unknown":
            return selectUnknownHost()
            return
        elif team == "exit":
            return
    for index,host in enumerate(HOSTS):
        print(f"{index} - {host}")
    while True:
        host = input("Enter host index: ")
        if host.isdigit(): #if the input is a number
            host = int(host)
            if host >= 0 and host < len(HOSTS):
                break
        elif host == "exit":
            return
    selected_host = HOSTS[int(host)] + "." + str(team)
    print(f"Selected host: {selected_host}")
    return selected_host

def selectUnknownHost():
    """
    Allows user to select a host from the unknown team and returns the selected host
    """
    try:
        unknown_hosts = requests.post(f'{SERVER_IP}/api/getUnknownHosts', verify=False).text.split()
    except:
        print("No unknown hosts.")
        return None
    for index,host in enumerate(unknown_hosts):
        print(f"{index} - {host}")
    while True:
        host = input("Enter host index: ")
        if host.isdigit():
            host = int(host)
            if host >= 0 and host < len(unknown_hosts):
                break
        elif host == "exit":
            return
    selected_host = unknown_hosts[int(host)]
    print(f"Selected host: {selected_host}")
    return selected_host

def mainLoop():
    while True:
        print("1. Get check in times")
        print("2. Select host by team")
        print("3. Select host by name (all teams)")
        print("4. Get shell on selected host") #todo
        print("5. Change sleep time for all hosts")
        userIn = input("Enter index of command: ")
        while True:
            if userIn == "1" or userIn == "2" or userIn == "3":
                break
            elif userIn == "exit":
                return
            userIn = input("Invalid input. Enter index of command: ")
        if userIn == "1":
            print(getCheckInTimes())
        elif userIn == "2":
            selected_host = selectHostByTeam()
            if selected_host:
                while True:
                    command = input(f"Enter command for {selected_host} (or exit): ")
                    if command == "exit":
                        break
                    t = threading.Thread(target=sendAndReceive, args=(selected_host,command))
                    t.start()
        elif userIn == "3":
            pass
        else:
            print("Invalid input")

def main():
    print("Welcome to the server client!")
    init()
    mainLoop()
    print("Exiting server client...")

if __name__ == "__main__":
    main()
