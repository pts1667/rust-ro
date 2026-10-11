# Max HP

*For information on the class, see [High Priest](High_Priest.md).*


## General

Max HP (also called MHP) affects HP Recovery, [Increase HP Recovery](Increase_HP_Recovery.md), and [Spiritual Cadence](Spiritual_Cadence.md).


## Base HP Calculation

Ragnarok Online does not calculate the base maximum HP of players in real time. Instead, this [lookup table](http://ro.doddlercon.com/misc/maxhp3.html) is used. The following pseudo-code describes how base HP is calculated:

```
var BASE_HP;

BASE_HP = 35 + (BASE_LEVEL * HP_JOB_B);

for (var i = 2; i <= BASE_LEVEL; i++)
{
  BASE_HP += Math.round(HP_JOB_A * i);
}
```

Where:
  **HP_JOB_A**: See the job modifier table
  **HP_JOB_B**: The number in parenthesis, if any. Otherwise 5.

Note:
  Base HP is the same for all characters with the same job and same level.


## Max HP Calculation

Maximum HP is calculated in real time from a player's Base HP, VIT, and the sum of all HP modifiers the player has equipped (see modifiers below). The following pseudo-code describes how maximum HP is calculated:

```
var MAX_HP;

MAX_HP = Math.floor( BASE_HP * (1 + VIT * 0.01) * TRANS_MOD );

MAX_HP += HP_MOD_A;

MAX_HP = Math.floor( MAX_HP * (1 + HP_MOD_B * 0.01) );
```

Where:
  **HP_MOD_A**: Sum of additive modifiers
  **HP_MOD_B**: Sum of multiplicative modifiers
  **TRANS_MOD**: 1.25 for transcendent classes, 1 otherwise


## Additive Modifiers

* Equipment bonus to VIT: +1 per point
* [Faith](Faith.md): +200 per level
* Apocalipse Card: +800 (Armor, refine rate 9~10)
* Arc Angeling Card: +300 (Headgear)
* Freezer Card: +300 (Footgear)
* Loli Ruri Card: +500 (Armor, combo with Miyabi Doll Card, Evil Nymph Card, and Harpy Card)
* Lude Card: +300 (Accessory, combo with Quve Card)
* Succubus Card: +1000 (Armor, combination with Incubus Card)
* Yellow Novus Card: +500 (Armor)
* Thiefbug Egg Card: +400 (Shield)
* Roda Frog Card: +400 (Armor)
* Pupa Card: +700 (Armor)
* Fabre Card: +100 (Weapon)
* Picky Egg Card: +100 (Armor)
* Remover Card: +800 (Armor, -40 per upgrade level)
* Survivor's Rod\[0\](Dex): +300
* Survivor's Rod\[1\](Dex): +400
* Survivor's Rod\[0\](Int): +300
* Survivor's Rod\[1\](Int): +400
* Valkyrian Shoes: +(5 * base level) ([Acolyte](Acolyte.md), [Mage](Mage.md), or [Archer](Archer.md) types only)
* [Super Novice](Super_Novice.md)s at level 99: +2000


## Multiplicative Modifiers

* Andre Egg Card: +5% (Shield)
* Eddga Card: -25% (Foot gear)
* Matyr Card: +10% (Foot gear)
* PecoPeco Card: +10% (Armour)
* Green Ferus Card: +10% (Foot gear)
* Verit Card: +8% (Foot gear)
* Firelock Soldier Card: +10% (Foot gear)
* Tao Gunka Card: +100% (Armour)
* Assassin Dagger: +20%
* Tidal Shoes and wool scarf Set: +10%
* Variant Shoes: +20% (-1% Maximum HP and Maximum SP per upgrade)
* [Battle Chant](Battle_Chant.md) HP Buff +100%
* [Frenzy](Frenzy.md): +200%
* [Deluge](Deluge.md): see skill description
* [Song of Lutie](Song_of_Lutie.md): see skill description
* [Taekwon Mission](Taekwon_Mission.md): +200% for those who in /taekwon and have 90 baselvl


## Job Modifiers


## Novices

| Column 1 | Column 2 |
| --- | --- |
| [Novice](Novice.md) | [Super Novice](Super_Novice.md) |
| 0 | 0 |


## First Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 |
| --- | --- | --- | --- | --- | --- |
| [Swordman](Swordman.md) | [Archer](Archer.md) | [Thief](Thief.md) | [Acolyte](Acolyte.md) | [Merchant](Merchant.md) | [Mage](Mage.md) |
| 0.7 | 0.5 | 0.5 | 0.4 | 0.4 | 0.3 |


## 2-1 Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 |
| --- | --- | --- | --- | --- | --- |
| [Knight](Knight.md) | [Hunter](Hunter.md) | [Assassin](Assassin.md) | [Priest](Priest.md) | [Blacksmith](Blacksmith.md) | [Wizard](Wizard.md) |
| 1.5 | 0.85 | 1.1 | 0.75 | 0.9 | 0.55 |


## 2-2 Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 |
| --- | --- | --- | --- | --- | --- |
| [Crusader](Crusader.md) | [Dancer](Dancer.md)/[Bard](Bard.md) | [Rogue](Rogue.md) | [Monk](Monk.md) | [Alchemist](Alchemist.md) | [Sage](Sage.md) |
| 1.1 (7) | 0.75 (3) | 0.85 | 0.9 (6.5) | 0.9 | 0.75 |


## Expanded Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 |
| --- | --- | --- | --- | --- |
| [TaeKwon Kid](TaeKwon_Kid.md) | [Ninja](Ninja.md) | [Gunslinger](Gunslinger.md) | [TaeKwon Master](TaeKwon_Master.md) | [Soul Linker](Soul_Linker.md) |
| 0.7 | 0.8 (0) | 0.75 (0) | 0.9 (6.5) | 0.75 |

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Max_HP) — GFDL 1.3+*
