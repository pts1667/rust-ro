# Homunculus System


## What is a Homunculus

The Homunculus is a pet which assist players in ways such as a support, an instant partymate, or a sacrifice for the player. The player must be an Alchemist or a Biochemist and must go through the [Bioethics Quest](Alchemist_Skill_Quest.md) to unlock the appropriate skills required to create a Homunculus. After the quest [Bioethics](Bioethics.md) the skill will be available to the player and the rest of the Homunculus Skill Branch shall be available through the distribution of skill points.

There are 4 types of Homunculi with various attributes unique to each.

| Name | Type | Food | Skills |
| --- | --- | --- | --- |
| [Amistr](Amistr.md) | Tank<br>[Demi Human](Demi_Human.md) (iRO)<br>[Brute](Brute.md) (other RO)<br>[Neutral](Neutral.md) | Zargon | [Castling](Castling.md) [Amistr Bulwark](Amistr_Bulwark.md) [Adamantium Skin](Adamantium_Skin.md) [Blood Lust](Blood_Lust.md) |
| [Vanilmirth](Vanilmirth.md) | Magic<br>Demi Human (iRO)<br>[Formless](Formless.md) (other RO)<br>Neutral | Scell | [Caprice](Caprice.md) [Chaotic Blessings](Chaotic_Blessings.md) [Instruction Change](Instruction_Change.md) [Self-Destruction](Self-Destruction.md) |
| [Filir](Filir.md) | Dodge/High ASPD<br>Demi Human (iRO)<br>[Brute](Brute.md) (other RO)<br>Neutral | Garlet | [Moonlight](Moonlight.md) [Flitting](Flitting.md) [Accelerated Flight](Accelerated_Flight.md) [S.B.R.44](S.B.R.44.md) |
| [Lif](Lif.md) | Support<br>Demi Human<br>Neutral | Pet Food | [Healing Hands](Healing_Hands.md) [Urgent Escape](Urgent_Escape.md) [Brain Surgery](Brain_Surgery.md) [Mental Charge](Mental_Charge.md) |


## How do you get a Homunculus?

To call out a Homunculus, an Embryo is required. These can be purchased from other Alchemists or self-made. In order to make an Embryo, an Alchemist must use the Skill [Prepare Potion](Prepare_Potion.md) with the following materials purchasable from the [Al De Baran](Al_De_Baran.md) Alchemist guild [alde_alche: 35, 179]:

* Dew of Yggdrasil
* Seed of Life
* Glass Tube
* Medicine Bowl
* Potion Creation Guide (Not Consumed)

Success rates of creating embryos are affected mainly by the Alchemist's DEX and LUK.

Once an Embryo is obtained, use the skill [Call Homunculus](Call_Homunculus.md) to summon the Homunculus. In the Pre-Noghalt patch, there will be a probability of failures when summoning Homunculi. With the Noghalt patch, however, the probability of failure is replaced with unique sprites of the original Homunculi. There are 4 basic types of Homunculi —Amistr, Vanilmirth, Filir, and Lif— and the chance of getting each is random. Although not completely proven, it is a popular "fact" that the creator of the embryo's stats plays in a role in the probability of summoning each Homunculus.


## Homunculus Status and Skill Points

Homunculus have their own set of stats apart from their masters, these will have a set amount upon summoning and will vary as it level. With each level, a Homunculus will receive a certain amount points to each stat (STR, INT, DEX, AGI, VIT, LUK) depending on which type of Homunculus. These stats are invisible under the Homunculus stat window (Alt+R). The Homunculus's ATK, MATK, HIT, CRITICAL, DEF, MDEF, FLEE, and ASPD will be visible. To calculate the Homunculus's stats, use the following equations or use RODE: Homunculus Calculator. The Homunculus also gains one skill point for every three levels. Unlike Status points, one can apply these to whichever skill one would like.


## Calculations

ATK

* Min ATK = STR + [STR(Rounded down to the nearest multiple of 10) / 10]<sup>2</sup> + Dex
* Max ATK = STR + [STR(Rounded down to the nearest multiple of 10) / 10]<sup>2</sup> + Max(STR + Level, DEX)

