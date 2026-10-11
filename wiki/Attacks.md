# Attacks

This page is devoted to describing how Attacks are resolved. As there are basically two kinds of attacks, physical and magical, it is split into two sections.


## Explanation of Conventions

Below are a number of conventions in the calculations that may be unfamiliar. Explanations follow:

*  [] - Square brackets hold the same priority as normal brackets, but indicate that the value of the contents should be rounded down to the nearest whole number (integer) once calculated.
*  rnd(x,y) - This is the expression of a random whole number (integer) between x and y. eg. rnd(3,50) would be a whole number (integer) between 3 and 50.
*  min(x,y) - This expression indicates that you should take the lower of x and y.
*  max(x,y) - This expression indicates that you should take the higher of x and y.
*  x^y - This is x to the power of y. That is, x multiplied by itself y times. eg. 5^3 would be 5*5*5.
*  {} - Curly brackets hold the highest priority, even higher than square and normal brackets. They indicate that the contents should be rounded down to the nearest whole number (integer) once calculated. Then, if the result is less than 0, it should be made equal to 0.


## Physical Attacks

Physical Attacks encompass all regular attacks (from monster or player) and all physical attack skills. The game goes through the following process to assess the result of an attack.

1.  Is the attack a skill attack? If so, go to step 6.
  1. All active attack skills are automatically considered skill attacks.
  1. [Double Attack](Double_Attack.md) is checked for on every non-skill attack with a Dagger weapon as the primary weapon (right-hand weapon). If a Double Attack is determined to have occurred (based on the level of the attackers Double Attack skill), then it automatically becomes a Skill Attack.
  1. When wielding dual weapons (only [Assassin](Assassin.md)s can do this), the left-hand weapon always counts as a skill attack, although if the right-hand weapon gets a crit then, the crit bubble will show on the left-hand weapon hit as well. You will also never see a "Miss!" from a left-hand weapon.
  1. The Sidewinder Card in a weapon gives the weapon a chance of a Double Attack equal to a Double Attack Level 1 with non-skill attacks. This over-rules the normal restriction of Double Attack only applying to a Dagger weapon.
  1. The monster skill [Combo Attack](NPC_COMBOATTACK.md) is a skill attack too.
  1. Defensive attack skills like [Counter Attack](Counter_Attack.md) and [Poison React](Poison_React.md) cause normal attacks. They are not skill attacks.
1.  Does a [Safety Wall](Safety_Wall.md) effect protect the target and is it a melee attack? If so, reduce the total number of hits left on the safety wall by 1 and then stop.
  1. A melee attack from a monster is any normal physical attack with a range of less than 4 tiles.
  1. A melee attack from a player is any normal physical attack from any weapon other than a Bow.
1.  Does a [Pneuma](Pneuma.md) effect protect the target and is it a ranged attack? If so, damage is 0, go to step 26.
  1. A ranged attack from a monster is any normal attack with a range of at least 4 tiles.
  1. A ranged attack from a player is any normal physical attack with a Bow or with a physical attack skill that is also designated as Ranged.
1.  Does a [Perfect Dodge](Perfect_Dodge.md) occur? If so, indicate with a "Lucky!" animation above the target's head and then stop.
  1. Player characters have a **1 + [LUK/10] + EquipmentBonuses** percent chance of perfect dodging an attack directed at them. This is the same as the right-hand part of their flee value.
  1. Monster targets cannot perfect dodge.
1.  Does a critical hit occur? If so, go to step 7.
  1. Player characters have a **(1 + LUK*0.3 + EquipmentBonuses) * CritModifier - TargetLuk/5** percent chance of causing a critical hit. This is not the same as the critical value shown in the status window!
  1. Monsters only do a critical hit if they have used the monster skills [Critical Slash](NPC_CRITICALSLASH.md) or [Counter Attack](Counter_Attack.md) to make an attack (and then it is an automatic critical hit).
