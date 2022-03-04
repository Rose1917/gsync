#!/bin/bash
. scripts/ui.sh

DEBUG="true"

CONF_FILE=".git-sync.toml"
CONF_PATH="$HOME/.git-sync.toml"

set -C

# debug
if [ "$DEBUG" = "true" ];then
	# set -x 
	if [ -f "$CONF_PATH" ]; then
		rm ~/.git-sync.toml
	fi

	ssh renyanjie.cn "rm -rf /home/march1917/git-sync"
	# if [ -f "$HOME/.ssh/id_rsa_sync.pub" ]; then 
	# 	rm $HOME/.ssh/id_rsa_sync*
	# fi


	sed -e "/git-sync/d" -i $HOME/.bashrc
	rm -rf ~/git-sync/meta


fi

# test if git is installed
dividing_line
echos "check if git is installed..."
if command -v git >/dev/null 2>&1;then
	show_ok
	echo "git is installed"

else
	show_warn
	echo "c not find the git"

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
	show_ok
	echo "installed git successfully"
fi

dividing_line
echos "installing the configuration file..."
# test if the git-sync.toml
if [  -f "$CONF_PATH" ];then
	echo "configuration file already exists"
else
	echo "creating the configuration file in $CONF_PATH"
	cp $CONF_FILE $CONF_PATH
	if [ -f "$CONF_PATH" ]; then 
		show_ok
		echo "installed the $HOME/.git-sync.toml successfully"
	else 
		show_err
		echo "something went wrong when we are trying to write to $CONF_PATH"
	fi
fi
dividing_line
echos "we need to configure your git server account"

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
		show_ok
		echo "you host is valid"
		break;
	else
		echo $SERVER_HOST $SERVER_PORT
		show_err
		echo "the host is not valid"
		echo "please re-input your host and port"
		echo ""
	fi
done

# set the user
dividing_line
USER_NAME=""
echos "please input your user to login the remote server"
echos "maybe the current user?($USER if you press enter)"
read -p "username:" USER_NAME
if [ "$USER_NAME" = "" ]; then 
	USER_NAME="$USER"
fi

# set the local dir
dividing_line
LOCAL_BASE=""
echos "please tell us where you want to download remote repos"
echos "maybe the default folder? ($HOME/git-sync if you press enter)"

while [ 1 ];
do
	read -e -p "local path:"  LOCAL_BASE
	if [ "$LOCAL_BASE" = "" ]; then 
		mkdir -p $HOME/git-sync
		LOCAL_BASE=$HOME/git-sync
	fi

	if [ -d $LOCAL_BASE ];then 
		show_ok
		echo "local folder:$LOCAL_BASE"
		break
	else
		show_err
		echo "directory does not exists, please re-input"
		echo ""
	fi
done



# set the remote dir


# dividing_line
# TOKEN_PATH=""
# # set the ssh token path
# while [ 1 ];
# do
# 	read -e -p "token_path:"  TOKEN_PATH
# 	if [ -f $TOKEN_PATH ];then 
# 		break
# 	else
# 		echo "token not exists, please re-input"
# 		echo ""
# 	fi
# done


# configure the toml file
sed -e "s,host.*,host = \"$SERVER_HOST\"," -i $HOME/.git-sync.toml
sed -e "s,port.*,port = $SERVER_PORT," -i $HOME/.git-sync.toml
sed -e "s,username.*,username = \"$USER_NAME\"," -i $HOME/.git-sync.toml
sed -e "s,local_base.*,local_base = \"$LOCAL_BASE\"," -i $HOME/.git-sync.toml

dividing_line
show_ok
echos "your configuration file is generated successfully"

dividing_line
echos "trying to test if ssh-key is set properly"

while [ 1 ];
do
	ssh -o BatchMode=yes "$USER_NAME@$SERVER_HOST" -p $SERVER_PORT "exit" >/dev/null 2>&1
	if [ $? -eq 0 ] ; then 
		show_ok 
		echo "the key is set properly"
		break
	else
		show_err
		echo "not set properly"
		echo "generating ssh-keys"

		if [ ! -f $HOME/.ssh/id_rsa_sync.pub ]; then
			ssh-keygen -t rsa -N "" -f $HOME/.ssh/id_rsa_sync
		fi

		# copy id to the server
		ssh-copy-id -i "$HOME/.ssh/id_rsa_sync.pub" "$USER_NAME@$SERVER_HOST" -p $SERVER_PORT >/dev/null 2>&1

		# append to .ssh/config
		echo "Host $SERVER_HOST" >>$HOME/.ssh/config
		echo "Hostname $SERVER_HOST" >>$HOME/.ssh/config
		echo "IdentityFile $HOME/.ssh/id_rsa_sync" >>$HOME/.ssh/config

	fi
done

# test if server is already in use
dividing_line
echos "test if the server is initialized already"

ssh "$USER_NAME@$SERVER_HOST"  -p $SERVER_PORT test -d "/home/$USER_NAME/git-sync"
if [ $? -eq 0 ];then 
	show_ok 
	echo "the server is set already"
else 
	show_err
	echo "the server is not set yet"
	show_wait
	echo "initialzing, it may take some time"
	ssh "$USER_NAME@$SERVER_HOST"  -p $SERVER_PORT 'bash -s' <./scripts/init-server.sh

	if [ $? -eq 0 ];then 
		show_ok 
		echo "the server is set already"
	fi

fi

# clone the meta to local
dividing_line 
show_wait
echos "getting the repos list..."
git clone -q $USER_NAME@$SERVER_HOST:git-sync/meta  $LOCAL_BASE/meta
if [ -d $LOCAL_BASE/meta ]; then 
	show_ok 
	echo "reading the repo list success"
else 
	show_err 
	echo "something went wrong when we are getting the repo list"
	exit
fi
# build the project(optional?)

# add gsync to the PATH

# add git-sync alias
dividing_line
echos "add the alias"
grep "alias gs = git-sync" $HOME/.bashrc 
if [ $? -ne 0 ];then
	show_ok
	echo "# git-sync alias"|tee >>$HOME/.bashrc
	echo "alias gsync="git-sync"" >>$HOME/.bashrc
	echo "we have added alias to your bashrc, use "source ~/.bashrc" to activate now."
fi

# add github support


# show the banner 
dividing_line
echo "    welcome to use gsync...";
echo '''
  ____ ______   ___   _  ____ 
 / ___/ ___\ \ / / \ | |/ ___|
| |  _\___ \\ V /|  \| | |    
| |_| |___) || | | |\  | |___ 
 \____|____/ |_| |_| \_|\____|
     '''

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

# TODO
# set local base
