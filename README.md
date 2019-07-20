# psx-pi-smbshare
psx-pi-smbshare began with the intent of allowing SMB sharing to Multiman and Open Playstation Loader from a Raspberry Pi.  It has evolved into a Pi-based swiss army knife for enhancing classic game consoles.  

## How it works
psx-pi-smbshare is a preconfigured Raspbian based image for Raspberry Pi 1, 2, and  3.  It runs a [Samba](https://en.wikipedia.org/wiki/Samba_(software)) share, a pi-compatible build of [ps3netsrv](https://github.com/dirkvdb/ps3netsrv--), and reconfigures the ethernet port to act as a router.  This gives low-latency, direct access to the Samba service through an ethernet cable connection between a PS2/PS3 and Raspberry Pi.  This configuration is achieved by running [setup.sh](/setup.sh).  A pre-supplied [image](https://github.com/toolboc/psx-pi-smbshare/releases/) can be applied directly to a Micro-SD card using something like [etcher.io](https://etcher.io/).  The image allows you to use the full available space on the SD card after the OS is first booted.

## What you can do with it
psx-pi-smbshare works out of the box on PS3 with [MultiMAN](http://www.psx-place.com/threads/update2-multiman-v-04-81-00-01-02-base-update-stealth-for-cex-dex-updates-by-deank.12145/).  This functionality allows you to stream and backup up various games and media to the Samba share service running on the Raspberry Pi.

psx-pi-smbshare also works out of the box on PS2 with [Open Playstation Loader](https://github.com/ifcaro/Open-PS2-Loader) and supports streaming of PS2 backups located on the Samba share service. It can also work with [POPStarter for SMB](https://bitbucket.org/ShaolinAssassin/popstarter-documentation-stuff/wiki/smb-mode) to allow streaming of PS1 games from Open Playstation Loader.


# Quickstart

*Prerequisites*
* Raspberry Pi 1, 2, or 3
* Micro-SD Card (8GB+ suggested)

A detailed [video guide](https://www.youtube.com/watch?time_continue=1&v=Ilx5NYoUkNA) is provided by Project Phoenix Media which walks through the processes described below.

## Flash the image
Download the latest [psx-pi-smbshare release image](https://github.com/toolboc/psx-pi-smbshare/releases/) and burn it to a Micro-SD card with [etcher.io](http://etcher.io)

## Accessing the SMB Share
With a wireless network configured, you should be able to access the SMB share by visiting `\\SMBSHARE\share` on windows or `smb://smbshare/share` on Mac / Linux.

![Accessing SMB](/Assets/smbshare.PNG)

The share is preconfigured with a folder structure to accomodate ps3netsrv and Open Playstation Loader expected file paths.

## Accessing USB drive(s) on the SMB Share
Plug and play auto-sharing of USB storage devices over SMB is supported:

* USB Drives are automounted to the /media/<Partition> directory

* When a USB drive is plugged in, the USB Drive becomes available on the SMB Share @ `\\SMBSHARE\share` 

* When a USB drive is removed, the device falls back to sharing the Micro-SD card @ `\\SMBSHARE\share` 

## Forwarding Active FTP session to a connected device
Assuming your console / device has an ip of 192.168.2.2, you may run the following script to forward an Active FTP session:

        sudo iptables -t nat -A PREROUTING -p tcp --dport 21 -j DNAT --to-destination 192.168.2.2:21
        sudo modprobe ip_nat_ftp ports=21

If you wish to permanently enable FTP forwarding, it is suggested to set a static ip of 192.168.2.2 on your console(s) when used with psx-pi-smbshare and modify "wifi-to-eth-route.sh" in the home directory of the pi to run the commands above by adding them directly before:

`sudo sh -c "echo 1 > /proc/sys/net/ipv4/ip_forward" `

This will ensure that Active FTP sessions are forwarded automatically at startup.

After you enable forwarding of Active FTP sessions to your console, it is suggested to use [Filezilla](https://filezilla-project.org/download.php?type=client) to connect to your console via "File => Site Manager => New Site" and connecting with the following settings:

![Accessing SMB](/Assets/FilezillaSettings.png)

![Accessing SMB](/Assets/FilezillaActiveFTPMode.png)

This configuration will forward all ftp requests made to the host ip of the psx-pi-smbshare device to the internal ip assigned to the connected console.  If using another FTP client, it is very important that it is configured to connect using an Active FTP transfer mode.

## Configuring for use with MultiMAN on PS3

*Prerequisites*
* Playstation 3 running a [recent release of MultiMAN](http://store.brewology.com/ahomebrew.php?brewid=24)

*Steps*
1. Launch MultiMAN
2. Select "Settings" => "Network Servers"
3. Configure using the Ip Address '192.168.2.1' (ip to the smbshare assigned by dhcp server running on the Pi) and Port '38008' (default)
4. You should see new section for the network server under 'Photos' / 'Music' / 'Video' / 'Retro' and a new location to copy games to when using copy ISO in the 'Games' menu.  

PS3 Games backed up to the network server can be found and loaded from the "Games" menu in MultiMAN.
PS1, PS2, and PSP games can be found and loaded from "Retro" => "PSONE" "PS2" OR "PSP"  
PS2 backups must be loaded from the HDD but can be copied directly to the SMB server.

## Configuring for use with Open Playstation Loader

*Prerequisites*
* Playstation 2 fat or slim running a [recent release of Open Playstation Loader](http://www.ps2-home.com/forum/viewtopic.php?p=29251#p29251) 

*Steps*
1. Connect the pi ethernet port into the ethernet port of the PS2 and power the pi using the PS2 usb or an external power supply 
2. Boot Open Playstation Loader and select "Settings" => "Network Config".  
Ensure that the following options are set:

        Ethernet Link Mode = Auto
        PS2 
            IP address type = Static
            IP address = 192.168.2.2
            Mask = 255.255.255.0
            Gateway = 192.168.2.1
            DNS Server = 8.8.8.8
        SMB Server
            Address Type = IP
            Address = 192.168.2.1
            Port = 445
            Share = share
            Password = <not set>

    ![PS2 OPL Settings](/Assets/PS2-OPL-settings.png)

Don't forget to select "Save Config" when you return to "Settings"

3. Reconnect or restart Open Playstation Loader
4. PS2 Games will be listed under "ETH Games".  To add PS2 games, copy valid .iso backups to `\\SMBSHARE\share\DVD` or `\\SMBSHARE\share\CD`

## Configuring for use with POPSLoader on Open Playstation Loader

*Prerequisites*
* Ensure that you have successfully followed the steps above for "Configuring for use with Open Playstation Loader"

*Steps*
1. Download the [ps2 network modules](https://bitbucket.org/ShaolinAssassin/popstarter-documentation-stuff/downloads/network_modules.7z) 
2. Extract the POPSTARTER folder 
3. Modify IPCONFIG.DAT to:
        
        192.168.2.2 255.255.255.0 192.168.2.1
4. Modify SMBCONFIG.DATA to:
        
        192.168.2.1 share
5. Copy the POPSTARTER folder to your memory card
6. Hop on the internet and look for a copy of a file named "POPS_IOX.PAK" with md5sum "a625d0b3036823cdbf04a3c0e1648901" and copy it to `\\SMBSHARE\share\POPS`.  This file is not included for "reasons".
7. PS1 backups must be converted to .VCD and run through a special renaming program in order to show up in OPL.

    To convert .bin + .cue backups, you can use the included "CUE2POP_2_3.EXE" located in `\\SMBSHARE\share\POPS\CUE2POPS v2.3`
    Copy your .VCD backups to `\\SMBSHARE\share\POPS` then run `\\SMBSHARE\share\POPS\OPLM\OPL_Manager.exe` to rename your files appropriately.
    
    Once converted and properly renamed, your games will show up under the "PS1 Games" section of OPL

    A detailed guide is available @ http://www.ps2-home.com/forum/viewtopic.php?f=64&t=5002
# Demos
* [Launching PS2 Backups with OPL](https://www.youtube.com/watch?v=FJEdWW6YhJo&feature=youtu.be)
* [Launching PS1 Backups with POPSLoader](https://youtu.be/qCiezaKglMs)

# Credits
Thx to the following:
* Jay-Jay for [OPL Daily Builds](https://github.com/Jay-Jay-OPL/OPL-Daily-Builds) 
* danielb for [OPLM](http://www.ps2-home.com/forum/viewtopic.php?f=64&t=189)
* dirkvdb for [ps3netsrv--](https://github.com/dirkvdb/ps3netsrv--)
* Pul Gasari for [Testing streaming games from Raspberry Pi to PS2 using psx-pi-smbshare](https://www.youtube.com/watch?v=FJEdWW6YhJo&feature=youtu.be)
