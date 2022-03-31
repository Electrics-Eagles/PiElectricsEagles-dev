import json
import datetime
import zipfile
import urllib.request


response = urllib.request.urlopen('https://api.github.com/repos/Electrics-Eagles/PiElectricsEagles-dev/actions/artifacts')
html = response.read()
print("**** PIELECTRICSEAGLES BUILDROOT PLUGIN *****")
date=datetime.datetime.now()
print(f"**** at: {date} Clone json *****")
artifact_json=json.loads(html)
print(f"**** at: {date} json loaded  *****")
artifact_id=artifact_json['artifacts'][0]['id']
url = f"https://nightly.link/Electrics-Eagles/PiElectricsEagles-dev/actions/artifacts/{artifact_id}.zip"

urllib.request.urlretrieve(url, 'bin.zip')
zip = zipfile.ZipFile('bin.zip')
zip.extractall()
print(f"**** at: {date} Cloned ok ****")
