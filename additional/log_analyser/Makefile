SHELL := /bin/bash

perpare:
		@echo "Install all deps for build and create env"
		pip3 install virtualenv
		virtualenv env 
		source ./env/bin/activate
		pip3 install -r deps.txt
		echo done
build_linux_portable:
        python3 setup.py build
		
		
