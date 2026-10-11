# Mercenary System


## Overview

The mercenary system is designed to add a new dimension to Ragnarok Online, allowing players to have an [NPC](NPC.md) henchman to fight at their side.  Players will be able to support, party with, and control a mercenary using an updated version of the [Homunculus](Homunculus_System.md) system.  Different from homunculi, any player can use mercenaries, but instead of a creature that grows alongside you, mercenaries have fixed stats based on the level of mercenary summoned.  A contract is made with a mercenary and scrolls are used to call them into battle.  Both homunculus and mercenary can be used by a player at once.  Mercenaries are all of the [Demi Human](Demi_Human.md) race.


## Summoning a Mercenary

Initially to make use of the mercenary system, you must visit one of the mercenary guilds across the [Rune-Midgarts Kingdom](Rune-Midgarts_Kingdom.md).  The Spearman Mercenary Guild is located in [Prontera](Prontera.md) (prontera 41, 337), the Bowman Mercenary Guild is in [Payon](Payon.md) (pay_arche 99, 167), and the Fencer Mercenary Guild is located in [Izlude](Izlude.md) (izlude 47, 139).  To invoke a contract with a mercenary and summon one to your side, you must purchase scrolls.  The cost of scrolls increases with higher level mercenaries, and the maximum level of mercenary that can be summoned is limited by a character's base level.  Some high-level contracts require a certain amount of Loyalty Points to purchase, and the Superior Mercenary Scroll (Level 10) can only purchased in exchange for loyalty points. Scrolls cannot be sold, transfered, or moved into storage or carts.

The stats of the mercenary summoned is based on the scroll used and they come with fixed skills which are different depending on the type of mercenary summoned.  Once a mercenary is summoned, it will last either until it dies or the contract expires.


## AI

The mercenary system is very similar to that used by the [Homunculus](Homunculus_System.md), as both are run by an AI on the client side.  Mercenaries by default use a non-aggressive AI and only attack targets hitting the player or targets the player is attacking.  The AI itself is located in the /AI/ folder in the RO directory.  Where as homunculus normally executes AI.Lua, mercenaries use a separate file AI_M.Lua.  Users can and are encouraged to customize their mercenary AI by modifying the files in the /AI/USER_AI/ folder. Instead of configuring it with /hoai command in-game, players use /merai instead to configure custom AI.


## Interface

The following are keys used to control mercenaries.

Enemy Targeting - Alt + Left Click (on monster)   Left clicking once will cause the mercenary to queue up the monster.  Left click again and the mercenary will immediately switch targets.

Moving - Alt + Left Click (on ground)   Alt and Left Clicking on the ground will cause the mercenary to stop everything and move to the target cell.

Standby - Ctrl + T   In standby the mercenary will stop everything and wait for orders at the master's side.

Information - Ctrl + R   Opens the mercenary status window.


## Usage


## Mercenary Details

Base Stats   A mercenary's level and statistics are fixed when summoning, based on the level of scroll used.  However, based on one's Loyalty Points with that mercenary's guild and the kill count, additional bonuses can be received. Attack speed, no matter what mercenary type or level, will always be fixed at 156 ASPD

Skills   Mercenary skills are based on the level of the mercenary summoned, and one's Loyalty Points with the mercenary guild. Skills act as if they have been used by a [Party](Party.md) member; for example, the [Sacrifice](Sacrifice.md) skill will not function if the mercenary's own level is not within 10 levels of the summoner.

Supportive Skills   The mercenary is considered very much the same as a player and all supportive magic should function, with the following exceptions:

| Column 1 | Column 2 |
| --- | --- |
| **Skill** | **Effect** |
| **[Increase AGI](Increase_AGI.md)** | Mercenary will receive a boost to its flee rate only. |
| **[Blessing](Blessing.md)** | Mercenary will receive a boost to its hit rate only, and will cure cursed status. |
| **[Heal](Heal.md)** | Heal only recovers 1/2 the normal amount that it would heal a player. |
| **[Endow Blaze](Endow_Blaze.md)** | Players can only cast Endow Blaze on the Mercenary if they are the summoner. |
| **[Endow Quake](Endow_Quake.md)** | Players can only cast Endow Quake on the Mercenary if they are the summoner. |
| **[Endow Tornado](Endow_Tornado.md)** | Players can only cast Endow Tornado on the Mercenary if they are the summoner. |
| **[Endow Tsunami](Endow_Tsunami.md)** | Players can only cast Endow Tsunami on the Mercenary if they are the summoner. |
| **[Ankle Snare](Ankle_Snare.md)** | A maximum of 10 traps can be placed at one time.  Traps can be placed directly beneath targets. |
| **[Provoke](Provoke.md)** | Can be cast on players as well as enemies.  Can be used to break stone curse and frozen states. |

