#!/bin/bash
DEBUG="true"

CONF_FILE=".git-sync.toml"
CONF_PATH="$HOME/.git-sync.toml"

# debug
if [ "$DEBUG" = "true" ];then
	rm ~/.git-sync.toml
fi

# test if git is installed
echo "check if git is installed..."
if command -v git >/dev/null 2>&1;then
	echo "git is installed"
else
	echo "can not find the git"

	echo "if you want us to install git for you?(Y/n):"
	read INPUT

	if [ $INPUT = "Y" ]; then
		. "./scripts/install_git.sh"
		git_install || exit
	else
		echo "aborting the installing.."
		unset INPUT
		exit 1
	fi
	echo "installed git successfully"
fi

echo ""
echo "installing the configuration file..."
# test if the git-sync.toml
if [  -f "$CONF_PATH" ];then
	echo "configuration file already exists"
else
	echo "creating the configuration file in $CONF_PATH"
	cp $CONF_FILE $CONF_PATH
fi


echo ""
echo "we need to configure your git server account"


# test the server 22 port is open
SERVER_HOST=""
SERVER_PORT=""
while [ 1 ];
do
	read -p "please input your server host:" SERVER_HOST
	read -p "please input your server ssh port(22 if you press enter):" SERVER_PORT
	if [ "$SERVER_PORT" = "" ];then
		SERVER_PORT=22
	fi

	if  timeout 2s nc -z $SERVER_HOST $SERVER_PORT 2>/dev/null ; then
		echo "you host is valid"
		break;
	else
		echo $SERVER_HOST $SERVER_PORT
		echo "the host is not valid"
		echo "please re-input your host and port"
		echo ""
	fi
done

# set the user
USER_NAME=""
echo "please input your user to login the remote server"
echo "maybe the current user?$USER"
read -p "username:" USER_NAME

TOKEN_PATH=""
# set the ssh token path
while [ 1 ];
do
	read -e -p "token_path:"  TOKEN_PATH
	if [ -f $TOKEN_PATH ];then 
		break
	else
		echo "token not exists, please re-input"
		echo ""
	fi
done


# configure the toml file
sed -e "s,host.*,host = \"$SERVER_HOST\"," -i $HOME/.git-sync.toml
sed -e "s,port.*,port = $SERVER_PORT," -i $HOME/.git-sync.toml
sed -e "s,username.*,username = \"$USER_NAME\"," -i $HOME/.git-sync.toml
sed -e "s,token-path.*,token-path = \"$TOKEN_PATH\"," -i $HOME/.git-sync.toml

# some deprecated code to congure the github

# if [ $(git auth status 2>&1|grep -c "not") -gt 0 ]; then
# 	echo "you have not logged in"
# 	echo "spawning the git login process..."
# 	git auth login
# else
# 	echo "you already logged in"
# fi


# echo "reading git configuration..."
# git_user=$(cat $HOME/.config/git/hosts.yml 2>&1|grep "user"|cut -d ":" -f 2|awk '{print $1}')
# git_token=$(cat $HOME/.config/git/hosts.yml 2>&1|grep "oauth_token"|cut -d ":" -f 2|awk '{print $1}')

# sed -e "s/username.*/username = \"$git_user\"/" -i $HOME/.git-sync.toml
# sed -e "s/git-token.*/git-token = \"$git_token\"/" -i $HOME/.git-sync.toml

# # configure the download-dir
# echo "please input the path where to store the folders:"
# read -p "(press enter to set ~/git-sync by default)" INPUT

# if [ "$INPUT" = "" ]; then
# 	if [ -d "$HOME/git-sync" ] && [ "`ls -A $HOME/git-sync`" != "" ]; then
# 		echo "git-sync exists and not empty"
# 		echo "please make sure the directory is empty and reinstall"
# 	else
# 		sed -e "s,download-dir.*,download-dir = \"$HOME/git-sync\"," -i $HOME/.git-sync.toml
# 	fi
# else
# 	sed -e "s,download-dir.*,download-dir = \"$INPUT\"," -i $HOME/.git-sync.toml
# fi

# # read the git cloud repos
# git repo list

# # log file

