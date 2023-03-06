---
title: Trading Guide
lang: en-US
---

# Trading Commands
trading command is the new feature introduced in this bot, there is some section that will be covered in this page, but generally trading command is grouped command used for trading between player and server, player can get discount on every trading command except the bar using `Title benefit` gained trough [Bounty](./bounty), to enable this command you need to set `server-market` to true on json config mentioned on [Installation Guide](../install). while there is 6 command total you could setting it so some are disabled or enabled trough [Trading Config](../config/trading)

## Market
this command using `/trading market`, the field `market_channel` and `market_channel_msg` on [Installation Guide](../install) need to be filled in order this command to fully functional, this command is for player so they could buy listed item on the [Config File](../config/trading), each item had stock so you need to chnage the config to restock or change the item list, player can select item they wanted and input quantity they want to buy, 
the player given the `receipt` of their transaction including details of what they buy and the price, discount also listed if they had, then player can press `confirm` to confirm their transaction or `cencel` to cencel it.

## Restourant
this command using `/trading restourant`, the fuction is for the player to buy guild food for for guilds they are on, the price is depend on the duration the food lasted, the price can be set by [Config](../config/trading) and the menu list can be change or updated trough meal section covered in [Config](../config/trading) but i sugested just keep default config by Ten from Rain server.
like market command player will be promted with `Receipt` to confirm their transaction.

## Bar
this command using `/trading bar`, the function is for player to post their request, player can type the items/equipment they want also filled the rerquested price to `bartender_role`, those request will be posted on the `request_channel` new thread and both bartender and player could discus togetheir regarding their request, btw since this is custom request player discount is inactive and admin could send item/equipments to player with `/send` command (those command is fixed and player will be charged correctly).

## Casino
this command use `/trading casino`, this one used to buy gacha ticket for `/pull` command, the price can be adjusted on [Trading Config](../config/trading). this command also promp `Receipt`

## Guild
this command use `/trading guild`, this one used to buy guid RP for guild player currently in, the price can be adjusted on [Trading Config](../config/trading). this command also promp `Receipt`

## Jewelry
this command use `/trading jewelry`, this one used to buy `gacha premium` used for upgrading caravan gem, skiping my mission or gacha cat ingame , the price can be adjusted on [Trading Config](../config/trading). this command also promp `Receipt`
