# Focused Arrow Strike

<!-- Skill info -->
- **Class:** Sniper
- **Secondary class:** Mercenary
- **Type:** Offensive
- **Levels:** 5
- **Prerequisites:** ([Archer](Archer.md)) [Improve Concentration](Improve_Concentration.md) Lv. 10, [Double Strafe](Double_Strafe.md) Lv. 5
- **SP cost:** (15 + (Skill level*3))
- **Cast time:** 2 seconds
- **Delay:** 1.5 seconds
- **Target:** Foe
- **Range:** 9
- **Area:** approx. 3x12 cells, in direction of target
- **Catalyst:** 1 Arrow
- **Weapon:** Bow


## Description

Shoot an arrow with great might towards a targeted enemy. The arrow will be shot straight towards the target and will inflict great damage to enemies in the way. Can only be activated with a [Bow](Bow.md) equipped.

| Column 1 | Column 2 | Column 3 |
| --- | --- | --- |
| **Level** | **Damage** | **SP Cost** |
| 1 | +150% | 18 SP |
| 2 | +200% | 21 SP |
| 3 | +250% | 24 SP |
| 4 | +300% | 27 SP |
| 5 | +350% | 30 SP |


## Notes

The area of effect for Focused Arrow Strike is a little tricky, and almost unique in the game. The line algorithm is likely the same as the one used for the final resting place of objects pushed by [Arrow Repel](Arrow_Repel.md).

* Maximum distance from the targeted object is 9 cells.
* The full damage area extends 12 cells from the caster, along the line from caster to target as determined by a Bresenham type algorithm, as well as the cells immediately tangent to this line. Covers 3x12 cells.
* Monsters in the 13th cell along the line receive a reduced damage.
* Focused Arrow Strike has a 20% base chance of bypassing enemy defense and flee (Also know as Crit) and is also increased by the sniper's [CRIT](Stats.md) rate. The chance to bypass defense with this skill is also lowered depending on the opponent's [LUK](Stats.md) (Also known as Crit Shield).
* The chance to bypass defense with Focused Arrow Strike is calculated as [ (crit rate + 20) - (monster's luk/5) ]%. Crit/LUK gear is recommended in order to maximize the chance of bypassing defense with this skill.
* Sniping Suit is a suggested armor when heavily using this skill.
* Only cards and equipment that increase [CRIT](Stats.md) in the status window will increase Focused Arrow Strike's chance to Crit. Cards that increase Crit rate but does not show in the status window such as Drosera cards and +7 CRIT, +10% damage vs race cards will not work with this skill.

**Related pages:**

* [Hunter](Hunter.md)
* [Sniper](Sniper.md)
* [Skills](Skills.md)
* [Ankle Snare](Ankle_Snare.md)
* [Beast Bane](Beast_Bane.md)
* [Beast Charge](Beast_Charge.md)
* [Blitz Beat](Blitz_Beat.md)
* [Claymore Trap](Claymore_Trap.md)
* [Detect](Detect.md)
* [Falconry Mastery](Falconry_Mastery.md)
* [Flasher](Flasher.md)
* [Freezing Trap](Freezing_Trap.md)
* [Land Mine](Land_Mine.md)
* [Phantasmic Arrow](Phantasmic_Arrow.md)
* [Remove Trap](Remove_Trap.md)
* [Sandman](Sandman.md)
* [Shockwave Trap](Shockwave_Trap.md)
* [Skid Trap](Skid_Trap.md)
* [Spring Trap](Spring_Trap.md)
* [Steel Crow](Steel_Crow.md)
* [Talkie Box](Talkie_Box.md)
* [Falcon Eyes](Falcon_Eyes.md)
* [Falcon Assault](Falcon_Assault.md)
* [Focused Arrow Strike](Focused_Arrow_Strike.md)
* [Wind Walker](Wind_Walker.md)
* [Quests](Quests.md)
* [Hunter Job Change Guide](Hunter_Job_Change_Guide.md)
* [Hunter Skill Quest](Hunter_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [Bow](Bow.md)

---
**Categories:** [AoE Skills](categories/AoE_Skills.md), [Item Consuming Skills](categories/Item_Consuming_Skills.md), [Mercenary](categories/Mercenary.md), [Offensive Skills](categories/Offensive_Skills.md), [Sniper](categories/Sniper.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Focused_Arrow_Strike) — GFDL 1.3+*
