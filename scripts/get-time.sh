#!/bin/bash

echo "$(date +'%X')"
while true; do
	# Get the current time in nanoseconds
	current_time=$(date +%s%N)

	# Calculate the time remaining until the next second (1 billion nanoseconds = 1 second)
	time_until_next_second=$((1000000000 - current_time % 1000000000))

	# Sleep until the next second
	sleep $(bc -l <<<"$time_until_next_second / 1000000000")

	# Call your custom script or command here to display the statusbar with the updated time
	# For example, you can use 'echo' to print the time to the console:
	echo "$(date +'%X')"
done
