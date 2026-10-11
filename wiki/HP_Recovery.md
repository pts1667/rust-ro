# HP Recovery


## General

HP recovery happens every 3 seconds while sitting and every 6 seconds while standing. Characters with [HP Recovery While Moving](HP_Recovery_While_Moving.md) also regen HP every 12 seconds while walking.

See also: [Increase HP Recovery](Increase_HP_Recovery.md), [Spiritual Cadence](Spiritual_Cadence.md), [Song of Lutie](Song_of_Lutie.md)


## Calculations

 var HPR = Math.max( 1, Math.floor(MAX_HP / 200) );
 HPR += Math.floor( VIT / 5 );
 HPR = Math.floor( HPR * (1 + HPR_MOD * 0.01) );

HPR_MOD : Sum of modifiers

Base value of 1 per 200 HP, minimum 1. Then +1 for every 5 [VIT](Stats.md). Finally increase by modifiers.


## Modifiers

* Muka Card: +10%
* Zombie Card: +20%
* Wooden Golem Card: +30%
* Arc Angeling Card: +100%
* Ghostring Card: -25%
* Succubus card: -20% without combo/+10% with
* Magistrate Hat [1]: +3% if equipped by [Taekwon](TaeKwon_Kid.md) class character

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/HP_Recovery) — GFDL 1.3+*
