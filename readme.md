# GSYNC(in develop)
### intruction
* gsync is a free command line sync tool based on git.
* why I developed this: it is hard for me to find a good sync tool in linux command line! Althougn I know git itself is good enough to be a sync tool. but i am tired of commiting files when i left my laptop in my bed room.And also, in some cases, I do not care the version control so much, I just want to make it easier to sync files on the different laptops.

### features
* open source and free
* no speed limit
* safe, gsyncis based on your own server
* multiple sync modes: on file changed, on file closed, on minutes.

### prerequisite
* git
* ssh


### usage
1. we strongly recommend you to create a new account on your remote server. Use the following command to create a new user. And remember this name, we will use it to configure the sync.
```shell
	useradd <YOUR_USER_NAME>
```
2. Install gsync use the install shell script. do as the prompt guides
```shell
	cd gsync
	./install
```

3. To create a new sync directory. use the following command.After that, ever change in the directory will be synced to your server.

```shell
	gsync track /path/to/the/directory
```

4. When you want to use in another computer, just install as step2. To see what can be synced, use the following command.
```shell
	gsync list --online
	gsync list --local # to see the repo tracked on this computer
	gsync list --all   # to print all the repos both in the remote and local
	gsync list # the short pattern of gsync --all
```

5. To add a remote repo to local and track it. use the following command.
```shell
	gsync track --online  repo_name
```

6. To untrack a repo
```shell
	gsync untrack --online repo_name
```

7. We strongly recommend you to set gsync start on boot. use the following command.
```shell
	gsync daemon --start
	gsync daemon --stop
	gsync daemon --enable
	gsync daemon --disable
	gsync daemon --status
```

8. You can see the logs to see if gsync went well.
```shell
	gsync log
```

9. If you want to change your server(to-do)
```shell
	gsync reconfig
```


### TODO
* add GUI support
* add github server support
	* if you have your own server
	* if you do not have your own server, use github
