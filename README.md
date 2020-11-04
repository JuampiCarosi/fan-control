This script has been tested on a 2011 iMac 21.5 inch running Ubuntu 20.04

Usage:
## Option A 
(here you don't create the actual command, just an alias, but it's more updatable, just `git pull` will get the job done

1. Clone the repo in home `cd ~/ && git clone https://github.com/juampapo546/Fan-control/`

2. Create the alias for this script

$ `sudo echo ' alias fan="sudo sh /home/$USER/fan-control/fan>> ~/.bashrc`


___

## Option B
here you create the command but you'll have to repeat the whole process every time you want to update)

1. Clone the repo in home `cd ~/ && git clone https://github.com/juampapo546/Fan-control/ fan-control`

2. Move the script to /bin and make it executable ` sudo mv fan-control/fan /bin/fan && sudo chmod +x /bin/fan/ `

3. (optional) Clean remainings of the repo `rm -rf ~/fan-control `

____

### Run fan!

 ``` 
sudo fan [ SELECT FAN TO CONTROL ] [SELECT THE PERCENTAGE YOU WANT THE FAN TO RUN ] <br>

		-hdd				-value (nummber from 1 to 100)  
		-cpu 
		-auto 
```
For example if you want cpu fan to run at it's 65% it's <br>
$ `sudo fan cpu 65`
