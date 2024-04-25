#!/bin/sh
target=""
if [ "$1" = "local" ]; then
	target="./merge/"
else
	target="root@Strawberry-Vault:/var/lib/aaura-w3-strawb/"
fi

rsync -e 'ssh -p 56019' -r --del markdown static result/ $target --progress 