Upkeep   Mercenaries require no feeding or upkeep to keep in play like homunculus or pets (though its contract automatically expires after 30 minutes).

Separation   If a mercenary moves more than 15 cells away it will be teleported to the summoner's position.

Player Versus Player   Mercenaries can be used in both PvP and WoE maps.  Mercenaries are unable to damage the emperium.

Appearance   Each class of mercenary has its own custom sprite.  Each level of mercenary uses a specific headgear that allows it to be identified.  These headgears cannot be chosen or changed and do not have any effect on the performance of your mercenary.


## End of Contract

The following are the 4 conditions in which a mercenary will leave its master.  In each case, the game will display a message in the chat box as to why the mercenary left.

Contractor Dies   The player summoning the Mercenary has died.

Mercenary Dies   The mercenary summoned has died.

Contract Ends   The mercenary contract lasts 30 minutes, after which the mercenary will leave.

Contract Canceled   The player manually dismisses the mercenary.


## Loyalty

As a player fights along side with mercenaries, that player can gain Loyalty Points.  More points gives the player faster Killcount Bonuses and allows the player to contract higher level mercenaries.  Number of Loyalty Points is specific to each guild of mercenaries, so for example if a player gains a large number of points with the Bowman Mercenary Guild, the amount will have no effect on their standing with the Fencer and Spearman guilds.

The following conditions affect one's Loyalty Points.

Fighting Together with Mercenary   When a suitably strong monster is killed by a player (more than 1/2 the base level of the player), their Killcount goes up by 1.  It doesn't matter if the player or the mercenary kills the monster, or if they work together to kill it.  For every 50 killcount, they gain one Loyalty Point.

Successfully Completing a Contract   If a player and their mercenary successfully survive the full duration of a 30 minute contract, they gain one Loyalty Point.

Mercenary Dies   If the mercenary is killed while bound by contract to a player, they lose one Loyalty Point.


## Killcount

As either the player or their mercenary kills monsters (monsters higher than 1/2 the base level of the owner), their killcount will increase.  At fixed killcount intervals, a mercenary will receive a random boost to one of their stats.  The number of kills required goes down based on the player's reputation with that mercenaries guild.  The mercenary can obtain multiple bonuses, which will increase another stat randomly or boost the same one further.  There's a total of 5 increases that can be received to each stat.

Killcount bonuses do not stack, and if a player receives a higher level bonus it will replace the existing bonus of that type.

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 |
| --- | --- | --- | --- | --- | --- |
|   | **Lvl 1** | **Lvl 2** | **Lvl 3** | **Lvl 4** | **Lvl 5** |
| **Flee Boost** | +15 | +30 | +45 | +60 | +75 |
| **Attack Power Boost** | +15 | +30 | +45 | +60 | +75 |
| **HP Boost** | +5% | +10% | +15% | +20% | +25% |
| **SP Boost** | +5% | +10% | +15% | +20% | +25% |
| **Hit Boost** | +15 | +30 | +45 | +60 | +75 |

In the case that the mercenary receives a boost to its HP, their HP will be completely recovered.  Likewise, if SP is increased they will fully recover their SP.

The boost to stats that are received is not permanent, and is lost at the end of the contract, regardless of how it ends.


