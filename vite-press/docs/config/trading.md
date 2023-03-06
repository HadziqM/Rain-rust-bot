---
title: Trading Config
lang: en-US
---

# How To Set Trading Command
first you need the json file that matched tag config. you can get the placeholder file [here](https://github.com/HadziqM/Rain-rust-bot/tree/senpai/static)
, there is some file that will be covered in this section.

## Trading Config
This is the main file that configure the `/trading` command behavior, you need to pick the file named `trading.json`.
```json
{
  "market": {
    "enabled": false,
    "price": 100
  },
  "bar": {
    "enabled": false,
    "price": 100
  },
  "casino": {
    "enabled": true,
    "price": 100
  },
  "jewelry": {
    "enabled": false,
    "price": 100
  },
  "restourant": {
    "enabled": true,
    "price": 750
  },
  "guild": {
    "enabled": false,
    "price": 100
  }
}
```
the content is stright forward, firstly the `enabled` will set the subcommand availability on the trading command, so if you set market to `false` the `/trading market` will respond to player that `Market is currently closed`.<br/>
the price field is stright forward, its the price of the product in the command (This doesnt effect `/trading market` and `/trading bar` since they have their own price set), example of `casino` price is the price for 1 gacha ticket cost

if you done modifying your json file you can send it to bot by using `/config trading`, and the `/trading` command will be updated automatically, if you experience error on uploading, refer to [Json Config Guide](../guide/json)

## Meals Config
this is the file that config the `Restourant` menu will serve, you will need the file named `meals.json`, the content contains list of this field
```json
    {
      "id": 369,
      "level": 3,
      "name": "idk really"
    }
```
the `id` is database meal id, the `level` is meal level and `name` is the food name that will appear on the discord while using `/trading restourant`

if you done modifying your json file you can send it to bot by using `/config meals`, and the `/trading restourant` menu will be updated automatically, if you experience error on uploading, refer to [Json Config Guide](../guide/json)

## Market Config
this on will be the list of item on `Market` subcommand, you need to get file named `market.json` on my site, much like meals the content only consist list of field
```json
    {
      "item": {
        "key": "0000",
        "count": 10,
        "types": 7
      },
      "treshold": 100,
      "price": 1000
    }
```
The `item` property there is the distribution data database use, the `types` is the types of item/equipment
```
0 = legs
1 = Head
2 = Chest
3 = Arms
4 = Waist
5 = Melee
6 = Ranged
7 = Item
```
the `key` is item value, you can see the item value using `/send` command, and since this code make use of [Chakratos Save Manager](https://github.com/Chakratos/mhf-save-manager/tree/master/app/I18N/en_GB) you can check his repo and find the `key` value there.<br/>
the `count` is the item quantity (bundled) for equipment you must set the value to 1, value beside it will break the game, as for item you can set between
`1 - 166535`. <br/>

The `treshold` field mean the item stock for market you can set it as many but never set it to negative.
then the `price` is the item price per bundle you dont want to set it to negative either.<br/>

if you done modifying your json file you can send it to bot by using `/config market`, and the market stall message will be updated automatically, if you experience error on uploading, refer to [Json Config Guide](../guide/json)

