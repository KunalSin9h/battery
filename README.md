# 🪫 Battery

Utility application for Low battery warning in [i3wm](https://i3wm.org/), It is currently bare-minimum, can be updated 
as time passes and requirements arise wile using it. 

<div align="center">
    <image src="https://i.imgur.com/WCbHwmR.png" alt="Demo image" />
</div>

> [!CAUTION]
> Use only on i3wm, as it does not have buttons for minimizing / closing on screen (TODO).

### Setup

Compile the binary as `battery`, and put it in `/usr/local/bin`.

Hence, on running this command.

```bash
which battery
```

you got 

```bash
/usr/local/bin/battery
```

Then can go ahead.

Create a new `systemd` service file.

```bash
sudo vim /etc/systemd/system/battery.service
```

Paste this in it.

```service
[Unit]
Description=Battery Warning

[Service]
Type=simple
User={YOUR_USERNAME}
Environment=DISPLAY=:0
Environment=XAUTHORITY=/home/{YOUR_USERNAME}/.Xauthority
Environment=XDG_RUNTIME_DIR=/run/user/1000
ExecStart=/usr/local/bin/battery

[Install]
WantedBy=multi-user.target
```

start service about new service

```bash
sudo systemctl start battery
```

Run it 24 / 7 all the time by,

```bash
sudo systemctl enable battery
```
