import sys
import os.path
import pickle as pk
import matplotlib
import matplotlib.pyplot as plt


if len(sys.argv) != 2:
    raise Exception(f"Incorrect number of arguments entered. Recieved {len(sys.argv)}.")
else:
    path = sys.argv[1]

if not os.path.exists(path):
    raise Exception(f"File {path} does not exist.")
else:
    with open(path, 'rb') as handle:
        pickle_file = pk.load(handle)

if len(pickle_file) != 2:
    raise Exception(f"File was improperly pickled. Required length of 2, received length of {len(pickle_file)}")
else:
    x = pickle_file[0]
    y = pickle_file[1]


data=open("last.file","r")
last=int(data.readline())
data.close()
open("last.file","w").write(str(last+1))
fig, ax = plt.subplots()
ax.plot(x, y)
ax.grid()
plt.savefig(str(last+1)+".png")
sys.exit()
