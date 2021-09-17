#!/usr/bin/env bash
cd /mnt/c/Users/Win10Home/Documents/Github/PiElectricsEagles/drone_code


if ! docker info > /dev/null 2>&1; then
 sudo dockerd & 
fi


cd drone_code
sudo cross build --target arm-unknown-linux-musleabi
echo "Done on Arch"


#    .---------- constant part!
#    vvvv vvvv-- the code from above
RED='\033[0;32m'
NC='\033[1;30' # No Color
printf "With love to classic .....${RED}  \n \n  S.T.A.L.K.E.R ${NC} n"