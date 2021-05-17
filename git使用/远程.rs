取消git的http代理，容易出现问题：
git config --global --unset http.proxy

使用本地库和远程关联：
git remote add origin http://…….
git push -u origin master

本地创建的分支与远程同步：
git push —-set-upstream origin ask


新员工拉取分支到本地：
git pull origin ask:ask


远程分支的合并和删除：
现在master里面
git pull
然后再ask里面
git rebase
然后切换到master里面
git merge ask 本地合并了
然后推送
git push
删除远程的分支
git push origin —delete ask
删除本地的分支
git branch -d ask


自动部署：

