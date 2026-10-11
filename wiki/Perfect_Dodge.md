# Perfect Dodge

*Also known as **Lucky Dodge***.


## Description

Every character starts with a value of 1.

10 luk increases your Perfect Dodge by 1.

*  Perfect Dodge allows the player to dodge a physical attack by a chance in % corresponding to the Perfect Dodge value.
*  It is calculated before [FLEE](FLEE.md) hence why it is not reduced by the mob effect, that is to say when a player has several [Monsters](Monster.md) attacking him.
*  Flee therefore triggers after the fact that a player lucky dodged something or not, thus giving him even more chance to flee a monster. This part of the flee is reduced by the mob effect.
*  It does not work against skill attacks, traps, or magic.
*  This overrides (potential) critical hits.
*  Does not have a cap. Thus, 100% Lucky Dodge is possible.


## Note

*  Homunculus' flee is similar to this, as it is not reduced by the mob effect.


## Calculation

 var LUCKY = 1;
 LUCKY += LUK * 0.1;
 LUCKY += LUCKY_MOD;

 LUCKY_MOD : Sum of modifiers
LUCKY is assumed to be accurate to one decimal. (verify?)


## Modifiers

* Lunatic Card: +1
* Poring Card: +1
* Yoyo Card: +5
* Choco Card: +5
* Wild Rose Card: +5 ([Thief](Thief.md) class only)
* Chemeti Whip: +2
* Fortune Sword: +20 (and 5 [LUK](Stats.md))
* [Grimtooth](Grimtooth.md): +5
* Infiltrator: +2
* Falcon Muffler: +5
* Morrigane's Manteau: +8 (and 2 [LUK](Stats.md))
* Valkyrian Manteau: +5, +(refine rate)*2 ([Mage](Mage.md), [Archer](Archer.md) and [Acolyte](Acolyte.md) classes only)
* [Perfect Tablature](Perfect_Tablature.md)

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Perfect_Dodge) — GFDL 1.3+*
