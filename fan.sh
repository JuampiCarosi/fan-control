#!/usr/bin/env bash
#
# forked from https://github.com/juampapo546/fan-control

#CMD="${0##*/}"
sysdir="/sys/devices/platform/applesmc.768"

#Define input variable for the functions
input=$2

declare -a fan_control_file fan_label label

#Match labels with fan number and get control files
fan_control_file[1]="$sysdir/fan1_manual"
fan_label[1]="$sysdir/fan1_label"
label[1]=$(< ${fan_label[1]} )
label[1]=${label[1],,}

if [[ "${label[1]}" != "exhaust" && "${label[1]}" != "master" ]]; then
	fan_control_file[2]="$sysdir/fan2_manual"
	fan_label[2]="$sysdir/fan2_label"
    label[2]=$(< ${fan_label[2]} )
    label[2]=${label[2],,}

	fan_control_file[3]="$sysdir/fan3_manual"
	fan_label[3]="$sysdir/fan3_label"
    label[3]=$(< ${fan_label[3]} )
    label[3]=${label[3],,}
fi

# fan() - set fan
# argument is fan number (starting from 1)
fan_function() {
    local fan_manual fan_max_file fan_max fan_min_file fan_min
    local fan_current_output_file
    local -i fan_100 fan_net fan_final
    fan="$1"
    # Getting fan files and data from applesmc.768
    fan_manual=$(cat ${fan_control_file[$fan]})

    fan_max_file="$sysdir/fan${fan}_max"
    fan_max=$(cat "$fan_max_file")

    fan_min_file="$sysdir/fan${fan}_min"
    fan_min=$(cat "$fan_min_file")

    fan_current_output_file="$sysdir/fan${fan}_output"

    #Putting fan on manual mode
    if [ "$fan_manual" = "0" ]; then
        echo "1" > "${fan_control_file[$fan]}"
    fi

    # Calculating the net value that will be given to the fans
    fan_100=$((fan_max - fan_min))
    # Calculating final percentage value
    fan_net=$((input * fan_100 / 100))
    fan_final=$((fan_net + fan_min))

    # Switch back fan1 to auto mode
    if [ "$input" = "auto" ]; then
        echo "0" > ${fan_control_file[$fan]}
        printf "fan mode set to auto"
    else

        # Writing the final value to the applemc files
        if $fan_final > "$fan_current_output_file"; then
            printf "fan set to %d rpm.\n" "$fan_final"
        else
            printf "Try running command as sudo\n"
        fi
    fi
}

# Commands
command="$1"
case "$command" in
    "")
        printf "Available fans:\n"
        printf "  %s\n" "${label[1]}"
        if [[ "${label[1]}" != "exhaust" && "${label[1]}" != "master" ]]; then
            printf "  %s\n" "${label[2]}"
            printf "  %s\n" "${label[3]}"
	    fi
        ;;
    ### AUTO CONTROL
    auto)
        echo "0" > "${fan_control_file[1]}"
        if [[ "${label[1]}" != "exhaust" ]]; then
		    echo "0" > "${fan_control_file[2]}"
		    echo "0" > "${fan_control_file[3]}"
	    fi
        ;;

    ####  HDD/CPU/ODD CONTROL
    hdd|cpu|odd)
        for i in 1 2 3; do
            if [ "${label[$i]}" = "$command" ]; then
                fan_function "$i"
            fi
        done
        ;;

    ### EXHAUST CONTROL
    exhaust|master)
        fan_function 1
        ;;

    ### MASTER CONTROL
    #master)
    #    fan_function 1
    #    ;;
esac