1.  Does a hit or a miss occur? If a miss, assign an damage value of 0 and go to step 18. If a hit, display an "Impact" animation.
  1. A hit has a **80 + AttackerHit - DefenderFlee** percent chance of occurring. An alternative formula for use with the monster hit table is **100 + PlayerHit - HitRequiredfor100%**.
  1. For players, DefenderFlee is reduced  by 10% of its value for each attacker beyond the second.
  1. Some skills automatically hit. Others increase AttackerHit by a certain amount.
1.  Calculate BaseATK.
  1. BaseATK for melee weapons is **STR + [STR/10]^2 + [DEX/5] + [LUK/5] + UpgradeBonus + ImpositioManus + ATKCards**.
  1. BaseATK for missile weapons is **DEX + [DEX/10]^2 + [STR/5] + [LUK/5] + UpgradeBonus + ImpositioManus + ATKCards**.
  1. Monsters have a BaseATK of 0.
1.  Calculate WeaponATK.
  1. WeaponATK for melee weapons is **rnd(min(DEX*(0.8+0.2*WeaponLevel),ATK), ATK)**. In the case of a critical hit, it is ATK.
  1. WeaponATK for bows is **rnd(ATK/100*min(ATK, dex*(0.8+0.2*WeaponLevel)), max(ATK, ATK/100*min(ATK, dex*(0.8+0.2*WeaponLevel)))) + rnd(0, ArrowATK - 1)**. In the case of a critical hit, it is ATK + ArrowATK.
  1. WeaponATK when weaponless (fist) is 0.
  1. WeaponATK for Monsters is simply **rnd(minATK, maxATK)** using the values shown in my monster list. In the case of a critical hit (due to a skill), use maxATK.
  1. In the case of Dual Wielded weapons, all cards apply to the right-hand weapon only.
1.  Apply [Size](Size.md) modifier to Weapon ATK.
  1. Size modifiers vary by weapon.
  1. Monsters count as using Fist.
  1. Adult Players are of medium size, adopted child players are of small size.
1.  Apply skill modifiers to the total of Base ATK + WeaponATK.
  1. Skill modifiers add together (eg. 1 + 0.25 + 0.9).
  1. [Power-Thrust](Power-Thrust.md) gives a skill modifier and may apply to any physical attack.
  1. [Provoke](Provoke.md) gives a skill modifier and may apply to any physical attack.
  1. Many physical attack skills provide a third possible skill modifier. Eg. [Spear Dynamo](Spear_Dynamo.md)
  1. Calculate as **(100 + PowerThrust% + Provoke% + AttackSkill%)/100**.
1.  Apply [Hard DEF](DEF.md) modifiers to the current total.
  1. Formula is **Total*(1-DEF/100)**.
  1. Player character Hard DEF is not the same as shown in the status window.
  1. A critical hit counts all Hard DEF as 0.
1.  Subtract [Soft DEF](DEF.md) from the current total.
  1. Soft DEF for players is equal to **[VIT*0.5] + rnd([VIT*0.3], max([VIT*0.3],[VIT^2/150]-1))**.
  1. Soft DEF for monsters is equal to **VIT + rnd(0,[VIT/20]^2-1)**.
  1. A Critical Hit reduces Vit DEF to 0.
1.  Subtract applicable Protection skill values from the current total.
  1. [Divine Protection](Divine_Protection.md) reduces damage from [Undead](Undead.md) element and [Demon](Demon.md) race monsters.
  1. [Dragonology](Dragonology.md) reduces damage from [Dragon](Dragon.md) class monsters.
1.  Add applicable Bane skill values to the current total.
  1. [Beast Bane](Beast_Bane.md) adds damage to [Brute](Brute.md) and [Insect](Insect.md) race monsters.
  1. [Demon Bane](Demon_Bane.md) adds damage to [Undead](Undead.md) element and [Demon](Demon.md) race monsters.
  1. [Dragonology](Dragonology.md) adds damage to [Dragon](Dragon.md) race monsters.
1.  Add Upgrade Damage to the current total.
  1. For player characters, this is the value on the right-hand side of ATK in the status window.
  1. Monsters do not get Upgrade damage.
