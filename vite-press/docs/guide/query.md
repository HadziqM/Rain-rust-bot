---
title: Query Guide
lang: en-US
---

# Query Using Bot
this is the new feature bot introduced for easier interface between server and admininstrator, this is prefix command that can be used by someone that have `admin_role` so you need to set it on config mentioned in [Installation Guide](../index).

## Query
this command using `<prefix>query` and you need code block for bot to get your code, example this what you input on discord chat (my default prefix is `?`)
````
?query
```
select * from users limit 5
```
````
the query result will be capped at 2000 caharacter so you cant query many data at once

## Execute
this command using `<prefix>exceute` and you also need code block for bot to get your code, this one will execute your code and doesnt return data, it will response with `success` if code succesfully executed on server database, example this what you input on discord chat (my default prefix is `?`)
````
?execute
```
update users set name='x' where name='y' 
```
````
the exceuten is limited 1 every command so you cant send many instruction on code block

## Execute Query
you can execute and query at same time using `<prefix>query` 
````
?query
```
insert into users (x,y,z) values (x,y,z) returning id
```
````
this one will exceute insert instruction then query resulted id
