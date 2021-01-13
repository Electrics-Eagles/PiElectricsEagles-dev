# PiElectricsEagles


Code Style:

1) Write Tidy ,Clean , Commented code
2) Do not do difficult actions in main.rs file
3) Use KISS,DRY , SOLID principa
4) add public function that returns code module verison
5) do not edit other modules if is not requrired.
6) add dependency if you really need it.
7) not break working API


**DO NOT COMMIT TARGET FOLDER WITH DOCS AND BUILD RESULT**

**SOURCE CODE ONLY**

| 1 | Windows / Ubuntu         | Win7 SP1 Higher or Ubuntu 12.04 and Higher | OS                    |
|---|--------------------------|--------------------------------------------|-----------------------|
| 2 | PiDebug                  | V2                                         | Debuging code via ssh |
| 3 | rust /cargo              | 1.46.0 higher                              | Build code            |
| 4 | Python                   | 3.6 and higher                             | Use python tools      |
| 5 | Dia                      | dia 0.97+git                               | Plot alogritms        |
| 6 | Firefox / Chrome / Opera | higher that 2018 year of release           | Web apps , google     |
| 7 | Whatsapp                 | Lastest                                    | Comunication          |
| 8 | Discord                  | not older that 2019 year of release        | Comunication          |
| 9 | any github gui software  | any                                        | code editing upload   |
|10 | Git                      | 2 and higher                               | code editing upload   |
|11| any comfortable ide for you| any                                       | coding                |

Good links

https://cdn.sparkfun.com/assets/learn_tutorials/6/7/6/PiZero_1.pdf


Core status :
![100](https://progress-bar.dev/100)


Bug fix status:
![40%](https://progress-bar.dev/40)

CLI status:
![0%](https://progress-bar.dev/0)

VS Stdio Code Plugin:
![5%](https://progress-bar.dev/5)


MPU6050 Driver:
![99%](https://progress-bar.dev/91)

iBUS:
![100%](https://progress-bar.dev/67)

ESC_Driver 
![95%](https://progress-bar.dev/95)


Logger
![100%](https://progress-bar.dev/100)

Config_Parser
![81%](https://progress-bar.dev/100)


We are using power of squrrels:
<img src="https://i.ibb.co/QkJ0P2g/Whats-App-Image-2020-10-15-at-12-19-41.jpg"></img>


All soft is V2 APi


Well how install all for build:
1) Install fresh Raspberry Pi zero no gui iso image
2) IN BOOT DIRECTORY CREATE EMTRY ssh file
3) connect to ssh 
---------------------------Do it via ssh on RPI ---------------------
install deps : 
```
sudo apt-get install samba samba-common-bin 
```

Create folder by :  
```
sudo mkdir -m 1777 /share 
```

Create config file :
```
sudo nano /etc/samba/smb.conf
```
Edit file:
```
[share]  
Comment = Pi shared folder  
Path = /share  
Browseable = yes  
Writeable = Yes  
only guest = no  
create mask = 0777  
directory mask = 0777  
```
---------------------------Do it via ssh on RPI ---------------------
---------------------------Do it on your local machine--------------
Update apt packages:
```
sudo apt update
```

Install rustup using curl installer 
```
curl https://sh.rustup.rs -sSf | sh
```
Install stabile compiler core by:
```
rustup default stable
```
install build deps :
```
sudo apt-get install  gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross
```
install build target 
```
rustup target add arm-unknown-linux-gnueabihf
```
go in folder get it by root
```
cd .conifg 
```
edit config 
```
sudo nano config.toml
```
content of file 
```
[target.arm-unknown-linux-gnueabihf]
linker = "$HOME/rpi_tools/arm-bcm2708/arm-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```
get toolchain 
```
rustup target add arm-unknown-linux-gnueabi
```
clone c++ linkers 

```
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
```
build file 
```
RUSTFLAGS="-C linker=$HOME/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc" cargo build --target arm-unknown-linux-gnueabihf --tests
```

build project 
```
cargo build --target=arm-unknown-linux-gnueabihf
```

install deb builder
```
cargo install cargo-deb
```
build to deb file
 ```
 cargo deb --target=arm-unknown-linux-gnueabihf
 ```
 ---------------------------Do it on your local machine--------------
 ---------------------------Do it via ssh on RPI ---------------------
 Open samba folder and install deb on machine.
 
 ---------------------------Do it via ssh on RPI ---------------------





