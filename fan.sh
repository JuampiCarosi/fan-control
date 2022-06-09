#!/usr/bin/env bash
#
# forked from https://github.com/juampapo546/fan-control

CMD="${0##*/}"
sysdir="/sys/devices/platform/applesmc.768"

declare -a fan_control_file fan_label label

# Match labels with fan number and get control files
fan_info() {
    local fan="$1"
    fan_control_file[$fan]="$sysdir/fan${fan}_manual"
    fan_label[$fan]="$sysdir/fan${fan}_label"
    label[$fan]=$(< "${fan_label[$fan]}" )
    label[$fan]=${label[$fan],,}                  # lowercase
    label[$fan]=${label[$fan]%% }                 # trim ending space
}

fan_info 1
if ! [[ "${label[1]}" =~ ^(exhaust|master)$ ]]; then
	fan_info 2
	fan_info 3
fi

# fan() - set fan
# argument is fan number (starting from 1)
fan_function() {
    local fan_manual fan_max_file fan_max fan_min_file fan_min
    local fan_current_output_file
    local -i fan_100 fan_net fan_final
    local fan="$1"
    # Getting fan files and data from applesmc.768
    fan_manual=$(cat "${fan_control_file[$fan]}")

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
        echo "0" > "${fan_control_file[$fan]}"
        printf "fan mode set to auto"
    else
        # Writing the final value to the applemc files
        if echo $fan_final > "$fan_current_output_file"; then
            printf "fan set to %d rpm.\n" "$fan_final"
        else
            printf "Try running command as sudo\n"
        fi
    fi
}

usage() {
    printf "usage: %s [fan_type] value\n" "$CMD"
    printf '  fan_type: "auto", "master", "exhaust", "hdd", "cpu" or "odd"\n'
    printf '  if fan_type is not "auto", value is an integer between 0 and 100\n'
    exit 1
}

if (($# == 0)); then
    printf "Available fans:\n"
    printf "  %s\n" "${label[1]}"
    if ! [[ "${label[1]}" =~ ^(exhaust|master)$ ]]; then
        printf "  %s\n" "${label[2]}"
        printf "  %s\n" "${label[3]}"
	fi
    exit 0
fi

# fan type and value
command="$1"
if [[ "$command" != "auto" ]]; then
    if (( $# == 2 )); then
        input="$2"
    else
        usage
    fi
fi

case "$command" in
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
                fan_function "$i" "$input"
            fi
        done
        ;;

    ### EXHAUST/MASTER CONTROL
    exhaust|master)
        fan_function 1 "$input"
        ;;

    *)
        printf 'unknown command %s\n' "$command"
        usage
esac
