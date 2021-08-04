import matplotlib.pyplot as plt
import matplotlib.pyplot as graph
import os 
from os.path import abspath
import statistics
import scipy as sy
import scipy.fftpack as syfp
import numpy as np
import weasyprint
from datetime import datetime
import datetime
names=['Gyroscope X axis','Gyroscope Y axis','Gyroscope Z axis','Accelerometer X axis','Accelerometer Y axis','Accelerometer Z axis']


def parser_of(data):
	x=[]
	y=[]
	z=[]
	a_x=[]
	a_y=[]
	a_z=[]
	a_v=[]
	for a in range(1,len(data)):
		data_values=data[a].split("::")
		x.append(float(data_values[1]))
		y.append(float(data_values[2]))
		z.append(float(data_values[3]))
		a_x.append(float(data_values[4]))
		a_y.append(float(data_values[5]))
		a_z.append(float(data_values[6]))
		a_v.append(float(a))
		data_={
		'x':x,
		'y':y,
		'z':z,
		'a_x':a_x,
		'a_y':a_y,
		'a_z':a_z,
		}
	return data_

def analyse_data(gyroname,data):
	x=0
	report_html="<ul>"
	for i in data:
		report_html+="<li>"+("Max value in : "+ names[x] +": "+str(min(data[i])))+"</li>"
		report_html+="<li>"+("Min value in : "+ names[x]+": "+str(max(data[i])))+"</li>"
		report_html+="<li>"+("Diff value in : " +names[x]+": "+str(max(data[i])-min(data[i])))+"</li>"
		try:
			report_html+="<li>"+("Min value in : "+names[x] +": "+str(statistics.mode(data[i])))+"</li>"
			report_html+="<li>"+("Min value in : "+ names[x]+": "+str(statistics.median(data[i])))+"</li>"
		except ValueError:
			print("Oops!  That was no valid number.  Try again...")
		x=x+1
	print("done")
	return report_html	

def fft_producing(path,gyroname,data):
	c=0
	for f in data:
		plt.cla()
		plt.clf()
		graph.clf()
		length=len(data[f])
		# Create time data for x axis based on array length
		x = sy.linspace(0.01, length*0.01, num=length)
		# Do FFT analysis of array
		FFT = sy.fft(data[f])
		# Getting the related frequencies
		freqs = syfp.fftfreq(len(data[f]), d=(x[1]-x[0]))
		plt.title(names[c]+" graph.")
		plt.xlabel('Frequency (Hz)')
		plt.ylabel('FFT (amplitude)')
		graph.plot(freqs[:len(freqs)//2], sy.log10(FFT[:len(FFT)//2]), 'x')
		graph.savefig(path+"/"+names[c]+"_fft.png")
		c=c+1
pass

def add_graph(gyroname):
	index_val=""
	array=os.listdir()
	for a in array:
		print(a)
		index_val+=(f'<img src="{a}"').format(a)+"</img>"
	return index_val

def convert_to_pdf(path,gyroname,index):
	print(os.listdir())
	filename="Vibro Report_"+str(datetime.datetime.now())+"_"+gyroname+".pdf"

	pdfkit.from_url(path+"/"+'tmp.html', filename)

pass


def plot_graphs(path,gyroname,data):
	a=0
	for f in data:
		path_to_save=abspath(names[a]+".png")
		plt.plot(data[f], label=names[a])
		plt.xlabel('Interrations (each one is 10ms)')
		plt.ylabel('Value (can be in Raw or in radians')
		plt.title(names[a]+" graph.")
		plt.legend()
		plt.savefig(path_to_save)
		plt.cla()
		plt.clf()
		a=a+1

	b=0
	for f in data:
		print(names[b])	
		path_to_save=abspath(path+"/"+names[b]+".png")
		plt.plot(data[f], label=names[b])
		plt.xlabel('Interrations (each one is 10ms)')
		plt.ylabel('Value (can be in Raw or in radians')
		plt.title(names[b]+" graph.")
		plt.legend()
		plt.savefig(path_to_save)
		b=b+1
pass
