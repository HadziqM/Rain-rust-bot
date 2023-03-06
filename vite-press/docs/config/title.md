---
title: Title Config
lang: en-US
---

# How To Set Config For Title
first you need the json file that matched tag config. you can get the placeholder file [here](https://github.com/HadziqM/Rain-rust-bot/tree/senpai/static)
, pick the one named `bounty_title.json`, the config consist `<name>_bounty` properties with namy same value (i call it title section) and `custom` field  with vaue list of `title section`. one that you need to focus on is the `title section`
```json
    {
      "image": {
        "url": "https://media.discordapp.net/attachments/1004207525408817323/1006334931607232542/01._Bounty_Expert.jpg?width=1150&height=658",
        "diameter": 180,
        "x_start": 695,
        "y_start": 170
      },
      "trigger": "4_0",
      "role_id": 1031595216538452038,
      "db_code": 0
    }
```
On the `image` properties, there is `url`, the url is the image url your title image is located, (i suggest just send the file in discord chat then coply the link there).
:::tip
there is parameter in the link if you use discord link, like for in the example above, the link had `?width=1150&height=658` this is usefull to resize your file so the bot will not need to download big original file
:::
there is also `diameter` `x_start` and `y_start` which specify the pixel location of will be edited profile image in, the detail of filling those can be explained in this image bellow ![image](https://media.discordapp.net/attachments/1068440173479739393/1082262980516921484/01._Bounty_Expert_edited.jpg)
:::info
the pixel value will change if you resize the image, so if you use the resize methode on the tips above you need to scale it with math
:::
the `trigger` properties is is the trigger when player will get the title, the code follow bellow for the first number
```
0 => Bronze Category
1 => Silver Category
2 => Gold Category
3 => Free Category
4 => Event Category
```
and for the second code
```
0 => BBQ01
1 => BBQ02
.
.
.
24 => BBQ25
```
so for example above `4_0` mean the title will trigger if player cleared Event Category BBQ01.<br/>
`role_id` propreties is self explained, its the role that bot will give to the player if player get the title.<br/>
`db_code` is the title value in database, only worked on `fixed title`, you never bother with it. <br/><br/>

If you done modifying your json file you can send it to bot by using `/config bounty_title`, and the title will be updated afterward, if you experience error on uploading, refer to [Json Config Guide](../guide/json).

Lastly you can test the title you just configured with `/test` command
## The Fixed Title
in the file there is properties name `bronze_bounty`,`silver_bounty`,`gold_bounty`,`gold_trading`,`silver_trading`,`bronze_trading`, their value use same config, they need clear `db_code` field since they are stored in db, as the result, you can leave it as default since i already configured it to be not edited, you can pretty much change anything but i will suggest leave the `db_code` and `trigger` as default.

## The Custom Title
the custom title use list field, so you can extend it by expanding the list, the `db_code` here is need to be 0, and instead of storing data on database, bot will use `role_id` for marking, so if someone get the custom title, on trigger Event BBQ01, if player submit the bounty again bot will scan their role, if player had the title role, title will not be given, if they didnt have the role, the player given the custom title.
:::tip
since the event only had 25 bbq, custom can only get 25, but there is a trick so you can add title infinetly by editing `role_id`, you can ask me if you still doesnt understand
:::
