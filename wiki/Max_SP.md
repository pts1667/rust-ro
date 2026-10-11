# Max SP


## General

Max SP affects [SP Recovery](SP_Recovery.md), [Increase SP Recovery](Increase_SP_Recovery.md), and [Spiritual Cadence](Spiritual_Cadence.md).


## Calculation

```
var BASE_SP = 10;
BASE_SP += BASE_LEVEL * SP_JOB;

var MAX_SP = BASE_SP;

MAX_SP = Math.floor( MAX_SP * (1 + INT * 0.01) );

MAX_SP += SP_MOD_A;

MAX_SP = Math.floor( MAX_SP * (1 + SP_MOD_B * 0.01) );

MAX_SP = Math.floor( MAX_SP * TRANS_MOD );
```

Where:
  **SP_JOB**: See the job modifier table
  **SP_MOD_A**: Sum of additive modifiers
  **SP_MOD_B**: Sum of multiplicative modifiers
  **TRANS_MOD**: 1.25 for transcendent classes, 1 otherwise

Notes:
  Base SP is the same for all characters with the same job and same level.
  Increase base SP by 1% per 1 INT. Then add +x SP modifiers. Finally add +x% SP modifiers.


## Additive modifiers

*  Equipment bonus to [INT](Stats.md): +1% per point
*  Andre Larva Card: +10
*  Carat Card: +150 (if headgear is +9 or higher)
*  Evil Nymph Card: +50
*  Incubus Card: +150
*  Roda Frog Card: +50
*  Willow Card: +80
*  Clip \[1\]: +10
*  Coif \[1\]: +100
*  Crown of Mistress: +100
*  Cute Ribbon: +10
*  Mage Hat: +150
*  Magician Hat: +50
*  Wizard Hat: +100
*  Valkyrian Shoes: +(Job Level*2) (Swordman, Merchant and Thief types only)


## Multiplicative modifiers

*  Dark Lord Card & Dark Illusion Card: +10% (-10% + 20%)
*  Firelock Soldier Card: +10% (if shoes are +9 or higher)
*  Miyabi Doll Card: +10%
*  Sohee Card: +15%
*  Verit Card: +8%
*  Assassin Dagger: +15%
*  Moonlight Dagger: +10%
*  Sleipnir: +20%
*  Variant Shoes: +20% (-1% Maximum HP and Maximum SP per upgrade)
*  [Gypsy's Kiss](Gypsy's_Kiss.md): see skill description


## Job Modifiers


## Novices

| Column 1 | Column 2 |
| --- | --- |
| [Novice](Novice.md) | [Super Novice](Super_Novice.md) |
| 1 | 1 |


## First Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 | Column 7 | Column 8 | Column 9 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Swordman](Swordman.md) | [Archer](Archer.md) | [Thief](Thief.md) | [Acolyte](Acolyte.md) | [Merchant](Merchant.md) | [Mage](Mage.md) | [TaeKwon Kid](TaeKwon_Kid.md) | [Ninja](Ninja.md) | [Gunslinger](Gunslinger.md) |
| 2 | 2 | 2 | 5 | 3 | 6 | 2 | 3 | 4 |


## 2-1 Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 | Column 7 |
| --- | --- | --- | --- | --- | --- | --- |
| [Knight](Knight.md) | [Hunter](Hunter.md) | [Assassin](Assassin.md) | [Priest](Priest.md) | [Blacksmith](Blacksmith.md) | [Wizard](Wizard.md) | [TaeKwon Master](TaeKwon_Master.md) |
| 3 | 4 | 4 | 8 | 4 | 9 | 4 |


## 2-2 Job Classes

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 | Column 7 |
| --- | --- | --- | --- | --- | --- | --- |
| [Crusader](Crusader.md) | [Dancer](Dancer.md)/[Bard](Bard.md) | [Rogue](Rogue.md) | [Monk](Monk.md) | [Alchemist](Alchemist.md) | [Sage](Sage.md) | [Soul Linker](Soul_Linker.md) |
| 4.7 | 6 | 5 | 4.7 | 4 | 7 | 9 |

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Max_SP) — GFDL 1.3+*
