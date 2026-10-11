# FLEE

*This article is about the [Substat](Stats.md). For the [Monk](Monk.md) skill, see [Flee (Skill)](Flee_(Skill).md).*


## Description

Flee is a character's ability to avoid being hit by regular attacks and most melee skills. The more Flee a character has, the less likely they will be hit by an attack. Generally considered to be the counterpart to [DEF](DEF.md)/[VIT](Stats.md). Flee does not affect being hit with some magic attacks and some skills. Flee is determined by ([Level](Levels.md) + [AGI](Stats.md)).

Being attacked by more than two monsters decreases your Flee by 10% per every additional monster. This reduction is counted before adding Flee from [Skills](Skills.md) that directly add Flee (ie. [Thief](Thief.md)'s [Improve Dodge](Improve_Dodge.md)). This means that if you are attacked by 4 monsters and your Flee is 150, your Actual Flee would be 120 assuming that you don't have any Flee from any skills directly. This also means that if you're attacked by more than 11 monsters at the same time, they will always hit you regardless of your Flee.


## Calculation

Full formula for calculating your Actual Flee is:

Actual Flee = Skill Bonus + [[Level](Levels.md) + [AGI](Stats.md) + Item Bonus] * [1 - ((Mobs -2) * 0.1)]

Item Bonus is bonus granted from [Items](Items.md) or [Cards](Card_System.md), Skill Bonus is bonus directly granted from Skills and Mobs is the amount of enemies that are attacking you.

Classes which usually benefit the most from (higher) Flee are:

*  [Super Novice](Super_Novice.md)
*  [Rogue](Rogue.md) and [Stalker](Stalker.md)
*  [Assassin](Assassin.md) and [Assassin Cross](Assassin_Cross.md)
*  [Blacksmith](Blacksmith.md) and [Mastersmith](Mastersmith.md)
*  [Hunter](Hunter.md) and [Sniper](Sniper.md)
*  [Bard](Bard.md) and [Minstrel](Minstrel.md)
*  [Dancer](Dancer.md) and [Gypsy](Gypsy.md)
*  [TaeKwon Kid](TaeKwon_Kid.md) and [TaeKwon Master](TaeKwon_Master.md)


## Dodge Calculation

Dodge Rate(%) = 100% - (Attacker [Hit](Stats.md) + 80 - Defenders Flee)
("Attackers Hit" also applies for monsters as (Baselevel + [DEX](Stats.md)))

Dodge Rate cannot be lower than 0% and not higher than 95% (except during WoE siege)


## Modifiers

Of course, builds will vary on the amount of Flee they end up with, so do not take this list to be true all of the time. Many characters will instead benefit more from DEF/VIT rather than flee. Cards which add to Flee include:
*  Alicel Card +10
*  Choco Card +10
*  Chonchon Card +2
*  Condor Card +10
*  Dragonfly Card +18 (Combo with Chonchon Card)
*  Dragon Tail Card +10
*  Dustiness Card +5
*  Eclipse Card +18 (Combo with Lunatic Card)
*  Female Thiefbug Card +1
*  Giant Whisper Card +10
*  Gryphon Card +2
*  Hode Card +5
*  Isis Card +5
*  Jakk Card +5
*  Kavach Icarus Card +10, +20 if the refine rate is 0~4
*  Marionette Card +5
*  Marse Card +5
*  Mastering Card +18 (Combo with Poring Card)
*  Myst Card +5
*  Nine Tail Card +20
*  Orc Baby Card +10, +15 if the refine rate is 9~10
*  Orc Zombie Card +5
*  Roween Card +5
*  Fur Seal Card +3
*  Toad Card +18 (Combo with Roda Frog Card)
*  Vagabond Wolf Card +18 (Combo with Wolf Card)
*  Vocal Card +18 (Combo with Rocker Card)
*  Wanderer Card +20 ([Thief](Thief.md) class only)
*  Whisper Card +20
*  Yao Jun / Hyegun Card +15

Cards which affect [AGI](Stats.md) also affect Flee, but due to there being such a large amount they aren't included here.

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/FLEE) — GFDL 1.3+*
