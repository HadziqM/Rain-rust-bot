---
title: Home
lang: en-US
---


# About This Project

This project is discord bot that used in rain server mainly to make easier to modarate player, 
the main target of this bot is for Monster Hunter Frontier server that use [Erupe](https://github.com/ZeruLight/Erupe) as server emulator, 
and for the records this is the translation of my typescript discord bot, it will be same will have more feature and more stable.

## Why Rewrite?

Typescript bot is good enough, but the node module eat 800Mb of storage, also a bit heavy on ram usage since it use web API for some stuff, 
also there is some cant be resolved bug because of dependency and inconsistence of Javacript type build conversion from Typescript.<br/><br/>
I use `Rewrite it With Rust` approach to solve those problems, Rust are strongly typing language with unmatched performance,
the concurency in rust way more secure with `Send and Sync` trait between thread.

### The Main Difference Feature
- Easy to maintain, error is handled properly and unit tested
- No dependency issue, using compiled language will produce exutable
- Easy to develop, using my macro will simplify lot of things
- Faster, using rust will definitely get you faster on benchmarking
- Added more feature
- Easy to setup, download the release, edit config.json and run exutable
- I want learn rust, so this is my playground

### Main Drawback
- Need to translate huge amount of code
- Using main dependency `serenity` is not complete yet for `current` realeased branch, so i use `next` branch
- Using its `next` unrealeased branch risk on breaking on github change, and it doesnt have documentation
- Compile time is depresing
- Still using third-party dependency for `asynchronous` with `tokio`, this will be no problem i guess

## Project Status
Still Testing.....<br/>
There is still lot untested command but developing is pretty much completed

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
* [x] change guild interface command
* [ ] add cheater scan command
* [x] make more beautiful bounty board


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
* [x] setup image processing for bounty
* [x] cached sahred process
