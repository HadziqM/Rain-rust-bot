# What is this?
this is the translation of my typescript discord bot, it will be same but more stable

## why do that?
well, i m just newbie programer and my last bot is just work, although sometime it produce ghost bug since long ago i dont know how to do thing properly

### the main difference feature of this will be
- easy to maintain, error is handled properly and unit tested
- no dependency issue, using compiled language will produce exutable
- easy to develope, since its more readable now?
- faster? using rust will definitely get you faster?
- added more feature
- easy to setup, download the release, edit config.json and run exutable
- i want learn rust, so this is my playground

### main drawback
- need to translate huge amount of code
- using main dependency `serenity` will be hard since its not really documented yet
- compile time is depresing
- literally no helper, `serenity` comunity is still so small, it will be hard if you face a problem
- still using third-party dependency for `asynchronous`, this will be no problem i guess


## Project Status
100% work in progress

## Project Roadmap
* [x] initialize project
* [x] basic `serenity` handling
* [x] sqlx interface binding
* [ ] image processing binding

### Initialize Project
* [x] setup dependency tree
* [x] setup easy code framework to handle
* [x] setup unit testing
* [x] setup github action

### Basic Serenity handling
* [x] setup token and gateway
* [x] setup ready event handler
* [x] basic slah command runner and register
* [x] register all slash command
* [x] unhandled error send to discord channel
* [x] implement button push create and handler
* [x] implement basic embed rich builder
* [x] modal builder for register command
* [x] handle modal interaction
* [ ] implement auto complete for guild command
* [x] create guide embed command
* [x] add emojis
* [ ] create custom build command
* [x] role specific command
* [x] get attachment and send attachment
* [ ] make bind command easier
* [ ] change guild interface command
* [ ] add cheater scan command
* [ ] make more beautiful bounty board


### Sqlx Interface Binding
* [x] install `Sqlx` crate
* [x] make .sql file for first initialize and update
* [x] make command to execute .sql init
* [ ] handle all database related (register,bounty,gacha)
* [x] error handling
* [ ] make testing database (may or may not)
* [ ] make update command to extend the bot
* [ ] make new table for image storing and processing
* [ ] extend for next project `rain server admin tools` for image editing, and basic game moderation

### Image procesing crate
* [ ] install `Image` crate
* [ ] learn basic ROI and IO
* [ ] setup image processing for member join event handler
* [ ] setup image processing for gacha pull
* [ ] setup image processing for bounty


## How to install
there is some 

### No briner
1. download the release
2. extract the zip
3. fill out `config.json`
4. run the exutable


### Chad
if you want build or edit from source, or the exutable isnt working on release </br>
you need following tool isntalled on your OS 

- Rust
- cargo
- git
- curl

