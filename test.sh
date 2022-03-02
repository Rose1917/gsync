#!/bin/bash
if [ -f $HOME/.ssh/*.pub ]; then
	echo "success"
else
	echo "error"
fi