## Item Pricing and Limitations

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 |
| --- | --- | --- | --- | --- |
| **Item Name** | **Level Requirement** | **Other Requirements** | **Price*** | **Effect** |
| Level 1 Scroll | Base Level 15+ | None | 7,000[Z](Zeny.md) | Summons a level 20 mercenary. |
| Level 2 Scroll | Base Level 25+ | None | 14,000Z | Summons a level 30 mercenary. |
| Level 3 Scroll | Base Level 35+ | None | 21,000Z | Summons a level 40 mercenary. |
| Level 4 Scroll | Base Level 45+ | None | 28,000Z | Summons a level 50 mercenary. |
| Level 5 Scroll | Base Level 55+ | None | 35,000Z | Summons a level 60 mercenary. |
| Level 6 Scroll | Base Level 65+ | None | 42,000Z | Summons a level 70 mercenary. |
| Level 7 Scroll | Base Level 75+ | 50+ Loyalty Points | 49,000Z | Summons a level 80 mercenary. |
| Level 8 Scroll | Base Level 85+ | 100+ Loyalty Points | 56,000Z | Summons a level 90 mercenary. |
| Level 9 Scroll | Base Level 90+ | 300+ Loyalty Points | 63,000Z | Summons a level 95 mercenary. |
| Level 10 Scroll | Base Level 90+ | 500+ Loyalty Points | 400 Loyalty Points | Summons a level 99 mercenary. |
| Mercenary Red Potion | - | None | 1750Z | Recovers a base of 1000 [HP](Max_HP.md). |
| Mercenary Blue Potion | - | None | 3500Z | Recovers a base of 100 [SP](SP.md). |
| Mercenary Concentration Potion | - | None | 560Z | Increases Mercenary [ASPD](ASPD.md) by 10%. |
| Mercenary Awakening Potion | Level 40+ Mercenary | None | 1050Z | Increases Mercenary ASPD by 15%. |
| Mercenary Berserk Potion | Level 80+ Mercenary | None | 2100Z | Increases Mercenary ASPD by 20%. |


## Fencer Mercenary

| Scroll Level | Mercenary Name | Level | HP | SP | ATK | Hit | Flee | DEF | MDEF | HP/SP Regen | Headgear | Skills |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | David | 20 | 502 | 70 | 174~261 | 50 | 47 | 26+5 | 0+3 | 8 HP per 8s<br>2 SP per 6s | Giant Band Aid | [Bash](Bash.md) Level 1<br>[Decrease AGI](Decrease_AGI.md) Level 1 |
| 2 | Ellen | 30 | 979 | 99 | 258~387 | 65 | 68 | 31+6 | 0+9 | 10 HP per 8s<br>2 SP per 6s | Angry Snarl | [Magnum Break](Magnum_Break.md) Level 3<br>[Provoke](Provoke.md) Level 5 |
| 3 | Luise | 40 | 1,555 | 134 | 326~489 | 80 | 85 | 36+7 | 3+9 | 13 HP per 8s<br>4 SP per 6s | Elephant Hat | [Benediction](Benediction.md) Level 1<br>[Weapon Quicken](Weapon_Quicken.md) Level 1 |
| 4 | Frank | 50 | 2,397 | 162 | 382~573 | 90 | 109 | 39+8 | 5+10 | 18 HP per 8s<br>4 SP per 6s | Baseball Cap | [Crash](Crash.md) Level 1<br>[Magnum Break](Magnum_Break.md) Level 5 |
| 5 | Ryan | 60 | 3,387 | 195 | 406~609 | 105 | 128 | 42+9 | 7+10 | 24 HP per 8s<br>5 SP per 6s | Funeral Hat | [Bash](Bash.md) Level 5<br>[Benediction](Benediction.md) Level 1<br>[Crash](Crash.md) Level 4 |
| 6 | Paolo | 70 | 4,495 | 241 | 436~654 | 120 | 145 | 45+10 | 12+17 | 33 HP per 8s<br>8 SP per 6s | Hibiscus | [Decrease AGI](Decrease_AGI.md) Level 3<br>[Sense](Sense.md) Level 1 |
| 7 | Jens | 80 | 5,889 | 279 | 468~702 | 135 | 168 | 47+11 | 15+17 | 45 HP per 8s<br>9 SP per 6s | Tiger Mask | [Bash](Bash.md) Level 10<br>[Berserk](Berserk.md) Level 1<br>[Scapegoat](Scapegoat.md) Level 1 |
| 8 | Thierry | 90 | 7,520 | 325 | 500~750 | 150 | 185 | 49+12 | 18+24 | 60 HP per 8s<br>13 SP per 6s | Crescent Helm | [Bowling Bash](Bowling_Bash.md) Level 5<br>[Compress](Compress.md) Level 1 |
| 9 | Steven | 95 | 9,052 | 348 | 524~786 | 155 | 190 | 51+12 | 22+27 | 104 HP per 8s<br>14 SP per 6s | Angeling Hat | [Shield Reflect](Shield_Reflect.md) Level 5<br>[Crash](Crash.md) Level 3 |
| 10 | Wayne | 99 | 12,355 | 451 | 760~1,040 | 159 | 204 | 64+30 | 30+45 | 228 HP per 8s<br>25 SP per 6s | Solar God Helm | [Bash](Bash.md) Level 10<br>[Bowling Bash](Bowling_Bash.md) Level 10<br>[Frenzy](Frenzy.md) Level 1 |


