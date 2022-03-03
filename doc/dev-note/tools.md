Fri Feb 18 00:11:53 CST 2022

#### basic idea
* user set the sync folder
* this program will automatically pull from the github when you start up and will automatically push when you close the shell

#### some constraints
* single file is less than 50MB
* single reponsitory is less than 1GB
* we recommend you to open a new github account to store the files

#### to-do
* provide more user-friendly interface to configure the sync folder
* provide the storage usage view


Sun Feb 27 11:54:02 CST 2022
* finished the dependency check


Tue Mar  1 03:41:52 CST 2022
* check the directory status
	* if it is already a git directory?
	* if it has the upstream directory?
	* git pull and git push?

* it seems we do not need to put it a service?
* we need to put some hooks!
	* if the gh-sync.toml is changed, we need to reload it to the memory
	* when we change the gh-sync.toml, we need to change if
		* we add repos:
		* we delete repos:

	* firstly, when file changes and closed, we need to judge if we need to commit and push
	* when we add files, we need to juedge if we need to track and commit it. 
	* on opening the directory, we need to juedge if we need to pull and merge.


Tue Mar  1 10:04:14 CST 2022
#### TO-DO
* parse the toml(when the file is changed, read it)


Tue Mar  1 23:04:16 CST 2022
#### TODO
* add a sync-download directory to download all the repos not created locally
* mkdir log
* add repos.is_admin
* add repo commit_info_type
* add self server?
* add remo
* add some animation in install scripts


Wed Mar  2 11:36:29 CST 2022
#### TODO
* learn how to setup
* add git dependency check
* add the speed test module


Thu Mar  3 12:57:54 CST 2022
* design the server repos
* design the commands and what they do
* design the installing path and base_dir
* add user directly
