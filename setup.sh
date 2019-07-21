#!/bin/bash

#
# psx-pi-smbshare setup script
#
# *What it does*
# This script will install and configure an smb share at /share
# It will also compile ps3netsrv from source to allow operability with PS3/Multiman
#
# *More about the network configuration*
# This configuration provides an ethernet connected PS2 or PS3 a low-latency connection to the smb share running on the raspberry pi
# This setup should work fine out the box with OPL and multiman

# Update packages
sudo apt-get -y update
sudo apt-get -y upgrade

# Install and configure Samba
sudo apt-get install -y samba samba-common-bin
cp ./samba-init.sh $HOME/samba-init.sh
chmod 755 $HOME/samba-init.sh
sudo cp $HOME/samba-init.sh /usr/local/bin
sudo mkdir -m 1777 /share

# Install ps3netsrv
sudo apt-get install -y git gcc
git clone https://github.com/dirkvdb/ps3netsrv--.git
cd ps3netsrv--
git submodule update --init
make CXX=g++
sudo cp ps3netsrv++ /usr/local/bin


# Install USB automount settings
cp ./automount-usb.sh $HOME/automount-usb.sh
chmod 755 $HOME/automount-usb.sh
sudo $HOME/automount-usb.sh

# Set samba-init + ps3netsrv, wifi-to-eth-route, setup-wifi-access-point, and Xlink Kai to run on startup
{ echo -e "@reboot sudo bash /usr/local/bin/samba-init.sh\n"; } | crontab -u pi -

# Start services
sudo /usr/local/bin/samba-init.sh
ps3netsrv++ -d /share/

# Not a bad idea to reboot
sudo reboot
