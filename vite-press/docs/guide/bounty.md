---
title: Bounty Guide
lang: en-US
---

# Bounty Event Guide
One of the biggest feature bot had is the bounty event, this feature originated by Eve from rain server idea, then developed further by using bot for automation, the bounty bot had right now is using different mechanics than the last bounty sistem typescript bot had, but this doesnt replace anything so can be used incrementally along with the old bot mechanics.

## The Original Bounty
The original bounty system is the feature that only registered user could participate, the bounty consist 25 challanges with one being special kind of bounty (The Ravi bounty), the player can take bounty and send their proof of clearing them in `receptionist_ch` using submit command, the submission then being judged by someone who had access to `judge_ch` if they got accepted then the sumission got published on `conquer_ch` and player will be given reward based on the bounty cleared, also while on it if those submission fulfill the requirements to get `Title` then the player will be given title and announced on `promotion_ch` and for the next submission forward the title benefit will be active permanently. The bounty have cooldown (we call it `Steak`), if its reach 0 then no one could take those bounty, any player that take bounty will consume one Steak on submission but the Steak will back if its rejected, the player also limited to take 1 bounty every 20 hours for different kind or 40 hours for same kind.

## The New Bounty
There is some change with new bounty
1. Adding category in bounty, now we have 5 bounty (Gold,Silver,Bronze,Free and Event) each category have 25 challanges, the `Free` category is pretty much same as original bounty, it has cooldown and anyone can take it freely, the `Event` category is same like the Ravi bounty from the original, it doesnt have any `Steak` and player could take it despite they are still on cooldown, the `Event` category also can have `costume title`, the detail can be accessed on [Title Config](../config/title). The `Gold`,`Silver` and `Bronze` is a progressive bounty and they doesnt have `Steak` like Free category player can only take `BBQ01` at first and cant take higher BBQ, reach certain BBQ and player can get `benfit title`.
2. More Title, the old title will be invalid and reseted, and you can get 6 `benefit title` from ranked bounty, and unlimited `costume title` from participating in `Event` category, the old bounty give benefit extra bounty coin on completition, the new title also take that approach but adding new feature giving bonus discount on every [Trading](./trading) transaction.
3. many user interface change including new bounty reward receipt on accepted submission.
4. add the pedia command to browse the bounty you seek since now we had 125 challange total.

## The Command
1. `/cooldown refresh` to refresh Free category `Steak` refer to [Bounty Config](../config/bounty)
2. `/cooldown user` to reset user cooldown
3. `/cooldown bounty` to set one `Free` category BBQ Steak
4. `/bounty pedia` to show you the selected bounty description refer to [Bounty Config](../config/bounty)
5. `/bounty submit` to submit your bounty clear proof
6. `/distribution` to send player(s) bounty reward bypassing the requirements (the title benefit also active this way)
