# Divest Shield

<!-- Skill info -->
- **Class:** Rogue
- **Type:** Active
- **Levels:** 5
- **Prerequisites:** ([Rogue](Rogue.md)) [Divest Helm](Divest_Helm.md) Lv. 5
- **SP cost:** (15+Skill Level)
- **Cast time:** 1 second
- **Delay:** 1 second
- **Duration:** 60s + Skill Adjustment and Dex Difference
- **Target:** Enemy
- **Range:** Melee
- **Status:** [Divest](Status_Effects.md)


## Description

Forcibly strip the shield off a target. While used on players in [PvP](PvP~d29d2.md) environment, the target cannot equip a shield for period of time. Used in [PvM](PvM~a1fb2.md), this skill decreases the Hard [DEF](DEF.md) of a monster by 15%.  The skill has a base success rate of 5%, and base duration of 60 seconds, which is improved with higher skill levels and adjusted by the difference in [DEX](Stats.md) between you and the target.

| Skill Level | Bonus Success Rate | Duration Bonus | SP Cost |
| --- | --- | --- | --- |
| 1 | +5% | +15s | 16 |
| 2 | +10% | +30s | 17 |
| 3 | +15% | +45s | 18 |
| 4 | +20% | +60s | 19 |
| 5 | +25% | +75s | 20 |


## Information

Casting
* [DEX](Stats.md) affects the [Cast Time](Skills.md) of the skill.
* The [Cast Time](Skills.md) cannot be interrupted except by use of [Spell Breaker](Spell_Breaker.md).
* The skill's range is checked after the skill is finished casting.  If the target is out of range when casting is complete, the skill will fail.
* If the opponent isn't wearing a shield, the skill will always fail.
* The Divest Shield status does not prevent players from using the shield slot for equipping two-handed weapons.

Duration and Effect
*  Success Rate [%] = 5 + (5 * Skill Level) + (Dex Difference / 5)
*  Duration [seconds] = 60 + (15 * Skill Level) + (Dex Difference / 2)
*  Dex Difference is your dex minus the target's dex.  If your dex is higher than the targets, the chance of success and duration will increase.  If your dex is lower than the targets, the duration and chance will decrease.
*  There is a base 5% chance of success, the success rate cannot be reduced below this 5%.

Example:

| Level 5 with 100 Dex | Level 5 with 100 Dex | Level 5 with 100 Dex |
| --- | --- | --- |
| Target's Dex | Success Rate | Duration |
| 50 | 40% | 160 seconds |
| 150 | 20% | 110 seconds |
| 185 | 13% | 92.5 seconds |

Other Information

*  The status cannot be removed normally.  The skill [Dispell](Dispell.md) does not remove the status.  The High Priest card from a Minstrel's [Tarot Card of Fate](Tarot_Card_of_Fate.md) and a Paladin casting [Battle Chant](Battle_Chant.md) can remove the status.
*  The divest status is lost when you log out.
*  Can be used on [Boss](Boss_Protocol.md) type monsters.
*  When used against a monster, the monster's Hard [DEF](DEF.md) is reduced by 15%.
*  Only shows animation on success.
*  You can divest a target in frozen status, and on success the frozen status is not lost.

**Related pages:**

* [Rogue](Rogue.md)
* [Stalker](Stalker.md)
* [Skills](Skills.md)
* [Back Stab](Back_Stab.md)
* [Close Confine](Close_Confine.md)
* [Divest Armor](Divest_Armor.md)
* [Divest Helm](Divest_Helm.md)
* [Divest Shield](Divest_Shield.md)
* [Divest Weapon](Divest_Weapon.md)
* [Double Strafe](Double_Strafe.md)
* [Gank](Gank.md)
* [Haggle](Haggle.md)
* [Intimidate](Intimidate.md)
* [Mug](Mug.md)
* [Piece](Piece.md)
* [Remove Trap](Remove_Trap.md)
* [Remover](Remover.md)
* [Scribble](Scribble.md)
* [Sightless Mind](Sightless_Mind.md)
* [Slyness](Slyness.md)
* [Snatch](Snatch.md)
* [Stalk](Stalk.md)
* [Sword Mastery](Sword_Mastery.md)
* [Vulture's Eye](Vulture's_Eye.md)
* [Counter Instinct](Counter_Instinct.md)
* [Full Divestment](Full_Divestment.md)
* [Preserve](Preserve.md)
* [Steal Backpack](Steal_Backpack.md)
* [Stealth](Stealth.md)
* [Quests](Quests.md)
* [Rogue Job Change Guide](Rogue_Job_Change_Guide.md)
* [Rogue Skill Quest](Rogue_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [Bow](Bow.md)
* [Dagger](Dagger.md)
* [One Handed Sword](One_Handed_Sword.md)

---
**Categories:** [Active Skills](categories/Active_Skills.md), [Rogue](categories/Rogue.md), [Status Inflicting Skills](categories/Status_Inflicting_Skills.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Divest_Shield) — GFDL 1.3+*
