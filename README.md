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
most is not implemented yet so you can leave it as it is except some few </br>
i will mark what you need to cahnge on the pointer `<-` comment </br>
just dont fill the actual file with my comment

```json
{
  "discord": {
    "token":"your token",              <- Change it with your discord token
    "prefix":"%",
    "author_id":455622761168109569     <- Change it with your discord id or leave it
   },
  "postgress": {                       <- all of this section need to change
    "user":"postgres",                 
    "host":"localhost",
    "password":"",
    "port":5432,
    "database":"erupe"
   },
  "mhfz_config": {   
    "account_creation":true
   },
  "bot_config": {
    "member_join":true,
    "member_leave":true,
    "role_moderation":true,
    "member_moderation":true,
    "gacha":true,
    "bounty":true,
    "transmog_contest":true,
    "mezfes_contest":true,
    "server_market":true,
    "pvp_contest":true,
    "speedrun_contest":true
   },
  "log_channel": {
    "err_channel":1031774270512169070,         <- fill with channel ID for bot to send
    "account_channel":1031774270512169070,     <- fill with channel ID for bot to send
    "transfer_channel":1031774270512169070,    <- fill with channel ID for bot to send
    "moderation_channel":0
   },
  "server_channel": {
    "member_join":0,
    "member_leave":0,
    "rule_channel":0,
    "rule_msg_id":0
  },
  "server_channel_url": {
    "guide_channel":0,
    "game_channel":0,
    "bot_channel":0
   },
  "server_role":{
    "admin_role":0,
    "member_role":0,
    "mute_role":0,
    "register_role":1031595216538452038     <- fill with role id, bot need to be higher
  },
  "bounty_channel": {
    "board":0,
    "conquered":0,
    "promotion":0,
    "cooldown_ch":0,
    "leaderboard_ch":0,
    "judge_ch":0
   },
  "bounty_message_id": {
    "cooldown_msg":0,
    "leaderboard_msg":0
   },
  "gacha_channel": {
    "pull":0
  },
  "transmog_contest":{
    "submitted_channel":0
  },
  "mezfes_contest" :{
    "leaderboard_channel":0,
    "leaderboard_msg_id":0
  },
  "pvp_contest" :{
    "leaderboard_channel":0,
    "leaderboard_msg_id":0
  },
  "speedrun_contest" :{
    "leaderboard_channel":0,
    "leaderboard_msg_id":0
  },
  "server_market" :{
    "market_channel":0
  }
}
```

## Other Config
there will be upcoming `Rain Moderation App` for further customzation (mainly for event and image editing)

## Detailed Command
the implemented command right now
1. Admin Command
   - `/interface` to send register and send save button
   - `/reset_save_cd` to reset user save cooldown
2. Binded Command
   - `/card` to show user hunter status
   - `App>Card` to check other user hunter status
   - `/dm_save` to send user their backup save file on dm
   - `/transfer` for transfer save file, save need to be approved by admin and have cd
3. Register Command
   - `/switch` to switch their main character for event
   - `/change_password` to change password
   - `/check` to check own username and id
   - `/create` to create blank account on MHFZ
