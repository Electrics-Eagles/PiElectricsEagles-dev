import os
import sys


def main():
    data=open("loggics.rs","wr",encoding='UtF-8')
    patched=open("loggics_patched.rs","wr",encoding='UtF-8')
    value=data.readlines()
    for a in value:
        if(value[a].find("{:patch_me:}")>0):
            value[a-1]="// Patched"
            value[a]=""
        patched.write(data[a])




main()