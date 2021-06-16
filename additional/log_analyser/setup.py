from cx_Freeze import setup, Executable
import os.path
import sys
import PySide2.QtXml

# Dependencies are automatically detected, but it might need
# fine tuning.
buildOptions = dict(
    packages=["PySide2","shiboken2","easygui", "os", "notifypy", "sys","time","random","datetime","math","scipy","statistics","matplotlib"],
    includes=[],
    excludes=[],
    include_files=["parser.py"],
    # replace_paths=[("*", "")],
    path=sys.path + ["lib"],
)

base = "Win32GUI" if sys.platform == "win32" else None

executables = [Executable("main.py", base=base)]
setup(
    name="Analyser",
    version="1.0",
    description="Make analyse of the all data from gyro :)",
    options={'py2app': buildOptions},
    setup_requires=['py2app'],
    executables=executables,
)