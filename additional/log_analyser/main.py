from PySide2 import QtWidgets
from gui import Ui_MainWindow
import sys
import math
import easygui
from parser import parser_of,plot_graphs,analyse_data,fft_producing,add_graph,convert_to_pdf
import os 
import time 
from notifypy import Notify
import random 

class Analyser(QtWidgets.QMainWindow, Ui_MainWindow): 

	def job_done(self):
		notification = Notify()
		notification.title = "Job Done"
		notification.message = "Work completed dude... Enjoy..."
		notification.send()


	def __init__(self):
		super().__init__()
		# Создание формы и Ui (наш дизайн)
		self.setupUi(self)
		self.show()
		self.pushButton.clicked.connect(self.import_data)
		self.pushButton_2.clicked.connect(self.set_input_path)
		self.pushButton_3.clicked.connect(self.set_output_path)
		self.pushButton_4.clicked.connect(self.open_dir)
	def set_input_path(self):
		global in_path
		in_path = easygui.fileopenbox()
		self.label.setText(in_path)
	pass
	def set_output_path(self):
		global out_path
		out_path = easygui.diropenbox()
		self.label_2.setText(out_path)
	pass

	def open_dir(self):
		print("Windows only :(")
		#os.startfile(out_path)
		print("hahahah Linux as well here :00))")
		os.system('xdg-open "%s"' % out_path)


	def import_data(self):
		name=self.lineEdit.text()
		print("GYRO NAME:"+name)
		os.mkdir(name)
		data=parser_of(open(in_path).readlines())
		begin=open("./config/final_page.html").readlines()
		s=open("./config/header.html").readlines()
		val=open("./config/method.html").readlines()
		os.chdir(out_path+"/"+name)
		self.progressBar.setValue(10)
		plot_graphs(out_path,name,data)
		self.progressBar.setValue(20)
		fft_producing(out_path,name,data)
		self.progressBar.setValue(60)
		analysed=analyse_data(name,data)
		self.progressBar.setValue(70)
		#''.join(map(str, begin)).join(''.join(map(str, s))).join(''.join(map(str, val))).join(analysed)
		html=open("index.html","a")
		html.write(''.join(map(str, begin)))
		html.write(''.join(map(str, s)))
		html.write(''.join(map(str, val)))
		html.write(''.join(map(str, analysed)))
		html.write(add_graph(name))
		self.progressBar.setValue(80)
		
		port=random.randint(0,65364)
		os.system(f"python3 -m http.server "+str(port) +"&")
		os.system(f"firefox http://0.0.0.0:"+str(port)+"/" +"&")
		print("Done !!")
		
		self.progressBar.setValue(100)
		self.job_done()

	pass



if __name__ == '__main__':
    app = QtWidgets.QApplication(sys.argv)
    calc = Analyser()
    sys.exit(app.exec_())
