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
* [ ] basic `serenity` handling
* [ ] sqlx interface binding
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
* [ ] create guide embed command
* [ ] add emojis (too lazy to install emoji plug for my vim)
* [ ] create custom build command
* [ ] role specific command
* [ ] get attachment and send attachment
* [ ] make bind command easier
* [ ] change guild interface command
* [ ] add cheater scan command
* [ ] make more beautiful bounty board


### Sqlx Interface Binding
* [ ] install `Sqlx` crate
* [ ] make .sql file for first initialize and update
* [ ] make command to execute .sql init
* [ ] handle all database related (register,bounty,gacha)
* [ ] error handling
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