for windows you need to have `linker` to build rust project </br>
detail of how to install rust and cargo is covered in rust book [here](https://doc.rust-lang.org/book/ch01-01-installation.html)

#### from release
1. download any release you want (its already contain source code)
2. extract the zip
3. build release the source and place it on main folder </br>
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
4. edit the `config.json` file
5. run the exutable

### from sorce
well if you want to stay upto date, you can follow this step
1. clone the repository
2. build release, place the exutable on main then rename example file </br>
for `MacOs` or `linux`
```shell
cargo build --release
cp ./target/release/rain-bot .
mv ./config.json.example config.json
```
for `windows`
```shell
cargo build --release
copy ./target/release/rain-bot.exe .
ren ./config.json.example config.json
```
3. edit `config.json` file
4. run the exutable


## How to edit config.json
well i will explain on the pointer `<-` comment </br>
just dont fill the actual file with my comment

```json
{
  "discord": {
    "token":"",                   <- your discord token from discord developer portal
    "prefix":"%"                  <- your normal command prefix, fill with anything you like
   },
  "postgress": {             
    "host":"",                    <- your postgress host address that host mhfz server
    "password":"",                <- your postgress database password
    "port":5432,                  <- your postgress port
    "database":"erupe"            <- your database name
   },
  "mhfz_config": {
    "account_creation":true       <- if you want bot to perform account creation (effect behavior)
   },
  "bot_config": {                 <- need to specify behavior of bot
    "member_join":true,           <- if you want bot to send greet message on member join
    "member_leave":true,          <- if you want bot to inform you someone leave
    "role_moderation":true,       <- if you want bot to moderate giving main role (bot need to be on higher role)
    "member_moderation":true,     <- if you want bot to aid in moderate member
    "gacha":true,                 <- if you want bot to perform gacha (effect database)
    "bounty":true,                <- if you want bot to perform bounty event (effect databse)
    "transmog_contest":true,      <- if you want bot to perform transmog event (effect database)
    "server_market":true,         <- if you want bot to perform server market event (effect database)
    "mezfes_contest":true,        <- if you want bot to perform mezfes event (effect database)
    "pvp_contest":true,           <- if you want bot to perform pvp event (effect database)
    "speedrun_contest":true       <- if you want bot to perform speedrun event (effect database)
   },
  "log_channels": {               <- fill all this section with channel id in string format
    "err_channel":"",             <- for bot to inform you if there is problem
    "account_channel":"",         <- log for account creation,can leave it empty
    "transfer_channel":"",        <- log for transfer channel, can leave it empty
    "moderation_channel":""       <- for bot to announce moderation behavior (ban,mute,kick,etc.)
   },
  "server_channel": {             <- you can leave this empty depend on how you fill bot config section
    "member_join":"",             <- channel id for bot to greet new member
    "member_leave":"",            <- channel id for bot to inform when member leave
    "rule_channel":"",            <- channel id to place rule channel and member button
    "rule_msg_id":"",             <- message id on rule channel so you can edit rule whenever you want
  },
  "server_channel_url": {         <- you can leave this empty depend on how you fill bot config section

    "guide_channel":""            <- the channel url for redirect button to those channel when pressed
    "game_channel":""             <- the channel url for redirect button to those channel when pressed
    "bot_channel":""              <- the channel url for redirect button to those channel when pressed
   },
  "server_role":{
    "admin_role":"",              <- the role id only for one that could use admin only command
    "member_role":"",             <- giving member role if you lock server could be accessible that member role
    "mute_role":""                <- role to give for muting member
  },
  "bounty_channel": {             <- will be ignored if your fill false on bot_config.bounty
    "board":"",                   <- channel id for bounty board
    "conquered":"",               <- channel id for bounty conquered
    "promotion":"",               <- channel id for bounty promotion
    "cooldown_ch":"",             <- channel id for bounty cooldown
    "leaderboard_ch":"",          <- channel id for bounty leaderboard
    "judge_ch":""                 <- channel id for bounty judge
   },
  "bounty_message_id": {          <- will be ignored if you fill false on bot_config.bounty
    "cooldown_msg":"",            <- message id on cooldown channel for bot to update cooldown
    "leaderboard_msg":""          <- message id on leaderboard channel for bot to update leaderboard
   },
  "gacha_channel": {              <- will be ignored if you fill false on bot_config.gacha
    "pull":""                     <- gacha channel id, leave it empty if you dont want to restrict
  },
  "transmog_contest":{            <- will be ignored if you fill false on bot_config.transmog
    "submitted_channel":""        <- contest channel id for show off
  },
  "mezfes_contest" :{             <- will be ignored if you fill false on bot_config.mezfes
    "leaderboard_channel":"",     <- leaderboard channel id
    "leaderboard_msg_id":""       <- leaderboard channel message id to update message
  },
  "pvp_contest" :{                <- will be ignored if you fill false on bot_config.pvp
    "leaderboard_channel":"",     <- leaderboard channel id
    "leaderboard_msg_id":""       <- leaderboard channel message id to update message
  },
  "speedrun_contest" :{           <- will be ignored if you fill false on bot_config.transmog
    "leaderboard_channel":"",     <- leaderboard channel id
    "leaderboard_msg_id":""       <- leaderboard channel message id to update message
  },
  "server_market" :{              <- will be ignored if you fill false on bot_config.transmog
    "market_channel":""           <- market channel id to show wares
  },
}
```

## Other Config
there will be upcoming `Rain Moderation App` for further customzation (mainly for event and image editing)

## Detailed Command
**WIP**
