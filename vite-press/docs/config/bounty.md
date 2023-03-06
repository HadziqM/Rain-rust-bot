---
title: Bounty Config
lang: en-US
---

# How To Set Bounty Challange

## Category Config
there is 5 json file used by bot for bounty and they are named after their bounty category
```
bronze_bounty.json = bronze category
silver_bounty.json = silver category
gold_bounty.json = gold category
free_bounty.json = free category
event_bounty.json = event category
```
and you can get those placeholder file [here](https://github.com/HadziqM/Rain-rust-bot/tree/senpai/static),<br/>

despite different name each one of them literally same config, they consist of list of 25 `Bounty Field` you can focus in that field, but never extend or shrink the list since its need to be perfectly 25 field
```json
    {
      "bbq": 0,
      "description": "Musou Deviljho (Behold the Nocturnal Celebration / Behold the Nocturnal Event)",
      "cooldown": 1,
      "icon": "https://media.discordapp.net/attachments/962787104356712448/1081632143412834375/bbq.png",
      "thumbnail": "https://media.discordapp.net/attachments/1073906822559301693/1073907039304155166/BBQ02.png?width=1181&height=657",
      "rules": [
        "HR equipment only",
        "no MS",
        "naked"
      ],
      "solo": {
        "coin": 1,
        "ticket": 1,
        "items": [
          {
            "key": "0700",
            "count": 1,
            "types": 7
          },
          {
            "key": "0700",
            "count": 1,
            "types": 7
          }
        ]
      },
      "multi": {
        "coin": 1,
        "ticket": 1,
        "items": [
          {
            "key": "0700",
            "count": 1,
            "types": 7
          },
          {
            "key": "0700",
            "count": 1,
            "types": 7
          }
        ]
      }
    }
```
the `bbq` field is the bbq code (never edit it and leave it be) for the record its BBQ code 0 mean BBQ01 and 24 is BBQ25<br/>
`cooldown` field is for bounty avalbility (use any positive value) to make it active and use 0 if you want to disable the BBQ<br/>
`description` is the bounty description.<br/>
`icon` is bounty icon (preferably monster icon) link. <br/>
`thumbnail` is bounty thumbnai or its actual picture link.<br/>
`rules` is list of rules for player to take the bounty. <br/>
`solo` and `multi` is the reward for clearing bounty by multiplayer or solo respectively, the `coin` inside it is bounty coin earned, `ticket` is gacha ticket earned, the `item` property there is the distribution data database use, the `types` is the types of item/equipment
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

for example above the bounty result will be from using`/bounty pedia`
![image](https://media.discordapp.net/attachments/1031774270512169070/1082293432590028860/image.png?width=591&height=658)

if you done modifying your json file you can send it to bot by using `/config bounty` and select the category that need to be configure, and bounty will be updated automatically , if you experience error on uploading, refer to [Json Config Guide](../guide/json)

## Refresh Config
this one used for `/cooldown refresh` to refresh Free category Steak, you can find the config file on placeholder [here](https://github.com/HadziqM/Rain-rust-bot/tree/senpai/static) named `bounty_refresh.json` and the content of the file is self explained.
```json
{
  "bbq1": 1,
  "bbq2": 1,
  "bbq3": 1,
  "bbq4": 1,
  "bbq5": 1,
  "bbq6": 1,
  "bbq7": 1,
  "bbq8": 1,
  "bbq9": 1,
  "bbq10": 0,
  "bbq11": 0,
  "bbq12": 0,
  "bbq13": 0,
  "bbq14": 0,
  "bbq15": 0,
  "bbq16": 0,
  "bbq17": 0,
  "bbq18": 0,
  "bbq19": 0,
  "bbq20": 0,
  "bbq21": 0,
  "bbq22": 0,
  "bbq23": 0,
  "bbq24": 0,
  "bbq25": 0
}
```
its pretty musch the Steak restored on each refresh for respective BBQ

if you done modifying your json file you can send it to bot by using `/config bounty_refresh`, and `cooldown_msg` on `cooldown_ch` will be updated , if you experience error on uploading, refer to [Json Config Guide](../guide/json)
