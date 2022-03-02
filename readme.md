# GIT-SYNC(in develop)
#### Genral Description
* git-sync is a free sync tool based on git.

#### usage
1. we strongly recommend you to create a new account on your remote server. Use the following command to create a new user. And remember this name, we will use it to configure the sync.
```shell
useradd <YOUR_USER_NAME>
```
2. Install git-sync use the install shell script. do as the prompt guides
```shell
cd git-sync
./install
```

3. To create a new sync directory. use the following command.After that, ever change in the directory will be synced to your server.
```shell
git-sync add /path/to/the/directory # you can also use gs, we have add the alias
```

4. When you want to use in another computer, just install as step2. To see what can be synced, use the following command.
```shell
git-sync list --available
git-sync list --all # to see all the packages already tracked
```

5. To add a remote repo to local and track it. use the following command.
```shell
git-sync clone repo_name
```

6. We strongly recommend you to set git-sync start on boot. use the following command.
```shell
git-sync enable
# you can also set it disabled
git-sync disabled
```

7. You can see the logs to see if git-sync went well.
```shell
git-sync log
```

8. You can also see the git-sync general running status
```shell
git-sync status
```

#### To-do
* add GUI support
* add github server support
	* if you have your own server
	* if you do not have your own server, use github
