use script_sdk_2::{Ctx, Script, Stop, Val, args, runtime};

fn monster_encyclopedia_4pr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Dungeon Monster Encyclopedia]^000000",
        "This is an Encyclopedia describing",
        "monsters living in Dungeons."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Ant Hell:Geffen Tower:Sphinx:Cancel")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            'l2: loop {
                if !(true) {
                    break 'l2;
                }
                'b2: {
                    match runtime::select_values(ctx, &[Val::from("1F:2F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Ant Hell 1F Monsters]^000000",
                                "1. Ant Egg",
                                "Merely an Ant Egg. It can't hurt",
                                "you.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Sticky",
                                "Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 1F Monsters]^000000",
                                "2. Andre",
                                "A diligent worker ant that lives",
                                "for the sake of the colony. Look",
                                "out though, hit one and you'll",
                                "fight them all.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 1F Monsters]^000000",
                                "3. Piere",
                                "A green worker ant that works just",
                                "as diligently as Andre.",
                                "^0099FFItem Drops^000000: Worm",
                                "Peeling, Garlet, Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 1F Monsters]^000000",
                                "4. Deniro",
                                "The red worker ant, and is",
                                "faster than Pieres and Deniros.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 1F Monsters]^000000",
                                "5. Vitata",
                                "Worker ants that are plump with",
                                "honey... So plump, they won't pick",
                                "up anything.",
                                "^0099FFItem Drops^000000: Worm Peeling, Scell",
                                "Honey"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 1F Monsters]^000000",
                                "6. Giearth",
                                "An elderly pixie that wanders caves",
                                "to gather ores. When he dies, he",
                                "plans to leave the world his",
                                "beautiful moustache.",
                                "^0099FFItem Drops^000000: Old Pixie's Moustache"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "1. Ant Egg",
                                "Ant eggs incapable of attacking, or",
                                "even feeling pain.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Sticky",
                                "Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "2. Andre",
                                "A yellow worker ant that may be the",
                                "slowest of its race, aside from",
                                "Vitata.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "3. Piere",
                                "A green worker and obviously",
                                "hailing from France.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "4. Deniro",
                                "The speediest of the worker ants",
                                "that comes in a flashy red colour.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "5. Vitata",
                                "Worker ants in charge of storing",
                                "honey inside their bellies.",
                                "^0099FFItem Drops^000000: Worm Peeling, Scell,",
                                "Honey"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "6. Giearth",
                                "An elderly pixie that wanders caves",
                                "to gather ores. Almost a dwarf, but",
                                "not quite.",
                                "^0099FFItem Drops^000000: Old Pixie's Moustache"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Ant Hell 2F Monsters]^000000",
                                "7. Phreeoni",
                                "An extremely strong bastard that is",
                                "forty percent tongue.",
                                "^0099FFItem Drops^000000: Tongue, Ant Jaw"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            'l4: loop {
                if !(true) {
                    break 'l4;
                }
                'b4: {
                    match runtime::select_values(ctx, &[Val::from("1F:2F:3F:4F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 1F Monsters]^000000",
                                "1. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 1F Monsters]^000000",
                                "2. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus, Garlet",
                                "Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 1F Monsters]^000000",
                                "3. Dustiness",
                                "This flying monster has a high",
                                "dodge rate, so if you have low",
                                "attack accuracy, you may want to",
                                "leave it alone.",
                                "^0099FFItem Drops^000000: Moth Dust, Moth Wing,",
                                "Insect Feeler, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 1F Monsters]^000000",
                                "4. Poison Spore",
                                "A black capped mushroom. It attacks",
                                "adventurers in fear of being eaten,",
                                "despite being poisonous and not",
                                "delicious.",
                                "^0099FFItem Drops^000000: Spore, Green Herb"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 2F Monsters]^000000",
                                "1. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 2F Monsters]^000000",
                                "2. Dustiness",
                                "This flying monster has a high",
                                "dodge rate, so if you have low",
                                "attack accuracy, you may want to",
                                "leave it alone.",
                                "^0099FFItem Drops^000000: Moth Dust, Moth Wing,",
                                "Insect Feeler, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 2F Monsters]^000000",
                                "3. Poison Spore",
                                "A black capped mushroom. It attacks",
                                "adventurers in fear of being eaten,",
                                "despite being poisonous and not",
                                "delicious.",
                                "^0099FFItem Drops^000000: Spore, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 2F Monsters]^000000",
                                "4. Argos",
                                "A monstrous spider that will attack",
                                "adventurers on sight. It's too big",
                                "for adventurers to squish with",
                                "their feet.",
                                "^0099FFItem Drops^000000: Cobweb, Scell, Bug Leg,",
                                "Green Herb, Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 2F Monsters]^000000",
                                "5. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 2F Monsters]^000000",
                                "6. Jakk",
                                "A spooky, Pumpkin-headed monster",
                                "that dresses in a slick formal",
                                "suit. It's been known to invade",
                                "Prontera on St. Hallow's Even in the",
                                "past.",
                                "^0099FFItem Drops^000000: Jack'o'Pumpkin, Zargon"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "1. Argos",
                                "A monstrous spider that will attack",
                                "adventurers on sight. It's too big",
                                "for adventurers to squish with",
                                "their feet.",
                                "^0099FFItem Drops^000000: Cobweb, Scell, Bug Leg,",
                                "Green Herb, Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "2. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "3. Jakk",
                                "A spooky, Pumpkin-headed monster",
                                "that dresses in a slick formal",
                                "suit. It's been known to invade",
                                "Prontera on St. Hallow's Even in the",
                                "past.",
                                "^0099FFItem Drops^000000: Jack'o'Pumpkin, Zargon"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "4. Myst",
                                "A strange, monster made of mist",
                                "that is attached to a phantom",
                                "window.",
                                "^0099FFItem Drops^000000: Trunk, Gas Mask"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "5. Marionette",
                                "A monster reborn as a cursed doll",
                                "that is bound to strings attached",
                                "to wooden sticks.",
                                "^0099FFItem Drops^000000: Golden Hair, Trunk"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "6. Bathory",
                                "A wart-nosed Witch wearing bunny",
                                "boxers that will attack anything",
                                "prettier than her. In other words,",
                                "she attacks everyone.",
                                "^0099FFItem Drops^000000: With Starsand"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 3F Monsters]^000000",
                                "7. Nightmare",
                                "A ghostly horse that radiates a",
                                "violet aura of evil.",
                                "^0099FFItem Drops^000000: Horseshoe, Blue Herb"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 4F Monsters]^000000",
                                "1. Myst",
                                "A strange, monster made of mist",
                                "that is attached to a phantom",
                                "window.",
                                "^0099FFItem Drops^000000: Trunk, Gas Mask"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 4F Monsters]^000000",
                                "2. Deviruchi",
                                "A minature demon that repeatedly",
                                "stabs humans with its pitchfork.",
                                "It's cute, but nonetheless a true",
                                "fiend of darkness.",
                                "^0099FFItem Drops^000000: Little Evil Horn,",
                                "Little Evil Wing, Zargon"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 4F Monsters]^000000",
                                "3. Raydric",
                                "The soul of a castle guard bound to",
                                "a living suit of armour through a",
                                "curse.",
                                "^0099FFItem Drops^000000: Elunium, Chivalry",
                                "Emblem"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 4F Monsters]^000000",
                                "4. Joker",
                                "A large, enchanted playing card. If",
                                "you don't have good attack",
                                "accuracy, the stakes are against",
                                "you when fighting Joker.",
                                "^0099FFItem Drops^000000: High Heels"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Geffen Dungeon 4F Monsters]^000000",
                                "5. Doppelganger",
                                "A ghostly double of a Swordman.",
                                "Perhaps the coolest and baddest",
                                "monster in all of Rune-Midgarts.",
                                "^0099FFItem Drops^000000: Spiky Band, Blue",
                                "Potion, Cursed Ruby, Ruby"
                            ])?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            'l6: loop {
                if !(true) {
                    break 'l6;
                }
                'b6: {
                    match runtime::select_values(ctx, &[Val::from("1F:2F:3F:4F:5F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Sphinx 1F Monsters]^000000",
                                "1. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly",
                                "Wing, Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 1F Monsters]^000000",
                                "2. Snake",
                                "Green snake that lives in the",
                                "forests and deserts. They're not",
                                "poisonous, but their bites still",
                                "hurt.",
                                "^0099FFItem Drops^000000: Snake Scale, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 1F Monsters]^000000",
                                "3. Zerom",
                                "An undead slave. Sadly, not even",
                                "death will bring peace to the",
                                "abusive hours of labour Zerom",
                                "suffers for his Pharaoh.",
                                "^0099FFItem Drops^000000: Panties"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 1F Monsters]^000000",
                                "4. Matyr",
                                "A hound saturated with evil. It's",
                                "always sleeping, but springs to",
                                "action after smelling an",
                                "adventurer.",
                                "^0099FFItem Drops^000000: Monster's Feed,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 1F Monsters]^000000",
                                "5. Requieum",
                                "An ancient slave that carries a",
                                "heavy coffin on its back. Weary",
                                "from its labour, Requiem simply",
                                "collapses, hoping the coffin will",
                                "hit its mark, when attacking.",
                                "^0099FFItem Drops^000000: Old Blue Box"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Sphinx 2F Monsters]^000000",
                                "1. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly",
                                "Wing, Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 2F Monsters]^000000",
                                "2. Matyr",
                                "A hound saturated with evil. It's",
                                "always sleeping, but springs to",
                                "action after smelling an",
                                "adventurer.",
                                "^0099FFItem Drops^000000: Monster's Feed,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 2F Monsters]^000000",
                                "3. Requiem",
                                "An ancient slave that carries a",
                                "heavy coffin on its back. Weary",
                                "from its labour, Requiem simply",
                                "collapses, hoping the coffin will",
                                "hit its mark, when attacking.",
                                "^0099FFItem Drops^000000: Old Blue Box"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 2F Monsters]^000000",
                                "4. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Sphinx 3F Monsters]^000000",
                                "1. Matyr",
                                "A hound saturated with evil. It's",
                                "always sleeping, but springs to",
                                "action after smelling an",
                                "adventurer.",
                                "^0099FFItem Drops^000000: Monster's Feed,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 3F Monsters]^000000",
                                "2. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 3F Monsters]^000000",
                                "3. Marduk",
                                "A gangly wizard of darkness. Look",
                                "out, it knows magic!",
                                "^0099FFItem Drops^000000: Flame Heart"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines(args![
                                "^FF0000[Sphinx 4F Monsters]^000000",
                                "1. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 4F Monsters]^000000",
                                "2. Marduk",
                                "A gangly wizard of darkness. Look",
                                "out, it knows magic!",
                                "^0099FFItem Drops^000000: Flame Heart"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 4F Monsters]^000000",
                                "3. Medusa",
                                "Monster with hair composed of",
                                "snakes. It is rumoured to turn",
                                "people into stone if they look into",
                                "her eyes.",
                                "^0099FFItem Drops^000000: Dead Medusa, Horrendous",
                                "Snake, White Herb"
                            ])?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.lines(args![
                                "^FF0000[Sphinx 5F Monsters]^000000",
                                "1. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 5F Monsters]^000000",
                                "2. Marduk",
                                "A gangly wizard of darkness. Look",
                                "out, it knows magic!",
                                "^0099FFItem Drops^000000: Flame Heart"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sphinx 5F Monsters]^000000",
                                "3. Medusa",
                                "Monster with hair composed of",
                                "snakes. It is rumoured to turn",
                                "people into stone if they look into",
                                "her eyes.",
                                "^0099FFItem Drops^000000: Dead Medusa, Horrendous",
                                "Snake, White Herb"
                            ])?;
                            ctx.next()?;
                        }
                        6 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn monster_encyclopedia_4pr(ctx: &Ctx) -> Script {
    monster_encyclopedia_4pr_body(ctx, Vec::new()).map(|_| ())
}

