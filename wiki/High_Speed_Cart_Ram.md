# High Speed Cart Ram

<!-- Skill info -->
- **Class:** Mastersmith
- **Type:** Offensive
- **Levels:** 10 (Fixed)
- **Prerequisites:** ([Merchant](Merchant.md)) [Mammonite](Mammonite.md) Lv. 10, ([Blacksmith](Blacksmith.md)) [Hammer Fall](Hammer_Fall.md) Lv. 5, ([Mastersmith](Mastersmith.md)) [Cart Boost](Cart_Boost.md) Lv. 1
- **SP cost:** 15
- **Cast time:** None
- **Delay:** [ASPD](ASPD.md) Based
- **Target:** Enemy
- **Range:** Melee
- **Status:** [Stun](Status_Effects.md)
- **Catalyst:** Zeny


## Description

High Speed Cart Ram (HSCR) can only be casted during the Cart Boost status initiated through the [Cart Boost](Cart_Boost.md) skill. Spend an amount of [Zeny](Zeny.md) determined by skill level to smash a Pushcart into an enemy, causing damage and possibly inflicting the [Stun](Status_Effects.md) status. This skill's level and the amount of weight carried in the Pushcart affect the damage inflicted with High Speed Cart Ram.

| Level | [ATK](Stats.md) (Weight/%) | [Zeny](Zeny.md) Cost | [Stun](Status_Effects.md) Chance |
| --- | --- | --- | --- |
| 1 | 15 | 600 | 5 |
| 2 | 14 | 700 | 10 |
| 3 | 13 | 800 | 15 |
| 4 | 12 | 900 | 20 |
| 5 | 11 | 1000 | 25 |
| 6 | 10 | 1100 | 30 |
| 7 | 9 | 1200 | 35 |
| 8 | 8 | 1300 | 40 |
| 9 | 7 | 1400 | 45 |
| 10 | 6 | 1500 | 50 |


## Formula

Current damage formula:

(ATK * [Cart Weight/(16-skill level)])% * Elemental Modifer

Current formula using [Power-Thrust](Power-Thrust.md) or [Maximum Power-Thrust](Maximum_Power-Thrust.md), damage formula is now:

(ATK * ([Cart Weight/(16-skill level)]+25 or 100)%) * Elemental Modifer

* ONLY cards that add ATK increase High Speed Cart Ram's damage. [Race](Race.md)/[Size](Size.md) (ignoring the [ATK](Stats.md) gained from Size cards) cards and Turtle General cards do not add any damage.


## Mechanics

* When cast with [Power-Thrust](Power-Thrust.md) activated, damage becomes ( 1333.33 + 25 )%, for a total of 1358% damage. *NOTE: 1333.33 is the round down to 2 digits from cart weight of 8000 / 6*
* When cast with [Maximum Power-Thrust](Maximum_Power-Thrust.md) activated, damage becomes ( 1333.33 + 100 )%, for a total of 1433% damage. *NOTE: 1333.33 is the round down to 2 digits from cart weight of 8000 / 6*
* Target's [VIT](Stats.md) and [Stun](Status_Effects.md) resistance will reduce the chance that the target will become stunned.
* Damage is reduced by target's [DEF](DEF.md), [VIT](Stats.md) (unless otherwise noted, i.e. if the player is wielding an Ice Pick or Combat Knife) and other reduction gear.
* This is a melee skill - it is blocked by [Safety Wall](Safety_Wall.md), although the chance to inflict [Stun](Status_Effects.md) will bypass Safety Wall.
* High Speed Cart Ram can miss based on the player's [Hit](Stats.md) and the target's [Flee](Stats.md). However, when the skill "misses", a small damaged hit is still guaranteed from [Weaponry Research](Weaponry_Research.md). This means the player will always have a chance to stun their opponent whether their attack "misses" or deals full damage.


## Information

Bypasses:
* [Shield Reflect](Shield_Reflect.md)
* [Guard](Guard.md)
* [Cicada Skin Shed](Cicada_Skin_Shed.md)
* [Parry](Parry.md)
* [Energy Coat](Energy_Coat.md)
Works/damage increases with:
* Ice Pick
* Combat Knife
* Endows (HSRC takes on the element of your weapon)
* +[ATK](Stats.md) cards/equipment
* [STR](Stats.md), [DEX](Stats.md), [LUK](Stats.md)
* [ASPD](ASPD.md) (only for damage per second, DPS)
* Player's weapon ATK + upgrades
Not compatible with:
* The Sign
* [Shattering Strike](Shattering_Strike.md)
* Status weapons, including Sword/Mailbreakers
* ATK increasing equipment that is not raw [ATK](Stats.md) (i.e. [Race](Race.md) cards, Masquerade, Zealotous Mask)

**Related pages:**

* [Blacksmith](Blacksmith.md)
* [Mastersmith](Mastersmith.md)
* [Skills](Skills.md)
* [Adrenaline Rush](Adrenaline_Rush.md)
* [Advanced Adrenaline Rush](Advanced_Adrenaline_Rush.md)
* [Dubious Salesmanship](Dubious_Salesmanship.md)
* [Enchantedstone Craft](Enchantedstone_Craft.md)
* [Greed](Greed.md)
* [Hammer Fall](Hammer_Fall.md)
* [Hilt Binding](Hilt_Binding.md)
* [Iron Tempering](Iron_Tempering.md)
* [Ore Discovery](Ore_Discovery.md)
* [Oridecon Research](Oridecon_Research.md)
* [Power Maximize](Power_Maximize.md)
* [Power-Thrust](Power-Thrust.md)
* [Skin Tempering](Skin_Tempering.md)
* [Smith Axe](Smith_Axe.md)
* [Smith Dagger](Smith_Dagger.md)
* [Smith Knucklebrace](Smith_Knucklebrace.md)
* [Smith Mace](Smith_Mace.md)
* [Smith Spear](Smith_Spear.md)
* [Smith Sword](Smith_Sword.md)
* [Smith Two-handed Sword](Smith_Two-handed_Sword.md)
* [Steel Tempering](Steel_Tempering.md)
* [Weapon Perfection](Weapon_Perfection.md)
* [Weapon Repair](Weapon_Repair.md)
* [Weaponry Research](Weaponry_Research.md)
* [Battle Machine Craft](Battle_Machine_Craft.md)
* [Cart Boost](Cart_Boost.md)
* [Coin Craft](Coin_Craft.md)
* [High Speed Cart Ram](High_Speed_Cart_Ram.md)
* [Maximum Power-Thrust](Maximum_Power-Thrust.md)
* [Shattering Strike](Shattering_Strike.md)
* [Upgrade Weapon](Upgrade_Weapon.md)
* [Quests](Quests.md)
* [Blacksmith Job Change Guide](Blacksmith_Job_Change_Guide.md)
* [Blacksmith Skill Quest](Blacksmith_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [Axe](Axe.md)
* [Mace](Mace.md)

---
**Categories:** [Item Consuming Skills](categories/Item_Consuming_Skills.md), [Mastersmith](categories/Mastersmith.md), [Offensive Skills](categories/Offensive_Skills.md), [Status Inflicting Skills](categories/Status_Inflicting_Skills.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/High_Speed_Cart_Ram) — GFDL 1.3+*
