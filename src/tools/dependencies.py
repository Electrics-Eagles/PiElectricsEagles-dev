# Alex Zaslavskis tools
import sys
from os import scandir, walk
import os
class Dependecies:

	def get_params(self):
		if(len(sys.argv)>2):
			print("PARRAMETRESS ARE OKAY")
			path=sys.argv[0]
			params =  sys.argv[1]
			return path,params
	pass


	def scan_folders(self,path):
		for root, dirs, files in os.walk(path, topdown=False):
			for name in files:
				print(os.path.join(root, name))
			for name in dirs:
				print(os.path.join(root, name))
	pass




x = Dependecies()
path,params=x.get_params()
x.scan_folders(path)
print()