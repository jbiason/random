#!/usr/bin/env bash

for loop in {1..1000}
do
	echo "Hello, I'm a script!"
	echo "I write stuff in the output."
	echo "Everything should go to a file."
	echo "But also, you need to capture warnings:"

	if (( $loop%7 == 0)); then
		echo "WARNING: This is a warning"
		echo "   It continues if the line starts with spaces"
		echo "   And keeps going till there are no more spaces-prefixes"
	fi

	if (( $loop%8 == 0)); then
		# ERR is just to make sure we find it easily in the logs
		echo "ERR: Sometimes, I also write in stderr!" >&2
		echo "ERR: Just for funsies!" >&2
	fi

	echo "Like this."
	echo "Then you're good to go."
	echo ""
done
