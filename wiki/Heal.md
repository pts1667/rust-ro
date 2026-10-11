# Heal

<!-- Skill info -->
- **Class:** Acolyte
- **Secondary class:** Crusader
- **Tertiary class:** Super Novice
- **Type:** Supportive
- **Levels:** 10 (Selectable)
- **Prerequisites:** ([Acolyte](Acolyte.md) & [Super Novice](Super_Novice.md)) None; ([Crusader](Crusader.md)) [Demon Bane](Demon_Bane.md) Lv. 5, [Faith](Faith.md) Lv. 10
- **SP cost:** (10 + (Skill level*3))
- **Cast time:** None
- **Delay:** 1 second
- **Target:** Self, Foe, or Other Players
- **Range:** 10 cells


## Description

Restores the target's HP. The amount of HP restores it based on the caster's base level, [INT](Stats.md) and the skill's level.

Heal used against [Undead](Undead.md) element targets counts as Offensive Healing - Heal does holy damage half as the normal healing (modified by undead element level) instead of restoring HP.

The amount of HP restored is determined by:

```
floor((Base Level + INT) / 8) * Base Heal

Base Heal = 4 + 8 * Skill Level
```

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 | Column 7 | Column 8 | Column 9 | Column 10 | Column 11 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **Level** | **1** | **2** | **3** | **4** | **5** | **6** | **7** | **8** | **9** | **10** |
| **SP cost** | 13 | 16 | 19 | 22 | 25 | 28 | 31 | 34 | 37 | 40 |
| **Base Heal** | 12 | 20 | 28 | 36 | 44 | 52 | 60 | 68 | 76 | 84 |

| Column 1 | Column 2 | Column 3 | Column 4 | Column 5 | Column 6 | Column 7 | Column 8 | Column 9 | Column 10 | Column 11 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|   | Skill Level | Skill Level | Skill Level | Skill Level | Skill Level | Skill Level | Skill Level | Skill Level | Skill Level | Skill Level |
| Base Level + INT | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| 0~15 | 12 | 20 | 28 | 36 | 44 | 52 | 60 | 68 | 76 | 84 |
| 16~23 | 24 | 40 | 56 | 72 | 88 | 104 | 120 | 136 | 152 | 168 |
| 24~31 | 36 | 60 | 84 | 108 | 132 | 156 | 180 | 204 | 228 | 252 |
| 32~39 | 48 | 80 | 112 | 144 | 176 | 208 | 240 | 272 | 304 | 336 |
| 40~47 | 60 | 100 | 140 | 180 | 220 | 260 | 300 | 340 | 380 | 420 |
| 48~55 | 72 | 120 | 168 | 216 | 264 | 312 | 360 | 408 | 456 | 504 |
| 56~63 | 84 | 140 | 196 | 252 | 308 | 364 | 420 | 476 | 532 | 588 |
| 64~71 | 96 | 160 | 224 | 288 | 352 | 416 | 480 | 544 | 608 | 672 |
| 72~79 | 108 | 180 | 252 | 324 | 396 | 468 | 540 | 612 | 684 | 756 |
| 80~87 | 120 | 200 | 280 | 360 | 440 | 520 | 200 | 680 | 760 | 840 |
| 88~95 | 132 | 220 | 308 | 396 | 484 | 572 | 660 | 748 | 836 | 924 |
| 96~103 | 144 | 240 | 336 | 432 | 528 | 624 | 720 | 816 | 912 | 1008 |
| 104~111 | 156 | 260 | 364 | 468 | 572 | 676 | 780 | 884 | 988 | 1092 |
| 112~119 | 168 | 280 | 392 | 504 | 616 | 728 | 840 | 952 | 1064 | 1176 |
| 120~127 | 180 | 300 | 420 | 540 | 660 | 780 | 900 | 1020 | 1140 | 1260 |
| 128~135 | 192 | 320 | 448 | 576 | 704 | 832 | 960 | 1088 | 1216 | 1344 |
| 136~143 | 204 | 340 | 476 | 612 | 748 | 884 | 1020 | 1156 | 1292 | 1428 |
| 144~151 | 216 | 360 | 504 | 648 | 792 | 936 | 1080 | 1224 | 1368 | 1512 |
| 152~159 | 228 | 380 | 532 | 684 | 836 | 988 | 1140 | 1292 | 1444 | 1596 |
| 160~167 | 240 | 400 | 560 | 720 | 880 | 1040 | 1200 | 1360 | 1520 | 1680 |
| 168~175 | 252 | 420 | 588 | 756 | 924 | 1092 | 1260 | 1428 | 1596 | 1764 |
| 176~183 | 264 | 440 | 616 | 792 | 968 | 1144 | 1320 | 1496 | 1672 | 1848 |
| 184~191 | 276 | 460 | 644 | 828 | 1012 | 1196 | 1380 | 1564 | 1748 | 1932 |
| 192~199 | 288 | 480 | 672 | 864 | 1056 | 1248 | 1440 | 1632 | 1824 | 2016 |
| 200~207 | 300 | 500 | 700 | 900 | 1100 | 1300 | 1500 | 1700 | 1900 | 2100 |
| 208~215 | 312 | 520 | 728 | 936 | 1144 | 1352 | 1560 | 1768 | 1976 | 2184 |
| 216~223 | 324 | 540 | 756 | 972 | 1188 | 1404 | 1620 | 1836 | 2052 | 2268 |
| 224~231 | 336 | 560 | 784 | 1008 | 1232 | 1456 | 1680 | 1904 | 2128 | 2352 |
| 232~239 | 348 | 580 | 812 | 1044 | 1276 | 1508 | 1740 | 1972 | 2204 | 2436 |
| 240~247 | 360 | 600 | 840 | 1080 | 1320 | 1560 | 1800 | 2040 | 2280 | 2520 |
| 248~255 | 372 | 620 | 868 | 1116 | 1364 | 1612 | 1860 | 2108 | 2356 | 2604 |


