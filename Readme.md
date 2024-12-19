## Set up Oracle Database for ARM Macos (M1,M2,...)

## install oracle database docker:
```md
git clone https://github.com/oracle/docker-images

cd OracleDatabase/SingleInstance/dockerfiles/\n./buildContainerImage.sh -v 19.3.0 -e

run -d --name oracle19 -e ORACLE_PWD=mypassword -p 1521:1521 oracle/database:19.3.0-ee
```
## Install Oracle Client in your Macos:
https://odpi-c.readthedocs.io/en/latest/user_guide/installation.html#scripted-installation
1: Scripted Installation
if Your Download folder 2.4.3. Configure Oracle Instant Client (read carefully this if Download folder is not the default may not have access)
```bash
ln -s /opt/oracle/instantclient_/libclntsh.dylib ~/myprograms
```

## Test connection:
1: Install SQL Developer in VScode:
```md
Username: SYS
Role: SYSDBA
Hostname: localhost
Port: 1521
Service Name: ORCLCDB
```

2: Install SQlPlus:
```bash
sqlplus /nolog

conn conn SYS/mypassword1@localhost:1521/ORCLCDB as SYSDBA
```