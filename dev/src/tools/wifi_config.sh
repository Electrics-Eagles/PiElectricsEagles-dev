#!/usr/bin/env bash

rm -rf /etc/hostapd/hostapd.conf
rm -rf /etc/dhcpcd.conf
rm -rf /etc/dnsmasq.conf
echo Enter Interface:

read interface
echo Enter SSID:


read ssid

echo Enter PASSWORD:
read password


read -p "Intall apt packages (y/n)?" choice
case "$choice" in
  y|Y ) sudo apt-get install hostapd -y sudo apt-get install dnsmasq -y;;
  n|N ) echo "User cancel operation";;
  * ) echo "invalid";;
esac




sudo systemctl stop hostapd
sudo systemctl stop dnsmasq



touch /etc/hostapd/hostapd.conf
#echo interface $interface >> /etc//etc/dhcpcd.conf
echo interface $interface >> /etc/dhcpcd.conf
echo static ip_address=192.168.0.10/24 >>  /etc/dhcpcd.conf
echo denyinterfaces eth0 >>  /etc/dhcpcd.conf
echo denyinterfaces wlan0 >>  /etc/dhcpcd.conf


sudo mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
echo interface=$interface >> /etc/dnsmasq.conf
echo   dhcp-range=192.168.0.11,192.168.0.30,255.255.255.0,24h >> /etc/dnsmasq.conf


echo interface=$interface >> /etc/hostapd/hostapd.conf
echo bridge=br0 >> /etc/hostapd/hostapd.conf
echo hw_mode=g >> /etc/hostapd/hostapd.conf
echo channel=7 >> /etc/hostapd/hostapd.conf
echo wmm_enabled=0 >> /etc/hostapd/hostapd.conf
echo macaddr_acl=0 >> /etc/hostapd/hostapd.conf
echo auth_algs=1 >> /etc/hostapd/hostapd.conf
echo ignore_broadcast_ssid=0 >> /etc/hostapd/hostapd.conf
echo wpa=2 >> /etc/hostapd/hostapd.conf
echo wpa_key_mgmt=WPA-PSK >> /etc/hostapd/hostapd.conf
echo wpa_pairwise=TKIP >> /etc/hostapd/hostapd.conf
echo rsn_pairwise=CCMP >> /etc/hostapd/hostapd.conf
echo ssid=$ssid >> /etc/hostapd/hostapd.conf
echo wpa_passphrase=$password >> /etc/hostapd/hostapd.conf
