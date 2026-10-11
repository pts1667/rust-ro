# Turn Undead

<!-- Skill info -->
- **Class:** Priest
- **Type:** Offensive
- **Levels:** 10 (fixed)
- **Prerequisites:** ([Priest](Priest.md)) [Lex Divina](Lex_Divina.md) Lv. 3, [Resurrection](Resurrection.md) Lv. 1
- **SP cost:** 20
- **Cast time:** 1 second
- **Delay:** 3 seconds
- **Target:** Enemy
- **Element:** Holy


## Description

This skill can only be used on [Undead](Undead.md) property monsters. It can be used on Boss monsters, but it will only deal damage through the "fail" calculation. Has a chance to deal [Holy](Holy.md) damage equal to the current HP of the target.

| Level | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base Success Chance % | 2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 |

The following increase success chance:
* Each base level: +0.1%
* Each point of INT: +0.1%
* Each point of LUK: +0.1%
* +0-20% depending on percentage of monster's HP remaining

Success rate is capped at 70%.


## Formula

| Column 1 | Column 2 |
| --- | --- |
| Chance to Kill | [(Skill Level*20) + LUK + INT + BaseLevel + (1 - TargetHP/TargetMaxHP)*200]/10 % *(max 70%)* |
| Damage Done if Failed | [Base Level + INT + Skill Level*10] |


## Examples

A 98 [Priest](Priest.md) with 99 + 15 = 114 INT (from level 10 [Blessing](Blessing.md) and [Job Bonus](Priest.md)), 1 + 37 = 38 LUK (from [Gloria](Gloria.md) and [Job Bonus](Priest.md)) and Turn Undead Level 3, casting Turn Undead on a Wraith Dead with a remaining 32265 out of 43021 HP (75% HP left) will have a success chance of:

`[(3 * 20) + 38 + 114 + 98 + (1 - 32265 / 43021) * 200] / 10 % =`

`[   60    + 38 + 114 + 98 + (1 -     0.75     ) * 200] / 10 % =`

`[   60    + 38 + 114 + 98 +             50           ] / 10 % =`

`36% chance for Turn Undead to be successful`

If the skill fails it will do damage according to this formula:

`[98 + 114 + 3 * 10] * ([Undead](Undead.md) element level modifier)  =`

`[98 + 114 + 30] * 2 =`

`484 damage done to the enemy`


## Obtainable via

[Grand Cross](Grand_Cross.md) - Lv.3, manually or while attacking

**Related pages:**

* [Priest](Priest.md)
* [High Priest](High_Priest.md)
* [Skills](Skills.md)
* [Aspersio](Aspersio.md)
* [B.S Sacramenti](B.S_Sacramenti.md)
* [Gloria](Gloria.md)
* [Impositio Manus](Impositio_Manus.md)
* [Increase SP Recovery](Increase_SP_Recovery.md)
* [Kyrie Eleison](Kyrie_Eleison.md)
* [Lex Aeterna](Lex_Aeterna.md)
* [Lex Divina](Lex_Divina.md)
* [Mace Mastery](Mace_Mastery.md)
* [Magnificat](Magnificat.md)
* [Magnus Exorcismus](Magnus_Exorcismus.md)
* [Redemptio](Redemptio.md)
* [Resurrection](Resurrection.md)
* [Safety Wall](Safety_Wall.md)
* [Sanctuary](Sanctuary.md)
* [Slow Poison](Slow_Poison.md)
* [Status Recovery](Status_Recovery.md)
* [Suffragium](Suffragium.md)
* [Turn Undead](Turn_Undead.md)
* [Assumptio](Assumptio.md)
* [Basilica](Basilica.md)
* [Meditatio](Meditatio.md)
* [Spiritual Thrift](Spiritual_Thrift.md)
* [Quests](Quests.md)
* [Priest Job Change Guide](Priest_Job_Change_Guide.md)
* [Priest Skill Quest](Priest_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [Book](Book.md)
* [Knuckle](Knuckle.md)
* [Mace](Mace.md)
* [Rod](Rod.md)

---
**Categories:** [Forced Element Skills](categories/Forced_Element_Skills.md), [Holy](categories/Holy.md), [Offensive Skills](categories/Offensive_Skills.md), [Priest](categories/Priest.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Turn_Undead) — GFDL 1.3+*