## Obtainable via

* Brionac - Lv. 5
* Enchanted Peach Tree Card - Lv. 1 on the wearer when attacking, Lv. 10 if the user has the skill mastered (You must relog or change map after obtaining Lv. 10 Heal, otherwise Lv. 1 still be used)
* Chepet Card - Lv. 5 on the enemy when attacking
* Light Epsilon - Lv. 3
* Loli Ruri Card - Lv. 3 on the wearer when receiving damage
* Level 3 Scroll and Level 5 Scroll - Lv. 3 and Lv. 5, one time usage
* Rudra Bow - Lv. 1
* Vitata Card - Lv. 1
* Hakujin \[0\] and Hakujin \[1\] - Lv. 1 on the wearer while performing physical attacks ([Ninja](Ninja.md) Class exclusive daggers).

**Related pages:**

* [Acolyte](Acolyte.md)
* [Skills](Skills.md)
* [Angelus](Angelus.md)
* [Aqua Benedicta](Aqua_Benedicta.md)
* [Blessing](Blessing.md)
* [Cure](Cure.md)
* [Decrease AGI](Decrease_AGI.md)
* [Demon Bane](Demon_Bane.md)
* [Divine Protection](Divine_Protection.md)
* [Heal](Heal.md)
* [Holy Light](Holy_Light.md)
* [Increase AGI](Increase_AGI.md)
* [Pneuma](Pneuma.md)
* [Ruwach](Ruwach.md)
* [Signum Crusis](Signum_Crusis.md)
* [Teleport](Teleport.md)
* [Warp Portal](Warp_Portal.md)
* [Quests](Quests.md)
* [Acolyte Job Change Guide](Acolyte_Job_Change_Guide.md)
* [Acolyte Skill Quest](Acolyte_Skill_Quest.md)
* [Acolyte Training Quest](Acolyte_Training_Quest.md)
* [Weapons](Weapons.md)
* [Mace](Mace.md)
* [Rod](Rod.md)

**Related pages:**

* [Crusader](Crusader.md)
* [Paladin](Paladin.md)
* [Skills](Skills.md)
* [Cavalier Mastery](Cavalier_Mastery.md)
* [Cure](Cure.md)
* [Defending Aura](Defending_Aura.md)
* [Demon Bane](Demon_Bane.md)
* [Divine Protection](Divine_Protection.md)
* [Faith](Faith.md)
* [Grand Cross](Grand_Cross.md)
* [Guard](Guard.md)
* [Heal](Heal.md)
* [Holy Cross](Holy_Cross.md)
* [Peco Peco Ride](Peco_Peco_Ride.md)
* [Resistant Souls](Resistant_Souls.md)
* [Sacrifice](Sacrifice.md)
* [Shield Boomerang](Shield_Boomerang.md)
* [Shrink](Shrink.md)
* [Smite](Smite.md)
* [Spear Mastery](Spear_Mastery.md)
* [Spear Quicken](Spear_Quicken.md)
* [Battle Chant](Battle_Chant.md)
* [Gloria Domini](Gloria_Domini.md)
* [Martyr's Reckoning](Martyr's_Reckoning.md)
* [Rapid Smiting](Rapid_Smiting.md)
* [Quests](Quests.md)
* [Crusader Job Change Guide](Crusader_Job_Change_Guide.md)
* [Crusader Skill Quest](Crusader_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [One Handed Sword](One_Handed_Sword.md)
* [Spear](Spear.md)
* [Two Handed Sword](Two_Handed_Sword.md)

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
**Categories:** [Acolyte](categories/Acolyte.md), [Crusader](categories/Crusader.md), [Super Novice](categories/Super_Novice.md), [Supportive Skills](categories/Supportive_Skills.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Heal) — GFDL 1.3+*
