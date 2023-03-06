---
title: Server Monitor
lang: en-US
---

# Server Monitor

## About
One of feature bot had is to monitor mhfz server status also sent its log file, this work by looking into database each minutes then send its result in `info_channel` channel editing `info_channel_msg`, so those config covered in [Installation](/install) must be filled for bot to run.

## History
This feature added because Erupe is still under development, in case server crash, bot will inform you and send the log if `send_log` on [Installation](/install) section is enabled this will helpfull for Erupe developer to analyze the server issues, since Erupe still doesnt have away to log its process into file, there is some bash command to capture `Stdout` and `StdErr` of Erupe while on run, the file could exceed 4GB (on experiment done by Rain Server) in a day using that bash command, The bot could send and reset the log file in every 5 minutes to avoid populating server memory (still on test) if the `send_log` enabled (i suggest you disabled it untill further notification).

## How to use
The monitor is enabled by default so you can get pinged on crash notification if you fill `maintainer_role` on [Installation](/install). but you could disabled it by using `/monitor` command if youare server admin, and use it again to anabled it back.

### Log File
First you need to make `log.txt` in bot folder for bot to send a file, those only placeholder for actual log.<br/>
Then on the erupe folder you need to use this command to log file while running erupe
```shell
erupe-ce 2>&1 | tee -a <path>/log.txt
```
the path its mentioned is the bot's forder path, so for example you can use this if bot's folder is inside erupe folder
```shell
erupe-ce 2>&1 | tee -a ./Rain-rust-bot/log.txt
```
::: warning
for windows, the path separator will use `\` instead `/` for unix
:::
