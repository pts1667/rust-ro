# Steal

<!-- Skill info -->
- **Class:** Thief
- **Secondary class:** Super Novice
- **Type:** Active
- **Levels:** 10 (Fixed)
- **Prerequisites:** None
- **SP cost:** 10
- **Cast time:** None
- **Delay:** 1 second
- **Target:** Monsters
- **Range:** Melee


## Description

Attempts to "steal" an item from the targeted [Monster](Monster.md). Only items dropped by the monster can be stolen. A successful attempt will not affect what is dropped when the monster dies. After success, it is not possible to Steal again from the same monster. [Boss](Boss_Protocol.md)-protocol monsters cannot be stolen from.

The formula for stealing is believed to be:

AdjustedDropRatio = DropRatio*(DEX + 3*Skill level - MonsterDEX + 10)/100

* DropRatio is the percent chance of a particular item being dropped. If the result of the steal check is lower than the AdjustedDropRatio for an item, then it is possible that item will be stolen.

| Skill Level | Success Chance |
| --- | --- |
| 1 | 10% |
| 2 | 16% |
| 3 | 22% |
| 4 | 28% |
| 5 | 34% |
| 6 | 40% |
| 7 | 46% |
| 8 | 52% |
| 9 | 58% |
| 10 | 64% |


## Uses

* Gathers additional loot for the player to [Overcharge](Overcharge.md), [Vend](Vending.md), or [turn in for EXP](Repeatable_EXP_Quests.md).

**Related pages:**

* [Thief](Thief.md)
* [Skills](Skills.md)
* [Back Slide](Back_Slide.md)
* [Detoxify](Detoxify.md)
* [Envenom](Envenom.md)
* [Find Stone](Find_Stone.md)
* [Hiding](Hiding.md)
* [Improve Dodge](Improve_Dodge.md)
* [Sand Attack](Sand_Attack.md)
* [Steal](Steal.md)
* [Stone Fling](Stone_Fling.md)
* [Quests](Quests.md)
* [Thief Job Change Guide](Thief_Job_Change_Guide.md)
* [Thief Skill Quest](Thief_Skill_Quest.md)
* [Thief Training Quest](Thief_Training_Quest.md)
* [Weapons](Weapons.md)
* [Dagger](Dagger.md)
* [Bow](Bow.md)

**Related pages:**

* [Super Novice](Super_Novice.md)
* [Skills](Skills.md)
* [Acolyte](Acolyte.md)
* [Angelus](Angelus.md)
* [Aqua Benedicta](Aqua_Benedicta.md)
* [Blessing](Blessing.md)
* [Cure](Cure.md)
* [Decrease AGI](Decrease_AGI.md)
* [Demon Bane](Demon_Bane.md)
* [Divine Protection](Divine_Protection.md)
* [Heal](Heal.md)
* [Increase AGI](Increase_AGI.md)
* [Pneuma](Pneuma.md)
* [Ruwach](Ruwach.md)
* [Signum Crusis](Signum_Crusis.md)
* [Teleport](Teleport.md)
* [Warp Portal](Warp_Portal.md)
* [Archer](Archer.md)
* [Improve Concentration](Improve_Concentration.md)
* [Owl's Eye](Owl's_Eye.md)
* [Vulture's Eye](Vulture's_Eye.md)
* [Mage](Mage.md)
* [Cold Bolt](Cold_Bolt.md)
* [Fire Ball](Fire_Ball.md)
* [Fire Bolt](Fire_Bolt.md)
* [Fire Wall](Fire_Wall.md)
* [Frost Diver](Frost_Diver.md)
* [Increase SP Recovery](Increase_SP_Recovery.md)
* [Lightning Bolt](Lightening_Bolt.md)
* [Napalm Beat](Napalm_Beat.md)
* [Safety Wall](Safety_Wall.md)
* [Sight](Sight.md)
* [Soul Strike](Soul_Strike.md)
* [Stone Curse](Stone_Curse.md)
* [Thunderstorm](Thunderstorm.md)
* [Merchant](Merchant.md)
* [Discount](Discount.md)
* [Enlarge Weight Limit](Enlarge_Weight_Limit.md)
* [Item Appraisal](Item_Appraisal.md)
* [Mammonite](Mammonite.md)
* [Overcharge](Overcharge.md)
* [Pushcart](Pushcart.md)
* [Vending](Vending.md)
* [Swordman](Swordman.md)
* [Bash](Bash.md)
* [Endure](Endure.md)
* [Increase HP Recovery](Increase_HP_Recovery.md)
* [Magnum Break](Magnum_Break.md)
* [Provoke](Provoke.md)
* [Sword Mastery](Sword_Mastery.md)
* [Thief](Thief.md)
* [Detoxify](Detoxify.md)
* [Envenom](Envenom.md)
* [Hiding](Hiding.md)
* [Improve Dodge](Improve_Dodge.md)
* [Steal](Steal.md)
* [Quests](Quests.md)
* [Super Novice Job Change Guide](Super_Novice_Job_Change_Guide.md)
* [Weapons](Weapons.md)
* [Dagger](Dagger.md)
* [Rod](Rod.md)
* [One Handed Sword](One_Handed_Sword.md)

---
**Categories:** [Active Skills](categories/Active_Skills.md), [Super Novice](categories/Super_Novice.md), [Thief](categories/Thief.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Steal) — GFDL 1.3+*
