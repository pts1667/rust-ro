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

pub fn sailor_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Sailor",
        args![
            "Hey everybody!",
            "Attention, attention!",
            "Come and ride the wind",
            "on a fascinating Ship!",
            "Hurry, hurry!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Byalan Island -> 150 Zeny.", "Alberta Marina  -> 500 Zeny.", "Cancel."])? {
        0 => {
            if ctx.player().zeny()? < 150 {
                ctx.lines_as("Sailor", args!["150 Zeny!", "Only 150 Zeny to ride!"])?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 150)?;
            ctx.warp("izlu2dun", 107, 50)?;
            return ctx.end();
        }
        1 => {
            if ctx.player().zeny()? < 500 {
                ctx.lines_as("Sailor", args!["500 Zeny!", "Only 500 Zeny to ride!"])?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
            ctx.warp("alberta", 188, 169)?;
            return ctx.end();
        }
        _ => {
            return ctx.close();
        }
    }
}

pub fn bonne_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as("Bonne", args!["Greetings!", "Izlude welcomes you."])?;
    ctx.next()?;
    ctx.lines_as(
        "Bonne",
        args!["Izlude is the satellite city of Prontera, capital of the Rune-Midgarts kingdom."],
    )?;
    ctx.next()?;
    ctx.lines_as("Bonne", args!["Izlude is key to our kingdom because of the Swordsman Association located here, as well as the fact that Izlude is in charge of protecting the Rune-Midgarts coastline."])?;
    ctx.next()?;
    ctx.lines_as("Bonne", args!["I know, this bridge might look weak and fragile, but it is actually state of the art, built with the most sophisticated technology."])?;
    ctx.next()?;
    ctx.lines_as(
        "Bonne",
        args!["No matter how strong storms may be, or how many people may stand on it, this bridge will NEEEEVER collapse."],
    )?;
    ctx.next()?;
    ctx.lines_as("Bonne", args!["Please enjoy", "your visit", "here in Izlude."])?;
    ctx.close()
}

