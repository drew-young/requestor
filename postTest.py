import requests
#res = requests.post('http://localhost:8080/hosts/newHost', json={"IP":"127.0.0.1","OS":"Windows"})
res = requests.post('http://localhost:8080/hosts/newHost', json={"IP":"127.0.0.1","OS":"Windows"})
if res.ok:
	print(res.text)
