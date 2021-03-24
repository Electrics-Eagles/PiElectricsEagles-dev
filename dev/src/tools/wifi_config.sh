#/usr/bin


echo Enter interface:
read interface
echo Enter SSID:
read ssid
echo Enter Password:
read password

rm -rf  /etc/dhcpcd.conf
rm -rf  /etc/sysctl.d/routed-ap.conf
rm -rf /etc/hostapd/hostapd.conf

sudo apt install hostapd -y
sudo systemctl unmask hostapd
sudo systemctl enable hostapd
sudo apt install dnsmasq
sudo DEBIAN_FRONTEND=noninteractive apt install -y netfilter-persistent iptables-persistent


echo  interface $interface >> /etc/dhcpcd.conf
echo     static ip_address=192.168.4.1/24  >> /etc/dhcpcd.conf
echo     nohook wpa_supplicant  >> /etc/dhcpcd.conf



echo  net.ipv4.ip_forward=1 >> /etc/sysctl.d/routed-ap.conf
sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo netfilter-persistent save

sudo mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
sudo nano /etc/dnsmasq.conf


echo interface=$interface  >> /etc/dnsmasq.conf
echo dhcp-range=192.168.4.2,192.168.4.20,255.255.255.0,24h >> /etc/dnsmasq.conf
echo domain=wlan   >> /etc/dnsmasq.conf
echo address=/gw.wlan/192.168.4.1  >> /etc/dnsmasq.conf


sudo rfkill unblock wlan

echo country_code=GB >> /etc/hostapd/hostapd.conf


echo interface=$interface  >> /etc/hostapd/hostapd.conf
echo ssid=$ssid  >> /etc/hostapd/hostapd.conf
echo hw_mode=g  >> /etc/hostapd/hostapd.conf
echo channel=7  >> /etc/hostapd/hostapd.conf
echo macaddr_acl=0  >> /etc/hostapd/hostapd.conf
echo auth_algs=1  >> /etc/hostapd/hostapd.conf
echo ignore_broadcast_ssid=0  >> /etc/hostapd/hostapd.conf
echo wpa=2  >> /etc/hostapd/hostapd.conf
echo wpa_passphrase=$password  >> /etc/hostapd/hostapd.conf
echo wpa_key_mgmt=WPA-PSK  >> /etc/hostapd/hostapd.conf
echo wpa_pairwise=TKIP >> /etc/hostapd/hostapd.conf
echo rsn_pairwise=CCMP >> /etc/hostapd/hostapd.conf