fn monster_encyclopedia_5pr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Dungeon Monster Encyclopedia]^000000",
        "This is an Encyclopedia describing",
        "Monsters living in Dungeons."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Orc Dungeon:Byalan Cave near Izlude:Prontera Culvert")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            'l2: loop {
                if !(true) {
                    break 'l2;
                }
                'b2: {
                    match runtime::select_values(ctx, &[Val::from("1F:2F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 1F Monsters]^000000",
                                "1. Chonchon",
                                "Fly monsters that move with great",
                                "speed. Amazingly, they can heal in",
                                "the presense of fecal matter.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Fly",
                                "Wing"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 1F Monsters]^000000",
                                "2. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 1F Monsters]^000000",
                                "3. Orc Zombie",
                                "Orcs that have risen back from the",
                                "dead. The honorable fighting spirit",
                                "of the Orc Warrior never dies!",
                                "^0099FFItem Drops^000000: Orc Claw, Sticky Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 1F Monsters]^000000",
                                "4. Orc Skeleton",
                                "The skeleton of an Orc that has",
                                "been brought back to life. Even in",
                                "death, Orcs continue to do battle.",
                                "^0099FFItem Drops^000000: Orc's Fang, Green",
                                "Herb"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 2F Monsters]^000000",
                                "1. Chonchon",
                                "Fly monsters that move with great",
                                "speed. Amazingly, they can heal in",
                                "the presense of fecal matter.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Fly",
                                "Wing"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 2F Monsters]^000000",
                                "2. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerbys.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 2F Monsters]^000000",
                                "3. Orc Skeleton",
                                "The skeleton of an Orc that has",
                                "been brought back to life. Even in",
                                "death, Orcs continue to do battle.",
                                "^0099FFItem Drops^000000: Orc's Fang, Green",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Orc Dungeon 2F Monsters]^000000",
                                "4. Zenorc",
                                "A dishonorable Orc whose body has",
                                "been cursed. They continue their",
                                "shameful ways by looting items that",
                                "have been dropped to the ground.",
                                "^0099FFItem Drops^000000: Zenorc's Fang, Sticky",
                                "Mucus, Yellow Herb"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            'l4: loop {
                if !(true) {
                    break 'l4;
                }
                'b4: {
                    match runtime::select_values(ctx, &[Val::from("1F:2F:3F:4F:5F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 1F Monsters]^000000",
                                "1. Plankton",
                                "Even though they seem",
                                "insignificantly small, be careful",
                                "not to step on them. Plankton are",
                                "light and can drift on the water.",
                                "^0099FFItem Drops^000000: Single Cell, Garlet",
                                "Sticky Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 1F Monsters]^000000",
                                "2. Kukre",
                                "Kukre look better than Thief Bugs",
                                "but basically loot items just the",
                                "same. Luckily, they don't attack",
                                "players in a group.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet",
                                "Monster's Feed, Red Herb, Insect",
                                "Feeler"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 1F Monsters]^000000",
                                "3. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea. Attack",
                                "using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 1F Monsters]^000000",
                                "4. Vadon",
                                "Covered in a thick, protective",
                                "shell, Vadons attack with powerful",
                                "pincers. Although they look like",
                                "crabs, their meat can't be eaten.",
                                "^0099FFItem Drops^000000: Nipper, Garlet, Solid",
                                "Shell, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 1F Monsters]^000000",
                                "5. Marina",
                                "Transparent jellyfish that attack",
                                "by stretching their flexible bodies",
                                "in a whip-like fashion. They live",
                                "in cool places near water.",
                                "^0099FFItem Drops^000000: Single Cell, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 2F Monsters]^000000",
                                "1. Plankton",
                                "Even though they seem",
                                "insignificantly small, be careful",
                                "not to step on them. Plankton are",
                                "light and can drift on the water.",
                                "^0099FFItem Drops^000000: Single Cell, Garlet",
                                "Sticky Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 2F Monsters]^000000",
                                "2. Kukre",
                                "Kukre look better than Thief Bugs",
                                "but basically loot items just the",
                                "same. Luckily, they don't attack",
                                "players in a group.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet",
                                "Monster's Feed, Red Herb, Insect",
                                "Feeler"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 2F Monsters]^000000",
                                "3. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea. Attack",
                                "using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 2F Monsters]^000000",
                                "4. Vadon",
                                "Covered in a thick, protective",
                                "shell, Vadons attack with powerful",
                                "pincers. Although they look like",
                                "crabs, their meat can't be eaten.",
                                "^0099FFItem Drops^000000: Nipper, Garlet, Solid",
                                "Shell, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 2F Monsters]^000000",
                                "5. Marina",
                                "Transparent jellyfish that attack",
                                "by stretching their flexible bodies",
                                "in a whip-like fashion. They live",
                                "in cool places near water.",
                                "^0099FFItem Drops^000000: Single Cell, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 2F Monsters]^000000",
                                "6. Thara Frog",
                                "Frogs of red colour, surely stronger",
                                "than Roda Frogs. However there is",
                                "obviously one thing in common about",
                                "them, an annoying croaking noise.",
                                "^0099FFItem Drops^000000: Spawn, Scell, Sticky",
                                "Webfoot"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 3F Monsters]^000000",
                                "1. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea. Attack",
                                "using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 3F Monsters]^000000",
                                "2. Thara Frog",
                                "Frogs of red colour, surely stronger",
                                "than Roda Frogs. However there is",
                                "obviously one thing in common about",
                                "them, an annoying croaking noise.",
                                "^0099FFItem Drops^000000: Spawn, Scell, Sticky",
                                "Webfoot"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 3F Monsters]^000000",
                                "3. Cornutus",
                                "Benign monsters that conceal",
                                "themselves in hard, turban shaped",
                                "shells. They try to live as",
                                "peacefully as they can in this",
                                "crazy, crazy world.",
                                "^0099FFItem Drops^000000: Conch, Scell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 3F Monsters]^000000",
                                "4. Marse",
                                "A miniature squid with miniature",
                                "tentacles. How it moves through",
                                "water with those tiny things is",
                                "still a scientific mystery.",
                                "^0099FFItem Drops^000000: Squid Ink, Tentacle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 3F Monsters]^000000",
                                "5. Obeaune",
                                "A female Mermaid that attacks with",
                                "its wild, flowing hair. Whether or",
                                "not its male version is Merman is",
                                "still under debate.",
                                "^0099FFItem Drops^000000: Heart of Mermaid, Fin"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 4F Monsters]^000000",
                                "1. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea. Attack",
                                "using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 4F Monsters]^000000",
                                "2. Marse",
                                "A miniature squid with miniature",
                                "tentacles. How it moves through",
                                "water with those tiny things is",
                                "still a scientific mystery.",
                                "^0099FFItem Drops^000000: Squid Ink, Tentacle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 4F Monsters]^000000",
                                "3. Obeaune",
                                "A female Mermaid that attacks with",
                                "its wild, flowing hair. Whether or",
                                "not its male version is Merman is",
                                "still under debate.",
                                "^0099FFItem Drops^000000: Heart of Mermaid, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 4F Monsters]^000000",
                                "4. Marine Sphere",
                                "Strange, round-shaped monsters that",
                                "pulse with destructive energy.",
                                "Gathering their Detonators may be",
                                "useful for Alchemists.",
                                "^0099FFItem Drops^000000: Tendon, Detonator"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 4F Monsters]^000000",
                                "5. Phen",
                                "A blue fish with a pointed nose and",
                                "sad, incredibly sad, vacant eyes.",
                                "^0099FFItem Drops^000000: Fish Tail, Sharp Scale,",
                                "Meat, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 4F Monsters]^000000",
                                "6. Sword Fish",
                                "Fish Monster with a sharp, long nose",
                                "that's just like a sword. Although it",
                                "has googly eyes, it's a dangerous",
                                "monster. Why wasn't it named Sword",
                                "Nose Fish?",
                                "^0099FFItem Drops^000000: Sharp Scale, Gill"
                            ])?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 5F Monsters]^000000",
                                "1. Marine Sphere",
                                "Strange, round-shaped monsters that",
                                "pulse with destructive energy.",
                                "Gathering their Detonators may be",
                                "useful for Alchemists.",
                                "^0099FFItem Drops^000000: Tendon, Detonator"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 5F Monsters]^000000",
                                "2. Sword Fish",
                                "Fish Monster with a sharp, long nose",
                                "that's just like a sword. Although it",
                                "has googly eyes, it's a dangerous",
                                "monster. Why wasn't it named Sword",
                                "Nose Fish?",
                                "^0099FFItem Drops^000000: Sharp Scale, Gill"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 5F Monsters]^000000",
                                "3. Marse",
                                "A miniature squid with miniature",
                                "tentacles. How it moves through",
                                "water with those tiny things is",
                                "still a scientific mystery.",
                                "^0099FFItem Drops^000000: Squid Ink, Tentacle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 5F Monsters]^000000",
                                "4. Obeaune",
                                "A female Mermaid that attacks with",
                                "its wild, flowing hair. Whether or",
                                "not its male version is Merman is",
                                "still under debate.",
                                "^0099FFItem Drops^000000: Heart of Mermaid, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 5F Monsters]^000000",
                                "5. Marc",
                                "A proud looking Sea Horse that sort",
                                "of looks like a dragon.",
                                "Unfortunately, you can't ride it.",
                                "^0099FFItem Drops^000000: Gill, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Byalan Cave 5F Monsters]^000000",
                                "6. Strouf",
                                "A lordly fish monster that looks",
                                "like the God of the Seas. Carries a",
                                "lightning trident to fight the",
                                "surface people.",
                                "^0099FFItem Drops^000000: Fin, Feather, Gill"
                            ])?;
                            ctx.next()?;
                        }
                        6 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            'l6: loop {
                if !(true) {
                    break 'l6;
                }
                'b6: {
                    match runtime::select_values(ctx, &[Val::from("1F.:2F.:3F.:4F.:Cancel.")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 1F Monsters]^000000",
                                "1. Thief Bug Egg",
                                "A filthy egg form a filthy Thief",
                                "Bug. They make horrible omelets.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 1F Monsters]^000000",
                                "2. Thief Bug Baby",
                                "The undeveloped version of the",
                                "Thief Bug. Even as babies, they're",
                                "pretty disgusting.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red",
                                "Herb, Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 1F Monsters]^000000",
                                "3. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 1F Monsters]^000000",
                                "4. Spore",
                                "Mushroom-like monsters that usually",
                                "live in forests or dungeons. The",
                                "strange, chocolate chip like nubs",
                                "on its cap are actually something",
                                "else.",
                                "^0099FFItem Drops^000000: Spore, Red Herb, Blue",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 1F Monsters]^000000",
                                "5. Tarou",
                                "A tiny, little white mouse. Its",
                                "squeaks can be heard in the Dead",
                                "Pit and the Prontera Culvert.",
                                "^0099FFItem Drops^000000: Rat Tail, Animal",
                                "Skin, Feather, Monster's Feed"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "1. Thief Bug Egg",
                                "A filthy egg form a filthy Thief",
                                "Bug. They make horrible omelets.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "2. Thief Bug Baby",
                                "The undeveloped version of the",
                                "Thief Bug. Even as babies, they're",
                                "pretty disgusting.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red",
                                "Herb, Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "3. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "4. Spore",
                                "Mushroom-like monsters that usually",
                                "live in forests or dungeons. The",
                                "strange, chocolate chip like nubs",
                                "on its cap are actually something",
                                "else.",
                                "^0099FFItem Drops^000000: Spore, Red Herb, Blue",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "5. Tarou",
                                "A tiny, little white mouse. Its",
                                "squeaks can be heard in the Dead",
                                "Pit and the Prontera Culvert.",
                                "^0099FFItem Drops^000000: Rat Tail, Animal",
                                "Skin, Feather, Monster's Feed"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "6. Plankton",
                                "Even though they seem",
                                "insignificantly small, be careful",
                                "not to step on them. Plankton are",
                                "light and can drift on the water.",
                                "^0099FFItem Drops^000000: Single Cell, Garlet",
                                "Sticky Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "7. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea that",
                                "attack using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 2F Monsters]^000000",
                                "8. Female Thief Bug",
                                "Large, brown insects that are",
                                "gruesome to the eye and disgusting",
                                "to the touch. Notorious for quickly",
                                "grabbing whatever drops to the",
                                "ground.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red Herb,",
                                "Jellopy, Garlet, Insect Feeler"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 3F Monsters]^000000",
                                "1. Thief Bug Egg",
                                "A filthy egg form a filthy Thief",
                                "Bug. They make horrible omelets.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 3F Monsters]^000000",
                                "2. Thief Bug Baby",
                                "The undeveloped version of the",
                                "Thief Bug. Even as babies, they're",
                                "pretty disgusting.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red",
                                "Herb, Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 3F Monsters]^000000",
                                "3. Female Thief Bug",
                                "Large, brown insects that are",
                                "gruesome to the eye and disgusting",
                                "to the touch. Notorious for quickly",
                                "grabbing whatever drops to the",
                                "ground.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red Herb,",
                                "Jellopy, Garlet, Insect Feeler"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 3F Monsters]^000000",
                                "4. Tarou",
                                "A tiny, little white mouse. Its",
                                "squeaks can be heard in the Dead",
                                "Pit and the Prontera Culvert.",
                                "^0099FFItem Drops^000000: Rat Tail, Animal",
                                "Skin, Feather, Monster's Feed"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 3F Monsters]^000000",
                                "5. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 3F Monsters]^000000",
                                "6. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus, Garlet",
                                "Green Herb"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 4F Monsters]^000000",
                                "1. Theif Bug Egg",
                                "A filthy egg form a filthy Thief",
                                "Bug. They make horrible omelets.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 4F Monsters]^000000",
                                "2. Thief Bug Baby",
                                "The undeveloped version of the",
                                "Thief Bug. Even as babies, they're",
                                "pretty disgusting.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red",
                                "Herb, Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 4F Monsters]^000000",
                                "3. Female Thief Bug",
                                "Large, brown insects that are",
                                "gruesome to the eye and disgusting",
                                "to the touch. Notorious for quickly",
                                "grabbing whatever drops to the",
                                "ground.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red Herb,",
                                "Jellopy, Garlet, Insect Feeler"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 4F Monsters]^000000",
                                "4. Male Thief Bug",
                                "A large, blue insect, the Male",
                                "Thief Bug is considerably powerful.",
                                "They're also very aggressive",
                                "towards humans.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red Herb,",
                                "Jellopy, Garlet, Insect Feeler,",
                                "Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Culvert 4F Monsters]^000000",
                                "5. Golden Thief Bug",
                                "A Thief Bug with a shell that",
                                "shimmers in a brilliant, golden",
                                "light. Beautiful, but still a nasty",
                                "bug through and through.",
                                "^0099FFItem Drops^000000: Blue Herb, Gold, Ora",
                                "Ora, Insect Feeler"
                            ])?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn monster_encyclopedia_5pr(ctx: &Ctx) -> Script {
    monster_encyclopedia_5pr_body(ctx, Vec::new()).map(|_| ())
}
