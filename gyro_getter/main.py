from flask import Flask
import random 
app = Flask(__name__)

@app.route('/')
def hello_world():
    return str(random.randint(0,1000))+","+str(random.randint(0,1000))+","+str(random.randint(0,1000))
