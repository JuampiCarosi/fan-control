# iMac Fan control
Note: This script has been tested on a 2011 iMac 21.5 (12,1) inch and MacBook 5,1 running Ubuntu 20.04, if you have problems with this script on other Macs please open an issue

Usage:
## Option A 
(here you don't create the actual command, just an alias, but it's more updatable, just `git pull` will get the job done)


1. Clone the repo in home 
```
cd ~/ && git clone https://github.com/juampapo546/Fan-control/
```

2. Create the alias for this script <br>
 ```
 sudo echo  'alias fan="sudo sh /home/$USER/Fan-control/fan.sh"' >> ~/.bashrc
 ```

___

## Option B
(here you create the command but you'll have to repeat the whole process every time you want to update)

1. Clone the repo in home <br>
```
cd ~/ && git clone https://github.com/juampapo546/Fan-control/ fan-control
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
