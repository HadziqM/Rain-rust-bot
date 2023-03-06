---
title: Commands
lang: en-US
---

# The Command That Exist In Bot

## Slash/Context Menu Command
the implemented command right now
1. Admin Command
   - `/interface` to send register and send save button
   - `/config` to change bot config, there is many types of config can be set, you can refer this to [Json Config Guide](/guide/json)
   - `/send` to send items/equipment to player using distribution, can be with or without bounty coin
   - `/purge` to delete all player binded data with discord
   - `/add` to add or deduct (if negative) players bounty coin or gacha ticket
   - `/monitor` to toogle server monitor, refer to [Monitor Guide](/guide/server)
   - `/mod_pass` to change user ingame password
   - `/test` to test image processing for title, reffer ti [Title Config](config/title)
   - `/distribution` to send bounty reward bypassing its rule
   - `/cooldown`had many subcommand mainly to change cooldown for bounty event
2. Binded Command
   - `/card` to show user hunter status
   - `App > Card` to check other user hunter status
   - `/event` to show user event status
   - `App > Event` to check other user event status
   - `/dm_save` to send user their backup save file on dm
   - `/transfer` for transfer save file, save need to be approved by admin and, will be automatically accepted after 5 minutes
   - `/guild` for player to join guild bypassing in game restriction, or to see guild info
3. [Register Command](/guide/register)
   - `/switch` to switch their main character for event
   - `/change_password` to change password
   - `/check` to check own username and id
   - `/create` to create blank account on MHFZ
   - `/bind` to bind existing account on MHFZ if autocreation is enabled
4. Gacha Command
   - `/pull` to perform gacha command if gacha enabled the config for gacha can be find on [Gacha Config](/config/gacha)
5. Trading Command
   - `/trading` consist many subcommand for trading refer to [Trading Guide](guide/trading)
6. Bounty Command
   - `/bounty` to submit your bounty or to see bounty description with pedia
   - `/App > Submit`the alternative version of submit command using context menu
7. Nice To Have Command
   - `/ferias` to link you ferias item matched your input.

## Prefix/Message Commands
1. Anyone Can Use
   - `<prefix>ping` to see discord connection ping,postgres conn ping, and json parsing file system speed
   - `<prefix>test` to show test embed (you can use this to set up bot config,copy its message id and channel id)
   - `<prefix>tag` to use tag command, reffer to [Tag Guide](guide/tag)
2. Admininstrator Role Only
   - `<prefix>query` to query server database, reffer to [Query Guide](guide/tag)
   - `<prefix>execute` to execute code to server database, reffer to [Query Guide](guide/tag)
