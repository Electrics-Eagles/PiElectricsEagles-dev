"""
Simple tool that can be run the any file from the autoload
Writen by Alex Zaslavskis

"""



import os
import json


def read_config():
    with open("run.json", "r") as write_file:
        data = json.load(write_file)
        return (data["command"]),(data["name"])

def main():
    command,name = read_config()
    print("Running the: "+str(name))
    os.system(command)
    print("Finished the: "+str(name))
pass


main()
