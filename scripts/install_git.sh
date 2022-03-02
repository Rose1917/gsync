#!/bin/sh
git_install(){
	if command -v apt >/dev/null 2>&1;then
		echo "apt distribution"
		sudo apt update
		sudo apt install git
	elif command -v dnf >/dev/null 2>&1;then
		echo "dnf distribution"
		sudo dnf install git
	elif command -v yum >/dev/null 2>&1;then
		echo "yum distribution"
		sudo yum install git
	elif command -v pacman >/dev/null 2>&1;then
		echo "pacman distribution"
		sudo pacman -S git
	elif command -v zypper >/dev/null 2>&1;then
		echo "zypper distribution"
		sudo zypper install git
	else
		echo "we can not recognize your system"
		echo "please install git manually"
		echo "see https://git-scm.com/ for more detail"
	fi
}
