# Raspberry Pi Setup

1. Download the executables from github, or clone the repository and run `cargo build --release`.
2. Download and install PostgreSQL 
```shell
sudo apt install postgresql libpq-dev postgresql-client postgresql-client-common -y
```
3. Create a user

```shell
> sudo su postgres
> createuser pi -P --interactive
```
4. Create a database called `lol_stocks`
```shell
$ psql
> create database test;
```
5. Setup Discord bot to run at startup
```shell
> cd /usr/lib/systemd/system
> sudo nano lol-stocks-discord@.service
```
6. In the file enter the below, changing `ExecStart` to the location of the executable, `Environment="DATABASE_URL` to your DB url, and `Environment="DISCORD_TOKEN` to your discord token.
```shell
### BEGIN INIT INFO
# Provides:          lol-stocks-discord
# Required-Start:    $all
# Required-Stop:
# Default-Start:     2 3 4 5
# Default-Stop:
# Short-Description: lol stocks discord server
### END INIT INFO

[Unit]
Description=%I lol-stocks-discord-bot
After=multi-user.target
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=location/of/program %I --no-prompt
User=pi
Group=pi
Type=idle
Restart=always
RestartSec=15
RestartPreventExitStatus=0
TimeoutStopSec=10
Environment="DATABASE_URL=ADD YOUR DB URL HERE"
Environment="DISCORD_TOKEN=ADD YOUR DISCORD BOT TOKEN HERE"
Environment="GRAPH_LOCATION=/location/for/graphs/here"

[Install]
WantedBy=multi-user.target
```
7. Setup Discord bot to run at startup
```shell
> sudo nano lol-stocks-web-server@.service
```
8. In the file make the same changes as above
```shell
### BEGIN INIT INFO
# Provides:          lol-stocks-web
# Required-Start:    $all
# Required-Stop:
# Default-Start:     2 3 4 5
# Default-Stop:
# Short-Description: lol stocks web server
### END INIT INFO

[Unit]
Description=%I lol-stocks-web-server
After=multi-user.target
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=location/of/program %I --no-prompt
User=pi
Group=pi
Type=idle
Restart=always
RestartSec=15
RestartPreventExitStatus=0
TimeoutStopSec=10
Environment="DATABASE_URL=ADD YOUR DB URL HERE"


[Install]
WantedBy=multi-user.target

```
9. Enable the service's so they run at startup
```shell
sudo systemctl enable lol-stocks-discord@main
sudo systemctl enable lol-stocks-web-server@main
```
10. You will be able to check an output but running
```shell
> sudo journalctl -eu lol-stocks-web-server@main
> sudo journalctl -eu lol-stocks-discord@main
```
11. To access the webserver, you will need to open port `8080`
12. Last step is to seed the database with teams. An example JSON payload can be found [here](teams_db_seed.json) which contains all the teams from the LEC and LCS.
13. If you wish to use the graphs `fontconfig` needs to be installed. If you have set up the Pi without a GUI, it might not be. To set it up, run:
```shell
sudo apt-get install fontconfig-config
```
14. Graphs also require for the `GRAPH_LOCATION` environmental variable key to be set.