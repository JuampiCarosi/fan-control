# iMac fan control
Note: This script has been tested on iMac 12,1 (2011, 21.5 inch), MacBook 5,1 and 5,2 running Ubuntu 20.04 and Macmini 3,1 (2009 Macmini), if you have problems with this script on other Macs please open an issue

Usage:
## Option A 
(here you don't create the actual command, just an alias, but it's more updatable, just `git pull` will get the job done)


1. Clone the repo in home 
```
cd ~/ && git clone https://github.com/juampapo546/fan-control/
```

2. Create the alias for this script <br> 
If you use bash :
 ```
 sudo echo  'alias fan="sudo sh /home/$USER/fan-control/fan.sh"' >> ~/.bashrc
 ```
If you use zsh :
 ```
 sudo echo  'alias fan="sudo sh /home/$USER/fan-control/fan.sh"' >> ~/.zshrc
 ```
If you have doubts you probably use bash, to be sure check if you have in your /home .bashrc or .zshrc 
___

## Option B
(here you create the command but you'll have to repeat the whole process every time you want to update)

1. Clone the repo in home <br>
```
cd ~/ && git clone https://github.com/juampapo546/fan-control/
```

2. Move the script to /bin and make it executable <br>
``` 
sudo mv ~/fan-control/fan.sh /bin/fan && sudo chmod +x /bin/fan 
```

3. (optional) Clean remainings of the repo <br>
```
rm -rf ~/fan-control 
```

____

### Run fan!

First check what fans are available for your mac

```
fan
```
Then choose one one of the output fans and run:

``` 
sudo fan [ SELECT FAN TO CONTROL ] [SELECT THE PERCENTAGE YOU WANT THE FAN TO RUN ] 

	-hdd (an example)		-value (nummber from 1 to 100)  
	-auto				-auto 	 
```
For example if you want cpu fan to run at it's 65% the input should be: <br>

```
sudo fan cpu 65
```
