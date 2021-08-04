FILE_LOCATION="/home/programmer/GitHub/PiElectricsEagles/dev/target/arm-unknown-linux-musleabi/debug/pielectricseagles"
DESTINATION_LOCATION="/mnt/pimylifeupshare/drone_code"
BUILD_COMMAND="./build.sh"
SSH_USERNAME="root"
SSH_PASSWORD="1234"
SSH_HOST="raspberrypi.local"
RUN_BIN="/home/pi/shared/drone_code"
SSH_PORT=22

import paramiko
import os 
import sys
from shutil import copyfile
import subprocess

def mount():
	print("if the samba folder is not mounter do it:")
	print('sudo mount -t cifs -o "domain=WORKGROUP,username=pi,password=raspberry,sec=ntlm,vers=1.0" //raspberrypi.local/pimylifeupshare /mnt/pimylifeupshare')
pass

def copy_file():
	copyfile(FILE_LOCATION, DESTINATION_LOCATION)
pass

def build():
	output = os.system(BUILD_COMMAND)
	print(output)
pass


def run_build():
	build()
	mount()
	copy_file()
	run_bin()
pass


def run_bin():
	print("SSH .......")
	ssh=paramiko.SSHClient()
	ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
	ssh.connect(SSH_HOST,SSH_PORT,SSH_USERNAME,SSH_PASSWORD)

	stdin,stdout,stderr=ssh.exec_command(RUN_BIN)
	outlines=stdout.readlines()
	resp=''.join(outlines)
	print(resp)

	print("SSH .......OK")

pass


run_build()


