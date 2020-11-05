#Define which fan is what with the labels
fan1_label='/sys/devices/platform/applesmc.768/fan1_label'
label1=$(cat $fan1_label | tr '[:upper:]' '[:lower:]') 

fan2_label='/sys/devices/platform/applesmc.768/fan2_label'
label2=$(cat $fan2_label | tr '[:upper:]' '[:lower:]') 

fan3_label='/sys/devices/platform/applesmc.768/fan3_label'
label3=$(cat $fan3_label | tr '[:upper:]' '[:lower:]') 

#Getting values from applemc

## FAN 1

fan1_control_file='/sys/devices/platform/applesmc.768/fan1_manual'
fan1_manual=$(cat $fan1_control_file)

fan1_max_file='/sys/devices/platform/applesmc.768/fan1_max'
fan1_max=$(cat $fan1_max_file)

fan1_min_file='/sys/devices/platform/applesmc.768/fan1_min'
fan1_min=$(cat $fan1_min_file)

fan1_current_output_file='/sys/devices/platform/applesmc.768/fan1_output'

##FAN 2

fan2_control_file='/sys/devices/platform/applesmc.768/fan2_manual'
fan2_manual=$(cat $fan2_control_file)

fan2_max_file='/sys/devices/platform/applesmc.768/fan2_max'
fan2_max=$(cat $fan2_max_file)

fan2_min_file='/sys/devices/platform/applesmc.768/fan2_min'
fan2_min=$(cat $fan2_min_file)

fan2_current_output_file='/sys/devices/platform/applesmc.768/fan2_output'

##FAN 3

fan3_control_file='/sys/devices/platform/applesmc.768/fan3_manual'
fan3_manual=$(cat $fan3_control_file)

fan3_max_file='/sys/devices/platform/applesmc.768/fan3_max'
fan3_max=$(cat $fan3_max_file)

fan3_min_file='/sys/devices/platform/applesmc.768/fan3_min'
fan3_min=$(cat $fan3_min_file)

fan3_current_output_file='/sys/devices/platform/applesmc.768/fan3_output'


# Putting fans on manual mode or set all to auto again
if [ $fan1_manual = "0" ]; then
    echo "1" > $fan1_control_file
fi

if [ $fan2_manual="0" ]; then
    echo "1" > $fan2_control_file
fi

if [ $fan3_manual="0" ]; then
    echo "1" > $fan3_control_file
fi


if [ $1 = auto ]; then
    echo "0" > $fan1_control_file
    echo "0" > $fan2_control_file
    echo "0" > $fan3_control_file

fi

#### START FAN 1 CONTROL
if [ "$1" = "fan1" ]; then
    fan1_input=$2

    # Calculating the net value that will be given to the fans
    fan1_100=$(($fan1_max-$fan1_min))

    # Calculating final percentage value
    fan1_net=$(($fan1_input*$fan1_100/100))
    fan1_final=$(($fan1_net+$fan1_min))

    # Switch back fan1 to auto mode
    if [ $2 = "auto" ]; then
      echo "0" > $fan1_control_file
      echo "fan1 fan mode set to auto"
      else

      # Writing the final value to the applemc files
      echo $fan1_final > $fan1_current_output_file && echo "fan1 fan set to" $fan1_final "rpm" || echo  "
Try running command as sudo"

    fi

fi

#### START FAN 2 CONTROL
if [ $1 =  "fan2" ]; then
    fan2_input=$2

    # Calculating the net value that will be given to the fan2 fan
    fan2_100=$(($fan2_max-$fan2_min))

    # Calculating percentage of fan2 value
    fan2_net=$(($fan2_input*$fan2_100/100))
    fan2_final=$(($fan2_net+$fan2_min))

    # Switch back fan2 to auto mode
    if [ $2 = "auto" ]; then
      echo "0" > $fan2_control_file
      echo "fan2 mode set to auto"
      else

      # Writing the final value to the fan2 files
      echo $fan2_final > $fan1_current_output_file && echo "fan2 fan set to" $fan2_final "rpm" || echo  "
Try running command as sudo"
    fi

fi

## START FAN 3 CONTROL
if [ $1 =  "fan3" ]; then
    fan3_input=$2

    # Calculating the net value that will be given to the fan2 fan
    fan3_100=$(($fan3_max-$fan3_min))

    # Calculating percentage of fan2 value
    fan3_net=$(($fan3_input*$fan3_100/100))
    fan3_final=$(($fan3_net+$fan3_min))

    # Switch back fan2 to auto mode
    if [ $2 = "auto" ]; then
      echo "0" > $fan2_control_file
      echo "fan3 mode set to auto"
      else

      # Writing the final value to the fan3 files
      echo $fan3_final > $fan3_current_output_file && echo "fan3 fan set to" $fan3_final "rpm" || echo  "
Try running command as sudo"
    fi

fi
