# Soul Destroyer

<!-- Skill info -->
- **Class:** Assassin Cross
- **Type:** Offensive
- **Levels:** 10 (Selectable)
- **Prerequisites:** ([Thief](Thief.md)) [Double Attack](Double_Attack.md) Lv. 5, [Envenom](Envenom.md) Lv. 5; ([Assassin Cross](Assassin_Cross.md)) [Cloaking](Cloaking.md) Lv. 3, [Enchant Poison](Enchant_Poison.md) Lv. 6
- **SP cost:** Lv.1-5 20, Lv.6-10 30
- **Cast time:** 0.5 seconds
- **Delay:** (1 + 0.2*Skill Level) seconds
- **Target:** Enemy
- **Range:** 10 cells

A ranged skill that consists of two parts of damage, one based on [INT](Stats.md) and the other based on [ATK](Stats.md).

| Level | Cast Delay (sec) | SP Cost | Damage (ATK part) | Damage (INT part, add 500-1000 random damage) |
| --- | --- | --- | --- | --- |
| 1 | 1 | 20 | ATK*1 | INT*5 |
| 2 | 1.2 | 20 | ATK*2 | INT*10 |
| 3 | 1.4 | 20 | ATK*3 | INT*15 |
| 4 | 1.6 | 20 | ATK*4 | INT*20 |
| 5 | 1.8 | 20 | ATK*5 | INT*25 |
| 6 | 2 | 30 | ATK*6 | INT*30 |
| 7 | 2.2 | 30 | ATK*7 | INT*35 |
| 8 | 2.4 | 30 | ATK*8 | INT*40 |
| 9 | 2.6 | 30 | ATK*9 | INT*45 |
| 10 | 2.8 | 30 | ATK*10 | INT*50 |


## Formula

Damage on a "hit" = (INT*5*SkillLv + Random(500~1000)) + (((ATK*SkillLv)*((100-Enemy DEF)/100) - Enemy VIT) (Elemental Modifier)

Damage on a "miss" = INT*5*SkillLv + Random(500~1000)

**In plain text**: Soul Destroyer has 2 parts to it's damage which add to each other in the end. One part is based on your [INT](Stats.md) (read: not [MATK](Stats.md)), and the other is based on the ATK of your right hand weapon and ATK increasing cards (The left-hand weapon's ATK doesn't matter). Both are actually considered, by the game's coding, to be Ranged Physical attacks.

Special notes:
*  If dual wielding, then the cards of both weapons factor in, but if using a weapon and a shield/nothing in the left hand, then only the right hand's cards increases damage.
*  Due to the nature of Soul Destroyer not taking the ATK of the left hand weapon in the damage calculations, having a non carded/% card weapon on the left hand does absolutely nothing in increasing your damage.


## INT part

*  The base damage follows the formula: [(INT * 5 * Skill Level) + Random(500~1000)].
*  Never misses.
*  Is non-elemental. No, not Neutral, but "non-elemental" meaning it is 100% unaffected by element of the target. This means that it will do 100% to Ghost element.
*  Because of the way it is coded, Immune(Raydric Card) DOES affect this portion.
*  It is affected by Thara Frog Card, Horn Card and Poo.
*  Is not affected by [MDEF](Stats.md), Maya Card or Golden Thief Bug Card.
*  Is not affected by any +% Damage Cards such as Vadon, Hydra, Skel Worker, Turtle General, etc.
*  The INT damage varies a little every time due to the random extra damage it gives as a bonus (ranges from 500-1000 dmg)


## ATK part

*  The base damage follows the formula: [(Total ATK - Left Hand Weapon's ATK) * Skill Level]
*  Can miss, depending on your hit and your target's [FLEE](FLEE.md), just like a normal attack.
*  Takes on the elemental property of your weapon (if you are dual wielding it takes on the element of the Right Hand).
*  Is affected by target's [DEF](DEF.md) and [VIT](Stats.md).
*  Is affected by all cards affecting any normal attack's damage (Immune, Cranial, Horn, Poo)
*  Applies any defense-bypassing weapons such as Ice Pick or Combat Knife. Ice Pick's damage mods apply as well.
*  Is not affected by any +% Damage Cards such as Vadon, Hydra, Skel Worker, Turtle General, etc.
*  Note that +ATK from any weapon Cards do apply, even if other parts of the card do not. Skel Worker's +15% on Medium does not apply, but it's +5 ATK does.

Add those two up and you get the final damage.


## Notes

*  Being ranged, the skill is completely blocked by [Pneuma](Pneuma.md).
*  [EDP](Enchant_Deadly_Poison.md) does not affect Soul Destroyer's damage in any way.
*  +% Cards do not affect Soul Destroyer's damage in any way.
*  Status cards' effects do not carry.
*  [Ice Pick](Ice_Pick.md) and Combat Knife affects only the physical part.
*  Items that allow your attacks to bypass the target's defense, such as Combat Knife and Ceaser's Sword apply in either hand.
*  Zipper Bear's SP cost-per-attack does carry through to SD. One for each Zipper Bear Card.
*  You cannot use any other skills during the after-cast delay.
*  [Cast Time](Skills.md) can be reduced by [DEX](Stats.md), [Suffragium](Suffragium.md), various [Cards](Card_System.md) and [Magic Strings](Magic_Strings.md).
*  When dual wielding and casting Soul Destroyer, you are not penalized for the lack of [Righthand Mastery](Righthand_Mastery.md) or [Lefthand Mastery](Lefthand_Mastery.md) skills.
*  The Sign does not affect Soul Destroyer's damage.
*  When dual wielding, the atk value of your support weapon will not contribute to Soul Destroyer's damage. Only atk cards will have effect. (The ONLY ATK that does not apply is that of any left-hand weapon. Aegis NEVER uses left hand's weapon for any skill's damage calculation.)

**Related pages:**

* [Assassin](Assassin.md)
* [Assassin Cross](Assassin_Cross.md)
* [Skills](Skills.md)
* [Cloaking](Cloaking.md)
* [Enchant Poison](Enchant_Poison.md)
* [Grimtooth](Grimtooth.md)
* [Katar Mastery](Katar_Mastery.md)
* [Lefthand Mastery](Lefthand_Mastery.md)
* [Righthand Mastery](Righthand_Mastery.md)
* [Poison React](Poison_React.md)
* [Sonic Acceleration](Sonic_Acceleration.md)
* [Sonic Blow](Sonic_Blow.md)
* [Venom Dust](Venom_Dust.md)
* [Venom Knife](Venom_Knife.md)
* [Venom Splasher](Venom_Splasher.md)
* [Advanced Katar Mastery](Advanced_Katar_Mastery.md)
* [Create Deadly Poison](Create_Deadly_Poison.md)
* [Enchant Deadly Poison](Enchant_Deadly_Poison.md)
* [Hallucination Walk](Hallucination_Walk_(Unimplemented).md)
* [Meteor Assault](Meteor_Assault.md)
* [Soul Destroyer](Soul_Destroyer.md)
* [Quests](Quests.md)
* [Assassin Job Change Guide](Assassin_Job_Change_Guide.md)
* [Assassin Skill Quest](Assassin_Skill_Quest.md)
* [Rebirth Walkthrough](Rebirth_Walkthrough.md)
* [Weapons](Weapons.md)
* [Dagger](Dagger.md)
* [Katar](Katar.md)
* [One Handed Sword](One_Handed_Sword.md)

---
**Categories:** [Assassin Cross](categories/Assassin_Cross.md), [Offensive Skills](categories/Offensive_Skills.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Soul_Destroyer) — GFDL 1.3+*
