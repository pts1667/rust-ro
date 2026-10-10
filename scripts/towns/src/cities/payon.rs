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

pub fn lady_payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Lady",
        args![
            "A long time ago,",
            "when Payon was still",
            "developing, many of the",
            "villagers lived in poverty."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Lady", args!["Many families had to struggle to survive, and often could not even afford to properly bury their dead. Some people threw their dead into the cave near the village."])?;
    ctx.next()?;
    ctx.lines_as(
        "Lady",
        args![
            "So in that cave, it is said that there are many walking Zombies,",
            "the dead who cannot rest in peace and are unable to pass on to the next world."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Lady", args!["The Zombies, upon sensing the warmth of a human, begin to assault them, but that doesn't mean these Undead hold a grudge against", "the living."])?;
    ctx.next()?;
    ctx.lines_as("Lady", args!["Their rotten bodies can't leave the cold, dark and damp cave, so it's instinctual for them to attack warmth which would speed up the decomposition of their bodies."])?;
    ctx.next()?;
    ctx.lines_as(
        "Lady",
        args![
            "The Zombies in the Payon Cave",
            "may be spooky, but their story",
            "is also kind of tragic."
        ],
    )?;
    ctx.close()
}

pub fn young_man_payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Young Man",
        args![
            "From your attire,",
            "I can see that you",
            "are a stranger here.",
            "Welcome to Payon."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Young Man", args!["You must be a well-experienced fighter, otherwise you'd never be able to arrive here after passing the steep, mountainous areas and dangerous creatures surrounding this city."])?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args!["I'm no expert at fighting, but someone once told me that sheer strength alone won't be able to win some battles."],
    )?;
    ctx.next()?;
    ctx.lines_as("Young Man", args!["Sometimes, you may encounter creatures protected by a hard-shell that don't be damaged by physical attacks. Only psychic power, like Magic, can easily defeat such creatures."])?;
    ctx.next()?;
    ctx.lines_as("Young Man", args!["Of course, not everyone can study magic. The point is that you should keep different kinds of friends and comrades close to you, as you can't possibly handle every situation by yourself."])?;
    ctx.close()
}

pub fn young_man_2payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Young Man",
        args!["I remember the story my dearly departed grandfather has told me."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args![
            "It's about this Amulet that possesses an Evil Power.",
            "With it, you could awaken",
            "the Dead from the Grave."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args![
            "Well, I'm not sure if it's true or not. But, I wonder, what would happen if I used it to summon",
            "my grandfather from the other realm...."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("?", args!["^3299CCNever think", "of such a thing...", "My son.^000000"])?;
    ctx.next()?;
    ctx.lines_as("Young Man", args!["EEEEEEK-!", "What was that?!", "G-grandpa...?"])?;
    ctx.next()?;
    ctx.mes("...")?;
    ctx.next()?;
    ctx.lines(args!["...", "......", "[Young Man]", "...", "G-God...?"])?;
    ctx.close()
}

pub fn guardsman_payon(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn guardsman_payon_ontouch(ctx: &Ctx) -> Script {
    ctx.lines_as("Guardsman", args!["This is the Central Palace of Payon. This place is open to the public, but in accordance with our laws, you must behave in an orderly fashion once inside."])?;
    ctx.next()?;
    ctx.lines_as(
        "Guardsman",
        args![
            "In the interest of protecting the peace, we will disarm your equipment once you enter.",
            "Your cooperation is",
            "much appreciated."
        ],
    )?;
    ctx.call(Function::Nude, args![])?;
    ctx.close()
}

pub fn woman_payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Woman",
        args![
            "Welcome to Payon.",
            "You must have had",
            "a hard time getting",
            "through the Payon Forest.",
            "How was your trip?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Woman",
        args!["We've been receiving less tourists because of the increasing numbers of monsters outside, so it's quieter nowadays."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Woman",
        args![" To be honest, things are getting tough because of all those monsters. ^666666*Sigh...*^000000"],
    )?;
    ctx.next()?;
    ctx.mes("[Woman]")?;
    if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.mes("Whoa~! I just noticed those arms of yours look pretty solid. You look pretty strong, guy. Just how many monsters have you killed?!")?;
    } else {
        ctx.mes("Oooh! I didn't notice before, but you look pretty strong beneath all of that feminine charm.")?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Woman",
        args!["Hey, I know of a good place for you to hunt. It just so happens that there's a cave in the middle of Payon."],
    )?;
    ctx.next()?;
    ctx.lines_as("Woman", args!["If you're interested, just head North, pass the forest, and go towards the Northwest. You'll know you've arrived when you're in the place filled with the smell of stinky monsters."])?;
    ctx.next()?;
    match ctx.menu(&[
        "It sounds dangerous!",
        "I better prepare myself...!",
        "That's a nice dress you're wearing~",
    ])? {
        0 => {
            ctx.lines_as("Woman", args!["Oh come on, don't be a coward.", "It's just a simple cave filled with normal monsters. It's quite safe. We've even established an Archer Village near that cave to prevent misfortune incidents. Hohoohoho~ "])?;
        }
        1 => {
            ctx.lines_as("Woman", args!["Oh don't worry about any preparations. There's a Tool Dealer right in front of the cave, so you can purchase anything you need from my husband, er, that guy~"])?;
        }
        2 => {
            ctx.lines_as(
                "Woman",
                args![
                    "Oh hohohoho!",
                    "So you've noticed?",
                    "I hear this is the",
                    "latest trend in Prontera",
                    "these days."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Woman",
                args![
                    "Most of the women in this town don't know anything about fashion! My husband bought this for me as",
                    "a present. He makes quite a lot of money, you know. Hohohoho~"
                ],
            )?;
        }
        _ => {}
    }
    ctx.close()
}

pub fn woman_2payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Jim's Mother",
        args![
            "Oh boy~",
            "There she goes again.",
            "Without a doubt, that",
            "woman is the town gossip."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Jim's Mother", args!["Please don't judge the rest", "of the people living in Payon by her behavior. She's the only loudmouth. I guess she's just too excited about what the fortune teller told her."])?;
    ctx.next()?;
    ctx.mes("[Jim's Mother]")?;
    if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.lines(args![
            "Ooh...!",
            "You've got",
            "such broad shoulders!",
            "Will you go out with me?",
            "I'll treat you to",
            "a nice dinner~"
        ])?;
    } else {
        ctx.lines(args![
            "My, you're a pretty girl!",
            "I'm sure you're always busy",
            "beating the boys away with a stick...",
            "Or a well timed insult joke."
        ])?;
    }
    ctx.next()?;
    if ctx.menu(&["Fortune Teller...? ", "Well, see you later~"])? == 0 {
        ctx.lines_as("Jim's Mother", args!["Oh yes...", "There's an extraordinary fortune teller in the Central Palace of Payon. The more Zeny you pay her, the better fortune you'll get!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Jim's Mother",
            args!["She told me", "I would meet", "a nice guy this month.", "Hohohoho~ "],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Jim's Mother",
        args!["Mmmm...?", "You don't have", "any time to stay", "and chit-chat with me?"],
    )?;
    ctx.close()
}

pub fn drunkard_payon(ctx: &Ctx) -> Script {
    if ctx.var("Class").get()? != constants::JOB_ARCHER {
        ctx.lines_as("Drunkard", args!["Hey...", "H-Hey...!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Drunkard",
            args![
                "I wonder why those",
                "stupid Archers even",
                "bother trying to aim!",
                "You're all weak!",
                "Weeeeak!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Drunkard", args!["Bwahahahaha!", "Buy me a drink?!"])?;
    } else {
        ctx.lines_as("Drunkard", args!["An Archer!", "Oh man, you guys!", "You guys are the best!"])?;
        ctx.next()?;
        ctx.lines_as("Drunkard", args!["Bwahahahaha!", "Buy me a drink?!"])?;
    }
    ctx.next()?;
    match ctx.menu(&["Alright, but only one drink.", "No thanks, pal.", "Oh my God! Hell no!"])? {
        0 => {
            ctx.var("Zeny").set(if ctx.player().zeny()? < 100 {
                0
            } else {
                ctx.player().zeny()? - 100
            })?;
            ctx.lines_as("Drunkard", args!["Thanks...!", "..Brother!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Drunkard",
                args![
                    "Most people don't even wanna",
                    "buy me drinks! Maybe cuz I used to fool around too much with the ladies back in my day!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Drunkard", args!["Though, the women I used to play with are grannies now! Hahahaha! One of them still primps herself with makeup and stuff! Can you believe that?!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Drunkard",
                args![
                    "I'm like...",
                    "Come on...!",
                    "Some faces are",
                    "beyond fixing!",
                    "Oh? I made a funny!",
                    "Bwahahahahahah!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Drunkard",
                args![
                    "^666666*Gulp~ Gulp~*^000000",
                    "Man, this is great!",
                    "You the maaaaaaan~!",
                    "Muhahahahaha!"
                ],
            )?;
        }
        1 => {
            ctx.lines_as(
                "Drunkard",
                args![
                    "Bah!",
                    "Kids nowadays!",
                    "Now respect for",
                    "their elders! Fine!",
                    "I'm not gonna beg you!"
                ],
            )?;
        }
        2 => {
            ctx.lines_as("Drunkard", args!["Fine...!", "Fine by me!"])?;
        }
        _ => {}
    }
    ctx.close()
}

pub fn monster_scholar_02(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Monster Scholar Vuicokk",
        args![
            "Nice to meet you.",
            "I am called Vuicokk.",
            "I am a scholar in the Monster Research organization of the Rune-Midgarts Kingdom. Do you have any questions about monsters?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Any news?", "Undead Monsters?", "Monster Research Organization?"])? {
        0 => {
            ctx.lines_as("Monster Scholar Vuicokk", args!["Payon is located deep inside the forest where it can easily be attacked by hordes of monsters. Monsters also come from the dangerous cave located near town."])?;
            ctx.next()?;
            ctx.lines_as("Monster Scholar Vuicokk", args!["Since Undead monsters roam the Payon Cave, it has attracted the attention of the monster academic world. My job here is to analyze their characteristics."])?;
        }
        1 => {
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args!["What is most remarkable of the Undead monsters in Payon is their origin Most of them used to be citizens of Payon!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args![
                    "However, these souls are",
                    "unable to rest in peace and still wander about as Undead bound",
                    "to this world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Monster Scholar Vuicokk", args!["These monsters cannot be classified with other monsters that have mutated from living creatures, so our wise and benevolent ruler, King Tristram III, has taken a great interest in Payon's Undead."])?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args!["After all, some of these", "Undead used to belong to", "the Rune-Midgarts Kingdom."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args![
                    "As his subjects,",
                    "King Tristram III",
                    "feels some responsibility",
                    "to release their souls."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Monster Scholar Vuicokk", args!["His Majesty has been supporting", "us in our search to discover how to eliminate all of the Undead in this world. We will try to accomplish this goal as soon as we", "possibly can."])?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args![
                    "For the safety of our people,",
                    "for the sake of their bereaved families, and in accordance with King Tristram III's order, we must succeed!"
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args![
                    "As you may well know,",
                    "monsters have been endlessly spawning in this world, and the threat of their attacks is grows greater every day."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args!["In response to this,", " the Monster Research", "Organization has been formed."],
            )?;
            ctx.next()?;
            ctx.lines_as("Monster Scholar Vuicokk", args!["Talented people around the world have joined forces in an effort to deduce the origin of monsters, and a way to eliminate them once and for all."])?;
            ctx.next()?;
            ctx.lines_as(
                "Monster Scholar Vuicokk",
                args![
                    "Of course, it's not",
                    "as easy as you would may believe. Many have sacrificed their lives in the pursuit of this knowledge."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Monster Scholar Vuicokk", args!["In our missions, the number of victims of monster attacks have been countless. Still, nothing can stop us. If our suffering can save humanity, so be it!"])?;
            ctx.next()?;
            ctx.lines_as("Monster Scholar Vuicokk", args!["^666666*Ahem*^000000 My apologies, I get too excited sometimes. But if you happen to meet other scholars such as myself, please treat them well. Our jobs are very difficult!"])?;
        }
        _ => {}
    }
    ctx.close()
}

pub fn waitress_payon(ctx: &Ctx) -> Script {
    ctx.lines_as("Pub Lady", args!["This place is always bustling with busy people. Little Novices come and go to become Archers, and everyone else is buying arrows while I have to stay here in this small shop."])?;
    ctx.next()?;
    ctx.lines_as("Pub Lady", args!["And I'm sick and tired of making this noodle soup. I have to shower all the time so I can get rid of the smell. And it's not so easy", "to get rid of."])?;
    ctx.next()?;
    ctx.lines_as("Pub Lady", args!["I feel so...", "Bored.", "And lonely..."])?;
    ctx.next()?;
    ctx.mes("[Pub Lady]")?;
    if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.mes(
            "Where I can find the right person, a hot and Sexy hunk who can take me away from here? Um, hey mister, are you listening?",
        )?;
    } else {
        ctx.lines(args![
            "Where I can find the right person, a cute, yet hard-bodied hunk who can take me away from here?",
            "Um, hey lady, are",
            "you listening?"
        ])?;
    }
    ctx.next()?;
    ctx.lines_as("Pub Lady", args!["The old fortune teller told me that I'd have great luck in the near future! But what's wrong with me? I'm just living day to day. Maybe I'm just dumb and wishy-washy."])?;
    ctx.next()?;
    ctx.lines_as(
        "Pub Lady",
        args![
            "I'm so sorry,",
            "I've said too much.",
            "Now I'm just acting stupid.",
            "I'm sorry you had to listen",
            "to all that."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Pub Lady", args!["So...", "How may I help you? "])?;
    ctx.next()?;
    match ctx.menu(&["Have you ever heard of Zombies?", "Fortune Teller...?", "I needs some booze."])? {
        0 => {
            ctx.lines_as(
                "Pub Lady",
                args![
                    "Of course I've",
                    "heard of Zombies!",
                    "This is Payon, after all.",
                    "Zombies are the walking",
                    "Undead, and you can easily",
                    "find them around here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Lady",
                args!["I hear that they fear holiness, so Archers prefer to use arrow made out of silver, a holy metal, against them."],
            )?;
            ctx.next()?;
            ctx.lines_as("Pub Lady", args!["Legend says that the chief of this town used silver arrows against Zombies that used to be his brethren in order to release their souls so that they may rest in peace."])?;
            ctx.next()?;
            ctx.lines_as(
                "Pub Lady",
                args![
                    "We believe that exorcising",
                    "Zombies in this way will lead them peacefully to the afterlife. Their souls no longer need to anguish."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Pub Lady", args!["You might not share our beliefs, but my grandfather was one of the Undead. I appreciate that the chief was able to free him from being bound to the world of the living."])?;
        }
        1 => {
            ctx.lines_as(
                "Pub Lady",
                args![
                    "Oh! Our fortune teller is a really extraordinary person. Well, she doesn't hang around here as",
                    "much as she used to do. "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Pub Lady", args!["She used to stay here to tell fortunes for our patrons, but ever since the chief recognized her talents, she now stays in the Central Palace. So you'd better go there if you want to see her."])?;
        }
        2 => {
            ctx.lines_as("Pub Lady", args!["You...", "needs some", "booze, eh?", "Don't we all?"])?;
            ctx.next()?;
            ctx.lines_as("Pub Lady", args!["But I'm so sorry, we sold out.", "And we can't afford to prepare alcohol anymore because of the hostile creatures out there. But please come again later. I'm sorry for the inconvenience."])?;
        }
        _ => {}
    }
    ctx.next()?;
    ctx.lines_as("Pub Lady", args!["Have a nice", "day, dearie."])?;
    ctx.next()?;
    ctx.lines_as(
        "Pub Lady",
        args![
            "^666666*Sob*^000000",
            "When will I be romanced",
            "by my perfectly formed,",
            "yet well read man?"
        ],
    )?;
    ctx.close()
}

pub fn chief_guardsman_payon(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn chief_guardsman_payon_ontouch(ctx: &Ctx) -> Script {
    ctx.lines_as("Chief Guardsman", args!["What brings", "you here? "])?;
    ctx.next()?;
    ctx.lines_as("Chief Guardsman", args!["I can see that you are not one of the Payon locals. I would just like to remind you to conduct yourself in an orderly manner. Remember,", "you are a guest here."])?;
    ctx.next()?;
    ctx.lines_as(
        "Chief Guardsman",
        args!["In the interest of protecting the public peace, I will disarm your equipment. Thank you for your cooperation."],
    )?;
    ctx.call(Function::Nude, args![])?;
    ctx.close()
}

pub fn archer_zakk_payon(ctx: &Ctx) -> Script {
    ctx.lines_as("Archer Zakk", args!["I'm kind of worried", "about one of my pals."])?;
    ctx.next()?;
    ctx.lines_as(
        "Archer Zakk",
        args!["Even though he's an expert at archery, no one likes his motor mouth. Even our chief is getting fed up with him!"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Your friend?", "Payon has a chief?", " Motor... Mouth?"])? {
        0 => {
            ctx.lines_as(
                "Archer Zakk",
                args![
                    "Ah, right. This buddy of mine is the number one archer in Payon.",
                    "He teaches newbie Archers around the Archer Village. It might be a good idea to talk to him at least once."
                ],
            )?;
        }
        1 => {
            ctx.lines_as(
                "Archer Zakk",
                args!["Our chief lives in the Central Palace. I guess you can say that he's the spiritual guide of Payon."],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Zakk", args!["He used to menace the monsters in Payon Forest, carrying his Gakkung. I remember watching him fight when I was just a little kid."])?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Zakk",
                args![
                    "But now he",
                    "seems old and weak.",
                    "Still, his eyes are as sharp as they used to be during his days",
                    "of battle, where he'd never miss",
                    "a target."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Zakk", args!["I admire our chief", "from the bottom", "of my heart. "])?;
        }
        2 => {
            ctx.lines_as("Archer Zakk", args!["You don't know", "what a motormouth is...?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Zakk",
                args![
                    "Motormouth",
                    "Noun. Some fool who chatters",
                    "way too much about stuff that doesn't really matter and doesn't know when to stop."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Zakk",
                args![
                    "But yeah, my pal is not only",
                    "a legend at archery, he's also well known for how long he's let that mouth of his run."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Zakk", args!["Anyway, my pal Wolt doesn't have", "a place of his own, so he stays at the Inn. Why don't you go and meet him? He's actually an okay guy if you can stand all the chatter."])?;
        }
        _ => {}
    }
    ctx.close()
}

pub fn archer_wolt_payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Archer Wolt",
        args![
            "Archers should",
            "practice as much",
            "as they can. Otherwise,",
            "they'll never become experts."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Archer Wolt",
        args![
            "Oh, hey there!",
            "I'm Wolt the Archer,",
            "but, erm, you can",
            "just call me 'Wolt.'"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Archer Wolt",
        args![
            "I know this is a bit of an unexpected question, but do",
            "you tend to spend a lot of your",
            "time in idleness?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Oh, hell no.", "Yeah. I guess..."])? {
        0 => {
            ctx.lines_as("Archer Wolt", args!["Ooh. That's good. In fact, that's great! If only all of us Archers had that kind of attitude. If you have time to just sit around, then you have the time to go out and practice!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "Yeah...",
                    "As Archers, we kind of look",
                    "down on people who slack off",
                    "on the training."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "If you don't work hard, then you'll end up being a horrible Archer. No one can depend on your aim!",
                    "I mean, nobody!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "Did you ask how we go about",
                    "our training? Well, the Archers of Payon don't have much time to just play around with their Bows.",
                    "I guess we go out and",
                    "engage in actual fighting."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Wolt", args!["So we just carry out Bows wherever we go, and spend all day looking for monsters to kill. Oh, and after we find some monsters, we", "kill them of course. That almost goes without saying."])?;
            ctx.next()?;
            if ctx.menu(&["I guess you'd need a good Bow. ", "You call that 'practice?!'"])? == 0 {
                ctx.lines_as(
                    "Archer Wolt",
                    args!["That's right!", "An Archer depends", "on the strength", "of his Bow!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Archer Wolt", args!["Bows constructed in Payon", "are the greatest on this continent! They are light and stout, made out of high quality tree Trunks from Payon Forest!"])?;
                ctx.next()?;
                ctx.lines_as("Archer Wolt", args!["The quality of the wood often determines the quality of the Bow. My lil' sweety was made out of a walnut tree, and is strong enough to bear thousands of pounds", "of force."])?;
                ctx.next()?;
                ctx.lines_as("Arche Wolt", args!["Oh, oh! And my Bow's stylish as well. It comes in a sophisticated ebony color, and I just look so cool and heroic while I'm killing monsters~!"])?;
                ctx.next()?;
                ctx.lines_as("Archer Wolt", args!["Oh, and the trees in Payon Forest are famous for the quality of their wood. But then, more and more of them have been turning into monsters. Is this the work of evil forces?!"])?;
                ctx.next()?;
                ctx.lines_as("Archer Wolt", args!["It's a pity because the tree monsters used to be beautiful, majestic trees. But then it's okay if we kill them, so that we can make Bows out of their wood."])?;
                ctx.next()?;
                ctx.lines_as("Archer Wolt", args!["And then we use these Bows to kill even more tree monsters... And then make more wood! Mwahahahaha! It's an endless cycle!"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("^666666*Ahem!*^000000 Speaking of endless...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Archer Wolt",
                    args!["Yeah...", "You're right.", "Monsters are everywhere,", "can you believe it?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archer Wolt",
                    args!["Do you understand", "why the Archer Village", "was built where it is?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Archer Wolt", args!["To the West, near Archer Village, you'll see Payon Cave. Inside the cave, an enormous amount of monster endlessly spawn without showing any sign of slowing down."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Archer Wolt",
                    args!["We Archers are here to protect our territory against them, as ordered by our honorable chief!"],
                )?;
                ctx.next()?;
                match ctx.menu(&["Cave, you say?", "Chief...? ", "Oh man, I hate this town!"])? {
                    0 => {
                        ctx.lines_as("Archer Wolt", args!["If you head North", "of town, you'll find", "Payon Cave."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archer Wolt",
                            args![
                                "Once inside, you'll encounter all sorts lots of ugly monsters. Like those nasty looking Bats, and",
                                "those Zombies..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Archer Wolt", args!["If we let them out of that place, they'd run all over Payon! So, we gotta get in there and clean that place up of monsters."])?;
                        ctx.next()?;
                        ctx.lines_as("Archer Wolt", args!["But since these monsters endlessly respawn, sometimes I feel like we're wasting our time and energy for nothing..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archer Wolt",
                            args![
                                "Whoa...",
                                "Alright, I think yet my mouth run",
                                "a marathon again. I better let you go. There's lots of training to do, and plenty of monsters to kill!"
                            ],
                        )?;
                    }
                    1 => {
                        ctx.lines_as(
                            "Archer Wolt",
                            args!["Our chief? Ah, he's such a swell guy. Then again, he's always scolds me for talking too much."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Archer Wolt", args!["He always working to make sure", "that Payon is safe from harm. Lately, I've been worried since he hasn't been coming out the Palace lately. That might be a bad sign."])?;
                        ctx.next()?;
                        ctx.lines_as("Archer Wolt", args!["A bad sign of what, I'm not really sure. There could be problems with Payon, or maybe he's got something he needs to handle on his own. Anyway, there's always guards in his room for some reason."])?;
                        ctx.next()?;
                        ctx.lines_as("Archer Wolt", args!["Heh heh heh~", "I'll get in trouble if he knows", "I told an outsider know too much about his affairs. Oh well! It's too late. I already told you! There's no use regretting it!"])?;
                        ctx.next()?;
                        ctx.lines_as("Archer Wolt", args!["But...", "It's not too late for me to just shut my mouth. Keep it closed. Shutting up now. Quiet as a mouse. Seeya later~!"])?;
                    }
                    _ => {}
                }
                return ctx.close();
            }
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "Hmm...?",
                    "And you don't?",
                    "What better practice",
                    "than the real, honest",
                    "to goodness thing?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "Though, you may have a point",
                    "there. I mean, you should be able to practice without having to suffer serious consequences.",
                    "You know, like death."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "But we've gotta make do",
                    "with what we've got!",
                    "An Archer's life is",
                    "endless training!",
                    "Endless practice!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "Hm...?",
                    "Are you tired of me repeating the same thing over and over again?",
                    "Oh, just bear with me. Think of it as practicing your patience~"
                ],
            )?;
        }
        1 => {
            ctx.lines_as(
                "Archer Wolt",
                args!["Eh...", "I guess it's important", "to set aside time to rest."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args!["But it's also a good idea to set aside time for practicing and training, and practicing and training!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "And it's a very bad idea to rest when you should be alert, or well, when you're supposed to be doing something else."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "That reminds me of the time",
                    "I set fire to my house during the holidays. It was an accident, of course, but boy, were my folks angry!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Wolt", args!["Hahahaha~", "It all started when I decided to take a bath. Here, in Payon, we use the old style baths, so we need to make a fire to heat the water. Cold baths are just so uncomfortable."])?;
            ctx.next()?;
            ctx.lines_as("Archer Wolt", args!["But nice, warm baths are veeeery comfortable. That was probably the best bath I had in my life! It was so comfortable, I fell asleep."])?;
            ctx.next()?;
            ctx.lines_as("Archer Wolt", args!["But while I was sleeping,", "I guess I didn't notice the fire reach the floor, walls and ceiling! Luckily, I was in a tub full of water, so I was okay."])?;
            ctx.next()?;
            ctx.lines_as("Archer Wolt", args!["I would've gotten help if it weren't for the fact that the fire had burned my clothes while I was napping. So, of course, I couldn't just run around town in the nude."])?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Wolt",
                args![
                    "So...",
                    "I just sat in the water for about an hour, completely naked, and yelling 'Help me,' until someone could hear me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Wolt", args!["But, it turned out that no one could hear me. Luckily, our roof caved in and totally demolished our walls so that my screams could finally reach passerby. And it was in that way that I survived."])?;
            ctx.next()?;
            match ctx.menu(&[
                "Um, what's the point of that story?",
                "I guess I better not get lazy then. ",
                "Blah blah blah. See ya. ",
            ])? {
                0 => {
                    ctx.lines_as(
                        "Archer Wolt",
                        args!["Oh...", "Umm...", "You mean, like,", "the moral of the story?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Archer Wolt", args!["Resting during the right times is perfectly fine. Just don't go and take a break when you shouldn't. That kind of behavior would make anyone in our town angry!"])?;
                    ctx.next()?;
                    ctx.lines_as("Archer Wolt", args!["Our chief is especially annoyed by that kind of irresponsible attitude. I guess, around here, only the town drunkard subscribes to that kind of policy."])?;
                    ctx.next()?;
                    match ctx.menu(&["Chief?", "Town Drunkard? ", "Man, you talk too much!"])? {
                        0 => {
                            ctx.lines_as(
                                "Archer Wolt",
                                args!["Our chief? Ah, he's such a swell guy. Then again, he's always scolds me for talking too much."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Archer Wolt", args!["He always working to make sure", "that Payon is safe from harm. Lately, I've been worried since he hasn't been coming out the Palace lately. That might be a bad sign..."])?;
                            ctx.next()?;
                            ctx.lines_as("Archer Wolt", args!["A bad sign of what, I'm not really sure. There could be problems with Payon, or maybe he's got something he needs to handle on his own. Anyway, there's always guards in his room for some reason."])?;
                            ctx.next()?;
                            ctx.lines_as("Archer Wolt", args!["Heh heh heh~", "I'll get in trouble if he knows", "I told an outsider know too much about his affairs. Oh well! It's too late. I already told you! There's no use regretting it!"])?;
                            ctx.next()?;
                            ctx.lines_as("Archer Wolt", args!["But...", "It's not too late for me to just shut my mouth. Complete silence. I'm not even opening my mouth. Starting right about... Now!", "Seeya later~!"])?;
                        }
                        1 => {
                            ctx.lines_as(
                                "Archer Wolt",
                                args![
                                    "You know...",
                                    "The guy in the pub.",
                                    "Loud, and obnoxious",
                                    "and annoying to liste--"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Archer Wolt",
                                args![
                                    "Hmm...?",
                                    "What's that look for?",
                                    "Anyway, when I have",
                                    "some spare cash, I try",
                                    "to buy him a drink."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Archer Wolt",
                                args!["Still...", "It's not a good", "idea to drink so much.", "Or as much as he does..."],
                            )?;
                        }
                        2 => {
                            ctx.lines_as("Archer Wolt", args!["...!"])?;
                            ctx.next()?;
                            ctx.lines_as("Archer Wolt", args!["...", "I...", "B-but...!"])?;
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Archer Wolt", args!["Oh good~!", "I guess you got", "the point of my story!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Wolt",
                        args![
                            "We, as Archers, put a lot of importance on training and practice, so we kind of look",
                            "down on people who aren't",
                            "diligent at all."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Wolt",
                        args![
                            "Still...",
                            "I have to admit that, at heart,",
                            "I'm a pretty lazy guy. I guess it's pretty amazing that someone like",
                            "me can even be an Archer!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Wolt",
                        args!["In any case,", "whatever you do", "in life, do it", "with passion!"],
                    )?;
                }
                2 => {}
                _ => {}
            }
        }
        _ => {}
    }
    ctx.close()
}

pub fn chief_payon(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn chief_payon_ontouch(ctx: &Ctx) -> Script {
    if ctx.player().base_level()? < 30 {
        ctx.lines_as("Guard", args!["Hey...", "Hey...!", "Show your respect", "to our chief!"])?;
        return ctx.close();
    }
    if ctx.player().base_level()? > 30 {
        if ctx.call(Function::Rand, args![1, 2])? == 1 {
            ctx.lines_as("Guard", args!["Hey...", "Hey...!", "Show your respect", "to our chief!"])?;
            ctx.next()?;
            ctx.lines_as("Guard", args!["Heeeey!", "I said... "])?;
            ctx.next()?;
            ctx.lines_as(
                "Chief",
                args![
                    "It's fine, it's fine.",
                    "It's been a long time",
                    "since I've spoken to",
                    "such young people."
                ],
            )?;
            ctx.next()?;
        }
        match ctx.menu(&[
            "Please tell me about Payon.",
            "Where are the guards from?",
            "Please tell me about the cave. ",
            "What does an Archer do?",
            "What does a Hunter do?",
        ])? {
            0 => {
                ctx.lines_as(
                    "Chief",
                    args!["Payon is the city of highlanders. As long as our history can relate, our city has been self-sufficient."],
                )?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Although our ancestors did not enjoy the benefits of cultural exchange with the Rune-Midgarts Kingdom as we do today, they knew how to make a living without", "any help."])?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Payon has developed its own cultures and ways. We've invented our own means to protect ourselves against the elements, and Payon men and women train as Archers and Hunters to defend themselves."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief",
                    args![
                        "I've heard of weak, cowardly",
                        "young people who fear the fields or dungeons filled with monsters. But to us, battle is a way of life."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["That is why his Majesty, King Tristram III wishes for us to train young people how to effectively fight against evil creatures."])?;
            }
            1 => {
                ctx.lines_as(
                    "Chief",
                    args![
                        "Even since I was young,",
                        "Prontera has sent civil",
                        "servants and envoys to Payon."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief",
                    args![
                        "Royal troops, Kafra Ladies, Guards... At first, there was conflict brought about by differences in our customs",
                        "and cultures."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["However, as time passed,", "we've been able to resolve such petty differences. I cannot deny that they've helped us speedily develop our trade with foreign nations."])?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Now, people who used to be outsiders are now bonifide Payon citizens. When I was young, I would never imagined such a thing possible. Hahahaha~!"])?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["It pleases me to see and meet with them. Even though have come from other places, they have a love for Payon that is as sincere as my own."])?;
            }
            2 => {
                ctx.lines_as(
                    "Chief",
                    args![
                        "Ah...",
                        "I used to go to the cave",
                        "to the North sometimes. But it",
                        "is harder now than it was then."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Evil creatures that I've never seen before endlessly respawn in that place. The monsters in Payon Cave today are different than the ones we used to fight with."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief",
                    args![
                        "Have you ever happened to see",
                        "the Zombies...? Some of them are warriors who entered the cave to protect this village, but never came back."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Although they are a threat, some of those Undead are also victims of that cave. I'm too old to endure that kind of pain..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief",
                    args!["I must do anything I can to stop the Undead, and release their souls from eternal anguish."],
                )?;
            }
            3 => {
                ctx.lines_as("Chief", args!["Ah, it is fortunate that archery is a specialty here in Payon. Because of the surrounding forest, we have an abundant supply of wood to create Bows."])?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["The forest also is a good training place for Archers to learn how to use their environment to impede the advance of their enemies while attacking from a safe distance."])?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["The forest can also be convenient in unexpected ways. When I was young, I ran out of arrows while fighting against monsters in the woods, but arrows dropped by other monsters saved my life! Mwahah!"])?;
            }
            4 => {
                ctx.lines_as(
                    "Chief",
                    args!["As foreign cultures were introduced to Payon, changes have been brought about to our battle style."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief",
                    args![
                        "Technology, especially that of explosives, enabled new methods",
                        "of battle. We were no longer limited to the use of just bows and arrows."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Trapping skills, which make", "hunting monsters much easier, were developed enable here in Payon. Experts in the new skills were dubbed 'Hunters' by our previous Chief."])?;
                ctx.next()?;
                ctx.lines_as("Chief", args!["Although highly effective, trapping is a really dangerous skill if not used properly. That's why we do not approve of the inexperienced becoming Hunters."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief",
                    args![
                        "However, we welcome people",
                        "who have proven to be capable to take the challenge of the Hunter class."
                    ],
                )?;
            }
            _ => {}
        }
        ctx.next()?;
        ctx.lines_as(
            "Chief",
            args!["So tired...", "I'd better rest.", "Please, take care", "of yourself."],
        )?;
        return ctx.close();
    }
    ctx.end()
}

pub fn guard_payon(ctx: &Ctx) -> Script {
    if ctx.player().base_level()? < 30 {
        ctx.lines_as("Guard", args!["Hey...!", "You're not", "allowed here!", "Go back outside!"])?;
        return ctx.close();
    }
    if ctx.player().base_level()? > 30 {
        ctx.lines_as("Guard", args!["I'm sorry,", "but you're", "not allowed here.", "Please leave."])?;
        return ctx.close();
    }
    ctx.end()
}

pub fn archer_joe_payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Archer Joe",
        args!["Payon!", "Such a wonderful", "place! Superb Bows", "and skillful Archers!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Archer Joe", args!["Hey you~!", "Have you heard", "of famous Payon?"])?;
    ctx.next()?;
    match ctx.menu(&["Yeah, of course~! ", "Pay...on?", "..."])? {
        0 => {
            ctx.lines_as(
                "Archer Joe",
                args![
                    "Oh! You the man!",
                    "You know the Archers of Payon!",
                    "We never miss our target! Even from a distance, the hearts of our foes are unsafe!"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["So, you like this place, huh? ", "Hahahaha~"])? == 0 {
                ctx.lines_as(
                    "Archer Joe",
                    args![
                        "Yes! I love this place!",
                        "I love this city so much,",
                        "I've even been doing research on it! If there's anything you wanna know about Payon, please ask me!"
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&[
                    "The people wear unique clothing here.",
                    "What's the building in the middle of town?",
                    "Who's that guy drinking over there? ",
                    "Talk to you later.",
                ])? {
                    0 => {
                        ctx.lines_as(
                            "Archer Joe",
                            args![
                                "Yes, I agree.",
                                "You must know this place used to be isolated because of the thick forests and the mountainous area."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Archer Joe", args!["Because of that, the Payon developed a culture of its own, which is quite different than that of the rest of Rune-Midgarts."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archer Joe",
                            args!["This garment is traditional Payon clothing! Why don't you try wearing one? It's very comfortable~"],
                        )?;
                    }
                    1 => {
                        ctx.lines_as("Archer Joe", args!["You mean the Central Palace? Strangers aren't allowed to enter that place. People say the royal family and their friends from outside gather there."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Archer Joe",
                            args!["I'd like to go there sometime, and see what it's like on the inside!"],
                        )?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Archer Joe",
                            args![
                                "Oh! That guy's notorious!",
                                "Whatever you do, don't treat",
                                "him to any drinks!",
                                "You'll regret it!"
                            ],
                        )?;
                    }
                    3 => {
                        ctx.lines_as("Archer Joe", args!["Okay!", "See ya!", "Catch you later!"])?;
                    }
                    _ => {}
                }
            }
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Archer Joe",
                args!["What a shame...", "How have you not", "heard of the Payon Archers?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Joe",
                args![
                    "Well, when you ",
                    "learn more about us,",
                    "let's talk again and I can tell you why the Payon Archers are",
                    "so great!"
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "Archer Joe",
                args![
                    "Why are",
                    "you so quiet?",
                    "You're not shy, are you?",
                    "Come on, there's no reason",
                    "to be bashful around me~"
                ],
            )?;
        }
        _ => {}
    }
    ctx.close()
}
