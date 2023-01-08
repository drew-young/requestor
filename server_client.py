import requests
import time
import threading

def addCommand(host, command):
    """
    Adds command to server for a given host and returns command ID
    """
    try:
        res = requests.post(f'{SERVER_IP}/hosts/{host}/addCommand', json={"command":command})
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
    try:
        res = requests.post(f'{SERVER_IP}/hosts/{host}/responses', json={"cmd_id":cmd_id})
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
    time.sleep(5)
    print(f"\nHost: {host} Command: {command} Response: {getResponse(host, cmd_id)}")

def init():
    """
    Sets variables for server IP, list of hosts, and number of teams
    """
    global SERVER_IP
    SERVER_IP = "http://localhost:8080"
    TEAMS = int() #number of teams
    HOSTS = list() #list of hostnames
    for i in range(TEAMS):
        pass

#todo function that checks to see if all hosts are active and prints out which hosts are down

def main():
    init()
    while True:
        command = input("\nEnter command: ")
        t = threading.Thread(target=sendAndReceive, args=("unknown.unknown1",command))
        t.start()

if __name__ == "__main__":
    main()