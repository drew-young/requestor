import requests
import time
import threading

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
    time.sleep(5)
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
    SERVER_IP = "https://localhost:443"
    TEAMS = int() #number of teams
    HOSTS = list() #list of hostnames
    for i in range(TEAMS):
        pass

#todo function that checks to see if all hosts are active and prints out which hosts are down

def mainLoop():
    command = input("\nEnter command: ")
    #List all teams with number of active hosts
    #Ask user which team they want to choose
    #Select that team and ask the user what host they want to choose
    #Ask the user what they want to do with that host (check in, run command async, shell, etc.)
    #Allow user to change sleep time
    #Allow user to mass change sleep time
    pass

def main():
    init()
    while True:
        command = input("\nEnter command: ")
        if command == "checkIn":
            print(getCheckInTimes())
            continue
        t = threading.Thread(target=sendAndReceive, args=("unknown.unknown1",command))
        t.start()

if __name__ == "__main__":
    main()