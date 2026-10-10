#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn forger_munster_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Munster", args!["My family used to live in Geffen. So I guess it was natural that we studied forging and eventually became Blacksmiths. Then, we finally moved to this town,", "Al De Baran."])?;
    ctx.next()?;
    if ctx.menu(&["About ^3355FFItem Upgrade^000000", "Quit"])? == 0 {
        ctx.lines_as(
            "Munster",
            args!["My father was a famous blacksmith in Geffen, and he taught me a lot about forging equipment."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munster",
            args!["One of the fundamentals is that the success probability of upgrading an item depends on the level of the weapon."],
        )?;
        ctx.next()?;
        ctx.lines_as("Munster", args!["For level 1 weapons, you may upgrade up to + 7 without the risk of breaking the weapon. Level 2 weapons can be upgraded to +6. Level 3 weapons can be upgraded to +5 safely."])?;
        ctx.next()?;
        ctx.lines_as("Munster", args!["For level 4 weapons, you can upgrade + 4 without too much risk. As for armors, you can upgrade them to +4. But if the upgrade for the equipment fails, it will be destroyed!"])?;
        return ctx.close();
    }
    ctx.lines_as("Munster", args!["Hmm...", "If you get a chance, try to visit my father's workshop here in Al de Baran. If I may say so, he's a pretty talented Blacksmith."])?;
    ctx.close()
}

pub fn smithing_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Quatro",
        args!["Have you heard that a famous Blacksmith moved here from Geffen?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Famous Blacksmith?", "End Conversation"])? == 0 {
        ctx.lines_as("Quatro", args!["From what I've heard, he's one of those Blacksmiths that can upgrade your weapons and armor. When you upgrade a weapon, its attack strength is increased."])?;
        ctx.next()?;
        ctx.lines_as("Quatro", args!["For each upgrade level, attack strength increases by 2 for level 1 weapons. On level 2 weapons, 3 attack strength is added."])?;
        ctx.next()?;
        ctx.lines_as("Quatro", args!["On level 3 weapons, 5 attack strength is added for each level, and for level 4 weapons, 7 attack strength is added for each level."])?;
        return ctx.close();
    }
    ctx.lines_as("Quatro", args!["This Blacksmith's family lives here, since his wife is sick and weak. Because of her condition, she needs to take medicinal herbs that grow near Al de Baran."])?;
    ctx.next()?;
    ctx.lines_as("Quatro", args!["They also have a dutiful son who's always helping out with the family business. I'm sure that kid will grow up to become a good Blacksmith like his father."])?;
    ctx.close()
}

pub fn young_man_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Miller",
        args!["Aren't level 4 weapons cool!", "I can't believe such powerful", "weapons exist!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Miller", args!["Well, they're rarely seen in the open market, but boss monsters will drop them by a low chance if you happen to be able to kill them."])?;
    ctx.close()
}

