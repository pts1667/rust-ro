# Restoration Items


## General

This pages covers healing, mana and restoration items. Healing items restore HP, mana items restore SP and restoration items restore both.


## Calculation

For mana items and the SP part of restoration items, replace [VIT](Stats.md) with [INT](Stats.md) and [Increase HP Recovery](Increase_HP_Recovery.md) with [Increase SP Recovery](Increase_SP_Recovery.md).

 Restored amount = floor(floor(**x** * (1 + **y** * 0.02 + **z** * 0.1)) * (1 + PR * 0.05))
 **x** is a random value from the used items base range
 **y** is the character's total VIT
 **z** is the character's level in Increase HP Recovery
 PR is the character's level in Potion Research

In English:

1.  Pick a random value from the item's base range
1.  Increase by 2% per point of [VIT](Stats.md) and 10% per level of [Increase HP Recovery](Increase_HP_Recovery.md). Round down.
1.  Increase by 5% per level of [Potion Research](Potion_Research.md). Round down.


## Aid Potion

[Aid Potion](Aid_Potion.md) is an [Alchemist](Alchemist.md) skill that lets the caster use potions on other characters. It only works with red, orange, yellow, white and blue potions. For blue potions, replace VIT/IHR as with regular mana items.

 Restored amount = floor(floor(floor(**x** * (1 + PR * 0.05 + AP * 0.1)) * (1 + **y** * 0.02)) * (1 + **z** * 0.1))
 **x** is a random value from the used items base range
 **y** is the character's total VIT
 **z** is the character's level in Increase HP Recovery
 PR is the caster's level in Potion Research
 AP is the caster's level in Aid Potion

In English:

1.  Pick a random value from the item's base range
1.  Increase by 5% per level of [Potion Research](Potion_Research.md) and 10% per level of [Aid Potion](Aid_Potion.md). Round down.
1.  Increase by 2% per point of [VIT](Stats.md). Round down.
1.  Increase by 10% per level of [Increase HP Recovery](Increase_HP_Recovery.md). Round down.


## List of items

Restoration values are base restoration unless otherwise noted. Where only 1 number is reported preceeded by a tilde (~), only one test was conducted due to item rarity. Where only 1 number is reported without a tilde, the item has a constant base restoration.


## Healing items

*  Aloe Leaflet (325~404 hp)
*  Apple (16~21 hp)
*  Apple Juice (25~34 hp)
*  Bag of Grain (61~70 hp)
*  Banana (17~20 hp)
*  Banana Juice (26~33 hp)
*  Bao (35~52 hp)
*  Bread (51~90 hp)
*  Candy (45~64 hp)
*  Candy Cane (105~144 hp)
*  Carrot (18~19 hp)
*  Carrot Juice (27~32 hp)
*  Condensed Red Potion (45~64 hp)
*  Condensed Yellow Potion (175~234 hp)
*  Condensed White Potion (325~404 hp)
*  Edible Mushroom (not much hp)
*  Fierce Cacao 99% (~630 hp)
*  Fresh Fish (102~151 hp)
*  Hearty Rice Cake (always restores 50% of HP)
*  Hinalle Leaflet (176~205 hp)
*  Ice Cream (112~142 hp, chance of freezing user)
*  Korean Rice Cake (always restores 10% of HP - note that it has 1 weight)
*  Lovely Choco-Tart (~360 hp)
*  Lucky Candy (45~64 hp)
*  Lucky Candy Cane (105~144 hp)
*  Lucky Cookie (160~199 hp)
*  Mastela Fruit (408~593 hp)
*  Meat (73~99 hp)
*  Milk (26~36 hp)
*  Monster's Feed (72~104 hp)
*  Novice Potion (22~32 hp)
*  Orange Potion (105~144 hp)
*  Pet Food (54~152 hp)
*  Piece of Cake (267~325 hp)
*  Potato (14~23 hp)
*  Pumpkin (19 hp)
*  Raw Fish (25~59 hp)
*  Red Herb (17~27 hp)
*  Red Potion (45~65 hp)
*  Rice Ball (200 hp)
*  Shrimp (119~193 hp)
*  Sushi (50~59 hp)
*  Well-baked Cookie (160~199 hp)
*  White Herb (71~114 hp)
*  White Potion (325~404 hp)
*  Yam (52~97 hp, small chance of stunning user)
*  Yellow Herb (38~57 hp)
*  Yellow Potion (175~234 hp)


## Mana items

*  Blue Herb (15~29 sp)
*  Blue Potion (46~60 sp)
*  Cheese (10~15 sp)
*  Cute Strawberry Choco (~100 sp)
*  Grape (10~15 sp)
*  Grape Juice (18~29 sp)
*  Lemon (10~19 sp)
*  Plain Rice Cake (always restores 20% of SP)
*  Strawberry (16~27 sp)


## Restoration items

*  Chocolate (1 sp, 1 hp)
*  Chocolate Drink (~50 sp, ~400 hp)
*  Green Ale (bottle) (always restores 50% of HP)
*  Green Ale (mug) (always restores 50% of HP and SP, chance of causing Chaos status on user)
*  Hand Made Chocolate (50 sp, 50 hp)
*  Hand Made White Chocolate (50 sp, 50 hp)
*  Honey (20~38 sp + 70~91 hp)
*  Orange (10~19 sp, small amount of hp)
*  Pancake (Caviar, Jam, Sour Cream, Mushroom, Honey - effects comparable to Royal Jelly)
*  Prickly Fruit (20-30 sp, 150-300 hp)
*  Pumpkin Pie (always restores 10% of HP and SP)
*  Red Prickly Fruit (20-30 sp, 270-330 hp)
*  Royal Jelly (40~59 sp + ~250 hp + cures all abnormal status ailments)
*  Salty Rice Cake (always restores 10% of HP and SP)
*  Tom Yum Goong (14~28 sp, 250-340 hp)
*  White Chocolate (50 sp, 50 hp)
*  Yggdrasilberry (always restores 100% of HP and SP)
*  Yggdrasil Seed (always restores 50% of HP and SP)


## External Links

*  [Healing item data](http://www.stud.ntnu.no/~magnusrk/shared/healing.html), including a tool to calculate the results for a given character.

---
**Categories:** [Items](categories/Items.md)
*Source: [iRO Wiki Classic](https://irowiki.org/classic/Restoration_Items) — GFDL 1.3+*