## Bowman Mercenary

| Scroll Level | Mercenary Name | [Level](Levels.md) | [HP](Max_HP.md) | [SP](SP.md) | [ATK](Stats.md) | [Hit](Stats.md) | [Flee](Stats.md) | [DEF](DEF.md) | [MDEF](Stats.md) | [HP](Max_HP.md)/[SP](SP.md) Regen | [Headgear](Headgears.md) | [Skills](Skills.md) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Mina | 20 | 256 | 200 | 170~255 | 48 | 36 | 7+5 | 5+3 | 7 HP per 8s<br>6 SP per 6s | Beanie | [Double Strafe](Double_Strafe.md) Level 2<br>[Berserk](Berserk.md) |
| 2 | Dororu | 30 | 457 | 70 | 228~342 | 70 | 48 | 11+8 | 7+5 | 8 HP per 8s<br>6 SP per 6s | Golden Gear | [Arrow Shower](Arrow_Shower.md) Level 2<br>[Sight](Sight.md) Level 1 |
| 3 | Nami | 40 | 732 | 93 | 260~390 | 92 | 61 | 15+12 | 9+10 | 11 HP per 8s<br>3 SP per 6s | Golden Bells | [Arrow Repel](Arrow_Repel.md) Level 1<br>[Weapon Quicken](Weapon_Quicken.md) Level 2 |
| 4 | Elfin | 50 | 1092 | 116 | 310~465 | 110 | 83 | 18+17 | 11+14 | 17 HP per 8s<br>3 SP per 6s | Alice Doll | [Magnificat](Magnificat.md) Level 1<br>[Sense](Sense.md) Level 1<br>[Tender](Tender.md) Level 1 |
| 5 | Clara | 60 | 2212 | 280 | 360~540 | 143 | 101 | 20+17 | 13+20 | 29 HP per 8s<br>9 SP per 6s | Kabuki Mask | [Double Strafe](Double_Strafe.md) Level 5<br>[Remove Trap](Remove_Trap.md) Level 1 |
| 6 | Dali | 70 | 3098 | 353 | 424~636 | 153 | 116 | 21+24 | 15+33 | 51 HP per 8s<br>16 SP per 6s | Crown of Ancient Queen | [Double Strafe](Double_Strafe.md) Level 7<br>[Skid Trap](Skid_Trap.md) Level 3<br>[Decrease AGI](Decrease_AGI.md) Level 1 |
| 7 | Karaya | 80 | 4051 | 415 | 468~702 | 171 | 135 | 24+27 | 16+39 | 72 HP per 8s<br>21 SP per 6s | Zealotus Mask | [Arrow Shower](Arrow_Shower.md) Level 10<br>[Freezing Trap](Freezing_Trap.md) Level 2<br>[Mental Cure](Mental_Cure.md) Level 1 |
| 8 | Hiyori | 90 | 5039 | 469 | 481~723 | 193 | 155 | 24+27 | 18+40 | 88 HP per 8s<br>24 SP per 6s | Silver Tiara | [Sandman](Sandman.md) Level 3<br>[Weapon Quicken](Weapon_Quicken.md) Level 2<br>[Provoke](Provoke.md) Level 3 |
| 9 | Kero | 95 | 5572 | 499 | 500~750 | 205 | 166 | 25+27 | 20+41 | 96 HP per 8s<br>26 SP per 6s | Nekomimi | [Double Strafe](Double_Strafe.md) Level 10<br>[Land Mine](Land_Mine.md) Level 5<br>[Arrow Repel](Arrow_Repel.md) Level 1 |
| 10 | Sukye | 99 | 7381 | 642 | 816~1124 | 222 | 182 | 49+27 | 49+41 | 126 HP per 8s<br>34 SP per 6s | Etc | [Arrow Repel](Arrow_Repel.md) Level 1<br>[Berserk](Berserk.md) Level 1<br>[Focused Arrow Strike](Focused_Arrow_Strike.md) Level 10 Level 1<br>[Weapon Quicken](Weapon_Quicken.md) Level 5 |


