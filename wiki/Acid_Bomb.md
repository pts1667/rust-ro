# Acid Bomb

<!-- Skill info -->
- **Class:** Biochemist
- **Type:** Offensive
- **Levels:** 10 (fixed)
- **Prerequisites:** ([Alchemist](Alchemist.md)) [Acid Terror](Acid_Terror.md) Lv. 5, [Bomb](Bomb.md) Lv. 5
- **SP cost:** 30
- **Cast time:** 1 second
- **Delay:** 1 second
- **Range:** 9
- **Element:** Neutral
- **Catalyst:** 1 Acid Bottle, 1 Bottle Grenade


## Description

Consume 1 Bottle Grenade and 1 Acid Bottle to hurl the contents of both items at a targeted enemy to inflict an amount of damage affected by this skill's level, the caster's [INT](Stats.md), and the target's [VIT](Stats.md). In [PvP](PvP~d29d2.md) situations, Acid Bomb only inflicts 50% of its damage on other characters, but it may damage the targeted character's Weapon or Armor. This skill has a 1 second Cast Time followed by a 1 second Skill Delay, and consumes 30 SP per cast. Has a Skill Level's % chance to break weapon or armor.

| Skill level | Number of Hits | Weapon/Armor Damaging Chance |
| --- | --- | --- |
| 1 | 1 | 1% |
| 2 | 2 | 2% |
| 3 | 3 | 3% |
| 4 | 4 | 4% |
| 5 | 5 | 5% |
| 6 | 6 | 6% |
| 7 | 7 | 7% |
| 8 | 8 | 8% |
| 9 | 9 | 9% |
| 10 | 10 | 10% |


## Formula

Damage = SkillLevel x [ A x (1 - B) x (1 - C) x (1 - D) x (1 - E) x F x G [ H x 0.7 x INT x INT x VIT / (INT + VIT) ] ]

*  A = 0.5 for player, 1 for monster and else
*  B = race reduction gear/card (0.3 Thara Frog Card, 0.1 Poopoo Hat, 0.05 Zealotus Mask, 0.1 Combat Knife)
*  C = element reduction gear/card (0.75 Ghostring Card)
*  D = neutral damage reduction gear/card (0.2 Raydric Card, 0.1 Noxious Card, 0.5 Deviling Card)
*  E = anti-sniping gear/card (0.05 Alligator Card, 0.35 Horn Card, 0.1 Noxious Card, 0.1 Long Mace)
*  F = WoE reduction (0.6 for WoE maps, 1 for PvM and PvP maps)
*  G =  2/3 for players with Assumptio, 1 for players without Assumptio
*  H = 2 for [Lex Aeterna](Lex_Aeterna.md), 1 otherwise
*  INT is the biochemist INT stat
*  VIT is the target/enemy VIT stat
*  Values in bracket are rounded down (floor mathematical function)


## Information

*  Bypasses:
  * [Defending Aura](Defending_Aura.md)
  * [Guard](Guard.md)
  * [Cicada Skin Shed](Cicada_Skin_Shed.md) & [Mirror Image](Mirror_Image.md)
  * [Flee](Stats.md) & [DEF](DEF.md)
*  Blocked by:
  * [Kaupe](Kaupe.md)
  * [Pneuma](Pneuma.md) (missed hits consume bombs)
  * [Blinding Mist](Blinding_Mist.md) (75% chance for the skill to fail to connect. Bombs are not consumed when the skill does not connect)
  * [Flip Tatami](Flip_Tatami.md)
  * Additionally, misses on characters with 0 [VIT](Stats.md).
*  Methods of reducing damage:
  * Ghostring Card
  * Lowering [VIT](Stats.md)
  * Skills: [Assumptio](Assumptio.md), [Kyrie Eleison](Kyrie_Eleison.md), [Skin Tempering](Skin_Tempering.md) (5% neutral reduction)
  * Anti-[Demi Human](Demi_Human.md) gear (i.e. Thara Frog Card, Poo Poo Hat)
  * Anti-Ranged gear (i.e. Long Mace)
*  Acid Bomb is not affected by:
  * Cards/card aspects that do not increase [INT](Stats.md)
  * The Sign
  * [Fiber Lock](Fiber_Lock.md)
  * Skills/items affecting [VIT-based DEF](DEF.md) (i.e. [Angelus](Angelus.md), [Solar Protection](Solar_Protection.md), [Grimtooth](Grimtooth.md))


## External Links

[Acid Bomb Calculator](http://ro.frenzart.com/ab.html)

**Related pages:**

* [Alchemist](Alchemist.md)
* [Biochemist](Biochemist.md)
* [Skills](Skills.md)
* [Acid Terror](Acid_Terror.md)
* [Aid Berserk Potion](Aid_Berserk_Potion.md)
* [Aid Potion](Aid_Potion.md)
* [Alchemical Weapon](Alchemical_Weapon.md)
* [Axe Mastery](Axe_Mastery.md)
* [Biochemical Helm](Biochemical_Helm.md)
* [Bomb](Bomb.md)
* [Potion Research](Potion_Research.md)
* [Prepare Potion](Prepare_Potion.md)
* [Spiritual Potion Creation (variant 1)](Spiritual_Potion_Creation_(variant_1).md)
* [Spiritual Potion Creation (variant 2)](Spiritual_Potion_Creation_(variant_2).md)
* [Spiritual Potion Creation (variant 3)](Spiritual_Potion_Creation_(variant_3).md)
* [Summon Flora](Summon_Flora.md)
* [Summon Marine Sphere](Summon_Marine_Sphere.md)
* [Synthesized Shield](Synthesized_Shield.md)
* [Synthetic Armor](Synthetic_Armor.md)
* [Homunculus](Homunculus_System.md)
* [Bioethics](Bioethics.md)
* [Biotechnology](Biotechnology.md)
* [Call Homunculus](Call_Homunculus.md)
* [Creature Creation](Creature_Creation.md)
* [Cultivation](Cultivation.md)
* [Drillmaster](Drillmaster.md)
* [Flame Control](Flame_Control.md)
* [Heal Homunculus](Heal_Homunculus.md)
* [Homunculus Resurrection](Homunculus_Resurrection.md)
* [Vaporize](Vaporize.md)
* [Acid Bomb](Acid_Bomb.md)
* [Aid Condensed Potion](Aid_Condensed_Potion.md)
* [Alchemy](Alchemy.md)
* [Cultivate Plant](Cultivate_Plant.md)
* [Full Chemical Protection](Full_Chemical_Protection.md)
* [Potion Synthesis](Potion_Synthesis.md)
* [Quests](Quests.md)
* [Alchemist Job Change Guide](Alchemist_Job_Change_Guide.md)
* [Alchemist Skill Quest](Alchemist_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [Axe](Axe.md)
* [Dagger](Dagger.md)
* [Mace](Mace.md)
* [One Handed Sword](One_Handed_Sword.md)

---
**Categories:** [Biochemist](categories/Biochemist.md), [Forced Element Skills](categories/Forced_Element_Skills.md), [Item Consuming Skills](categories/Item_Consuming_Skills.md), [Neutral](categories/Neutral.md), [Offensive Skills](categories/Offensive_Skills.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Acid_Bomb) — GFDL 1.3+*