1.  Round damage down and if the total is less than 1, increase to 1.
1.  Add applicable Mastery skill values to the current total.
  1. There are currently Mastery skills for Spear (all), 1-H Sword (Daggers and 1-H Swords), Two-Hand Sword (2-H Swords only), Mace and Katar.
1.  Add [Weaponry Research](Weaponry_Research.md) damage and [Envenom](Envenom.md) bonus damage as applicable.
  1. Weaponry Research damage occurs to every attack by a [Blacksmith](Blacksmith.md) with the [Weaponry Research](Weaponry_Research.md) skill.
  1. [Envenom](Envenom.md) bonus damage applies to Envenom skill attacks only.
1.  Apply the Elemental Modifier to the current total.
  1. The Elemental Modifier depends on the property of the attack and the property of the target.
  1. Normal attacks from monsters are always [Neutral](Neutral.md) property.
  1. Skill attacks from monsters may have other properties.
  1. Normal melee attacks from players have the property of the weapon wielded. The property of the weapon may be over-ridden by various spell effects however ([Enchant Poison](Enchant_Poison.md), [Aspersio](Aspersio.md), etc.).
  1. Normal bow attacks from players have the property of the arrow being used. This may be over-ridden by an elemental bow (Rudra's Bow is the only known elemental bow). The property of the weapon and arrow may be over-ridden by various spell effects however ([Enchant Poison](Enchant_Poison.md), [Aspersio](Aspersio.md), etc.).
  1. Skill attacks from players have a property determined by the skill. In many cases, this is merely the property of the weapon, but it may be forced to be a particular elemental property (eg. [Magnum Break](Magnum_Break.md) forces [Fire](Fire.md) property).
  1. [Cart Revolution](Cart_Revolution.md) is a rare skill that forces the elemental modifier to be applied twice. Once with a Neutral property and once with the property of the weapon wielded.
1.  Add Enhancement damage.
  1. The only source of Enhancement damage is Very's on forged weapons. Each Very is worth 5 damage.
1.  Apply Damage Bonus modifiers.
  1. Damage Bonuses are based on [Size](Size.md), [Race](Race.md), [Element](Element.md) and Special. Bonuses of the same type stack, while bonuses of differing types multiply together. The overall calculation can be expressed as **(1+SizeBonusesTotal/100) * (1+RaceBonusesTotal/100) * (1+ElementBonusesTotal/100) * (1+SpecialBonusesTotal/100)**.
  1. Many cards and other equipment can give [Size](Size.md), [Race](Race.md) or [Element](Element.md) Bonuses.
  1. Currently, there are no known Special Bonuses.
1.  Apply Damage Reduction modifiers.
  1. Damage Reductions are based on Size, Race, Element and Special. Bonuses of the same type stack, while bonuses of differing types multiply together. The overall calculation can be expressed as (1-SizeReductionsTotal/100) * (1-RaceReductionsTotal/100) * (1-ElementReductionsTotal/100) * (1+SpecialReductionsTotal/100).
  1. Many cards and items of equipment can give [Size](Size.md), [Race](Race.md) or [Element](Element.md) Reductions.
  1. The [Energy Coat](Energy_Coat.md) skill gives a Special Reduction.
  1. Special Reductions and rules apply to wielding two weapons.
1.  Round damage down and if the total is less than 1, increase to 1.
1.  If a "multi-attack" (double attack included), multiply total by number of hits.
1.  If attacking a "herb plant" and the total is greater than 1 then reduce damage to 1.
  1. Herb plants are Blue, Green, Red, Shining, Yellow and White Plants and Red and Black Mushrooms.
  1. The monster Antonio also acts like a "herb plant".
  1. Some monsters can also force this status onto themselves by performing [Barrier](NPC_BARRIER.md).
  1. Exception applies to [Gloria Domini](Gloria_Domini.md), which deals full damage to this status.
1.  If [Kyrie Eleison](Kyrie_Eleison.md) is in effect on the target, soak up as much damage as possible. If damage is reduced to 0 and at least 1 point of damage was soaked up, display the Guard effect to the defender, deduct the total damage from the defense pool, deduct 1 hit from the total hit defense and then stop. If damage is not reduced to 0, cancel Kyrie Eleison effect on the target.
1.  If a "multi-attack" (double attack included), divide total by number of hits.
1.  Is the current total less than 1? If so, display the appropriate number of Misses and then stop.
  1. The display for a Miss is the word "Miss!" in red rising up and behind from above the attackers head.
  1. The appropriate number of Misses is 1 for a Double Attack, 2 for a missed attack with Katars, 0 for a missed attack with a left-hand weapon (while dual wielding) and 1 in any other case.
1.  Was it a single non-Katar Hit or Critical Hit? If so, display the Hit, adjust the hps of the target and stop.
  1. The display for a Hit is a number equal to the final total in white rising up and behind from above the targets head. There is a sound associated with each different weapon type.
  1. A Critical Hit adds a spiky red box around the Hit display and has a different sound associated with it.
1.  Display the appropriate number of Hits with the appropriate totals and adjust the hps appropriately.
  1. Certain skills have multiple hits of equal power (eg. Pierce and the monster skill Multi-Attack).
  1. Dual wielding gives 2 attacks, each with a different total.
  1. Double Attack adds 1 extra Hit with the total (rounded down) of the primary weapon.
  1. A Katar adds a second hit with damage equal to a fraction of the total and a second "impact" animation.
  1. A Critical Hit adds a spiky red box around the Hit display and has a different sound associated with it.
  1. When dual wielding or using a katar, a Critical Hit with the primary normal attack causes the secondary skill attack to appear to be a critical as well. It is not.
1.  If a single attack with more than one hit, display the total of all damage in the attack.
  1. The display is a number in yellow above the head of the Target. This number fades after a few seconds and is replaced by any other attack with multiple hits (including magic attacks).

Note: In the case of a Dual Wielding Assassin, steps 1-28 should be completed separately for each weapon.

*  The full damage calculation for each Hit by a Monster (melee or ranged) is:

 (([({rnd(MinATK,MaxATK) * SkillModifiers * (1 - DEF/100) - VitDEF - ProtectionSkills} + EnvenomSkill) * ElementalModifier * DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) / NumberOfMultiHits

*  The full damage calculation for each melee Hit by a Player is:

 (([((({(STR + [STR/10]^2 + [DEX/5] + [LUK/5] + UpgradeBonus + ImpositioManus + ATKCards + rnd(min(DEX,ATK), ATK)*SizeModifier) * SkillModifiers * (1 - DEF/100) - VitDEF + BaneSkill + UpgradeDamage} + MasterySkill + WeaponryResearchSkill + EnvenomSkill) * ElementalModifier) + Enhancements) * DamageBonusModifiers * DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) / NumberOfMultiHits

*  The full damage calculation for each ranged Hit by a Player is:

 ([({(DEX + [DEX/10]^2 + [STR/5] + [LUK/5] + UpgradeBonus + ImpositioManus + ATKCards + (rnd(ATK/100*min(ATK, dex*(0.8+0.2*WeaponLevel)), max(ATK, ATK/100*min(ATK, dex*(0.8+0.2*WeaponLevel)) + rnd(0, ArrowATK - 1)) * SizeModifier) * SkillModifiers * (1 - DEF/100) - VitDEF + BaneSkill + Upgrades} + EnvenomSkill) * ElementalModifier * DamageBonusModifiers * DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) / NumberOfMultiHits

*  The full damage calculation for each Hit by a Player (melee or ranged) against a "Herb Plant" is:

 1/NumberOfMultiHits

Note: Yes, these formula are hellishly complex. The game does not have one huge calculation like this. It uses an algorithmic process like the stepwise one I show above.


## Magical Attacks

Magical Attacks encompass all attacks using a Magic Attack Skill. Both Player Characters and Monsters can use Magic Attack Skills. As successfully cast Magic Attack Skills cannot be evaded, there is no need to check for a successful hit. However, there are many Magic Attack Skills with casting times and there are rules governing the casting process.

1.  Calculate the casting time.
  1. **ActualCastTime = BaseCastTime * EquipmentMods * max(1 - Dex/150,0)**.
  1. EquipmentMods to casting time come from Phen Card, Tower Keeper Card, etc.
1.  Is the casting time 0? If yes, go to step 5.
1.  Start casting.
  1. Casting circle/pentagram appears around target.
  1. Casting effect (coloured) appears around caster.
  1. Casting sound can be heard.
1.  Does the spell get interrupted before casting time is up? If yes, stop.
  1. If the caster takes at least 1 point of damage from a hit (not [poison](Status_Effects.md) or [bleeding](Status_Effects.md)), then the spell is interrupted.
  1. Most status effects will also interrupt a spell.
  1. Phen Card and Bloody Butterfly Card makes any spell cast uninterruptible, except for [Spell Breaker](Spell_Breaker.md)
1.  If a single target spell, is the target still in range? If no, stop.
  1. The target cannot be out of sight (including around a corner or cloaked/hidden).
1.  Show appropriate effect animations.
1.  If an area affect spell, are there any valid targets? If no, stop. If yes, show appropriate effect animations.
  1. Only those within the area of effect are considered valid targets.
  1. [Hiding](Hiding.md) avoids some numerous spells, both directly targeted and Area of Effect spells.
1.  Assess the result of a successful attack.

The result of successful attack is assessed according to the following process.

1.  Calculate the basic ATK value. The formula is rnd(minMATK,maxMATK).
1.  Apply Item Modifier.
  1. Most Regular Rod weapons give a 15% bonus to MATK. Therefore *1.15. Exceptions are Hypnotist's Staff, which gives %25 bonus, Bazerald, which gives 10% bonus and The Sign accessory, which gives 5%.
1.  Apply Skill Modifier.
  1. Varies according to the skill.
1.  Apply MDEF modifiers to the current total.
  1. Formula is **Total*(1-MDEF/100)**.
1.  Deduct Vit/2 and Int from the current total.
1.  Round the total down and if the current total is less than 1, then it becomes 1.
1.  Apply Elemental Modifier.
  1. The Elemental Modifier depends on the property of the magic attack skill and the property of the target.
1.  If attacking a "herb plant" and the total is greater than 1 then reduce damage to 1.
  1. Herb plants are Blue, Green, Red, Shining, Yellow and White Plants and Red and Black Mushrooms.
  1. The monster Antonio also acts like a "herb plant".
  1. Some monsters can also force this status onto themselves by performing Barrier.
1.  If the final damage is 0 or less, display a number of Misses equal to the number of "hits" in the attack and stop.
  1. The display for a Miss is the word "Miss" in red rising up and behind from above the attackers head.
1.  Display a number of Hits equal to the number of "hits" in the attack.
  1. The display for a Hit is a number equal to the final total in white rising up and behind from above the targets head. There is a sound associated with each magic attack skill.
1.  If more than one Hit, then display the total of all damage in the attack.
  1. The display is a number in yellow above the head of the Target. This number fades after a few seconds and is replaced by any other attack with multiple hits (including physical attacks).

*  The full damage calculation for each Hit in a magic attack is:

 {rnd(minMATK,maxMATK) * ItemModifier * SkillModifier * (1-MDEF/100) - INT - VIT/2} * Elemental Modifier


## Unknowns/Errors

Following is a list of things that I'm sure I don't know about perfectly yet. If you have accurate and tested information about this, please let me know.

*  Interaction of Safety Wall with KE (both monster and player versions of SW).
*  Do Safety Wall and Pneuma come before or after the skill check?
*  Interaction of Sidewinder card with all weapons, crap as it is.
*  Timing of Status Effect checks within the combat algorithms for both physical and magical attacks.
*  How SkillModifers stack together.
*  Exact position of Divine Protection in the scheme of things.
*  How to round things up and down in the right spots and get min 1 damage + 0.5 damage per double attack vs "herb plants".
*  How Hard DEF and Soft DEF degrade while being attacked by multiple enemies.

---
**Categories:** [Mechanics](categories/Mechanics.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Attacks) — GFDL 1.3+*
