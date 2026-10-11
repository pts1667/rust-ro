# Weight Limit


## General

The weight limit is how much you can carry. If a character is carrying 50% or more of the weight limit, he has minor overweight. Minor overweight cancels [HP Recovery](HP_Recovery.md), [SP Recovery](SP_Recovery.md), [Increase HP Recovery](Increase_HP_Recovery.md) and [Increase SP Recovery](Increase_SP_Recovery.md) as well as restricting the use of skills that create items, like [Find Stone](Find_Stone.md) and [Arrow Crafting](Arrow_Crafting.md). When carrying 90% or more of the weight limit, the character has major overweight. During major overweight, a character cannot attack or use skills at all.

[Pushcart](Pushcart.md) does not increase your character's weight limit. See the skill description for how it works.


## Calculation

 var MAX_WGT = 2000;
 MAX_WGT += 30 * BASE_STR;
 MAX_WGT += WGT_JOB;
 MAX_WGT += WGT_MOD;

 BASE_STR : Base stat value of STR
 WGT_JOB : See the job modifier table
 WGT_MOD : Sum of modifiers

Base value 2000. Add 30 per base STR. Add mod from job. Add modifiers.


## Modifiers

*  [Enlarge Weight Limit](Enlarge_Weight_Limit.md): +200 per level
*  [Peco Peco Ride](Peco_Peco_Ride.md): +1000 when mounted
Note that there are two versions of [Enlarge Weight Limit](Enlarge_Weight_Limit.md). One of them is a [Merchant](Merchant.md) class skill, while the other can be obtained through the use of the [Kafra Shop](Kafra_Shop.md) item Gym Pass. These two versions do stack.


## Job modifiers

| [Swordsman](Swordman.md) | [Archer](Archer.md) | [Thief](Thief.md) | [Acolyte](Acolyte.md) | [Merchant](Merchant.md) | [Mage](Mage.md) | [Novice](Novice.md) | [TaeKwon Kid](TaeKwon_Kid.md) | [Gunslinger](Gunslinger.md) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 800 | 600 | 400 | 400 | 800 | 200 | 0 | 800 | 800 |
| [Knight](Knight.md) | [Hunter](Hunter.md) | [Assassin](Assassin.md) | [Priest](Priest.md) | [Blacksmith](Blacksmith.md) | [Wizard](Wizard.md) | [Super Novice](Super_Novice.md) | [Soul Linker](Soul_Linker.md) | [Ninja](Ninja.md) |
| 800 | 700 | 400 | 600 | 1000 | 400 | 0 | 400 | 600 |
| [Crusader](Crusader.md) | [Dancer](Dancer.md)/[Bard](Bard.md) | [Rogue](Rogue.md) | [Monk](Monk.md) | [Alchemist](Alchemist.md) | [Sage](Sage.md) |   | [TaeKwon Master](TaeKwon_Master.md) |   |
| 800 | 700 | 400 | 600 | 1000 | 400 |   | 400 |   |

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Weight_Limit) — GFDL 1.3+*
