# Skills


## Overview

In Ragnarok, there are generally 3 types of skills: Passive, Active, Offensive. Passive skills are always "activated" although they may require the user be attacked or otherwise influenced by other environmental effects in order to go off. Active skills must be manually activated by the player- some have a set time period in which they stop by themselves, others can be manually deactivated, and some are one-time-use skills that require manual activation each time the player desires to use them. Active skills **do not** inflict actual damage, differentiating them from Offensive skills. All Offensive skills are *also* Active skills, but not vise-versa. Finally, Offensive skills are simply Active skills that also do damage to other players and/or [Monsters](Monster.md).

Skills can be further broken down into other sub-categories such as Melee, Ranged, Magic, Supportive, etc. Skills in these categories also fall under one of the three "main" categories listed in the previous paragraph. Some skills fall under multiple sub-categories as well. Ranged skills do not require the player to be standing adjacent to the target, whereas Melee skills generally do. Magic skills almost always involve a [Cast Time](Skills.md) and are usually Ranged. Supportive skills are termed due to their helpful effects they give to a single player or group of players, enhancing their attack and defensive capabilities.


## Skill Use Phases

Skill use is a five-step process, three points seperated by two phases.

* Skill used
  * Skill name appears above character's head.
* Cast time
  * Cast bar is visible above character's head.
* Skill activation
  * Skill name appears above character's head again.
* Cast delay
  * No visible indication. When the delay ends you will be able to use another skill.
* Skill use complete
  * No visible indication.


## Phases in Detail


## Skill Used

This is when the character uses the skill. SP cost and other casting costs are checked, but do not take effect. If the character cannot pay any of the costs, the skill will not activate.


## Cast Time

During the cast time, the character cannot move, cannot attack and cannot use other skills. If the character takes damage during cast time, the skill is canceled (exceptions: wearing a Phen Card or Bloody Butterfly Card, being [Sacrificed](Sacrifice.md) by a [Crusader](Crusader.md)/[Paladin](Paladin.md), or while casting certain skills). If a skill has no base cast time or modifiers reduce the effective cast time to 0 or less, no cast bar appears and the cast time phase is skipped entirely.

Cast time is indicated by a "progress bar" above the character's head, sometimes called cast bar. The bar starts out black and is filled with green as cast time proceeds. When the cast bar is completely green, cast time ends. Some skills do not have a cast bar, even though they have a casting time, such as [Bowling Bash](Bowling_Bash.md) and [Brandish Spear](Brandish_Spear.md).

Cast time is sometimes called (pre-)cast delay, easily confused with (post-)cast delay.

Exceptions:

* [Free Cast](Free_Cast.md) skill allows movement and attacking
* Bugs may sometimes allow attacking
* Phen Card and Bloody Butterfly Card both allow taking damage without being interrupted (but not during [War of Emperium](War_of_Emperium.md))
* The [Crusader](Crusader.md) skill [Sacrifice](Sacrifice.md) allows the caster to take damage without being interrupted (works in War of Emperium)
* Certain skills cannot be interrupted


## Calculation

Cast time is only calculated once. It ignores changes to the modifiers during cast. This means it is possible to start casting and equip a phen clip afterwards to get the phen effect for most of the cast time without the cast time increase.

The base cast time depends on skill and skill level.

 Cast time = Base time * (1 - DEX/150) * (1 - 0.15 * SUF) * (1 - x * 0.01)
 SUF is level of Suffragium active
 x is sum of modifiers.

In English:

1.  Start with base time.
1.  Reduce by 2/3% per point of DEX.
1.  Reduce by 15% per level of Suffragium.
1.  Reduce by x%.


## Modifiers

* Bloody Butterfly Card: +30%
* [Magic Strings](Magic_Strings.md): see skill description
* Phen Card: +25%
* [Suffragium](Suffragium.md): -15% * Skill Lv.
* Tower Keeper Card: -5%
* Kathryne Keyron Card: -1% per upgrade of the headgear
* Orlean's Gown \[1\]: +15%
* [Foresight](Foresight.md): -50%

Bear in mind that although Bloody Butterfly Card adds 30%, the full set gives a bonus of -15% meaning that it would only be an end result of +15% (vs. the Phen's 25%).

[Magic Strings](Magic_Strings.md) and [Foresight](Foresight.md) are treated as separate factors in the calculation; they are not added to the other modifiers.


## Skill Activation

SP cost and other casting costs are checked again. If the character can pay the costs, they are paid and the skill takes effect. If not, nothing happens. If the character is inside a chat room, no skill effect happens, but the costs are paid as usual.


## Cast Delay

During cast delay, the character can move and attack freely, but cannot use skills. There is no visible indication of cast delay.

Cast delay is often called skill delay for non-magic skills.


## Calculation

The base delay depends on the skill and skill level.

 Cast delay = base delay * (1 - x * 0.01)
 x is sum of modifiers

In English:

1.  Start with base delay.
1.  Reduce by x%. (Rounding and other details unknown)


## Modifiers

1.  [Magic Strings](Magic_Strings.md): see skill description
1.  Kiel D-01 Card
1.  Expert Ring
1.  Diabolus Robe
1.  Koneko Hat
1.  Sniping Suit
1.  Tae Goo Lyeon (upgrade over +9)
1.  Thorn Staff of Darkness
1.  Speedy Recovery Wand
1.  Glorious Apocalypse (upgrade over +9)
1.  Glorious Arc Wand (upgrade over +9)
1.  Glorious Cur Wand

**Equipment Sets**
1.  Sprint [Armor](Armor.md), Shoes and Ring (the Ring increases the delay by 5%)
1.  Archer's Battleground [Armor](Armor.md), [Garment](Garments.md) and Shoes
1.  Gunslinger's Battleground [Armor](Armor.md), [Garment](Garments.md) and Shoes


## SP Consumption

SP consumption cannot reach 0, even if the sum of sp consumption modifers reach more than 100% a skill will always use a minimum of 1 SP.

 SP Concumption = Maximum (1, base SP Consumption * (1 - x * 0.01))
 x is sum of sp consumption modifiers


## Modifiers

1.  [Gypsy's Kiss](Gypsy's_Kiss.md): see skill description
1.  Pharaoh Card
1.  Tae Goo Lyeon (upgrade over +9)
1.  Staff of Bordeaux (if the player has Dragonology)
1.  Glorious Holy Avenger (upgrade over +6)
1.  Glorious Rapier (upgrade over +6)

**Equipment Set**
1.  [Icarus Wings and Bowman Scarf set](Equipment_Sets.md)
1.  [Odin's Blessing and Ulle's Cap set](Equipment_Sets.md) (if base Dex is over 70)
1.  [Skull Cap and Elven Ears set](Equipment_Sets.md)
1.  [Shinobi Sash and Ninja Suit set](Equipment_Sets.md)
1.  [Acolyte Card set](Card_Sets.md)
1.  [Monk Card set](Card_Sets.md)
1.  [Rogue Card set](Card_Sets.md)


## See Also

* [Guild Skills](Guild_System.md)
* [Monster Exclusive Skills](Monster_Exclusive_Skills.md)

---
**Categories:** [Mechanics](categories/Mechanics.md), [Skills](categories/Skills.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Skills) — GFDL 1.3+*
