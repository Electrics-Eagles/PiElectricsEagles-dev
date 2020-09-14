PATH_TO_SERVE="./"
PORT = 4542

import http.server
import socketserver
import os
import webbrowser

def web_server():
	
	Handler = http.server.SimpleHTTPRequestHandler
	with socketserver.TCPServer(("", PORT), Handler) as httpd:
		print("serving at port", PORT)
		httpd.serve_forever()
pass


def build_via_cargo_docs():
	os.chdir(PATH_TO_SERVE)
	if PATH_TO_SERVE=="./":
		print("YOU WHAT REMOVE SYSTEM NOT BE IDIOT")
	else:
		os.system("rm -rf "+PATH_TO_SERVE)
	os.system("cargo doc")	
pass


def make_server():
	os.chdir(PATH_TO_SERVE)
	web_server()

pass


def main():
	print("WELCOME TO DOCS SERVER")
	build_via_cargo_docs()
	webbrowser.open_new_tab("http://localhost:"+str(PORT))
	make_server()

pass

main()