## Spearman Mercenary

| Scroll Level | Mercenary Name | Level | HP | SP | ATK | Hit | Flee | DEF | MDEF | HP/SP Regen | Headgear | Skills |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Rodin | 22 | 2071 | 100 | 168~252 | 52 | 23 | 30+4 | 1+3 | 11 HP per 8s<br>2 SP per 6s | Nurse Cap | [Pierce](Pierce.md) Level 1<br>[Regain](Regain.md) Level 1 |
| 2 | Lancer | 34 | 2523 | 131 | 208~312 | 71 | 35 | 33+4 | 1+3 | 12 HP per 8s<br>3 SP per 6s | Dark Blinder | [Brandish Spear](Brandish_Spear.md) Level 2<br>[Lex Divina](Lex_Divina.md) Level 1 |
| 3 | Nathan | 41 | 3397 | 161 | 248~372 | 81 | 42 | 36+22 | 55+12 | 51 HP per 8s<br>3 SP per 6s | Goblin Leader Mask | [Pierce](Pierce.md) Level 2<br>[Recuperate](Recuperate.md) Level 1<br>[Sacrifice](Sacrifice.md) Level 1 |
| 4 | Roan | 55 | 4580 | 191 | 300~450 | 105 | 57 | 39+35 | 5+17 | 102 HP per 8s<br>4 SP per 6s | Sheep Hat | [Crash](Crash.md) Level 1<br>[Defending Aura](Defending_Aura.md) Level 1 |
| 5 | Orizaro | 60 | 5899 | 221 | 350~510 | 120 | 62 | 42+39 | 10+19 | 144 HP per 8s<br>4 SP per 6s | Lazy Smokie | [Guard](Guard.md) Level 3<br>[Pierce](Pierce.md) Level 5 |
| 6 | Thyla | 72 | 7874 | 252 | 370~555 | 133 | 75 | 46+15 | 10+84 | 77 HP per 8s<br>30 SP per 6s | Grief for Greed | [Brandish Spear](Brandish_Spear.md) Level 5<br>[Weapon Quicken](Weapon_Quicken.md) Level 2 |
| 7 | Ben | 81 | 10260 | 330 | 380~570 | 142 | 84 | 50+63 | 15+51 | 394 HP per 8s<br>14 SP per 6s | Marionette Doll | [Berserk](Berserk.md) Level 1<br>[Sacrifice](Sacrifice.md) Level 1 |
| 8 | Pinaka | 90 | 13167 | 366 | 400~600 | 153 | 110 | 55+74 | 20+57 | 591 HP per 8s<br>16 SP per 6s | Lion Mask | [Pierce](Pierce.md) Lv10<br>[Provoke](Provoke.md) Level 5<br>[Scapegoat](Scapegoat.md) Level 1 |
| 9 | Kuhlmann | 95 | 14648 | 398 | 440~660 | 158 | 120 | 60+77 | 25+63 | 683 HP per 8s<br>20 SP per 6s | Cross Helm | [Brandish Spear](Brandish_Spear.md) Level 10<br>[Defending Aura](Defending_Aura.md) Level 3<br>[Guard](Guard.md) Level 7 |
| 10 | Roux | 99 | 18000 | 413 | 700~950 | 169 | 129 | 70+90 | 30+75 | 978 HP per 8s<br>23 SP per 6s | Lord Kaho Horns | [Clashing Spiral](Clashing_Spiral.md) Level 5<br>[Guard](Guard.md) Level 10<br>[Sacrifice](Sacrifice.md) Level 3<br>[Weapon Quicken](Weapon_Quicken.md) Level 5 |


## Monster Mercenary

