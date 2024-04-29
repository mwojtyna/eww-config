#!/bin/bash

if [ "$1" = "up" ]; then
	amixer sset Master 1%+
elif [ "$1" = "down" ]; then
	amixer sset Master 1%-
elif [ "$1" = "toggle" ]; then
	amixer sset Master toggle
fi