MATK = INT + [INT(Rounded down to nearest multiple of 5)/5]<sup>2</sup>

HIT = Level + DEX

CRIT = [LUK/3] + 1

DEF

* Armor Def = [Level / 10] + [VIT / 5]
* VIT Def = VIT - 1

MDEF = [Level / 10] + [INT / 5]

FLEE = Level + AGI

ASPD = 200 - (Delay/10) + ( [4*Delay*AGI/1000] + [Delay*DEX/1000] ) / 10

Special note for Vanilmirth owners: Due to Instruction Change being bugged. (This has not been fixed on any Official RO server) The STR bonus from Instruction Change will be lost when one teleport/change maps/ or relog.


## Evolution

New Homunculus are born with 20 intimacy (Awkward).

After the implementation of Noghalt, Homunculi can evolve. Use a Stone of Sage (simply double click it) when the Homunculus's intimacy is Loyal to evolve it. With Evolution, the Homunculus will obtain random bonus of 1-10 for each of the stats, increased HP and SP, and a new sprite.

After evolving, the Homunculus' intimacy resets to 10 (Hate). Once its intimacy reaches 910 (1 point before Loyal intimacy) again, a fourth, new skill is unlocked.


## Controlling the Homunculus


## Manual Functions

Here is a listing of all manual functions with the default AI. In general, there will be no need for any other manual functions in most Custom AIs

* Alt + Single Right Click: Target monster
* Alt + Double Right Click: Attack monster
* Alt + T: Standby, idle/passive mode, cancels all commands and returns to master
* Alt + Right Click Ground: Move to location (15 tile range)
* Alt + R: Information window

If the player is dead, manual commands can't be issued. The AI will continue to function as normal however.


## AI

The AI is the brain behind the Homunculus. All Homunculus behavior is controlled by an AI script. There is a choice between the use of the default script or a custom-made script. The Default Script is both poorly made and simple. The Primary problems with the default script are that the Homunculus will either kill steal everything (default AIed Vanilmirth and Filir) or will not attack anything at all (default AIed Amistr and Lif). Hence it is recommended to make a custom AI or use a premade one made by users. To switch between the Default AI and the User AI, use the /hoai command. The current most popular AI available publicly is AzzyAI; which is linked at the very bottom of the page.


## Installing AI

Once AI script is downloaded, go to the default RO directory. This is generally located at C:\Program Files\Gravity\Ragnarok Online\AI. Insert the downloaded AI Script into the USER_AI folder and relog or resummon the Homunculus. This should be done every time there is a change in script. While there are infinite possibilities with AI scripts, it is necessary to understand that there are things it can and cannot do. These limitations are what separates it from bots.


## Possibilities

* Automatically attacking monsters within player's sight range
* Auto Casting Homunculus Skills
* Different tactics for individual monsters (ex: what priority to attack, using a skill on that monster etc)
* Reading the HP, SP, MAXHP and MAXSP values of the Homunculus or its owner
* Reading the target of a attacking monster (to avoid kill stealing)
* Automatically shut down the Ragnarok Online client using

```
os.exit()
```

  * If owner's alchemist is being attacked, it logs out after 5 seconds after calling os.exit()
  * Otherwise, if the alchemist is not attacking or being attacked, that character will log out immediately after os.exit() is called.


## Impossibilities

* Auto Feeding
* Auto Looting
* Using skills while their owner is [Vending](Vending.md)
* Homunculus maintain its AI when out of range of the Alchemist
* Having the Alchemist move around. (Except when auto casting the [Castling](Castling.md) Skill.)
* Reading the HP, SP, MAXHP and MAXSP values of someone other than the Homunculus or its owner
* Auto player skills (Disabled after 5-Feb-2008)
* Auto Casting Ground Spells (Disabled after 5-Feb-2008)
* Auto [Aid Potion](Aid_Potion.md) (Disabled after 5-Feb-2008)
* Auto [Cart Revolution](Cart_Revolution.md) (Disabled after 5-Feb-2008)
* Auto [Homunculus Resurrection](Homunculus_Resurrection.md) (Disabled after 5-Feb-2008)


