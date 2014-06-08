#!/bin/bash

for file in *; do
	# echo -n $file
	new_file=$(echo $file|sed -e 's/[^A-Za-z0-9._-]/_/g')
	if [ "$file" = "$new_file" ]; then
		# echo " not messing..."
		continue
	fi
	echo "$file => $new_file"
	mv "$file" $new_file
done

