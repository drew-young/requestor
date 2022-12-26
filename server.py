#Start a web server
#Take POST requests on /newHost with an IP and OS
#Assign host with hostname and return the identifier of the host
#Take GET requests on /hosts/IDENTIFIER/commands and return stored commands
#Take POST requests on /hosts/IDENTIFIER/responses and print responses

#TODO Accept CLI input (API)

from flask import Flask
import requests
import json

app = Flask(__name__)

@app.route("/") #HOMEPAGE
def homePage():
    return "Homepage of C2"

def main():
    app.run(host="127.0.0.1",port="8080")

if __name__ == "__main__":
    main()