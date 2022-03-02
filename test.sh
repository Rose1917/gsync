SERVER_HOST="renyanjie.cn"
SERVER_PORT=22
while [ 1 ];
do
	read -p "please input the host:" SERVER_HOST
	read -p "please input the port:" SERVER_PORT
	if [ telnet $SERVER_HOST $SERVER_PORT <<EOF 
		EOF|grep -q "connected" ]; then
		echo "you host is valid"
	else
		echo "the host is not valid"
	fi
done
