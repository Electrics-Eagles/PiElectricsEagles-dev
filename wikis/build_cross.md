1. Make sure that rust is installed on your PC . The build will work only on Linux or WSL 2 [Rust installation manual](https://www.rust-lang.org/learn/get-started)
1. Make sure that docker is installed 
1. Clone repo using : ``git clone https://github.com/Electrics-Eagles/PiElectricsEagles``
1. Navigate dev folder ``cd dev``
1. Install cross if is not installed ``cagro install cross`` and install docker or podman 
1. Check is you desktop/laptop has normal internet connection 
1. Finnaly build using ``cross build --target arm-unknown-linux-musleabi``
1. Result file copy to raspberry pi zero (no wifi) and try to run it 

