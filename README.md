This script has been tested on a 2011 iMac 21.5 inch running Ubuntu 20.04

Usage: 

1) Clone this repo and move the fan.sh to /home/{your user}/bin 

2) Create the alias for this script

$ sudo echo ' alias fan="sudo sh /home/{your user}/bin/fan.sh "' >> ~/.bashrc

3) Run fan !

fan [ SELECT FAN TO CONTROL ( cpu, hdd or auto to set both fans to auto )] [SELECT THE PERCENTAGE YOU WANT THE FAN TO RUN ( value from 1 to 100, just the number | type auto to set automatic control)] 

