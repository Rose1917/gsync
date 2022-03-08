#!/bin/bash
if [ -f $HOME/.ssh/*.pub ]; then
	echo "success"
else
	echo "error"
fi
rm -rf ~/git-sync/meta
git clone "renyanjie.cn:git-sync/meta" ~/git-sync/meta


echo '''
  ____ ______   ___   _  ____ 
 / ___/ ___\ \ / / \ | |/ ___|
| |  _\___ \\ V /|  \| | |    
| |_| |___) || | | |\  | |___ 
 \____|____/ |_| |_| \_|\____|
     '''
                              
