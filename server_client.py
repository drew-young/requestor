import requests

def main():
    while True:
        command = input("Enter command: ")
        res = requests.post('http://localhost:8080/hosts/unknown.unknown1/addCommand', json={"command":command})
        if res.ok:
            print(res.text)

if __name__ == "__main__":
    main()