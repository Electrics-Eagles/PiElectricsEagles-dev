
import os  # Import only os templates
import sys

"""
██████╗░██╗███████╗██╗░░░░░███████╗░█████╗░████████╗██████╗░██╗░█████╗░░██████╗
██╔══██╗██║██╔════╝██║░░░░░██╔════╝██╔══██╗╚══██╔══╝██╔══██╗██║██╔══██╗██╔════╝
██████╔╝██║█████╗░░██║░░░░░█████╗░░██║░░╚═╝░░░██║░░░██████╔╝██║██║░░╚═╝╚█████╗░
██╔═══╝░██║██╔══╝░░██║░░░░░██╔══╝░░██║░░██╗░░░██║░░░██╔══██╗██║██║░░██╗░╚═══██╗
██║░░░░░██║███████╗███████╗███████╗╚█████╔╝░░░██║░░░██║░░██║██║╚█████╔╝██████╔╝
╚═╝░░░░░╚═╝╚══════╝╚══════╝╚══════╝░╚════╝░░░░╚═╝░░░╚═╝░░╚═╝╚═╝░╚════╝░╚═════╝░

███████╗░█████╗░░██████╗░██╗░░░░░███████╗░██████╗
██╔════╝██╔══██╗██╔════╝░██║░░░░░██╔════╝██╔════╝
█████╗░░███████║██║░░██╗░██║░░░░░█████╗░░╚█████╗░
██╔══╝░░██╔══██║██║░░╚██╗██║░░░░░██╔══╝░░░╚═══██╗
███████╗██║░░██║╚██████╔╝███████╗███████╗██████╔╝


WIFI configurator V1 
Makes wifi configuration easy form console .
Author Alex Zaslavskis 
03/01/2021 16:34

"""
# There are config templates
interface = """
    static ip_address=192.168.4.1/24
    nohook wpa_supplicant
"""

ap = """
# https://www.raspberrypi.org/documentation/configuration/wireless/access-point-routed.md
# Enable IPv4 routing
net.ipv4.ip_forward=1
"""

wlano = """

dhcp-range=192.168.4.2,192.168.4.20,255.255.255.0,24h
                # Pool of IP addresses served via DHCP
domain=wlan     # Local wireless DNS domain
address=/gw.wlan/192.168.4.1
                # Alias for this router
"""

network_config = """
country_code=GB
interface=wlan0
#ssid=NameOfNetwork
hw_mode=g
channel=7
macaddr_acl=0
auth_algs=1
ignore_broadcast_ssid=0
wpa=2
#wpa_passphrase=AardvarkBadgerHedgehog
wpa_key_mgmt=WPA-PSK
wpa_pairwise=TKIP
rsn_pairwise=CCMP
"""

# There are logo templates
logo = """


██████╗░██╗███████╗██╗░░░░░███████╗░█████╗░████████╗██████╗░██╗░█████╗░░██████╗
██╔══██╗██║██╔════╝██║░░░░░██╔════╝██╔══██╗╚══██╔══╝██╔══██╗██║██╔══██╗██╔════╝
██████╔╝██║█████╗░░██║░░░░░█████╗░░██║░░╚═╝░░░██║░░░██████╔╝██║██║░░╚═╝╚█████╗░
██╔═══╝░██║██╔══╝░░██║░░░░░██╔══╝░░██║░░██╗░░░██║░░░██╔══██╗██║██║░░██╗░╚═══██╗
██║░░░░░██║███████╗███████╗███████╗╚█████╔╝░░░██║░░░██║░░██║██║╚█████╔╝██████╔╝
╚═╝░░░░░╚═╝╚══════╝╚══════╝╚══════╝░╚════╝░░░░╚═╝░░░╚═╝░░╚═╝╚═╝░╚════╝░╚═════╝░

███████╗░█████╗░░██████╗░██╗░░░░░███████╗░██████╗
██╔════╝██╔══██╗██╔════╝░██║░░░░░██╔════╝██╔════╝
█████╗░░███████║██║░░██╗░██║░░░░░█████╗░░╚█████╗░
██╔══╝░░██╔══██║██║░░╚██╗██║░░░░░██╔══╝░░░╚═══██╗
███████╗██║░░██║╚██████╔╝███████╗███████╗██████╔╝
╚══════╝╚═╝░░╚═╝░╚═════╝░╚══════╝╚══════╝╚═════╝░
"""