pub fn shell_gathering_lady_ald(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Joanne",
        args!["I enjoy gathering shells from the sea. It's really fun and relaxing~"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Shell Gathering?", "End Conversation"])? == 0 {
        ctx.lines_as("Joanne", args!["When you see bubbles popping up from the sand or muddy puddles, try digging into the ground a bit. You might find some shells underneath the ground!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Joanne",
            args!["Have you heard", "of Ambernite?", "That shell monster", "is pretty tough~"],
        )?;
        ctx.next()?;
        ctx.lines_as("Joanne", args!["It's usually seen at the beach near the west province of Prontera. If you ever try attacking it without being prepared, you might be in trouble."])?;
        ctx.next()?;
        ctx.lines_as("Joanne", args!["Ambernite is", "pretty strong!", "So look out for it!"])?;
        return ctx.close();
    }
    ctx.lines_as("Joanne", args!["Ambernite is", "pretty strong!", "So look out for it!"])?;
    ctx.close()
}

pub fn canal_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Panama", args!["Al De Baran is known world wide as the City of Canals. The waterways really add a sophisticated, romantic touch to our fair city."])?;
    ctx.next()?;
    match ctx.menu(&["About the Canals", "End Conversation"])? {
        0 => {
            ctx.lines_as(
                "Panama",
                args![
                    "Well, a canal is an artificial waterway used for travel,",
                    "shipping, or irrigation."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Panama", args!["However, the canals over here are just for show. If we needed to transport anything, we just use the Kafra Corporation Teleport service!"])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("Panama", args!["I have that you will enjoy your stay in Al De Baran."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn forest_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Isenberg",
        args![
            "Mt. Mjolnir and Payon Forest.",
            "Both of those places are tough",
            "to travel through."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Mt.Mjolnir?", "Payon Forest...?", "End Conversation"])? {
        0 => {
            ctx.lines_as(
                "Isenberg",
                args!["To arrive here from Prontera or Geffen, you've got to cross the Mjolnir Mountains."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Isenberg",
                args![
                    "If you've made it here by foot without using the Kafra Teleportation service,",
                    "then good job!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Isenberg", args!["The Mjolnir Mountains are really steep, and it's full of aggressive and hostile monsters. So it's always a risk to travel through there alone."])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Isenberg",
                args!["If you wish to visit Alberta or the city of Payon, you must first travel through the Payon Forest."],
            )?;
            ctx.next()?;
            ctx.lines_as("Isenberg", args!["The Payon Forest is a winding, intricate maze where it's easy to get lost. Unless you concentrate and keep track of your path, you might be stuck wandering in that dangerous place."])?;
            ctx.next()?;
            ctx.lines_as("Isenberg", args!["Payon, the Archer Village, was built deep inside this steep and rugged forest so that it may be protected from outside invaders. So I guess that a good decision on their part."])?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Isenberg",
                args![
                    "The huge mountains surrounding this town",
                    "blocks people from outside to come into this town.",
                    "That may be a part of the reason how we have been able to",
                    "keep this beautiful canal and mysterious alchemy",
                    "without any influence from outside."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn slot_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Epthiel",
        args!["Some weapons or armor have Slots where you can insert Cards obtained from monsters."],
    )?;
    ctx.next()?;
    match ctx.menu(&["About the number of Slots", "Relation between Cards and Slots", "End Conversation"])? {
        0 => {
            ctx.lines_as(
                "Epthiel",
                args!["Items dropped by monsters possess more Slots than ordinary weapons or armor sold in NPC shops."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Epthiel",
                args!["I guess you can assume that an item with more Slots is more valuable than the same item with fewer Slots."],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("Epthiel", args!["Once a Card is inserted into a Slot, it is impossible to remove it. So please be careful when you insert Cards into weapons or armor."])?;
            ctx.next()?;
            ctx.lines_as("Epthiel", args!["Also, when you mouse over equipment in the Item Window or Vending Window, the name of the item will be followed by the number of its Slots in brackets."])?;
            ctx.next()?;
            ctx.lines_as(
                "Epthiel",
                args!["For example, a Shield with 1 Slot, when moused over, would display the name 'Shield [1].'"],
            )?;
            ctx.next()?;
            ctx.lines_as("Epthiel", args!["You may also right-click an item, and check the Card Slot window below the item description window for the number of Slots."])?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as("Epithiel", args!["Have you ever obtained a card from a monster?"])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn phracon_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Joy",
        args!["Level 1 weapons, which are the lowest grade, need a metal named ^3355FFPhracon^000000 in order to be upgraded."],
    )?;
    ctx.next()?;
    match ctx.menu(&["About Phracon", "Advice about Phracon", "End Conversation"])? {
        0 => {
            ctx.lines_as(
                "Joy",
                args!["Phracon is a pretty common metal and can be found all over the Midgard continent."],
            )?;
            ctx.next()?;
            ctx.lines_as("Joy", args!["Although it lacks the strength of other metals, it's easy to find and obtain. You can get Phracons by killing monsters or by buying them in Forging Shops in towns."])?;
            ctx.next()?;
            ctx.lines_as(
                "Joy",
                args!["When you no longer need Phracons because you are using higher level weapons, you can sell them for some zeny!"],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Joy",
                args!["Well, I hear lots of monsters carry Phracons and will drop them once killed. Why don't you go hunting for them?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Joy", args!["It shouldn't be too difficult. Once I found a Phracon that dropped after killing a Bebe Savage! But if you're desperate, you can always buy them at the Forging Shop."])?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as("Joy", args!["Good luck with finding Phracons!"])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn alchemy_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Chemirre", args!["Alchemists, one of the 2nd Jobs, are able to create items out of several materials using knowledge from the ancient age of Al De Baran."])?;
    ctx.next()?;
    match ctx.menu(&["About Alchemy in Payon", "Definition of Alchemy", ". . . . .", "End Conversation"])? {
        0 => {
            ctx.lines_as(
                "Chemirre",
                args!["Most people don't know that there was an oriental form of Alchemy that developed in Payon."],
            )?;
            ctx.next()?;
            ctx.lines_as("Chemirre", args!["These Payon Alchemists were able to create Gold out of different materials. However, Payon Alchemy never advanced as much as the Alchemy in Al De Baran."])?;
            ctx.next()?;
            ctx.lines_as("Chemirre", args!["Materials for Alchemy in Payon were scarce and interest in that field eventually waned. Now, you can only study Alchemy here in Al De Baran."])?;
            ctx.next()?;
            ctx.lines_as("Chemirre", args!["Still, I can't help but wonder what secrets were lost after the Payon art of Alchemy disappeared from the face of the Earth..."])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Chemirre",
                args!["Alchemists specialize in chemical research in order to create useful items out of various things."],
            )?;
            ctx.next()?;
            ctx.lines_as("Chemirre", args!["I also hear that they create all sorts of Potions, and can even summon certain monsters! It seems that their studies have all sorts of nifty applications."])?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Chemirre",
                args![
                    "You are bored, aren't you?",
                    "Alright then, I will tell you a story about monster cards and item slots.",
                    "As you already know, if you ever have obtained a monster card before,"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Chemirre",
                args![
                    "you can only insert a monster card to an item",
                    "that satisfies the card's location requirement.",
                    "For instance, let's say, you have obtained a Poring Card."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Chemirre",
                args![
                    "When you right click on the card, you will see",
                    "its ability as LUK+2 and Perfect Dodge+1",
                    "and its location as 'Armor'. "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Chemirre",
                args![
                    "If you try to insert this card to a dagger with many slots,",
                    "it is not going to work because the card only can be inserted to",
                    "armor items."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Chemirre",
                args![
                    "Almost every armor items that are being sold",
                    "in town shops do not have slots on them.",
                    "That means, you can only obtain",
                    "slotted armors by hunting monsters."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Chemirre",
                args![
                    "Ah, let me tell you how you can insert a card to an item.",
                    "If you want to insert a card on your equipped armor,",
                    "you must unequip the armor first.",
                    "And then, double click a card that you want to use.",
                    "Then a list of armor, that you can insert the card, will be displayed."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Chemirre", args!["It is not that complicated, is it?"])?;
            return ctx.close();
        }
        3 => {
            ctx.lines_as(
                "Chemirre",
                args![
                    "You can talk about Rune-Midgarts' alchemy",
                    "without talking about the Al De Baran Alchemist Guild!",
                    "Long Live Alchemists!"
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn little_kid_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Bebe",
        args![
            "A while ago I went out for a walk toward Mt. Mjolnir with my pet Savage Bebe. His name is NukNuk!",
            "We got attacked, but luckily we weren't hurt."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Attacked?", "About Mt.Mjolnir", "End Conversation"])? {
        0 => {
            ctx.lines_as(
                "Bebe",
                args!["I was walking up a narrow path, and out of the blue, a giant and ugly plant started to attack me and NukNuk!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Bebe", args!["I was so surprised, so me and NukNuk had to run away. I threw rocks at it, but I don't think I hurt it. It must have been really strong!"])?;
            ctx.next()?;
            ctx.lines_as("Bebe", args!["What really surprised me was the plant that attacked me was a huge flower with the face of a person! So, look out for those. They're dangerous!"])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Bebe",
                args!["Even though people are fascinated by the scenic beauty of Mt. Mjolnir, it's full of dangerous monsters!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bebe",
                args!["There are Flowers, Insects, Bees, Butterflies and Moths that are big enough to kill you if you're not careful!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Bebe", args!["Then again, most of these monsters won't hurt you if you don't attack first. But some of them will attack you once they see you!"])?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as("Bebe", args!["By the way, where is my NukNuk...?", "NukNuk! Come out!"])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn insect_guy_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Stromme", args!["Even to a strong Swordman, the Insects or Mt. Mjolnir pose a considerable threat. You've got to know your enemy before engaging it in battle!"])?;
    ctx.next()?;
    if ctx.menu(&["About Insects", "End Conversation"])? == 0 {
        ctx.lines_as(
            "Stromme",
            args!["Honey Bees, Butterflies and Moths seem like simple creatures, but that doesn't mean you should underestimate them."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Stromme",
            args!["These Insects have evolved over time, and can counter attacks from threats like you adventurers!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Stromme",
            args!["There are also carnivorous Insects, such as praying Spiders, praying Mantises, and the millipede like Argiopes."],
        )?;
        ctx.next()?;
        ctx.lines_as("Stromme", args!["These monsters have mutated and are too strong for a person at certain levels. You should especially watch out for Argiopes."])?;
        ctx.next()?;
        ctx.lines_as(
            "Stromme",
            args!["Luckily, their eyesight is pretty bad, so it won't notice you if you walk a safe distance away from it."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Stromme",
        args![
            "No matter how harmless and pretty insects are,",
            "take heed to not touch them.",
            "They are extremely strong unlike their innocent looking.",
            "Don't belittle the livings in the Mt. Mjolnir."
        ],
    )?;
    ctx.close()
}

pub fn sylvia_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Sylvia",
        args!["I came all the way here from Prontera because I heard the Kafra Main Office was somewhere here in Al De Baran."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sylvia",
        args!["It shouldn't be that hard to find, but I'm awful at following directions. I always get lost, no matter how hard I try!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Sylvia", args!["If that wasn't bad enough, I left my Magnifiers back in Prontera, so now I have to find someone to help me with these weapons I've got to appraise!"])?;
    ctx.next()?;
    if ctx.menu(&["Appraise?", "That's very nice."])? == 0 {
        ctx.lines_as(
            "Sylvia",
            args!["Equipment that is dropped by monsters can't be equipped right away."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sylvia", args!["If you right-click the equippable item in the Item Inventory, you'll see that it is Unidentified and that Appraisal is needed. What to do?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Sylvia",
            args!["Well, in that case, you've gotta use ^3355FF Magnifier^000000!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Sylvia", args!["Even without a Blacksmith, Alchemist or Merchant in your party, you can appraise your equipment! Of course, a Magnifier is consumed each time you use one..."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Sylvia",
        args!["Hey...", "Was that a hint of sarcasm in your voice when you said that?"],
    )?;
    ctx.close()
}

pub fn issei_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Issei", args!["Al De Baran is such a wonderful place with its romantic canals and classic architecture. I love nothing more than to stroll through this city with my beautiful girlfriend."])?;
    ctx.next()?;
    if ctx.menu(&["You have a girlfriend?", "End Conversation."])? == 0 {
        ctx.lines_as("Issei", args!["Hey...", "Is that so hard to believe?! Yeah, ask anyone! She really exists! Although, sometimes, just sometimes mind you, she gets too excited about weapons and armor."])?;
        ctx.next()?;
        ctx.lines_as("Issei", args!["I mean, instead of enjoying a romantic dinner, she'll just go on about how equipment dropped from monsters is higher quality than those sold in shops..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Issei",
            args!["I mean, why should I care if equipment dropped by monsters tend to have more Slots?! I can't even kill a Poring!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Issei", args!["As you can see,", "I'm a lover,", " not a fighter."])?;
        return ctx.close();
    }
    ctx.lines_as("Issei", args!["So, you don't think of me stupid, do you?"])?;
    ctx.close()
}

pub fn joo_jahk_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Joo Jahk", args!["I'm a tourist", "from Payon,", "the City of Forests."])?;
    ctx.next()?;
    ctx.lines_as("Joo Jahk", args!["The tempature here in Al De Baran is very cool, probably because of the waterways. Do you think the water in the canals is drinkable?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Joo Jahk",
        args!["Well, it's too late for me, since I already drank some. Still, I'm a little worried..."],
    )?;
    ctx.next()?;
    if ctx.menu(&["Continue.", "End conversation."])? == 0 {
        ctx.lines_as("Joo Jahk", args!["On one of my travels around Midgard, I've heard from a really high level Mage that physical attacks, or magic with Neutral Property, won't damage Spiritual Property monsters."])?;
        ctx.next()?;
        ctx.lines_as("Joo Jahk", args!["Maybe that advice will come in handy, now that you know that. Always remember the importance of the Properties of your skills and weapons when battling monsters."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Joo Jahk",
        args!["On the other hand, the water I drank did taste pretty good. Hopefully it didn't have anything too weird in it..."],
    )?;
    ctx.close()
}

pub fn citizen_alde(ctx: &Ctx) -> Script {
    ctx.lines_as("Gavin", args!["Welcome!", "The town of", "Al De Baran", "welcomes you!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Gavin",
        args![
            "Well, that might be an exaggeration. After all, it's just me that's welcoming you.",
            "Hey there!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Now, tell me about monsters.", "End conversation."])? == 0 {
        ctx.lines_as(
            "Gavin",
            args![
                "Monsters...?",
                "Aren't we straying off topic a little bit? Ah, you must be one of those adventurers!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gavin",
            args!["Can't get your mind off the job, eh? Alright, now there was some monster that I saw just recently..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gavin",
            args![
                "Ah, now I remember! Just a few days ago, I saw a really interesting looking monster! It was a Poring with Angel's wings!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gavin",
            args![
                "I swear! He was jumping around somewhere near Mt. Mjolnir with some ordinary Porings. I think he was, like, their leader."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Gavin",
        args![
            "Awww...",
            "Don't be too disappointed that there's only one person in your welcome wagon!"
        ],
    )?;
    ctx.close()
}

pub fn town_girl_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Nastasia",
        args!["Somewhere in the world there is an ^3355FFAssassin Guild^000000, where they teach people the subtle art of assassination."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nastasia",
        args!["But isn't killing illegal? And do they even collect educational tuition?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Continue conversation.", "End Conversation."])? == 0 {
        ctx.lines_as(
            "Nastasia",
            args!["Although Assassins benefit from being very quick and having lots of AGI, they should still have some DEX."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nastasia",
            args![
                "DEX is especially important if you want to hit monsters with wings. Those monsters are quick moving and fast in attacking."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nastasia",
            args!["In general, if you want to hit monsters that are as fast, or even faster, than you are, you're going to need some DEX."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Nastasia",
        args!["It's usually said that in this world, nothing is free. Still, if you don't have to pay money to learn to be an Assassin..."],
    )?;
    ctx.close()
}

pub fn bell_keeper_a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Bell Keeper",
        args!["I have been charged by the Committee of 'Heaven on Earth' to guard this entrance of the Clock Tower."],
    )?;
    ctx.next()?;
    if ctx.menu(&["About Clock Tower.", "Quit."])? == 0 {
        ctx.lines_as(
            "Bell Keeper",
            args!["Every floor of this tower is connected to each other by a certain device we like to call 'Warp Gear.'"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bell Keeper",
            args!["Even though there are interconnecting warps everywhere in the Clock Tower, beware the 'Random Warp.'"],
        )?;
        ctx.next()?;
        ctx.lines_as("Bell Keeper", args!["The 'Random Warp' will transport you to an unknown spot. Be advised if you don't want to suddenly be separated from your party..."])?;
        ctx.next()?;
        ctx.lines_as("Bell Keeper", args!["Remember, Random Warps are shown in green on the mini-map. So keep your eyes peeled for that, as well as for those dangerous Clocks."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Bell Keeper",
        args!["Please take heed that this Clock Tower is filled with extremely dangerous monsters."],
    )?;
    ctx.close()
}

pub fn rs125_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "RS125",
        args![
            "I may sound inhuman rather robotic",
            "however, I hope you will not be afraid of me. I am as humane as you are."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "RS125",
        args![
            "I may have a machine heart and I may disturb you with loud noises from the heart,",
            "that will never stop me from running for future of Al De Baran."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Listen to his story.", "End Conversation"])? == 0 {
        ctx.lines_as(
            "RS125",
            args![
                "It's been 3 years already.",
                "My brother 996 used to be a short track athlete in the Al De Baran city field team.",
                "Back then, people gave him a nickname, 'Al De Baran's Peco Peco',",
                "for his amazingly fast legs..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "RS125",
            args![
                "He became so popular for his exciting play,",
                "so every time when the 'Al De Baran Turbo Track' was held once every 4 years,",
                "many people from all over the continent came to this city only to see my brother.",
                "I was his manager at the time and I was so stressed out because of his fans."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "RS125",
            args![
                "However, there is nothing last forever...",
                "One day, a girl from Payon beat my brother from a game."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "RS125",
            args![
                "My brother couldn't accept the fact that he lost the game",
                "so he did too much of practice and had a serious heart attack.",
                "He is still in bed."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "RS125",
            args![
                "I am my brother's only hope and the future of Al De Baran!",
                "Please wish me luck, I will beat her, 'Breezy Havana' from Payon!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "RS125",
        args![
            "I want to travel around the world one of these days.",
            "If I can see the ocean from the port of Alberta, it must be so wonderful.",
            "After the next year's athletic competition, I will go on a round-the-world tour with my brother."
        ],
    )?;
    ctx.close()
}

pub fn threatening_looking_man(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Threatening-Looking Man",
        args![
            "Hey, you don't come inside someone else's house without permission.",
            "This is ridiculous!",
            "How dare you to come inside of my house and talk to me as if that is a normal thing to do?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Threatening-Looking Man", args!["Hahahaha...chill out, I was just joking."])?;
    ctx.next()?;
    if ctx.menu(&["Continue", "Quit"])? == 0 {
        ctx.lines_as(
            "Threatening-Looking Man",
            args![
                "You may know this already, but",
                "we have a system called, the mercenary system in this world.",
                "Yes, I am a mercenary soldier."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Threatening-Looking Man",
            args![
                "It is simple. You just pay for someone to aid you in fight.",
                "Better mercenary soldier you want, more money you have to pay, you know?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Threatening-Looking Man",
            args![
                "Let's stop talking about boring stuffs.",
                "I will tell you how you can find a good mercenary soldier."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Threatening-Looking Man",
            args![
                "Check its nose if it is clean and wet.",
                "A good mercenary soldier must have the wet nose",
                "because it shows that the soldier is at his best in health condition.",
                "If the nose is dry, that means that he caught a cold."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Threatening-Looking Man",
            args![
                "And don't forget to check the soldier's ankle.",
                "The best mercenary soldier has thin ankles and a white neck!",
                "If he has long hair, it's better! If the hair is permed and wavy, that's perfect!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Threatening-Looking Man",
            args![
                "Lastly, you have to check whether he is ready to serve you with quality service!",
                "That means, he must do his best in aiding you in fight!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Threatening-Looking Man",
        args![
            "Get out, now!",
            "If you a cop, show me a warrant,",
            "if you are a member of my family, prove it with your birth mark!"
        ],
    )?;
    ctx.close()
}

pub fn friendly_looking_man_ald(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Friendly-Looking Man",
        args![
            "You don't have to listen to a guy right next to my room.",
            "Two years ago, he was in a mercenary training center and fell off from a tree",
            "while trying to gather a nut from it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Friendly-Looking Man",
        args!["He keeps talking to himself loud and it gives me a headache...", "Gosh!"],
    )?;
    ctx.close()
}

pub fn fussy_man_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Fussy Man",
        args![
            "Aaaaarrrggghhh...I AM IN TROUBLE!",
            "My little chicken has left me!",
            "Oh, my god! Oh, my god!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["What do you call the chicken?", ". . . . ."])? == 0 {
        ctx.lines_as(
            "Fussy Man",
            args![
                "I used to call it 'Amazing Picky'...",
                "*Sob* What should I do! How could this happen!",
                "Please, please help me to find my sweet little chicken!"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["What? That is such a boring name!", ". . . . ."])? == 0 {
            ctx.lines_as(
                "Fussy Man",
                args![
                    "Don't be so ridiculous!",
                    "'Amazing Picky' is the most wonderful and the most unique name",
                    "in this world, and my chicken deserves the name!"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Fussy Man",
            args![
                "You don't care, do you?",
                "I am only child in my family, so I have been thinking of my little chicken as my brother!",
                "I want my chicken back...*Sob*"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Fussy Man",
        args![
            "You don't care, do you?",
            "I am only child in my family, so I have been thinking of my little chicken as my brother!",
            "I want my chicken back...*Sob*"
        ],
    )?;
    ctx.close()
}

pub fn master_alde(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Master",
        args![
            "The Kafra Corporation Headquarters is located here in Al De Baran.",
            "Do you know",
            "what that means?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Master",
        args!["That means those cute Kafra Employees come here for their lunch breaks! Isn't that great?!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Master",
        args!["Alright, then!", "Pop Quiz Time!", "Who's your", "favorite Kafra girl?"],
    )?;
    ctx.next()?;
    if ctx.var("Sex").get()? == constants::SEX_FEMALE {
        ctx.lines_as(
            "Master",
            args!["Oh, and don't worry. I know that girls have some kind of opinion about how pretty other girls are."],
        )?;
        ctx.next()?;
    }
    if ctx.menu(&["Awesome!", "No way, I ain't a perv."])? == 0 {
        ctx.lines_as("Master", args!["Alright, here we go!", "Choose your favorite Kafra Lady!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Master",
            args!["The original Kafra Mascot, the classic blue haired lady! Candidate Number One: ^3355FFPavianne^000000!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Master", args!["Her graceful ponytail takes men's breath away! The fan favorite amongst teen males! Candidate Number Two: ^5533FFBlossom^000000!"])?;
        ctx.next()?;
        ctx.lines_as("Master", args!["Her long, straight hair, like silk from the East, is her charm point. Direct from Payon, it's Candidate Number Three: ^555555Jasmine^000000!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Master",
            args!["A tomboy with bright orange, shortly cut hair. Candidate Number Four: ^1133DDRoxie^000000!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Master",
            args![
                "Intelligent, sophisticated and never seen without her luxurious glasses. It's Candidate Number Five: ^33FF55Leilah^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Master",
            args![
                "Pretty, cute and fresh faced. Although She looks young and immature, she's the best staff!",
                "Candidate Number (6) ^AAAA00Curly Sue^000000 !!"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&[
            "(1) Pavianne",
            "(2) Blossom",
            "(3) Jasmine",
            "(4) Roxie",
            "(5) Leilah",
            "(6) Curly Sue",
        ])? {
            0 => {
                ctx.lines_as(
                    "Master",
                    args!["Oh~", "So you're a lover of classics. I respect that very much."],
                )?;
                ctx.next()?;
                ctx.lines_as("Master", args!["I'll also guess that you tend to enjoy the original movie more than sequels, and dislike bad imitations. Am I right?"])?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Master",
                    args![
                        "Hmmm...",
                        "Blossom strikes me as the girl-next-door type. So I guess that's the type of girl you're attracted to, eh?"
                    ],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Master",
                    args![
                        "So...",
                        "Long, luxurious hair is important to you, hmm? I suppose it such hair makes a woman look quite elegant."
                    ],
                )?;
                return ctx.close();
            }
            3 => {
                ctx.lines_as(
                    "Master",
                    args!["Ah, so you tend to like active, spontaneous types. I can understand that..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Master", args!["Since Roxie isn't exactly the demure housewife type, you probably have an open mind when it comes to defining femininity, right?"])?;
                return ctx.close();
            }
            4 => {
                ctx.lines_as(
                    "Master",
                    args!["Ah, so you like the intellectual type. That's good, that's good."],
                )?;
                ctx.next()?;
                ctx.lines_as("Master", args!["Still, that Leilah can be cold as stone sometimes. I've seen her shrug off many young men and crush even more hearts!"])?;
                return ctx.close();
            }
            5 => {
                ctx.lines_as("Master", args!["Say whaaat?!", "She's too young!"])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Master",
        args![
            "But I worked so hard on this delightful survey! Come now, be a sport! Admiring a pretty woman is like appreciating fine art."
        ],
    )?;
    ctx.close()
}

pub fn kafra_service_alde(ctx: &Ctx) -> Script {
    let mut l_kafrapassmoney = Val::from(0);
    ctx.fx().cutin("kafra_01", 2)?;
    ctx.lines_as("Kafra Pavianne", args!["Welcome! I'm Pavianne,", "one of the senior Kafra Employees. The Kafra Corporation Service is always trying to satisfy 100 % of our customers' expectations."])?;
    ctx.next()?;
    ctx.lines_as("Kafra Pavianne", args!["Due to a change in customer support policy, we no longer accept Kafra Passes. However, we are offering refunds for our customers who still possess these passes."])?;
    ctx.next()?;
    if ctx.menu(&["Sell Kafra Pass", "Alright, bye~"])? == 0 {
        if ctx.call(Function::CountItem, args![1084])? == 0 {
            ctx.lines_as("Kafra Pavianne", args!["I'm sorry,", "but you don't", "have any Kafra Passes."])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        } else {
            l_kafrapassmoney = (ctx.call(Function::CountItem, args![1084])?.try_mul(Val::from(2000))?);
            ctx.lines_as("Kafra Pavianne", args!["Let's see..."])?;
            if ctx.call(Function::CountItem, args![1084])? == 1 {
                ctx.lines(args![
                    "You have 1 Kafra Pass.",
                    "You can sell that pass to us for 2000 zeny. Would you like to sell this Kafra Pass back to the Kafra Corporation?"
                ])?;
            } else {
                ctx.lines(args![
                    ((Val::from("You have ") + ctx.call(Function::CountItem, args![1084])?) + Val::from(" Kafra Passes.")),
                    ((Val::from("If you want to sell them to us, you will receive ") + l_kafrapassmoney.clone())
                        + Val::from(" zeny. Would you like to sell these back to the Kafra Corporation?"))
                ])?;
            }
            ctx.next()?;
            if ctx.menu(&["Yes", "No"])? == 0 {
                if ctx.call(Function::CountItem, args![1084])? == 0 {
                    ctx.lines_as("Kafra Pavianne", args!["I'm sorry, but you don't have any Kafra Passes."])?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                }
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(1084), ctx.call(Function::CountItem, args![1084])?],
                )?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()? + l_kafrapassmoney.clone()))?;
                ctx.lines_as("Kafra Pavianne", args!["Thank you."])?;
            }
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
    }
    ctx.lines_as("Kafra Pavianne", args!["Thank you,", "have a good day."])?;
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn kafra_service_2alde(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    ctx.lines_as(
        "Kafra Blossom",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "The Kafra Employees are",
            "always here to serve you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Blossom",
        args!["We appreciate your continued use of the Kafra Service. Please feel free to ask me if you have any questions."],
    )?;
    ctx.next()?;
    match ctx.menu(&["How does Kafra Storage work?", "How do you teleport people?"])? {
        0 => {
            ctx.lines_as("Kafra Blossom", args!["Well, adventurers like yourself can place items into Kafra Storage, so that you don't have to carry all of your stuff around."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Blossom",
                args!["Now, the Kafra Storage Window is separated into three tabs into which items are automatically sorted."],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Blossom", args!["The ^3355FFItem^000000, ^3355FFEquip^000000, and ^3355FFEtc^000000 tabs work just like the tabs in your character Item Inventory."])?;
            ctx.next()?;
            ctx.lines_as("Kafra Blossom", args!["Multiple items of the same type will only take up one Slot in the Item and Etc. tabs. For example, 324 Jellopies would take up only one Slot, and 22 Red Potions would take another Slot."])?;
            ctx.next()?;
            ctx.lines_as("Kafra Blossom", args!["However, in the Equip tab, each and every single item takes up its own Slot. I guess that's because each and every single equipment can be uniquely upgraded by forging or through Cards."])?;
            ctx.next()?;
            ctx.lines_as("Kafra Blossom", args!["There's a total of 300 Slots for all three item categories in the Kafra Storage, so it might be helpful to remember that."])?;
            ctx.next()?;
        }
        1 => {
            ctx.lines_as(
                "Kafra Blossom",
                args!["Oh, I get that question all the time. '^CC0066Oh Blossom, how do you do it?^000000' Well..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Blossom", args!["Well, I couldn't really go too much into detail, of course. That's confidential information. But I can tell you our teleportation works through a mix of magic and technology."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Blossom",
                args!["Also, the Kafra girls alone can't teleport our customers. We just receive and process your teleportation request."],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Blossom", args!["Behind the scenes, skilled professionals and technicians are working 24 hours a day to ensure that you teleport quickly and safely to your destination."])?;
            ctx.next()?;
        }
        _ => {}
    }
    ctx.lines_as(
        "Kafra Blossom",
        args!["Anyway, I hope you enjoy your visit here in the Kafra Corporation Headquarters."],
    )?;
    if ctx.call(Function::Rand, args![1, 11])? == 9 {
        ctx.next()?;
        ctx.lines_as("Kafra Blossom", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Kafra Blossom", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Kafra Blossom", args!["Oh Mansoo..."])?;
    }
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn kafra_jasmine_alde(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_03", 2)?;
    ctx.lines_as(
        "Kafra Jasmine",
        args!["Welcome!", "The Kafra service is", "always on your side."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Jasmine",
        args!["Thank you for coming all the way to visit us at the Kafra Corporation Headquarters here in Al De Baran!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Kafra Jasmine", args!["The Kafra Service is always behind our customers with a dependable reputation that has been established over five thousand, eight hundred years..."])?;
    ctx.next()?;
    match ctx.menu(&["What?! I can't believe that!", "Ahh~ Shut Up!", "Your service is great!"])? {
        0 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, args![0])?,
                args!["What?!", "I can't", "believe that!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, args![0])?,
                args!["FIVE THOUSAND AND EIGHT HUNDRED YEARS?! THAT'S INSANE!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Jasmine", args!["Arrrrghh! Shut up and listen! It took me a week to memorize all this! My memory isn't as good as the other Kafra Employees...!"])?;
            ctx.next()?;
            ctx.lines_as("Kafra Jasmine", args!["Now, um...", "As I was saying, the Kafra Corporation was founded eight thousand, five hundred years ago by, um, Emilio Alexander Kafra... Inventor of the word 'Kafra?'"])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Jasmine",
                args!["He...", "He was a great man. He... Argh! I can't remember!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Jasmine",
                args![
                    "Oh no...!",
                    "This can't be the right story! Five million, eight hundred years?! It's impossible!"
                ],
            )?;
        }
        1 => {
            ctx.lines_as("Kafra Jasmine", args!["Listen...", "Punk."])?;
            ctx.next()?;
            ctx.lines_as("Kafra Jasmine", args!["I was a member of the Kafra Garrison before joining the Kafra Service Team. My specialty was ^990000Magnum Break^000000, so if you know what's good for you, don't mess with me."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Jasmine",
                args![
                    "I'm trying my best to live as quietly and as femininely as I can, so don't make me break your knuckles! You got it?!"
                ],
            )?;
        }
        2 => {
            ctx.lines_as("Kafra Jasmine", args!["Hooray!", "That's great news to hear. We're always working hard to make sure that our customers are satisfied with the services that we provide."])?;
        }
        _ => {}
    }
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn kafra_service_3alde(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    ctx.lines_as(
        "Kafra Roxie",
        args![
            "Welcome~!",
            "The Kafra Corporation will always support Midgard's adventurers with our excellent services."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Roxie",
        args![
            "My name is Roxie!",
            "I hope you enjoy",
            "your visit here in",
            "Kafra Corporation's",
            "Headquarters."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Roxie",
        args![
            "I'm here to answer any of your questions regarding Kafra Corporations policies, as well as take note of any of your feedback."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Kafra Policies", "I love Kafra!"])? {
        0 => {
            ctx.lines_as(
                "Kafra Roxie",
                args!["So, you'd like more details on our policies and eligibility for our services? What would you like me to explain?"],
            )?;
            ctx.next()?;
            match ctx.menu(&["Kafra Storage", "Cart Rental", "Actually, never mind."])? {
                0 => {
                    ctx.lines_as("Kafra Roxie", args!["As you probably already know, our customers must have at least Basic Skill level 6 in order to use the Kafra Storage."])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["As for the reason for this certain policy, we've had problems with young, fresh faced Novices that would put everything into their Storage."])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["Now you remember your days as a Novice. Everything was new and exciting, but zeny was scarce. Well, a lot of Novices would even put their weapons and armor in Kafra Storage."])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["However, by this time, they've already spent what little zeny they had to open their Storage. But they don't have enough to access their Storage again!"])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["So, these weaponless, armorless Novices must fight monsters with their bare hands until they gather the zeny to open their Kafra Storage again!"])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["It's a silly mistake, to be sure, but we here at Kafra Corporation value human life, and decided on the Basic Skill Level 6 Requirement to prevent this kind of mishap."])?;
                }
                1 => {
                    ctx.lines_as("Kafra Roxie", args!["As you may know, the Kafra Corporation has a special relationship with the Merchant Guild, as well as the Blacksmith and Alchemist guilds in Midgard."])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["The Kafra Corporation only rents Carts to Merchants, Blacksmiths and Alchemists since these job associations have a special contract with us."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Roxie",
                        args!["Also, it'd be really impractical to rent carts out to people who couldn't create or sell goods."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Roxie", args!["As for Super Novices, well, we're really not supposed to rent carts to them since the Super Novice Society in Al De Baran doesn't have a contract with us."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Roxie",
                        args!["If a Kafra Employee rented a Cart to a Super Novice, she'd probably get in big trouble with Leilah..."],
                    )?;
                }
                2 => {
                    ctx.lines_as(
                        "Kafra Roxie",
                        args!["Oh, alright~!", "If you have any questions,", "please let me know!"],
                    )?;
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as("Kafra Roxie", args!["Thank you!", "It's great to know that we're appreciated by our customers! All of us are working hard to make sure that our service meets your standards of excellence~"])?;
        }
        _ => {}
    }
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn kafra_service_4alde(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_06", 2)?;
    ctx.lines_as(
        "Kafra Curly Sue",
        args!["Hello, hello!!", "I'm Curly Sue,", "the newest member", "of the Kafra Staff!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Curly Sue",
        args!["I may still need to learn more about serving our customers, but I'm always doing my best!"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Where's your mommy, kid?", "End conversation."])? == 0 {
        ctx.lines_as("Kafra Curly Sue", args!["Waaaaaaah~!", "I'm not a kid!"])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Kafra Curly Sue",
        args!["Here at Kafra Corporation, we are all doing our very best to give you the excellent service that you expect from us."],
    )?;
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn kafra_employee_reserve1(ctx: &Ctx) -> Script {
    let mut l_choose_prize = Val::from(0);
    let mut l_choose_sub_select = Val::from(0);
    let mut l_index_points = Val::from(0);
    let mut l_index_quantity = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_points = Val::from(0);
    let mut l_random_while = Val::from(0);
    let mut l_s = Val::from(0);
    let mut l_select_price: Vec<Val> = Vec::new();
    let mut l_sound_word = Val::from(0);
    let mut l_total = Val::from(0);
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "^3355FFWait a minute! Right now,",
            "you're carrying too many items",
            "in your inventory. Please come",
            "back after storing some of",
            "your things in Kafra Storage."
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Kafra Employee",
        args![
            ((Val::from("Welcome, ") + ctx.call(Function::StrCharInfo, args![0])?) + Val::from("~")),
            "Here, you can exchange",
            "the Special Reserve Points",
            "you've earned by using the",
            "Kafra Services for some",
            "neat and useful prizes~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Please remember that each window has a different amount of special reserve points you can Use",
            "You can use ^7D0781from 100p to 3000p^000000 in here."
        ],
    )?;
    ctx.next()?;
    ctx.mes("[Kafra Employee]")?;
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 11000 {
        ctx.lines(args![
            "Um, but I don't think",
            "you're able to carry",
            "very much right now.",
            "It looks like you have",
            "too much stuff inside",
            "your inventory."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Please put some of",
                "your things into Kafra",
                "Storage. To use this",
                "service, we ask that you",
                "have about ^FF00001,100^000000 free units",
                "of weight in your inventory."
            ],
        )?;
        return ctx.close();
    }
    l_total = ctx.var("resrvpts").get()?;
    ctx.lines(args![
        "Let's see...",
        (ctx.call(Function::StrCharInfo, args![0])? + Val::from("...")),
        "Ah, you have a total of",
        (l_total.clone() + Val::from(" Special Reserve Points.")),
        "Now what you would like",
        "to exchange them for?"
    ])?;
    ctx.next()?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_select_price, &Val::from(base + 0), Val::from(516), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 1), Val::from(100), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 2), Val::from(7), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 3), Val::from(200), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 4), Val::from(15), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 5), Val::from(300), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 6), Val::from(25), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 7), Val::from(400), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 8), Val::from(35), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 9), Val::from(500), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 10), Val::from(50), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 11), Val::from(600), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 12), Val::from(60), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 13), Val::from(700), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 14), Val::from(75), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 15), Val::from(800), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 16), Val::from(85), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 17), Val::from(900), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 18), Val::from(100), false);
    runtime::local_set(&mut l_select_price, &Val::from(base + 19), Val::from(1000), false);
    l_s = Val::from(runtime::select_values(
        ctx,
        &[
            Val::from("100 = Potato 7 ea"),
            Val::from("200 = Potato 15 ea"),
            Val::from("300 = Potato 25 ea"),
            Val::from("400 = Potato 35 ea"),
            Val::from("500 = Potato 50 ea"),
            Val::from("600 = Potato 60 ea"),
            Val::from("700 = Potato 75 ea"),
            Val::from("800 = Potato 85 ea"),
            Val::from("900 = Potato 100 ea"),
            Val::from("1000 = 1st Lottery Chance!"),
            Val::from("Next Articles"),
            Val::from("Cancel"),
        ],
    )?);
    if l_s == 11 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_select_price, &Val::from(base + 0), Val::from(501), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 1), Val::from(1100), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 2), Val::from(7), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 3), Val::from(1300), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 4), Val::from(15), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 5), Val::from(1500), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 6), Val::from(25), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 7), Val::from(1700), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 8), Val::from(35), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 9), Val::from(1900), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 10), Val::from(50), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 11), Val::from(2100), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 12), Val::from(60), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 13), Val::from(2300), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 14), Val::from(75), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 15), Val::from(2500), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 16), Val::from(85), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 17), Val::from(2800), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 18), Val::from(100), false);
        runtime::local_set(&mut l_select_price, &Val::from(base + 19), Val::from(3000), false);
        l_s = Val::from(runtime::select_values(
            ctx,
            &[(Val::from(
                "1100 = Red Potion 7 ea:1300 = Red Potion 15 ea:1500 = Red Potion 25 ea:1700 = Red Potion 35 ea:1900 = Red Potion 50 ea:2100 = Red Potion 60 ea:",
            ) + Val::from(
                "2300 = Red Potion 75 ea:2500 = Red Potion 85 ea:2800 = Red Potion 100 ea:3000 = 2nd Lotery Chance!::Cancel",
            ))],
        )?);
        l_choose_sub_select = Val::from(1);
    }
    if l_s != 12 {
        ctx.mes("[Kafra Employee]")?;
        l_index_points = ((l_s.clone().try_mul(Val::from(2))?).try_sub(Val::from(1))?);
        l_index_quantity = (l_s.clone().try_mul(Val::from(2))?);
        if runtime::op(
            &l_total.clone(),
            "<",
            &runtime::local_get(&l_select_price, &l_index_points.clone(), false),
        )?
        .is_true()
        {
            l_points = (runtime::local_get(&l_select_price, &l_index_points.clone(), false).try_sub(l_total.clone())?);
            ctx.lines(args![
                "I'm sorry, but you don't",
                "enough Special Reserve",
                "Points to exchange for this",
                "reward. You need at least",
                ((Val::from("^0000FF") + l_points.clone()) + Val::from("^000000 more points."))
            ])?;
            return ctx.close();
        }
        l_total = (l_total
            .clone()
            .try_sub(runtime::local_get(&l_select_price, &l_index_points.clone(), false))?);
        ctx.lines(args![
            "After receiving this",
            "reward, you'll have",
            ((Val::from("^AC0000") + l_total.clone()) + Val::from("^000000 Special Reserve")),
            "Points left. Would you",
            "like to redeem your",
            "points for this reward?"
        ])?;
        ctx.next()?;
        if ctx.menu(&["Exchange.", "Cancel"])? == 0 {
            ctx.var("resrvpts").set(l_total.clone())?;
            if l_s.clone().number()? < 10 {
                ctx.call(
                    Function::GetItem,
                    vec![
                        runtime::local_get(&l_select_price, &Val::from(0), false),
                        runtime::local_get(&l_select_price, &l_index_quantity.clone(), false),
                    ],
                )?;
            } else {
                ctx.mes("[Kafra Employee]")?;
                if l_choose_sub_select == 0 {
                    ctx.lines(args![
                        "^0000FF1st Lottery Chance!!^000000",
                        "It's time to test out",
                        "your luck. Get ready!"
                    ])?;
                } else {
                    ctx.lines(args![
                        "Uh oh...",
                        "It's that time again~",
                        "It's Kafra Lottery Time!",
                        "Let's see how good your",
                        "luck is today. Ready?"
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "How many times",
                        "would you like to spin",
                        "the lottery machine?",
                        "You can spin it 1 to 5 times."
                    ],
                )?;
                ctx.next()?;
                'l1: loop {
                    if !(({
                        let (input, status) = runtime::input_number(ctx, Some(1), Some(5))?;
                        l_input = input;
                        Val::from(status)
                    }) != 0)
                    {
                        break 'l1;
                    }
                    'b1: {
                        ctx.lines_as(
                            "Kafra Employee",
                            args!["Excuse me...?", "Please choose", "a number from 1 to 5."],
                        )?;
                        ctx.next()?;
                    }
                }
                l_choose_prize = ctx.call(Function::Rand, args![1, 20])?;
                'l2: loop {
                    if !(!l_input.clone().loosely_equals(&l_random_while.clone())) {
                        break 'l2;
                    }
                    'b2: {
                        l_sound_word = ctx.call(Function::Rand, args![1, 3])?;
                        if l_sound_word == 1 {
                            ctx.lines(args!["^3355FFDrrrrrrrrrrrrrrrrrr...", "Tuum tuum tuum!^000000"])?;
                        } else if l_sound_word == 2 {
                            ctx.lines(args!["^3355FFChika chika chika", "Shooooooooooom~^000000"])?;
                        } else if l_sound_word == 3 {
                            ctx.lines(args!["^3355FFTuk tuk tuk tuk", "Flaaaaaavaaaaah~^000000"])?;
                        }
                        ctx.next()?;
                        l_random_while = (l_random_while.clone() + Val::from(1));
                    }
                }
                ctx.mes("[Kafra Employee]")?;
                if l_choose_sub_select == 0 {
                    ctx.lines(args![
                        "Ooh, something",
                        "came out! Let's see",
                        "what you've won~",
                        "Oh goodness, it's...!"
                    ])?;
                    ctx.next()?;
                    ctx.mes("[Kafra Employee]")?;
                    if l_choose_prize.clone().number()? <= 10 {
                        ctx.items().give(516, 100)?;
                        ctx.lines(args![
                            "Hm? F-fourth prize?",
                            "You got the 4th prize!!",
                            "Well, that's not too bad.",
                            "That's 100 Potatoes!",
                            "When they're sliced, then fried, they make a great snack when",
                            "drinking with your friends~"
                        ])?;
                    } else if l_choose_prize.clone().number()? <= 15 {
                        ctx.items().give(602, 4)?;
                        ctx.lines(args![
                            "It's Third Prize!",
                            "4 Butterfly Wings~",
                            "When you're in trouble,",
                            "just wave one of these",
                            "to take you away...",
                            "To your safe place."
                        ])?;
                    } else if l_choose_prize.clone().number()? <= 19 {
                        ctx.items().give(2403, 1)?;
                        ctx.lines(args![
                            "Second Prize!",
                            "A brand new shiny pair",
                            "of Shoes! Its elegant design",
                            "and durability comes with our",
                            "highest recommendation. We",
                            "hope you enjoy your new shoes~"
                        ])?;
                    } else if l_choose_prize == 20 {
                        ctx.items().give(2328, 1)?;
                        ctx.lines(args!["Whoa...!", "First Prize!", "Your very own"])?;
                        ctx.next()?;
                        ctx.lines(args!["set of Wooden Mail!", "Today must be your", "lucky day, adventurer!"])?;
                    }
                } else {
                    ctx.lines(args![
                        "It looks like",
                        "something came",
                        "out! What could it be?",
                        "Ooh, you just won..."
                    ])?;
                    ctx.next()?;
                    ctx.mes("[Kafra Employee]")?;
                    if l_choose_prize.clone().number()? <= 10 {
                        ctx.items().give(501, 100)?;
                        ctx.lines(args![
                            "F-fourth prize...?!",
                            "Boooo! 100 Red Potions.",
                            "Wait.. That's actually pretty",
                            "good! Yaaaaaay~ Now you",
                            "can look like a high roller by",
                            "sharing them with your friends!"
                        ])?;
                    } else if l_choose_prize.clone().number()? <= 16 {
                        ctx.items().give(2201, 1)?;
                        ctx.lines(args![
                            "Third Prize!",
                            "Your very own pair",
                            "of suave Sunglasses!",
                            "It'll give you an edge in",
                            "the war of looking cool,",
                            "or when playing poker~"
                        ])?;
                    } else if l_choose_prize.clone().number()? <= 19 {
                        ctx.items().give(2226, 1)?;
                        ctx.lines(args![
                            "Second Prize!",
                            "A... Cap? Hmmm,",
                            "these have pretty good",
                            "Defense, but I'm not so",
                            "sure of how fashionable",
                            "this hat is. Oh well..."
                        ])?;
                    } else if l_choose_prize == 20 {
                        ctx.items().give(505, 3)?;
                        ctx.lines(args![
                            "Oh wow...!",
                            "First Prize!",
                            "3 Blue Potions~",
                            "With enough of these,",
                            "you can use your skills",
                            "with a bit more impunity~"
                        ])?;
                    }
                }
            }
            ctx.next()?;
        }
    }
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Alright then. Please",
            "use our services to",
            "collect more and more",
            "Special Reserve Points",
            "for even better rewards.",
            "Thank you for your patronage."
        ],
    )?;
    ctx.close()
}

pub fn kafra_employee_reserve2(ctx: &Ctx) -> Script {
    let mut l_choose_prize = Val::from(0);
    let mut l_points: Vec<Val> = Vec::new();
    let mut l_s = Val::from(0);
    let mut l_sound_word = Val::from(0);
    ctx.lines_as(
        "Kafra Employee",
        args![
            ((Val::from("Welcome~ ") + ctx.call(Function::StrCharInfo, args![0])?) + Val::from(".")),
            "Currently, we, Kafra Center is having a special event for our customers."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "You can get free gifts by using special reserve points with ^FF0000Special Kafra ^529DFFGift Event!^000000",
            "Kafra Corporation added new gifts in this event."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Kafra Employee", args!["Do you want to use your points?"])?;
    ctx.next()?;
    if ctx.menu(&["Yes, I do", "Maybe in next time"])? == 0 {
        ctx.mes("[Kafra Employee]")?;
        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 11000 {
            ctx.lines(args![
                "....Oh dear... What are you carrying so many things...?",
                "I don't think you can keep the received items~"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "I'm sorry~ but~",
                    "Please, visit Kafra warehouse and store your items until you have free space of ^0000FF1100^000000 and come back.",
                    "I apologize for inconvenience~"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines(args![
            ((Val::from("Your special reserve points are ^FF0000") + ctx.var("resrvpts").get()?) + Val::from("^000000~")),
            "Choose a category to test your luck."
        ])?;
        ctx.next()?;
        let base = Val::from(1).number()?;
        runtime::local_set(&mut l_points, &Val::from(base + 0), Val::from(5000), false);
        runtime::local_set(&mut l_points, &Val::from(base + 1), Val::from(7000), false);
        runtime::local_set(&mut l_points, &Val::from(base + 2), Val::from(10000), false);
        l_s = Val::from(runtime::select_values(
            ctx,
            &[
                Val::from("5000p = 1st Lottery Chance!"),
                Val::from("7000p = 2nd Lottery Chance!"),
                Val::from("10000p = 3rd Lottery Chance!"),
                Val::from("Cancel"),
            ],
        )?);
        if l_s != 4 {
            ctx.mes("[Kafra Employee]")?;
            if runtime::op(
                &ctx.var("resrvpts").get()?,
                "<",
                &runtime::local_get(&l_points, &l_s.clone(), false),
            )?
            .is_true()
            {
                ctx.lines(args![
                    "I'm sorry~ dear~",
                    "You can't choose the selected chance because you do not have enough special reserve points.",
                    "Please check your special reserve points and choose another one~"
                ])?;
                return ctx.close();
            }
            ctx.var("resrvpts").set(
                (ctx.var("resrvpts")
                    .get()?
                    .try_sub(runtime::local_get(&l_points, &l_s.clone(), false))?),
            )?;
            ctx.lines(args![
                ((Val::from("^0000FF") + shared::other_global_functions::f_getnumsuffix(ctx, vec![l_s.clone()])?)
                    + Val::from(" Lottery Chance!!^000000"))
            ])?;
            ctx.next()?;
            ctx.lines_as("Kafra Employee", args!["It's time to try your luck."])?;
            ctx.next()?;
            ctx.lines_as("Kafra Employee", args!["Let's see how lucky you are. Now! Get ready!"])?;
            ctx.next()?;
            l_sound_word = ctx.call(Function::Rand, args![1, 3])?;
            if l_sound_word == 1 {
                ctx.mes("'Drrrrrr~ Drrrrrr~'")?;
            } else if l_sound_word == 2 {
                ctx.mes("'Rrrrrrr...'")?;
            } else if l_sound_word == 3 {
                ctx.mes("'Boing.. Boing.. Clink!'")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args!["Something has come out~ Let's see what you got~", "G~ U~ E~ S~ S~ W~ H~ A~ T~"],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Employee", args!["^FF0000Oh, my goodness! It's!!^000000"])?;
            ctx.next()?;
            ctx.mes("[Kafra Employee]")?;
            l_choose_prize = ctx.call(Function::Rand, args![1, 20])?;
            if l_s == 1 {
                if l_choose_prize.clone().number()? < 15 {
                    ctx.items().give(501, 150)?;
                    ctx.lines(args![
                        "What a pity!",
                        "You got the 4th prize!!",
                        "The prize is ^00FF00150 Red Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["Whoa~ 150 potions! It is enough to share with your friends~"],
                    )?;
                } else if l_choose_prize.clone().number()? < 18 {
                    ctx.items().give(645, 15)?;
                    ctx.lines(args![
                        "The 3rd~~",
                        "The 3rd prize!",
                        "The prize is ^00FF0015 Concentration Potion~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args![
                            "We always use this when we need to concentrate on something.",
                            "However, overdose is not good for your body~"
                        ],
                    )?;
                } else if l_choose_prize.clone().number()? < 20 {
                    ctx.items().give(505, 3)?;
                    ctx.lines(args![
                        "The 2nd~~",
                        "The 3rd prize~~",
                        "The prize is ^00FF003 Blue Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Employee", args!["Try these when your spiritual power is low~"])?;
                } else if l_choose_prize == 20 {
                    ctx.items().give(608, 1)?;
                    ctx.lines(args![
                        "Whoa~!! The first... The First!!!",
                        "Congratulations~~ You got the 1st prize~",
                        "The prize is ^00FF001 Yggdrasil Seed~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["I guess you spent entire luck for this lottery chance~"],
                    )?;
                }
            } else if l_s == 2 {
                if l_choose_prize.clone().number()? < 15 {
                    ctx.items().give(504, 10)?;
                    ctx.lines(args![
                        "What a pity!",
                        "You got the 4th prize!!",
                        "The prize is ^00FF0010 White Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["The greatest among potions! Use it before you fall into faint~"],
                    )?;
                } else if l_choose_prize.clone().number()? < 18 {
                    ctx.items().give(656, 15)?;
                    ctx.lines(args![
                        "The 3rd~~",
                        "The 3rd prize!",
                        "The prize is ^00FF0015 Awakening Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args![
                            "An awakening potion is better than a concentration potion!",
                            "Overdose is not good for your body~"
                        ],
                    )?;
                } else if l_choose_prize.clone().number()? < 20 {
                    ctx.items().give(657, 10)?;
                    ctx.lines(args![
                        "The 2nd~~",
                        "The 3rd prize~~",
                        "The prize is ^00FF0010 Berserk Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Employee", args!["Overdose may cause madness~"])?;
                } else if l_choose_prize == 20 {
                    ctx.items().give(608, 1)?;
                    ctx.items().give(607, 1)?;
                    ctx.lines(args![
                        "Whoa~!! The first... The First!!!",
                        "Congratulations~~ You got the 1st prize~",
                        "The prize is ^00FF001 Yggdrasilberry~^000000",
                        "The prize is ^00FF001 Yggdrasil Seed~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["I guess you spent entire luck for this lottery chance~"],
                    )?;
                }
            } else if l_s == 3 {
                if l_choose_prize.clone().number()? < 15 {
                    ctx.items().give(504, 30)?;
                    ctx.lines(args![
                        "What a pity!",
                        "You got the 4th prize!!",
                        "The prize is ^00FF0030 White Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["The greatest among potions! Use it before you fall into faint~"],
                    )?;
                } else if l_choose_prize.clone().number()? < 18 {
                    ctx.items().give(505, 10)?;
                    ctx.lines(args![
                        "The 3rd~~",
                        "The 3rd prize!",
                        "The prize is ^00FF0010 Blue Potions~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Employee", args!["Try these when your spiritual power is low~"])?;
                } else if l_choose_prize.clone().number()? < 20 {
                    ctx.items().give(608, 1)?;
                    ctx.items().give(526, 10)?;
                    ctx.lines(args![
                        "The 2nd~~",
                        "The 3rd prize~~",
                        "The prize is ^00FF001 Yggdrasil Seed~^000000",
                        "The prize is ^00FF0010 Royal Jellies~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Kafra Employee", args!["What a gift set~", "These are very healthy food so ~"])?;
                } else if l_choose_prize == 20 {
                    ctx.items().give(607, 3)?;
                    ctx.items().give(608, 2)?;
                    ctx.lines(args![
                        "Whoa~!! The first... The First!!!",
                        "Congratulations~~ You got the 1st prize~",
                        "The prize is ^00FF003 Yggdrasilberries~^000000",
                        "The prize is ^00FF002 Yggdrasil Seeds~^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["I guess you spent entire luck for this lottery chance~"],
                    )?;
                }
            }
            ctx.mes("Congratulations~~")?;
            return ctx.close();
        }
    }
    ctx.lines_as(
        "Kafra Employee",
        args![
            "No Problem~",
            "Collect more~ and more~ special reserve points~",
            "Thank you for using Kafra Corporation's services~~"
        ],
    )?;
    ctx.close()
}

pub fn gatekeeper_ct(ctx: &Ctx) -> Script {
    shared::cities_aldebaran::f_clocktowergate(ctx, args!["4th", 7026, "c_tower4", 185, 44])?;
    Ok(())
}

pub fn gatekeeper_ct1(ctx: &Ctx) -> Script {
    shared::cities_aldebaran::f_clocktowergate(ctx, args!["B4th", 7027, "alde_dun04", 79, 267])?;
    Ok(())
}
