# DEF


## General

DEF (*def*ense) protects against physical attacks. Physical attacks are generally attacks based wholly or partially on [ATK](Stats.md). However, there are exceptions.

The descriptions used here refer to hard and soft DEF. These names are used for clarity. The vast majority of players do not use these terms in general conversation, and instead refer Armor Def (hard) and Vit Def (soft). Hard DEF works as a percentage reduction. Soft DEF is a subtractive reduction.


## Hard DEF

var HARD_DEF = 0;
HARD_DEF += HARD_DEF_A;
HARD_DEF = Math.floor( HARD_DEF * (1 + HARD_DEF_B / 100 ) );

HARD_DEF_A : Sum of additive bonuses
HARD_DEF_B : Sum of multiplicative bonuses

Base value of 0. Add bonuses from armor, cards, skills, etc.


## Additive Bonuses

* Equipment bonus to DEF: +1 per point
* Armor [upgrades](Refinement_System.md): +0.7 per point (inaccurate, exact value unknown)
* Argiope Card: +1
* Argos Card: +1
* Cornutus Card: +1
* Dokebi Card: +1
* Evil Druid Card: +1
* Ghoul Card: +1
* Martin Card: +1
* Megalodon Card: +1
* Munak Card: +1
* Pasana Card: +1
* Sandman Card: +1
* Stainer Card: +1
* Swordfish Card: +1
* Steel Chonchon Card: +2
* Ambernite Card: +2
* Solider Card: +2
* Tao Gunka Card: -50


## Multiplicative Bonuses

A number of skills and items reduce def in this way however. These effect both hard and soft def, and are often used to reduce damage from [Ice Pick](Ice_Pick.md) and [Occult Impaction](Occult_Impaction.md).

* [Grimtooth](Grimtooth.md): -50%
* [Spear Dynamo](Spear_Dynamo.md): -5% per level
* Spike: -66.7%


## Soft DEF

var SOFT_DEF = 0;
SOFT_DEF += Math.floor( VIT * 0.5 );
SOFT_DEF += Math.max( Math.floor( VIT * 0.3 ) , Math.floor( VIT^2 / 150 ) - 1 );
SOFT_DEF += SOFT_DEF_A;
SOFT_DEF = Math.floor( SOFT_DEF * (1 + SOFT_DEF_B / 100 ) );

SOFT_DEF_A : Sum of additive bonuses
SOFT_DEF_B : Sum of multiplicative bonuses

Base value of 0. Add complex result of VIT. Add bonuses from other sources.


## Additive Modifiers

[Divine Protection](Divine_Protection.md): +3 per level


## Multiplicative Modifiers

[Angelus](Angelus.md): +5% per level


## Bypassing DEF

The skills and weapons below are known to either bypass DEF, or have a reversed effect, thus increasing damage from the amount it.

Incomplete!


## Skills

* [Acid Bomb](Acid_Bomb.md) - increase damage, but on base [VIT](Stats.md) not DEF
* [Battle Chant](Battle_Chant.md)
* [Clashing Spiral](Clashing_Spiral.md)
* [Gloria Domini](Gloria_Domini.md)
* [Gravitational Field](Gravitational_Field.md)
* [Guillotine Fist](Guillotine_Fist.md)
* [Martyr's Reckoning](Martyr's_Reckoning.md)
* [Occult Impaction](Occult_Impaction.md) - increases damage based on DEF - see section on DEF reversal
* [Soul Destroyer](Soul_Destroyer.md)
* [Tarot Card of Fate](Tarot_Card_of_Fate.md)


## Weapons

* Ahlspiess
* Brocca
* Bloody Roar
* Caesar's Sword
* Combat Knife
* Dragon Killer
* Dragon Slayer
* Executioner
* Exorciser
* [Ice Pick](Ice_Pick.md) - increases damage based on DEF - see section on DEF reversal
* Legacy of Dragon
* Slaughter
* Weeder Knife


## Skills

Skills that work with the above weapons include:

* [Back Stab](Back_Stab.md)
* [Bash](Bash.md)
* [Bowling Bash](Bowling_Bash.md)
* [Brandish Spear](Brandish_Spear.md)
* [Counter Attack](Counter_Attack.md)
* [Flying Kick](Flying_Kick.md)
* [Grimtooth](Grimtooth.md)
* [Holy Cross](Holy_Cross.md)
* [Mammonite](Mammonite.md)
* [Martyr's Reckoning](Martyr's_Reckoning.md)
* [Pierce](Pierce.md)
* [Raging Trifecta Blow](Raging_Trifecta_Blow.md)
* [Snatch](Snatch.md)
* [Sonic Blow](Sonic_Blow.md)
* [Soul Destroyer](Soul_Destroyer.md)
* [Spear Boomerang](Spear_Boomerang.md)
* [Spear Stab](Spear_Stab.md)
* [Throw Spirit Sphere](Throw_Spirit_Sphere.md)


## DEF Reversal

In special conditions, such as for the Ice Pick weapon and [Occult Impaction](Occult_Impaction.md) skill, the DEF formula is reversed.

In this special case of reversed DEF, the damage multiplier is found by (Hard DEF + Soft DEF)/100. For example, against a 5+15 DEF target, a normal 200 ATK weapon would only deal 175 damage; an Ice Pick on a character with 200 ATK, would do 40 damage. Conversely, against a 50+90 DEF target, the weapons would do 10 and 280 damage respectively.


## See Also

* [MDEF](Stats.md)
* [VIT](Stats.md)

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/DEF) — GFDL 1.3+*