# There are warring templates
warring = """ \n \n 
Warring it removes only the config files
Warring it removes only the config files
Warring it removes only the config files
Warring it removes only the config files
"""

# Import the necessary packages




def remove():
    # Run bash commands
    os.system("rm -rf /etc/dhcpcd.conf")
    os.system("rm -rf /etc/sysctl.d/routed-ap.conf")
    os.system("rm -rf /etc/hostapd/hostapd.conf")
    print(warring)


pass


def config_wifi(interface_, ssid_, password):
    # Create config using and funcipotns bash commands
    cli_work()
    create_file("/etc/dhcpcd.conf", "#config created auto")

    #interface_=input("Enter Interface: ")

    append_file("/etc/dhcpcd.conf", "interface " + interface_)
    append_file("/etc/dhcpcd.conf", "interface " + interface)

    create_file("/etc/sysctl.d/routed-ap.conf", ap)
    os.system("sudo rfkill unblock wlan")
    os.system("sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE")
    os.system("sudo netfilter-persistent save")
    create_file("/etc/sysctl.d/routed-ap.conf", "#config created auto")
    append_file("/etc/sysctl.d/routed-ap.conf", "interface=" + interface)
    append_file("/etc/sysctl.d/routed-ap.conf", wlano)

    create_file("/etc/hostapd/hostapd.conf", network_config)
    #ssid_=input("Enter SSID: ")
    #password=input("Enter Password: ")
    append_file("/etc/hostapd/hostapd.conf", "ssid=" + ssid_)
    append_file("/etc/hostapd/hostapd.conf", "wpa_passphrase=" + password)
    print("Job done")


pass


def cli_work():
    os.system("sudo apt install hostapd -y ")
    os.system("sudo apt install dnsmasq -y ")
    os.system("sudo systemctl unmask hostapd")
    os.system("sudo systemctl enable hostapd")
    os.system(
        "sudo DEBIAN_FRONTEND=noninteractive apt install -y netfilter-persistent iptables-persistent"
    )


pass


def create_file(file, content):
    f = open(file, "w")
    f.write(content)
    f.close()


pass


def append_file(file, content):
    file_object = open(file, 'a')
    file_object.write(content)
    file_object.close()


pass


def tui(interface_, ssid_, password):

    if (entry == "1"):
        remove()
    if (entry == "2"):
        config_wifi()
    if (entry == "3"):
        os.system("exit")


pass


def main():
    #python3 install.py -c wlan0 wifi 1234567890

    if (sys.argv[1] == "-h"):
        print(
            "-c runs script in cli mode , so example  tool.py -c wlan0 wifi 1234567890 where wlan0 is interface wifi is ssid of network and 1234567890 is password \n \n \n "
        )
        print(
            "-r runs script in remove  mode . Scrpit just remove config files. , so example  tool.py -r  \n \n \n "
        )
        print(
            "-t runs script in text  mode . Script just display Text Interface . , so example  tool.py -t "
        )

    if (sys.argv[1] == "-r"):
        remove()
    if (sys.argv[1] == "-c"):
        config_wifi(str(sys.argv[2]), str(sys.argv[3]), str(sys.argv[4]))
    if (sys.argv[1] == "-t"):
        print(logo)
        print("\n \n ")
        print("WIFI configurator")
        print("\n \n  Remove config : 1")
        print("  Configure wifi  : 2")
        print("  Exit  : 3 \n \n ")
        entry = input("Enter you choose: ")
        if (entry == "1"):
            remove()
        if (entry == "2"):
            interface_ = input("Enter Interface: ")
            ssid_ = input("Enter SSID: ")
            password = input("Enter Password: ")
            config_wifi(interface_, ssid_, password)
        if (entry == "3"):
            os.system("exit")


main()
