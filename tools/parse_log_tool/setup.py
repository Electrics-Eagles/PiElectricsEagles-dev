# coding: utf-8

from cx_Freeze import setup, Executable
from tqdm import tqdm
#import tdqm

executables = [Executable('index.py')]

excludes = ['bz2']

options = {
    'build_exe': {
        'include_msvcr': True,
        'excludes': excludes,
    }
}

setup(name='CLI Parse Tool Windows and Linux',
      version='0.0.3',
      description='This app will help you parse log',
      executables=executables,
      options=options)
