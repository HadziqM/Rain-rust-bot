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
* [x] image processing binding

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
* [x] implement auto complete for guild command
* [x] create guide embed command
* [x] add emojis
* [x] create custom build command
* [x] role specific command
* [x] get attachment and send attachment
* [x] make bind command easier
* [ ] change guild interface command
* [ ] add cheater scan command
* [ ] make more beautiful bounty board


### Sqlx Interface Binding
* [x] install `Sqlx` crate
* [x] make .sql file for first initialize and update
* [x] make command to execute .sql init
* [x] handle all database related (register,bounty,gacha)
* [x] error handling
* [x] make testing database (may or may not)
* [x] make update command to extend the bot

### Image procesing crate
* [x] install `Image` crate
* [x] learn basic ROI and IO
* [x] setup image processing for member join event handler
* [x] setup image processing for gacha pull
* [ ] setup image processing for bounty
* [x] cached sahred process


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
    "token":"your token",
    "prefix":"%",
    "author_id":455622761168109569
   },
  "postgress": {
    "user":"postgres",
    "host":"localhost",
    "password":"",
    "port":5432,
    "database":"erupe"
   },
  "mhfz_config": {   
    "account_creation":true,                    <- set this according to your erupe client config
    "sending_log":false                         <- you need to run bash script to create log and move it to bot folder named ./log.txt
   },
  "bot_config": {                               <- bot config to be added on server
    "gacha":true,
    "server_market":true
   },
  "log_channel": {
    "err_channel":1031774270512169070,           <- channel to log bot error (dont make it private)
    "account_channel":1031774270512169070,       <- channel to log user account created
    "transfer_channel":1031774270512169070,      <- channel to judge player save file (make it private)
    "info_channel":1031774270512169070,          <- channel for server monitor
    "info_channel_msg":1070671548408664124,      <- message in info channel id to be edited continiusly
    "erupe_channel":1031774270512169070,         <- channel to dump log file
    "market_channel":1031774270512169070,        <- you can ignore this if server_market is false
    "market_channel_msg":1076848380971589634,    <- same as above
   },
  "server_role":{
    "admin_role":1031595216538452038,
    "register_role":1031595216538452038,
    "judge_role":1017643913667936318,            <- role pinged on transfer savefile
    "maintainer_role":1017643913667936318        <- role pinged on server crash
  }
}
```

## Other Config
there will be upcoming `Rain Moderation App` for further customzation (mainly for event and image editing)

## Detailed Command
the implemented command right now
1. Admin Command
   - `/interface` to send register and send save button
   - `/config` to change bot config
   - `/market` to send item to player, can be with or without bounty coin
   - `/purge` to delete all player binded data with discord
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
   - `/bind` to bind existing account on MHFZ if autocreation is enabled
4. Gacha Command
   - `/pull` to perform gacha command if gacha enabled
5. Market Command
   - `/stall` for player to buy item in stall set in config if server market enabled
6. Admin Only Message Command
   - `<Prefix>query` add codeblock affterward to query your sql code on server database and get the decoded result
   - `<Prefix>excute` add codeblock affterward to execute your code on server database
7. Some Usefull Message Command
   - `<prefix>ping` to see discord connection ping,postgres conn ping, and json parsing file system speed
   - `<prefix>test` to show test embed (you can use this to set up bot config,copy its message id and channel id)
## For developer
`senpai` branch is used for release candidate and you can clone from there</br>
`kohai` branch is used for development, you can pull request there</br>
`sensei` branch is used for release, solely for building exutable and github action</br>

### About Hertz Library
the library is only for procedural macro attributes, most of macro will need 2 attribute, one is integer for cooldown,</br>
and second is boolean for defer (if you think command takes more than 3 sec), the macro only accept function with arguments, </br>
`Mybundle` trait, so you can use `SlashBundle`,`ComponentBundle`, `ModalBundle` or the trait itself if you want compount command </br>
the macro also only accept return type `Result<(),MyErr>`, the macro will produce the public async function with name </br>
`discord_<function name>` with no return type, and you can use it on event handler.