pub fn charfri_izlude(ctx: &Ctx) -> Script {
    ctx.mes("[Charfri]")?;
    if ctx.call(Function::Rand, args![2])? == 1 {
        ctx.mes("Some people may think Izlude is just a satellite city of Prontera, and not really that important...")?;
        ctx.next()?;
        ctx.lines_as(
            "Charfri",
            args!["But Izlude is a beautiful town right next to the ocean, as well as beautiful Byalan Island."],
        )?;
        ctx.next()?;
        ctx.lines_as("Charfri", args!["You'll have to board on a ship at the port to get to Byalan Island. There are dangerous dungeons on that island, so don't go snooping around just anywhere."])?;
        return ctx.close();
    }
    ctx.mes("Though it is very beautiful, Byalan Island has a mysterious dungeon that extends deep under the sea.")?;
    ctx.next()?;
    ctx.lines_as(
        "Charfri",
        args!["People who've actually been there have said that if you go deep enough, the dungeon actually descends underwater."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charfri",
        args![
            "Surprisingly, once you're underwater, you can breathe just like a fish. Maybe some kind of supernatural force is in effect."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charfri",
        args![
            "Ah...",
            "The people who've seen the underwater view say it is so fantastic that they've kept dreaming of it ever since."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Charfri", args!["But unfortunately, the monsters are too strong for ordinary people to merely go sightseeing there. Still, just once, I'd like to go down there..."])?;
    ctx.close()
}

pub fn cuskoal_izlude(ctx: &Ctx) -> Script {
    ctx.mes("[Cuskoal]")?;
    if ctx.call(Function::Rand, args![2])? == 1 {
        ctx.mes("The Arena here is THE place for capable young people from all over the Rune-Midgarts Kingdom to challenge themselves and test their skills.")?;
        ctx.next()?;
        ctx.lines_as("Cuskoal", args!["You can battle with monsters of differing levels. So, the number of stages you survive will be a testament to your battle prowess."])?;
        ctx.next()?;
        ctx.lines_as("Cuskoal", args!["So, whaddya say?"])?;
        return ctx.close();
    }
    ctx.mes("The pubs in Prontera are always full of people from local areas and from out-of-town. It can get pretty busy.")?;
    ctx.next()?;
    ctx.lines_as(
        "Cuskoal",
        args!["It's a pretty good place to stop by for general information and to listen to rumors."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cuskoal",
        args!["So if you listen carefully, you just might get lucky and learn some very useful information for yourself."],
    )?;
    ctx.close()
}

pub fn dega_izlude(ctx: &Ctx) -> Script {
    ctx.mes("[Dega]")?;
    let roll = ctx.call(Function::Rand, args![3])?;
    if roll == 1 {
        ctx.lines(args!["Mt. Mjornir, located north of Prontera, is a tough", "and steep climb."])?;
        ctx.next()?;
        ctx.lines_as("Dega", args!["Aside from the dangers of the mountain itself, insanely vicious insects live there too. I mean, they'll just attack you for no reason."])?;
        ctx.next()?;
        ctx.lines_as("Dega", args!["If you ever want to visit somewhere past Mt. Mjolnir, then you prepare yourself for the challenge.  Or you could walk around it."])?;
        return ctx.close();
    }
    if roll == 2 {
        ctx.mes("Some monsters in the world have the unique ability to sense mystical energy, and can detect Magic spells before they are cast.")?;
        ctx.next()?;
        ctx.lines_as(
            "Dega",
            args!["Golem of the desert is one of them. Don't underestimate it due to its sluggishness..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dega",
            args!["If you try to cast magic near it, it will notice and saunter over to smash you. So you better watch out for Golem."],
        )?;
        return ctx.close();
    }
    ctx.lines(args![
        "There's a very delightful place where you can find every",
        "type of Poring."
    ])?;
    ctx.next()?;
    ctx.lines_as("Dega", args!["It's somewhere near the bridge connecting the forest and the desert, on the way to the city of Payon which is Southeast from here."])?;
    ctx.next()?;
    ctx.lines_as(
        "Dega",
        args!["There are not only pink Porings but also Drops, which can be found at the desert, and the green Poporing."],
    )?;
    ctx.next()?;
    ctx.lines_as("Dega", args!["But be careful, before you realize it, you may come face to face with Ghostring, a deadly Poring that floats around in the air like a ghost."])?;
    ctx.next()?;
    ctx.lines_as(
        "Dega",
        args!["Well, of course, they are all very cute, but Ghostring is an EXCEPTION. It is very very dangerous."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Dega",
        args!["If you are lucky enough, you might even bump into Angeling, the Poring with Angel wings."],
    )?;
    ctx.next()?;
    loop {
        match ctx.menu(&["Ghostring?", "Angeling?", "End Conversation."])? {
            0 => {
                ctx.lines_as("Dega", args!["Ghostring is a grayish Poring that floats around in the air like a ghost. Just like other ghosts, physical attacks can't do any damage to it."])?;
                ctx.next()?;
                ctx.lines_as("Dega", args!["Those whose main attack methods are physical like Swordman and Archer might have to run for their lives when facing Ghostrings."])?;
                ctx.next()?;
                ctx.lines_as("Dega", args!["But don't leave just yet~! There is great news for people with those jobs. Making a weapon of some elemental property is the key."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Dega",
                    args!["This way, even a Swordman or an Archer can inflict damage, the way Magic does, on Ghostrings."],
                )?;
                ctx.next()?;
            }
            1 => {
                ctx.lines_as("Dega", args!["Angelings are immune to Magic attacks. If people who can only attack with Magic face an Angeling, then it's time for", "them to run."])?;
                ctx.next()?;
                ctx.lines_as("Dega", args!["If you've got an extra knife or sword, you could give it a shot. But it will be very difficult alone, don't you think?"])?;
                ctx.next()?;
            }
            _ => {
                ctx.lines_as("Dega", args!["Good Luck~"])?;
                return ctx.close();
            }
        }
    }
}

pub fn kylick_izlude(ctx: &Ctx) -> Script {
    if ctx.call(Function::Rand, args![2])?.is_true() {
        ctx.lines_as(
            "Kylick",
            args!["I was thinking, even though the people of Izlude live so close to the ocean..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kylick",
            args!["There are other cultures that have completely developed by living off of the sea. Of course, I'm talking about Amatsu."],
        )?;
        ctx.next()?;
        ctx.lines_as("Kylick", args!["I hear the cuisine there is really good! Although the idea of eating raw fish is new to me, I would love to go there, and try it just once!"])?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Kylick",
            args![
                "Don't you think Binoculars",
                "are really COOL?! You can",
                "see all sorts of places...!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kylick", args!["Here in Izlude, we are responsible for maintaining peace not only on land but also at sea. That's why this city has a huge telescope."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kylick",
            args!["This telescope constantly watches over the sea, so that we can prevent any serious trouble from happening. You know..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kylick",
            args!["An ounce of", "prevention is worth", "a pound of cure", "after all, right?"],
        )?;
        return ctx.close();
    }
}

pub fn red_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as("Red", args!["The only skill that's needed for a Swordman is ^FF2400Bash^000000! Bash, Bash and ONLY ^FF2400Bash^000000! No need to waste time and effort for smaller skills! Everything else is for cowards and wusses!"])?;
    ctx.next()?;
    ctx.lines_as("Cebalis", args!["What are you talking about!? The ideal Swordman resolutely stands alone, surrounded by countless enemies and smashing them all with one awesome attack."])?;
    ctx.next()?;
    ctx.lines_as(
        "Cebalis",
        args![
            "^EE0000MAGNUM BREAK!^000000",
            "That's right, Magnum Break",
            "is the skill that does",
            "the job right~!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Cebalis", args!["Well... Sometimes the explosive damage might accidentally hit some wandering monsters, and those guys end up coming after you, but that's a risk a Swordman should be willing to take!!"])?;
    ctx.next()?;
    ctx.lines_as("Red", args!["That's exactly why you're dumb, you idiot! And what's this about the 'the ideal Swordman?' I still remember the last time you used Magnum Break..."])?;
    ctx.next()?;
    ctx.lines_as("Red", args!["You ended up running away from all those monsters you hit with that stupid skill! Weakling! All those Porings around you got hit and they all tried to kill you. "])?;
    ctx.next()?;
    ctx.lines_as("Cebalis", args!["Hmpf. As I recall, you were running away too, apparently too busy to use your precious Bash. In any case, Magnum Break is THE skill for a Swordman~!!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Cebalis",
        args![
            "Something simplistic like Bash",
            "is just one of the little steps towards Magnum Break."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Red",
        args!["Oh man~", "Hey, I know you just", "heard everything.", "So what do you think?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Red", args!["Which one do you think is better? The critical damage skill, ^FF2400Bash^000000, or the Splash damage skill, ^EE0000Magnum Break^000000?"])?;
    ctx.next()?;
    if ctx.menu(&["Bash", "Magnum Break"])? == 0 {
        if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN {
            ctx.lines_as("Red", args!["Hahahaha!!! I knew you'd see things my way!! You ARE a great guy!! Undoubtedly, only ^FF2400Bash^000000 suits a Swordman. Please tell that to this BONEHEAD over here~ Hahaha!"])?;
            ctx.next()?;
            ctx.lines_as("Red", args!["Hmm, let me give you a bit of advice. After you achieve level 5 'Bash', the amount of SP consumed by the skill increases greatly, so watch out for your SP."])?;
            return ctx.close();
        }
        ctx.lines_as("Red", args!["Hahahaha!! See!? Someone who pursues a different job agrees with me~! You really are a great guy! Hahaha!! Undoubtedly, only ^FF2400Bash^000000 suits a Swordman. Please tell that to this NIMROD over here~ Hahaha!"])?;
        return ctx.close();
    }
    if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN {
        ctx.lines_as(
            "Cebalis",
            args!["Alright!! ^EE0000Magnum Break^000000 is the BEST!! Now you're talking~!! You know the stuff~ HaHaHa!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cebalis",
            args!["You wanna know some useful information? Okay, okay lemme tell ya! Magnum Break has Fire Property."],
        )?;
        ctx.next()?;
        ctx.lines_as("Cebalis", args!["So it won't be too effective against monsters with the Water property, but this is THE skill to use against Undead and Earth property monsters!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Cebalis",
            args!["And most importantly, look around before you use it. Otherwise you'll be in BIG trouble~ "],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Cebalis", args!["Right?! ^EE0000Magnum Break^000000 is THE BEST!!! You know what you're talking about, eh? I don't know why this jerkface is being sooooo stubborn."])?;
    ctx.close()
}

pub fn cebalis_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as("Red", args!["The only skill that's needed for a Swordman is ^FF2400Bash^000000! Bash, Bash and ONLY ^FF2400Bash^000000! No need to waste time and effort on smaller skills~~ Everything else is for cowards and wusses!"])?;
    ctx.next()?;
    ctx.lines_as("Cebalis", args!["What are you talking about!? The ideal Swordman resolutely stands alone, surrounded by countless foes, smashing them all with one awesome attack..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Cebalis",
        args![
            "^EE0000MAGNUM BREAK!^000000",
            "That's right, Magnum Break",
            "is the perfect",
            "Swordman skill."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cebalis",
        args![
            "Well...",
            "Sometimes the explosion accidentally hits some monsters that are just wandering around."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cebalis",
        args!["Then they all end up coming after you, but that's a risk a true Swordman should be willing to take."],
    )?;
    ctx.next()?;
    ctx.lines_as("Red", args!["That's exactly why you're dumb, you idiot! And what was that about the 'ideal Swordman?' You remember the last time you used Magnum Break?!"])?;
    ctx.next()?;
    ctx.lines_as("Red", args!["You had to run away from all those Porings hit by that stupid skill! You weakling! All those Porings that you hit tried to kill you! "])?;
    ctx.next()?;
    ctx.lines_as(
        "Cebalis",
        args![
            "Oh shut up. And those were Poporings. As I recall, you were running away too, apparently too busy to use your precious Bash."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Cebalis", args!["In any case, Magnum Break is THE skill for a Swordman~!! Something simplistic like Bash is just one of those little steps towards Magnum Break."])?;
    ctx.next()?;
    ctx.lines_as(
        "Red",
        args!["Oh man~", "Hey, I know you", "heard everything.", "So what do you think?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Red", args!["Which one do you think is better? The critical damage skill, ^FF2400Bash^000000, or the Splash damage skill, ^EE0000Magnum Break^000000?"])?;
    ctx.next()?;
    if ctx.menu(&["Bash", "Magnum Break"])? == 0 {
        if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN {
            ctx.lines_as("Red", args!["Hahahaha!!! I knew you'd see things my way!! You ARE a great guy!! Without a doubt, only ^FF2400Bash^000000 suits a Swordman. Please tell that to FUNBOY over here!! Hahaha."])?;
            ctx.next()?;
            ctx.lines_as("Red", args!["Hmm, let me give you a bit of advice. After you achieve level 5 Bash, the amount of SP consumed by the skill increases greatly, so watch out for your SP."])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Red",
            args![
                "Hahahaha!!",
                "See!? Someone who pursues a different job agrees with me~! You really are a great guy! Hahaha!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Red",
            args!["Without a doubt, only ^FF2400Bash^000000 suits a Swordman. Please tell that to this MORON over here!! Hahaha~"],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseClass").get()? == constants::JOB_SWORDMAN {
        ctx.lines_as(
            "Cebalis",
            args![" Alright!! ^EE0000Magnum Break^000000 is the BEST!! Now you're talking~!! You know your stuff, kid. HaHaHa~!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cebalis",
            args![
                "You wanna know some useful information? Okay, okay lemme tell ya! The explosion from 'Magnum Break' has the Fire Property."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cebalis", args!["So it won't be very effective against Water property monsters, but this is THE skill to use against Undead and Earth property monsters.!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Cebalis",
            args!["And most importantly, look around before you use it. Otherwise you'll be in BIG trouble~ "],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Cebalis", args!["I'm right, aren't I?! ^EE0000Magnum Break^000000 is THE BEST!!! You know what you're talking about, eh? I don't know why this LARDFACE is sooooo stubborn."])?;
    ctx.close()
}

pub fn soldier_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Soldier",
        args!["HeHeHeHe..HaHaHaHa ", "Huh? Why am I so happy?", "You wanna know?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Sure, why?", "Not really, I don't care."])? == 0 {
        ctx.lines_as("Soldier", args!["Ah~~ There's not much for us to do these days. You see, Merchants buy items dropped by monsters. But you knew that, right? "])?;
        ctx.next()?;
        if ctx.menu(&["Of course", "Eh? Really?"])? == 0 {
            ctx.lines_as("Soldier", args!["HaHa~ In fact, that was actually part of our job. But there were more and more hunters who came to us in order to get paid and it became too much to handle."])?;
            ctx.next()?;
            ctx.lines_as("Soldier", args!["We had to work overtime every day. Ah, it was a nightmare...! Anyway, the government eventually made a wise decision in creating the Registration System."])?;
            ctx.next()?;
            ctx.lines_as("Soldier", args!["The Office of Prize Compensation only pays those who have the Registration. Of course, you'd have to be a merchant and stay in the same place all day long."])?;
            ctx.next()?;
            ctx.lines_as("Soldier", args!["The Office gives away the registration to any merchant who fulfills those requirements. So nowadays, the hunters sell their goods to the registered merchants."])?;
            ctx.next()?;
            ctx.lines_as("Soldier", args!["So nowadays, the hunters sell their goods to the registered merchants.  Not too many people come to us for that anymore."])?;
            ctx.next()?;
            ctx.lines_as("Soldier", args!["I mean we are still busy, but that's nothing compared to how it was before. People who have felt suffering know how to appreciate even the slightest comfort."])?;
            return ctx.close();
        }
        ctx.lines_as("Soldier", args!["What?! What do you mean you didn't know?! Well, you know you can get items by killing monsters. If you bring and sell those to a merchant, you can make some money. "])?;
        ctx.next()?;
        ctx.lines_as("Soldier", args!["HaHa, in fact, that used to be part of our job. But there were more and more hunters who come in order to get paid, so it became too much to handle."])?;
        ctx.next()?;
        ctx.lines_as("Soldier", args!["We had to work overtime every day. Ah, it was a nightmare...! Anyway, the government eventually made a wise decision in creating the Registration System."])?;
        ctx.next()?;
        ctx.lines_as("Soldier", args!["The Office of Prize Compensation only pays those who have the Registration. Of course, you'd have to be a merchant and stay in the same place all day long."])?;
        ctx.next()?;
        ctx.lines_as("Soldier", args!["The Office gives away the registration to any merchant who fulfills those requirements. So nowadays, the hunters sell their goods to the registered merchants."])?;
        ctx.next()?;
        ctx.lines_as(
            "Soldier",
            args![
                "So nowadays, the hunters sell their goods to the registered merchants.  Not too many people come to us for that anymore."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Soldier", args!["I mean we are still busy, but that's nothing compared to how it was before. People who have felt suffering know how to appreciate even the slightest comfort."])?;
        return ctx.close();
    }
    ctx.lines_as("Soldier", args!["Okay Good Bye~~"])?;
    ctx.close()
}

pub fn aaron_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as("Aaron", args!["Don't you think Strong VIT and training in a unique breathing method which enables quick HP recovery are the greatest advantages for a Swordman?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Aaron",
        args![
            "If you train your skills very hard, you can even see your HP recovering. The amount",
            "recovered depends",
            "vitality, or VIT."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Aaron",
        args!["So if you invest more of your stats in VIT, you'll recover more HP overall when resting."],
    )?;
    ctx.next()?;
    ctx.lines_as("Aaron", args!["But of course, it'd be good to have high Attack, wouldn't it? You can either acquire a good weapon or bring up your STR to support you Attack."])?;
    ctx.next()?;
    ctx.lines_as(
        "Aaron",
        args!["You know you'll need some strength anyway to swing good weapons easily, anyway."],
    )?;
    ctx.next()?;
    ctx.lines_as("Aaron", args!["Another important thing is how accurate you can hit your opponents. DEX is the key here. If you train DEX, then the gap between the MIN and MAX damage will also decrease."])?;
    ctx.next()?;
    ctx.lines_as(
        "Aaron",
        args!["Hm...", "Are you bored by all this talk? Or do you want me to go on?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Tell me more please.", "End conversation."])? == 0 {
        ctx.lines_as("Aaron", args!["Hmm...", "In that case, I'll explain about the other attributes to you briefly. In order to attack and evade quickly, you've gotta pay attention to AGI. "])?;
        ctx.next()?;
        ctx.lines_as("Aaron", args!["In case you want to make more critical hits, it's a good idea to invest in LUK. INT also increases Max SP, which is needed to use various skills... But it's really up to you."])?;
        return ctx.close();
    }
    ctx.lines_as("Aaron", args!["Okay then,", "train hard~~"])?;
    ctx.close()
}

pub fn sailor_2izlude(ctx: &Ctx) -> Script {
    ctx.lines_as("Sailor", args!["Wanna", "head back?"])?;
    ctx.next()?;
    if ctx.menu(&["Yeah, I'm tired to death.", "Nope, I love this place!"])? == 0 {
        ctx.warp("izlude", 176, 182)?;
        return ctx.end();
    }
    ctx.close()
}
