#!/usr/bin/bash
. ./scripts/color.sh
dividing_line(){
	echo -e "${Yellow}=========================================================${Color_Off}"
}

show_ok(){
	echo -n " 👌"
}

show_err(){
	echo -n " ❌"
}

show_warn(){
	echo -n " ⚠"
}
show_wait(){
	echo -n " 🦄"
}

echos(){
	if [ $# == 1 ]; then
		echo -e "${Green}${1}${Color_Off}"
	fi
}
