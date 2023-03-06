---
title: Gacha Config
lang: en-US
---

# How To Configure Bounty Banner
The banner is pretty much json file that you can find the placeholder file [here](https://github.com/HadziqM/Rain-rust-bot/tree/senpai/static), 
pick the one named `gacha.json`, in there config is devided in some properties, `ur` field one is the rarest (0.1%), then odd wise follow this rule `ur`<`ssr1`<`ssr2`<`sr1`<`sr2`<`sr3`<`r1`<`r2`, all of those properties use list of `Gacha Item` field. and you just focus in this section
```json
    {
      "key":"AF41",
      "count":1,
      "types":5
    }
```
The `types` is the types of item/equipment that game use for its binary
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

Lastly if you done modifying your json file you can send it to bot by using `/config gacha`, and the gacha banner for `/pull` command will updated afterward, if you experience error on uploading, refer to [Json Config Guide](../guide/json)
