# Requestor

Requestor is a Command-and-Control (C2) written in Rust that communicates over HTTPS. This tool was made for the RIT Red Team to use during compeitions such as IRSeC, ISTS, UB Lockdown, etc. 

## Prerequisites

To set up this C2, you must have a server that is accessible by a competition host via port 443. This server must have docker installed.

## Setting up the server

The first step is to clone the requestor repository.

```
$ git clone https://github.com/drew-young/requestor.git
```

Following this, navigate to the docker-server directory and use docker compose to start the server.

```
$ docker compose up --build -d
```

Ensure the docker containers are running.
```
$ docker ps
```

## Editing the docker-compose.yaml file

The docker-compose.yaml file contains information regarding the server, database, and reverse proxy. You can edit this file to change variables such as:

- MYSQL_DATABASE: The database created that stores commands, hosts, teams, etc.
- MYSQL_USER: The default user that is created for MySQL.
- MYSQL_PASSWORD: The password for the user created.
- MYSQL_ROOT_PASSWORD: The password for the root account in MySQL.
- DATABASE_URL: The URL of the database (can use Docker hostnames).
- PWNBOARD_URL: The URL of Box Access for PWNBoard.
- INIT_PASSWORD: The password needed to initialize the C2 server with the Database.
- RESET_PASSWORD: The password needed to reset the whole C2 database.
- PWNBOARD_ENABLED: "true" or "false" for if the C2 should post data to PWNBoard.

## Interacting with the C2

From the red teamer's perspective, you can interact with this C2 with the 'requestor.py' tool. This script is located in the repository. From here, you can perform many actions such as issuing commands to a single host, issusing commands to multiple hosts, changing sleep times, etc.

## Deploying the Malware

To deploy Requestor in a competition environment, the following steps must be followed.

Ansible is used to automate the deployment process. 

The first step is to update the inventory.yaml file. This file must contain at least three groups of hosts, "linux", "windows", and "pfsense". Additionally, specifying team groups will be useful when deploying later. Here is an example inventory.yaml file.

```
---
all:
  children:
    pfsense:
      vars:
        ansible_user: admin
        ansible_password: changeme
      children:
        routers:
          hosts:
            192.168.253.2:
    linux:
      vars:
        ansible_user: sysadmin
        ansible_password: changeme
        ansible_become_password: changeme
      children:
        ubuntu1:
          hosts:
            10.[1:14].1.10:
    windows:
      vars:
        ansible_connection: psrp
        ansible_psrp_cert_validation: ignore
        ansible_psrp_protocol: https
        ansible_psrp_auth: ntlm
        ansible_psrp_credssp_auth_mechanism: ntlm
        ansible_become_method: runas
      children:
        ad:
          vars:
            ansible_user: "Administrator"
            ansible_password: "Change.me!"
            ansible_become_user: "Administrator"
            ansible_become_password: "Change.me!"
          hosts:
            10.[1:14].1.60:

    # team groups
    team01:
      vars:
        team_number: '01'
      hosts:
        192.168.253.2:
        10.1.1.10:
        10.1.1.60:
```

After this file is created, the variables must be updated. The variables are located at `ansible/roles/requestor/vars/main.yaml`.

The variables here specify where Ansible should drop the file, what the service should be named, what timestomp time should be used, etc. Below is an example vars.yaml file:

```
pfsense_binary: "/usr/bin/getty" #binary location for pfsense
pfsense_service: "/etc/rc.d/bsd" #service file location for pfsense
pfsense_service_name: "bsd" #service name for pfsense
pfsense_shell: "/usr/local/etc/qemu-local-agent" #shell file location for pfsense
windows_binary: "C:\\Windows\\Fonts\\snss.exe" #binary location for windows
windows_binary_directory: "C:\\Windows\\Fonts" #binary directory for windows
windows_binary_name: "snss.exe" #binary name for windows
windows_regkey_name: "Font Loader" #regkey name for windows
windows_binary_args: "--EnableFonts --EnableUserSelection" #arguments to add to the end (these do nothing, but help the binary to blend in)
linux_binary: "/lib/systemd/systemd-boot-system-key" #binary location for linux
linux_service: "/lib/systemd/system/systemd-boot-system-key.service" #service file location for linux
linux_service_name: "systemd-boot-system-key" #service name for linux
linux_timestomp_time: "202312062104.01" #linux timestomp
pfsense_timestomp_time: "202312062104.01" #router timestomp
```

Once this has been updated, the playbook `deploy.yaml` can be executed to drop the agent.

***

# Compiling

The server and client for this C2 are written in Rust. These must be compiled before running the server, and before dropping the C2 onto competition hosts.

## Compiling the Server

Compile the server with Rust.

## Compiling the Client

Compile the client with Rust.

## Authors and acknowledgment

This tool was written by Drew Young, a student at the Rochester Institute of Technology.
