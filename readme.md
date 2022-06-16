sysmon_bot (System Monitor Bot)
===

Telegram bot that monitors host system state and notifies subscribers when certain readings (eg. cpu load/temperature) exceed configured thresholds

Build & run in docker
---

Prerequisites: (1) [docker](https://www.docker.com/), (2) git, (3) unix shell (e.g. bash).

```sh
git clone https://github.com/art-in/sysmon_bot
cd sysmon_bot

./docker/scripts/build.sh        # build inside container
./docker/scripts/run.sh          # run inside container
```


Run prebuilt docker image
---

Prerequisites: (1) [docker](https://www.docker.com/).

```sh
# download image from docker hub and run it (set params first)
docker run -di \
    --name sysmon_bot \
    --restart unless-stopped \
    --mount type=bind,src=<host_dir>/config.toml,dst=/opt/project/config.toml \
    --mount type=bind,src=<host_dir>/,dst=/opt/project/db/ \
    --pid="host" \
    artinphares/sysmon_bot
```