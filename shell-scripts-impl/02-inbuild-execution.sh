#!/bin/bash

currentdate=$(date)
echo "Current date and time :: $currentdate"

current_directory=$(pwd)
echo "The Current working directory :: $current_directory"

files_list=$(ls)
echo "File List in the current directory  :: $files_list"

disk_usage=$(du)
echo "Find the Disk usage in the System  :: $disk_usage"