## Not Allowed

* Using a 3rd party program.
* Reading in real time the output of the Homunculus traceai, so that one gains a read out on what is around the player (including things that the game client won`t show). According to a GM post on the iRO Forums "Doing this when monster leveling would not be such a big deal but in PVP/WoE purposes it is a very unfair/unintended thing. Users violating that rule will be suspended."
* Feeding the Homunculus or moving the owner around using a 3rd party program.
* Players are responsible for the actions of their Homunculus. For example, If the Homunculus [kill-steals](Kill_Steal.md) people, then the player is held accountable and may be punished accordingly.


## Feeding and Intimacy

Much like pets, Homunculus require feeding. Feed the Homunculus when its hunger is between 11~25 to get 1 full point of intimacy. Feeding at the other times will give less than a full point and may even reduce the intimacy. If one forgets to feed the Homunculus, its intimacy will decrease, and it will leave the master if it goes below zero intimacy.

| Hunger | Intimacy |
| --- | --- |
| Starving at 1%~10% | -1 (every 200 seconds) |
| 1%~10% | 0.5 |
| 11%~25% | 1 |
| 26%~75% | 0.75 |
| 76%~90% | -0.05 |
| 91%~100% | -0.5 |

The homunculus also display emotes as a way of warning the player. The emotes used are:

| Status | Emote |
| --- | --- |
| Hunger drops to 75% | /ok |
| Hunger drops to 25% | /hmm |
| Hunger below 11% every 20 seconds | /sob |
| Player feeds without proper item | /swt |
| Player feeds at 1%~75% | /ho |
| Player feeds at 76%~90% | /swt2 |
| Player feeds at 91%~100% | /wah |
| Failed to use skill | /... |

Intimacy is crucial for evolution, obtaining the final skill, and AFKing. Intimacy has not been proven to affect Homunculus stat growth. Intimacy points will not show in the Homunculus window. However for each interval of intimacy points the Homunculus has, the window will show:

| Intimacy | Status |
| --- | --- |
| 1-3 | Hate with Passion |
| 4-10 | Hate |
| 11-100 | Awkward |
| 101-250 | Shy |
| 251-750 | Neutral |
| 750-910 | Cordial |
| 911-1000 | Loyal |


## Manipulating Intimacy

The player dying, the Homunculus dying, or the Homunculus vaporizing does not affect intimacy. Stressing it (other than using ultimate skills) is the only way that to lower its intimacy. One can stress the Homunculus by not feeding it (hunger < 11) or overfeeding it (hunger > 75).

Each hour of starving cause 18 Intimacy loss (3600 seconds per hour / 200 seconds per loss).

If the player is away from the computer for a full 24 hours of starving, the Homunculus loses 432 Intimacy
18*24=432

If the player can no longer Call Homunculus or Homunculus Resurrection, then the Homunculus has left its master for good. However, Alchemists are able to create a new Homunculus.


## Experience & Leveling


## General Information

* When a [Monster](Monster.md) is killed by a homunculus alone:
  * Owner gets full [Base Experience](Base_EXP_Chart.md), but no [Job Experience](Job_EXP_Chart.md), from the monster.
  * Homunculus gets the full experience from the monster.
* If a player kills a monster and his/her homunculus doesn't contribute to the kill at all:
  * Player gets all Job Experience from the monster.
  * Homunculus get no Experience at all,
* If a monster is killed by having both the player and his/her homunculus dealing damage to the monster.
  * Player gets **125%** of the monster's Base Experience - For example, a Sidewinder gives 1,996 base experience. If both the player and the homunculus participate in the fight, the player gets 2,495 experience (1,996 * **1.25**) instead!
  * Player gets a portion of the monster's Job Experience based on how much damage is dealt by the player (the Job Experience pool is 125% of normal)
  * Homunculus gets a portion of the monster's EXP based on how much damage is dealt by the homunculus
* Whenever the player dies, if the homunculus is above 80% HP, it will automatically be [Vaporize](Vaporize.md)d. If it is below 80%, it will not vaporize and is able to continue to fight, and the player continues to receive EXP from Homunculus, but manual commands cannot be used.
* It's a common mis-conception that when a homunculus defeats a monster, they get the job experience of the monster. In actuality, they get a copy of the base exp of the monster, modified by the damage they did to it and how many people attacked it.

| Level | EXP | Total EXP | Level | EXP | Total EXP | Level | EXP | Total EXP | Level | EXP | Total EXP |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | - | - | 26 | 51,750 | 317,250 | 51 | 719,750 | 7,899,500 | 76 | 3,479,000 | 55,428,000 |
| 2 | 50 | 50 | 27 | 61,150 | 378,400 | 52 | 781,500 | 8,681,000 | 77 | 3,673,600 | 59,101,600 |
| 3 | 110 | 160 | 28 | 70,910 | 449,310 | 53 | 844,460 | 9,525,460 | 78 | 3,870,760 | 62,972,360 |
| 4 | 180 | 340 | 29 | 81,030 | 530,340 | 54 | 908,630 | 10,434,090 | 79 | 4,070,480 | 67,042,840 |
| 5 | 260 | 600 | 30 | 91,510 | 621,850 | 55 | 974,010 | 11,408,100 | 80 | 4,272,760 | 71,315,600 |
| 6 | 350 | 950 | 31 | 102,350 | 724,200 | 56 | 1,040,600 | 12,448,700 | 81 | 4,477,600 | 75,793,200 |
| 7 | 630 | 1,580 | 32 | 117,580 | 841,780 | 57 | 1,121,280 | 13,569,980 | 82 | 4,711,730 | 80,504,930 |
| 8 | 950 | 2,530 | 33 | 133,300 | 975,080 | 58 | 1,203,400 | 14,773,380 | 83 | 4,948,750 | 85,453,680 |
| 9 | 1,310 | 3,840 | 34 | 149,510 | 1,124,590 | 59 | 1,286,960 | 16,060,340 | 84 | 5,188,660 | 90,642,340 |
| 10 | 1,710 | 5,550 | 35 | 166,210 | 1,290,800 | 60 | 1,371,960 | 17,432,300 | 85 | 5,431,460 | 96,073,800 |
| 11 | 2,150 | 7,700 | 36 | 183,400 | 1,474,200 | 61 | 1,458,400 | 18,890,700 | 86 | 5,677,150 | 101,750,950 |
| 12 | 3,180 | 10,880 | 37 | 206,480 | 1,680,680 | 62 | 1,561,530 | 20,452,230 | 87 | 5,955,830 | 107,706,780 |
| 13 | 4,300 | 15,180 | 38 | 230,200 | 1,910,880 | 63 | 1,666,350 | 22,118,580 | 88 | 6,237,750 | 113,944,530 |
| 14 | 5,510 | 20,690 | 39 | 254,560 | 2,165,440 | 64 | 1,772,860 | 23,891,440 | 89 | 6,522,910 | 120,467,440 |
| 15 | 6,810 | 27,500 | 40 | 279,560 | 2,445,000 | 65 | 1,881,060 | 25,772,500 | 90 | 6,811,310 | 127,278,750 |
| 16 | 8,200 | 35,700 | 41 | 305,200 | 2,750,200 | 66 | 1,990,950 | 27,763,450 | 91 | 7,102,950 | 134,381,700 |
| 17 | 10,800 | 46,500 | 42 | 338,450 | 3,088,650 | 67 | 2,120,350 | 29,883,800 | 92 | 7,431,500 | 141,813,200 |
| 18 | 13,560 | 60,060 | 43 | 372,510 | 3,461,160 | 68 | 2,251,710 | 32,135,510 | 93 | 7,763,660 | 149,576,860 |
| 19 | 16,480 | 76,540 | 44 | 407,380 | 3,868,540 | 69 | 2,385,030 | 34,520,540 | 94 | 8,099,430 | 157,676,290 |
| 20 | 19,560 | 96,100 | 45 | 443,060 | 4,311,600 | 70 | 2,520,310 | 37,040,850 | 95 | 8,438,810 | 166,115,100 |
| 21 | 22,800 | 118,900 | 46 | 479,550 | 4,791,150 | 71 | 2,657,550 | 39,698,400 | 96 | 8,781,800 | 174,896,900 |
| 22 | 28,090 | 146,990 | 47 | 525,590 | 5,316,740 | 72 | 2,817,340 | 42,515,740 | 97 | 9,165,840 | 184,062,740 |
| 23 | 33,630 | 180,620 | 48 | 572,630 | 5,889,370 | 73 | 2,979,380 | 45,495,120 | 98 | 9,553,880 | 193,616,620 |
| 24 | 39,420 | 220,040 | 49 | 620,670 | 6,510,040 | 74 | 3,143,670 | 48,638,790 | 99 | 9,945,920 | 203,562,540 |
| 25 | 45,460 | 265,500 | 50 | 669,710 | 7,179,750 | 75 | 3,310,210 | 51,949,000 | - | - | - |


## Leveling Spots

* The following are some recommended spots for leveling your homunculus:
  * Levels 1-10:
    * Ant Hell 1 (Ant Eggs. Set up AI to have vani move around to kill)
  * Levels 10-20:
    * Culvert 2 (Thief Bugs, Familiar, Plankton, Spore, etc)
    * Byalan Dungeon 1 (Plankton, Marina, Vadon)
  * Levels 15-30:
    * Culvert 3 (Male Thief Bugs, Female Thief Bugs, etc)
    * Geffen Dungeon 1 (PoPoring, Poison Spore, etc)
    * Payon Forest 2 (Wolves, Spore, PoPoring, etc)
  * Levels 30-40:
    * Sunken Ship 1 (Pirate Skeleton, Kukre, Hydra, etc)
    * Orc Dungeon 1  (Drainliar, Orc Zombie, Orc Skeleton, etc)
  * Levels 40-50:
    * Orc Dungeon 1 (again)
    * Orc Dungeon 2 (Set up AI to kill everything EXCEPT Zenorc to help stay alive)
  * Levels +50:
    * Orc Dungeon 2 (Set up AI to kill all monsters on the map)

* ALSO: During Clock Tower spawn events when the map was reverted to the OLD Clock Tower spawns, it is possible to AFK level +90 homunculus in the "spawn room" located just North West of the large central room.


## FAQ for Homunculus

* They gain a skill point for every **three** levels. There is no Job Level equivalent.
* They obtain additional EXP from server rates, but NOT Battle Manuals.
* Their Flee and Defense is not reduced by the number of monsters.
* Their Hit is capped at 99%. Even if it has way more hit than required for a monster, it will always miss at a 1% chance.
* All Homunculus have a base attack speed of 1.4 seconds / attack (130). Different stat growths lead to different attack speeds of each Homunculus.
* Homunculus Stat Growth is RANDOM. It is possible see how well the Homunculus is doing on average at Doddler's Chart in the Useful Links section. However, nothing will affect what stats will grow and what stats will not.
* Homunculus can hit Ghost Property Monsters.
* It is possible to fail to create the embryo.


## Known Bugs

* Though the window shows additional Flee, [Filir](Filir.md) does not receive any from the [Accelerated Flight](Accelerated_Flight.md) skill.
* The Vanilmirth Skill [Instruction Change](Instruction_Change.md) is bugged and only adds the STR when it levels up. If the player leaves the map or teleports the additional STR will be lost.
* After [Amistr](Amistr.md)´s [Castling](Castling.md) skill, monsters do not always change target.
* If using a bugged AI Homunculus can kill themselves in PvP.
* If the Homunculus was Vaporized or killed while status effected, it will bug up. (It will get immobilized, vanish and it will get the grim reaper--like the one while affected by [Curse](Status_Effects.md)-- use [Vaporize](Vaporize.md) and [Call Homunculus](Call_Homunculus.md) to fix this bug).
* Homunculus can have permanent movement speed increase until one relogs, map changes, teleports, or Vaporizes. It is possible while using Call Homunculus and a [Movement Speed](Movement_Speed.md) increasing item, skill, or gear such as Speed Potion, [Increase AGI](Increase_AGI.md), or Bloody Axe at the same time.
* Both the Homunculus Hunger countdown clock (1 minute timer) and the starvation clock (20 seconds timer) will reset if one relogs, changes map, teleports, or uses [Vaporize](Vaporize.md) and [Call Homunculus](Call_Homunculus.md) together.
* The [Ninja](Ninja.md)'s [Watery Evasion](Watery_Evasion.md) causes permanent [AGI](Stats.md) reduction to each and every type of homunculus. Once its AGI has been reduced, it cannot be restored without the help of a Game Master. If a homunculus runs into this more than once, its possible for their AGI and [Flee](Stats.md) to be reduced below 0.
* On iRO, all Homunculus races have been changed to Demihuman for balancing reasons. However, the alternate form of the [Vanilmirth](Vanilmirth.md) is still considered Formless and not Demihuman. Therefore, all Demihuman reductions do not apply versus alternate [Vanilmirth](Vanilmirth.md)s. In addition, the Battlegrounds equipment set penalties (damage received is tripled versus non-Demihuman monsters) apply for the alternate [Vanilmirth](Vanilmirth.md) because it is not considered Demihuman.


## See Also

*  [AI](AI.md)
*  [Mercenary System](Mercenary_System.md)


## External Links

* [AzzyAI](http://forums.warpportal.com/index.php?/topic/144-azzy-ai-130b-new-forum-same-old-version/)
* [Mir AI](http://www.mirandablade.altervista.org/index.php?pg=mirai&lng=en) (Last updated: 2006)
* [Rampage AI](http://www.gorgerush.net/~faithful/RampageAI/download/) (No longer updated)
* [Doddler's Growth Comparison Charts](http://ro.doddlercon.com/homunstats/)
* [Max Period Homu Calculator](http://www.scs.ryerson.ca/~c5tam/homuncalc/)
* [Homunculus Stat Calculator (enjoy.ne.jp)](http://ww4.enjoy.ne.jp/~to4/hom/hom.html)
* [Detailed Stat Growth Chart](http://img.photobucket.com/albums/v189/romonk/Homun/HomunculusStatProjection.jpg)

**Related pages:**

* [Homunculus System](Homunculus_System.md)
* [Skills](Skills.md)
* [Amistr](Amistr.md)
* [Adamantium Skin](Adamantium_Skin.md)
* [Amistr Bulwark](Amistr_Bulwark.md)
* [Blood Lust](Blood_Lust.md)
* [Castling](Castling.md)
* [Filir](Filir.md)
* [Accelerated Flight](Accelerated_Flight.md)
* [Flitting](Flitting.md)
* [Moonlight](Moonlight.md)
* [S.B.R.44](S.B.R.44.md)
* [Lif](Lif.md)
* [Brain Surgery](Brain_Surgery.md)
* [Healing Hands](Healing_Hands.md)
* [Mental Charge](Mental_Charge.md)
* [Urgent Escape](Urgent_Escape.md)
* [Vanilmirth](Vanilmirth.md)
* [Caprice](Caprice.md)
* [Chaotic Blessings](Chaotic_Blessings.md)
* [Instruction Change](Instruction_Change.md)
* [Self-Destruction](Self-Destruction.md)
* [Alchemist](Alchemist.md)
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

**Related pages:**

* [Levels](Levels.md)
* [Experience](Experience.md)
* [Quest Table](Quest_Table.md)
* [Base EXP Chart](Base_EXP_Chart.md)
* [Job EXP Chart](Job_EXP_Chart.md)
* [Party](Party.md)
* [Guild System](Guild_System.md)
* [Homunculus System](Homunculus_System.md)
* [Leveling Spots](Leveling_Spots.md)
* [Mage Leveling](Mage_Leveling.md)
* [Crusader Leveling](Crusader_Leveling.md)
* [Rogue Leveling](Rogue_Leveling.md)
* [Stalker Leveling](Stalker_Leveling.md)
* [Wizard Leveling](Wizard_Leveling.md)

---
**Categories:** [Alchemist](categories/Alchemist.md), [Homunculus](categories/Homunculus.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Homunculus_System) — GFDL 1.3+*
