#Getting values from applemc
fan_hdd_control_file='/sys/devices/platform/applesmc.768/fan2_manual'
hdd_manual=$(cat $fan_hdd_control_file)

fan_cpu_control_file='/sys/devices/platform/applesmc.768/fan3_manual'
cpu_manual=$(cat $fan_cpu_control_file)

fan_hdd_max_file='/sys/devices/platform/applesmc.768/fan2_max'
hdd_max=$(cat $fan_hdd_max_file)

fan_hdd_min_file='/sys/devices/platform/applesmc.768/fan2_min'
hdd_min=$(cat $fan_hdd_min_file)

fan_cpu_max_file='/sys/devices/platform/applesmc.768/fan3_max'
cpu_max=$(cat $fan_cpu_max_file)

fan_cpu_min_file='/sys/devices/platform/applesmc.768/fan3_min'
cpu_min=$(cat $fan_cpu_min_file)

fan_cpu_current_output_file='/sys/devices/platform/applesmc.768/fan3_output'

fan_hdd_current_output_file='/sys/devices/platform/applesmc.768/fan2_output'

# Putting fan 2 (hdd) and fan 3 (cpu) on manual mode or set all to auto again
if [ $cpu_manual="0" ]; then
    echo "1" > $fan_cpu_control_file
fi

if [ $hdd_manual = "0" ]; then
    echo "1" > $fan_hdd_control_file
fi

if [ $1 = auto ]; then
    echo "0" > $fan_cpu_control_file
    echo "0" > $fan_hdd_control_file
fi

#### START CPU CONTROL
if [ $1 =  "cpu" ]; then
    cpu_input=$2

    # Calculating the net value that will be given to the cpu fan
    cpu_100=$(($cpu_max-$cpu_min))

    # Calculating percentage of cpu value
    cpu_net=$(($cpu_input*$cpu_100/100))
    cpu_final=$(($cpu_net+$cpu_min))

    # Switch back cpu to auto mode
    if [ $2 = "auto" ]; then
      echo "0" > $fan_cpu_control_file
      echo "cpu mode set to auto"
      else

      # Writing the final value to the cpu files
      echo $cpu_final > $fan_cpu_current_output_file
      echo "cpu fan set to"  $cpu_final "rpm"
    fi

fi

#### START HDD CONTROL
if [ "$1" = "hdd" ]; then
    hdd_input=$2

    # Calculating the net value that will be given to the fans
    hdd_100=$(($hdd_max-$hdd_min))

    # Calculating final percentage value
    hdd_net=$(($hdd_input*$hdd_100/100))
    hdd_final=$(($hdd_net+$hdd_min))

    # Switch back hdd to auto mode
    if [ $2 = "auto" ]; then
      echo "0" > $fan_hdd_control_file
      echo "hdd fan mode set to auto"
      else

      # Writing the final value to the applemc files
      echo $hdd_final > $fan_hdd_current_output_file
      echo "hdd fan set to" $hdd_final "rpm"
    fi

fi

if [ $1 = "-h"  ]; then
    echo "fan [ SELECT FAN TO CONTROL ( cpu, hdd or auto to set both fans to auto )] [SELECT THE PERCENTAGE YOU WANT THE FAN TO RUN ( value from 1 to 100, just the number | type auto to set automatic control)]"
fi