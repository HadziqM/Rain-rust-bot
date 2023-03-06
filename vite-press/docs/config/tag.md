---
title: Tag Config
lang: en-US
---

# How To Set Tag Config
first you need the json file that matched tag config. you can get the placeholder file [here](https://github.com/HadziqM/Rain-rust-bot/tree/senpai/static)
, pick the one named `tag.json`, the config consist `tag` properties with vaue of command list, you can create as many command as you want by extending the list. i will explain each field, first the one you focused is editing this section inside the list.
```json
    {
      "desc": "",
      "command": "",
      "url": ""
    }
```
The `command` is the command name to invoke, so for example you set `command:"guide"`, you can access that command by using `<prefix>tag guide`, you can inpute those with anyting.
:::warning
you cant input space on the `command` or name it with `list`, it will break the tag command
:::
The `desc` field is the command description or the text you want to show as guide you can fit text here with limitation on 4000 characters, this willl show in the embed as the content of guide you make.<br/>
The `url` field is the image url if you want to fit image in your guide, this take any kind of url.
:::info
if the url doesnt have image or its invalid url, the image will not loaded in the embed, likewise you can use it or leave the `url` empty to not show any image in your guide
:::
Lastly if you done modifying your json file you can send it to bot by using `/config tag`, and the tag command will be updated afterward, if you experience error on uploading, refer to [Json Config Guide](../guide/json)

