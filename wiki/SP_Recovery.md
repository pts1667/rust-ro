# SP Recovery


## General

SP regen happens every 4 seconds while sitting and every 8 seconds while standing or walking. The skill [Magnificat](Magnificat.md) halves these times. Being in a fort owned by the player's guild doubles SP regen.

See also: [Increase SP Recovery](Increase_SP_Recovery.md), [Spiritual Cadence](Spiritual_Cadence.md)


## Calculations

 var SPR = 1;
 SPR += Math.floor( MAX_SP / 100 );
 SPR += Math.floor( INT / 6 );
 if (INT >= 120) {
  SPR += Math.floor(INT / 2 - 56);
 }
 SPR = Math.floor( SPR * (1 + SPR_MOD * 0.01) );

* SPR_MOD : Sum of modifiers

Base value of 1. Then +1 for every 100 SP and +1 for every 6 INT. If INT is at least 120, +1 for every 2 INT above 112. Finally increase by modifiers.


## Modifiers

* Sohee Card: +3%
* Eggyra Card: +15%
* Sleipnir: +15%
* Incubus card: -20% without combo/+10% with
* Ayam [1]: +3% if equipped by [Taekwon](TaeKwon_Kid.md) class character

---
**Categories:** [General Information](categories/General_Information.md), [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/SP_Recovery) — GFDL 1.3+*
