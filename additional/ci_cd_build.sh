echo PiElectricsEagles code build script V0.01 Alpha


echo Check all dependecies 


prompt_confirm() {
  while true; do
    read -r -n 1 -p "${1:-Continue?} [y/n]: " REPLY
    case $REPLY in
      [yY]) echo ; return 0 ;;
      [nN]) echo ; return 1 ;;
      *) printf " \033[31m %s \n\033[0m" "invalid input"
    esac 
  done  
}


if ! command -v apt &> /dev/null
then
    echo "apt could not be found"
    echo run on apt based system
    exit
    
fi



if ! command -v cargo &> /dev/null
then
    echo "cargo could not be found"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    exit
    
fi


if ! command -v docker &> /dev/null
then
    echo "install docker please could not be found"
    echo "if you are on ubuntu just type y if not install use offical docs "
    prompt_confirm "Are you using ubuntu?" || exit 0
    sudo apt-get update
    sudo apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release -y 
    
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    echo \
  "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
  
  sudo apt-get update
  sudo apt-get install docker-ce docker-ce-cli containerd.io -y
  
    
    
 
exit
    
fi

if ! command -v cross &> /dev/null
then
    echo "cross  not be found"
    echo "cross instaled using cargo "
    cargo install cross
  

    exit
    
fi


if ! command -v git &> /dev/null
then
    echo "git  not be found"
    echo "git instaled using apt "
    sudo apt install git -y
  
    exit
    
fi


echo "moving to build"

#git clone https://github.com/Electrics-Eagles/PiElectricsEagles

echo "moving to folder "

#cd PiElectricsEagles
cd drone_code

sudo chmod -R 777 /var/run/docker.sock

cross build --target arm-unknown-linux-musleabi

echo " jooohhoo all done  copy your bin file "




#    .---------- constant part!
#    vvvv vvvv-- the code from above
RED='\033[0;32m'
NC='\033[1;30' # No Color
printf "With love to classic .....${RED}  \n \n  S.T.A.L.K.E.R ${NC} n"
  
#prompt_confirm "Are you want to exit?" || exit 0

#############################################OPTIONAL#####################################################
#Optional 

#sudo cp  /home/linux/Desktop/PiElectricsEagles/drone_code/target/arm-unknown-linux-musleabi/debug/pielectricseagles /media/linux/rootfs/usr/bin/pielectricseagles
#echo Copiend binary to sd card
