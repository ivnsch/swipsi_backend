pg_dump -U postgres -d bikematch -Fc -f backup.dump
scp backup.dump root@206.189.62.237:/root/swipsi/
