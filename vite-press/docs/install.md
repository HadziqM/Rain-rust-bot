---
title: Installation
lang: en-US
---

# How To Install
for installation you need to get exutable that could run on your server, 
Then you need to fill the config.json to run the bot with your config.

## Get The App Exutable
There is two ways to install this preject into your server.


### No Brainer Way
This way is easiest way but only works if platform you use is already covered on github release.<br/>
1. download the release match your platform
2. extract the zip
3. fill out `config.json`
4. run the exutable

### Chad Way
if you want build or edit from source, or your platform isnt covered on release. <br/>
you need following tool isntalled on your OS


- Rust latest stable release
- Toolchain for your target os
- git
- curl

for windows you will need to have `linker` to build rust project <br/>
detail of how to install rust and cargo is covered in rust book [here](https://doc.rust-lang.org/book/ch01-01-installation.html)<br/><br/>

If you have mentioned tools installed then you can follow this step to install
1. download any release you want (its already contain source code)
2. extract the zip
3. build release the source and place it on main folder <br/>
<br/>
for `MacOs` or `linux`
```shell
cargo build --release
cp ./target/release/rain-bot .
```
for `windows`
```shell
cargo build --release
copy ./target/release/rain-bot.exe .
```

## How to edit config.json
there is something that need to be filled before running the bot <br/>

```json
{
  "discord": {
    "token":"Your Token",
    "prefix":"?",
    "author_id":455622761168109569
   },
  "postgress": {
    "user":"postgres",
    "host":"localhost",
    "password":"",
    "port":6432,
    "database":"erupe"
   },
  "mhfz_config": {   
    "account_creation":true,
    "sending_log":false
   },
  "bot_config": {
    "member_join":true,
    "member_leave":true,
    "gacha":true,
    "bounty":true,
    "server_market":true
   },
  "log_channel": {
    "err_channel":1031774270512169070,
    "account_channel":1031774270512169070,
    "transfer_channel":1031774270512169070,
    "info_channel":1031774270512169070,
    "info_channel_msg":1077951127317921874,
    "market_channel":1031774270512169070,
    "market_channel_msg":1076848380971589634,
    "erupe_channel":1031774270512169070,
    "request_channel":1031774270512169070,
    "join_channel":1031774270512169070,
    "leave_channel":1031774270512169070
   },
  "server_role":{
    "admin_role":1031595216538452038,
    "bartender_role":1017643913667936318,
    "member_role":0,
    "register_role":1017643913667936318,
    "judge_role":1017643913667936318,
    "maintainer_role":1017643913667936318
  },
  "bounty": {
    "board_ch":1031774270512169070,
    "conquered_ch":1031774270512169070,
    "promotion_ch":1031774270512169070,
    "cooldown_ch":1031774270512169070,
    "cooldown_msg":1081347237566750740,
    "leaderboard_ch":1031774270512169070,
    "receptionist_ch":1031774270512169070,
    "judge_ch":1031774270512169070
   }
}
```
The `discord` and `postgress` section is self explained, you must fill them properly,<br/>
As for `mhfz_config` fill the one that matched your server configuration, the send log is working but experimentall, it will be explained further in [Server Monitor](/guide/server),<br/>
Then `bot_config` is to enable and disable bot features, set them to match your prefferences,<br/>
The `log_channel` section consist the channel the bot will operate based on config, so some can be ignored if you set it false in`bot_config`,
The `server_role` section consist role that bot will give access command to, `admin` for admin command,`register` to give register role and `judge` for save file judge is the must filled field here,<br/>
Lastly `bounty` section can be ignored if you set bounty field on `bot_config` to false