| Scroll Name | Level | HP | SP | ATK | Hit | Flee | DEF | MDEF | ASPD | Skills |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wild Rose | 38 | 4000 | 50 | ? | ? | 123 | ? | ? | ? | [Bash](Bash.md) Level 5 |
| Doppelganger | 72 | 7800 | 200 | 231~247 | 147 | 122 | 43 | 18 | 166 | [Bash](Bash.md) Level 5<br>[Weapon Quicken](Weapon_Quicken.md) Level 5 <br>[Sacrifice](Sacrifice.md) Level 3 |
| Egnigem Cenia | 78 | ? | ? | ? | ? | ? | ? | ? | ? | Unknown |


## Mercenary Skills

Mercenaries have access to a wide variety of skills, both new skills and skills that are currently used by player classes.  The exact skills a mercenary receives varies from mercenary to mercenary, but usually they get anywhere from 2-4 skills.  Stronger and higher level skills are usually only available to high level mercenaries.  Skills from summoned mercenaries can be cast from the AI code or manually by assigning it to a hotkey for you to target.

The following is a breakdown of the different skills that the mercenaries have available to them.


## Player Skills Used by Mercenaries

The following is a list of existing skills which are available to mercenaries, broken down by what classes are able to obtain them.  These skills perform identical to the skill obtained by players.


## Common Mercenary Skills

[Berserk](Berserk.md)
[Decrease AGI](Decrease_AGI.md)
[Lex Divina](Lex_Divina.md)
[Magnificat](Magnificat.md)
[Provoke](Provoke.md)
[Sense](Sense.md)
[Sight](Sight.md)


## Fencer Mercenary Skills

[Bash](Bash.md)
[Bowling Bash](Bowling_Bash.md)
[Frenzy](Frenzy.md)
[Magnum Break](Magnum_Break.md)
[Parry](Parry.md)
[Shield Reflect](Shield_Reflect.md)


## Bowman Mercenary Skills

[Arrow Repel](Arrow_Repel.md)
[Arrow Shower](Arrow_Shower.md)
[Double Strafe](Double_Strafe.md)
[Focused Arrow Strike](Focused_Arrow_Strike.md)
[Freezing Trap](Freezing_Trap.md)
[Land Mine](Land_Mine.md)
[Remove Trap](Remove_Trap.md)
[Sandman](Sandman.md)
[Skid Trap](Skid_Trap.md)


## Spearman Mercenary Skills

[Brandish Spear](Brandish_Spear.md)
[Clashing Spiral](Clashing_Spiral.md)
[Defending Aura](Defending_Aura.md)
[Guard](Guard.md)
[Pierce](Pierce.md)
[Sacrifice](Sacrifice.md)


## New Mercenary Skills

In addition to player skills, mercenaries have access to the following new skills.  These skills are non-specific to any particular mercenary class.

| Skill | Description | Type |
| --- | --- | --- |
| [Benediction](Benediction.md) | Recovers the target from both [Curse](Status_Effects.md) and [Blind](Status_Effects.md) statuses. | Supportive |
| [Compress](Compress.md) | Recovers the target from the [Bleeding](Status_Effects.md) status. | Supportive |
| [Crash](Crash.md) | Deals three hits with a chance to [Stun](Status_Effects.md). | Offensive |
| [Mental Cure](Mental_Cure.md) | Recovers the target from both [Hallucination](Status_Effects.md) and [Chaos](Status_Effects.md) statuses. | Supportive |
| [Recuperate](Recuperate.md) | Recovers the target from both [Poison](Poison.md) and [Silence](Status_Effects.md) statuses. | Supportive |
| [Regain](Regain.md) | Recovers the target from both [Sleep](Status_Effects.md) and [Stun](Status_Effects.md) statuses. | Supportive |
| [Scapegoat](Scapegoat.md) | The mercenary gives all of its remaining HP to the summoner and dies. Any excess HP is discarded. | Supportive |
| [Tender](Tender.md) | Recovers the target from both [Frozen](Status_Effects.md) and [Stone](Status_Effects.md) statuses. | Supportive |
| [Weapon Quicken](Weapon_Quicken.md) | Temporarily increase [Attack Speed](ASPD.md) of a Mercenary for the skill's duration. | Supportive |


## References

*  [Doddler's Guide on episode 11.3](http://rode2.doddlercon.com/guides/ep113.php?id=8)

---
**Categories:** [General Information](categories/General_Information.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Mercenary_System) — GFDL 1.3+*
