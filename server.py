#Start a web server
#Take POST requests on /newHost with an IP and OS
#Assign host with hostname and return the identifier of the host
#Take GET requests on /hosts/IDENTIFIER/commands and return stored commands
#Take POST requests on /hosts/IDENTIFIER/responses and print responses

#TODO Accept CLI input (API)

from flask import Flask, request
import requests
import json
import threading

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

def newHost(data):
    """
    Creates a host object, stores it, and returns an identifier.

    :param data Data received from POST request of new host
    :return Identifier of host
    """
    pass

@app.route("/hosts/")
def hostsPage():
    """
    Returns a list of current hosts
    """
    return "Host1\nHost2\nHost3"

@app.route("/hosts/<identifier>/commands")
def getCommands(identifier):
    return "Command1"

@app.route("/hosts/<identifier>/responses", methods=["POST"])
def getResponse(identifier):
    return "Thanks!"

def main():
    website = threading.Thread(target=runApp)
    website.daemon = True
    website.start()
    while 1:
        pass

if __name__ == "__main__":
    main()