#Define input variable for the functions
input=$2

#Match labels with fan number
fan1_label='/sys/devices/platform/applesmc.768/fan1_label'
label1=$(cat $fan1_label | tr '[:upper:]' '[:lower:]') 

fan2_label='/sys/devices/platform/applesmc.768/fan2_label'
label2=$(cat $fan2_label | tr '[:upper:]' '[:lower:]') 

fan3_label='/sys/devices/platform/applesmc.768/fan3_label'
label3=$(cat $fan3_label | tr '[:upper:]' '[:lower:]') 


# Fan functions
function_fan1 () {
    # Getting fan files and data from applesmc.768
    fan1_control_file='/sys/devices/platform/applesmc.768/fan1_manual'
    fan1_manual=$(cat $fan1_control_file)

    fan1_max_file='/sys/devices/platform/applesmc.768/fan1_max'
    fan1_max=$(cat $fan1_max_file)

    fan1_min_file='/sys/devices/platform/applesmc.768/fan1_min'
    fan1_min=$(cat $fan1_min_file)

    fan1_current_output_file='/sys/devices/platform/applesmc.768/fan1_output'

    #Putting fan on manual mode
    if [ $fan1_manual = "0" ]; then
        echo "1" > $fan1_control_file
    fi

    # Calculating the net value that will be given to the fans
    fan1_100=$(($fan1_max-$fan1_min))
    # Calculating final percentage value
    fan1_net=$(($input*$fan1_100/100))
    fan1_final=$(($fan1_net+$fan1_min))

    # Switch back fan1 to auto mode
    if [ $input = "auto" ]; then
    echo "0" > $fan1_control_file
    echo "fan mode set to auto"
    else

    # Writing the final value to the applemc files
    echo $fan1_final > $fan1_current_output_file && echo "fan set to" $fan1_final "rpm" || echo  "
        Try running command as sudo"

    fi
    
}

function_fan2 () {

    #Getting values from applemc.768

    fan2_control_file='/sys/devices/platform/applesmc.768/fan2_manual'
    fan2_manual=$(cat $fan2_control_file)

    fan2_max_file='/sys/devices/platform/applesmc.768/fan2_max'
    fan2_max=$(cat $fan2_max_file)

    fan2_min_file='/sys/devices/platform/applesmc.768/fan2_min'
    fan2_min=$(cat $fan2_min_file)

    fan2_current_output_file='/sys/devices/platform/applesmc.768/fan2_output'
    
    # Putting fan on manual mode
    if [ $fan2_manual="0" ]; then
        echo "1" > $fan2_control_file
    fi

    # Calculating the net value that will be given to the fan2 fan
    fan2_100=$(($fan2_max-$fan2_min))

    # Calculating percentage of fan2 value
    fan2_net=$(($input*$fan2_100/100))
    fan2_final=$(($fan2_net+$fan2_min))

    # Switch back fan2 to auto mode
    if [ $input = "auto" ]; then
        echo "0" > $fan2_control_file
        echo "fan mode set to auto"
    else

        # Writing the final value to the fan2 files
        echo $fan2_final > $fan2_current_output_file && echo "fan set to" $fan2_final "rpm" || echo  "
    Try running command as sudo"
    fi
}

function_fan3 () {

    #Getting values from applemc.768

    fan3_control_file='/sys/devices/platform/applesmc.768/fan3_manual'
    fan3_manual=$(cat $fan3_control_file)

    fan3_max_file='/sys/devices/platform/applesmc.768/fan3_max'
    fan3_max=$(cat $fan3_max_file)

    fan3_min_file='/sys/devices/platform/applesmc.768/fan3_min'
    fan3_min=$(cat $fan3_min_file)

    fan3_current_output_file='/sys/devices/platform/applesmc.768/fan3_output'

    # Putting fan on manual mode
    if [ $fan3_manual="0" ]; then
        echo "1" > $fan3_control_file
    fi

    # Calculating the net value that will be given to the fan2 fan
    fan3_100=$(($fan3_max-$fan3_min))

    # Calculating percentage of fan2 value
    fan3_net=$(($input*$fan3_100/100))
    fan3_final=$(($fan3_net+$fan3_min))

    # Switch back fan2 to auto mode
    if [ $input = "auto" ]; then
    echo "0" > $fan2_control_file
    echo "fan mode set to auto"
    else

    # Writing the final value to the fan3 files
    echo $fan3_final > $fan3_current_output_file && echo "fan set to" $fan3_final "rpm" || echo  "
    Try running command as sudo"
    fi
}


# Commands
case $1 in

    "")
        echo "Available fans:"
        echo "  $label1"
        echo "  $label2"
        echo "  $label3"
    ;;
    ### AUTO CONTROL
    auto)
        echo "0" > $fan1_control_file
        echo "0" > $fan2_control_file
        echo "0" > $fan3_control_file
    ;;

    ####  HDD CONTROL
    hdd)
        if [ $label1 = "hdd" ]; then
            function_fan1
        fi


        if [ $label2 =  "hdd" ]; then
            function_fan2

        fi

        if [ $label3 =  "hdd" ]; then
            function_fan3

        fi    
    ;;

    ####  CPU CONTROL
    cpu)
        if [ $label1 = "cpu" ]; then
            function_fan1
        fi


        if [ $label2 =  "cpu" ]; then
            function_fan2

        fi

        if [ $label3 =  "cpu" ]; then
            function_fan3

        fi
    ;;      

    ####  ODD CONTROL
    odd)
        if [ $label1 = "odd" ]; then
            function_fan1
        fi


        if [ $label2 =  "odd" ]; then
            function_fan2

        fi

        if [ $label3 =  "odd" ]; then
            function_fan3

        fi
        
    ;;
esac







