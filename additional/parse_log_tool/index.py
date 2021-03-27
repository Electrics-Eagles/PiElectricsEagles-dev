import argparse
import webbrowser
def open_squrrel():
	print("Open your default broswer to show squrrel")
	webbrowser.open_new("https://i.imgur.com/En3sleF.jpeg")
pass

def get_esc_data():
	input_log_file=input("Log file: ")
	output_log_file=input("Output log file: ")
	print("log file: "+input_log_file)
	print("output log file: "+output_log_file)
	input_log= open(input_log_file).read().split("\n")
	esc_1=[]
	esc_2=[]
	esc_3=[]
	esc_4=[]
	for i in range(len(input_log)):
		if "esc_1" in input_log[i]:
			esc_1.append(input_log[i+1].split(" ")[4])
		if "esc_2" in input_log[i]:
			esc_2.append(input_log[i+1].split(" ")[4])
		if "esc_3" in input_log[i]:
			esc_3.append(input_log[i+1].split(" ")[4])
		if "esc_4" in input_log[i]:
			esc_4.append(input_log[i+1].split(" ")[4])
	write_file=open(output_log_file,"w")
	for f in range(len(esc_1)):
		write_file.write(esc_1[f]+";"+esc_2[f]+";"+esc_3[f]+";"+esc_4[f]+"\n")
	print("Ready : Done")
pass

def get_gyroscope_data():
	input_log_file=input("Log file: ")
	output_log_file=input("Output log file: ")
	print("log file: "+input_log_file)
	print("output log file: "+output_log_file)
	input_log= open(input_log_file).read().split("\n")
	x=[]
	y=[]
	z=[]
	for i in range(len(input_log)):
		if "GYRO VALUE:" in input_log[i]:
			x.append(input_log[i+1].split(" ")[4])
			y.append(input_log[i+2].split(" ")[4])
			z.append(input_log[i+3].split(" ")[4])
	print()
	write_file=open(output_log_file,"w")
	for f in range(len(x)):
		write_file.write(x[f]+";"+y[f]+";"+z[f]+"\n")
	print("Ready : Done")
pass

def throllite():
	input_log_file=input("Log file: ")
	output_log_file=input("Output log file: ")
	print("log file: "+input_log_file)
	print("output log file: "+output_log_file)
	input_log= open(input_log_file).read().split("\n")
	throllite=[]
	for i in range(len(input_log)):
		if "throllite" in input_log[i]:
			throllite.append(input_log[i+1].split(" ")[4])
			
	print()
	write_file=open(output_log_file,"w")
	for f in range(len(throllite)):
		write_file.write(throllite[f]+"\n")
	print("Ready : Done")
pass


def get_acc_data():
	input_log_file=input("Log file: ")
	output_log_file=input("Output log file: ")
	print("log file: "+input_log_file)
	print("output log file: "+output_log_file)
	input_log= open(input_log_file).read().split("\n")
	x=[]
	#y=[]
	z=[]
	for i in range(len(input_log)):
		if "acc_z" in input_log[i]:
			z.append(input_log[i+1].split(" ")[4])
		if "acc_x" in  input_log[i]:
			x.append(input_log[i+1].split(" ")[4])
	
	print()
	write_file=open(output_log_file,"w")
	for f in range(len(x)):
		write_file.write(x[f]+";"+z[f]+"\n")
		#print(len(z))
	print("Ready : Done")
pass
def main():
	while True:
		print("Electrics Eagles Log Parsing tool (c) 2021")
		print("This tool is ispried by good software and squrrels")
		print("Had look here: https://i.imgur.com/En3sleF.jpeg")

		print("1. Parse logs and get esc data")
		print("2. Parse logs and get gyroscope data")
		print("3. Parse logs and acclerometer data")
		print("5. Parse logs and get throllite")
		#print("6. Parse logs and get custom")
		print("7. Open squrrel photo")

		choose=input("Your choose? : ")
		if(int(choose)==7):
			open_squrrel()
		if(int(choose)==1):
			get_esc_data()
		if(int(choose)==2):
			get_gyroscope_data()
		if(int(choose)==3):
			get_acc_data()
		if(int(choose)==5):
			throllite()
		

pass



main()

