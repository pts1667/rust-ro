use script_sdk_2::{Ctx, Script, Stop, Val, args, runtime};

fn monster_encyclopedia_6pr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Dungeon Monster Encyclopedia]^000000",
        "This is an Encyclopedia describing",
        "Monsters living in Dungeons."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Mjolnir Dead Pit:Payon Cave:Pyramid")],
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
                    match runtime::select_values(ctx, &[Val::from("1F:2F:3F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Dead Pit 1F Monsters]^000000",
                                "1. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Fly Wing",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 1F Monsters]^000000",
                                "2. Tarou",
                                "A tiny, little white mouse. Its",
                                "squeaks can be heard in the Dead",
                                "Pit and the Prontera Culvert.",
                                "^0099FFItem Drops^000000: Rat Tail, Animal",
                                "Skin, Feather, Monster's Feed"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 1F Monsters]^000000",
                                "3. Martin",
                                "An adorable mole wearing a safety",
                                "helmet. He's deathly afraid of",
                                "cave-ins and occasionally stops to",
                                "cower in fear.",
                                "^0099FFItem Drops^000000: Mole Whiskers, Mole",
                                "Claw"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 1F Monsters]^000000",
                                "4. Drainliar",
                                "A blood red bat that's much",
                                "stronger than Familiar. It also",
                                "tends to pursue any human it",
                                "finds.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Red Herb"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Dead Pit 2F Monsters]^000000",
                                "1. Martin",
                                "An adorable mole wearing a safety",
                                "helmet. He's deathly afraid of",
                                "cave-ins and occasionally stops to",
                                "cower in fear.",
                                "^0099FFItem Drops^000000: Mole Whiskers, Mole",
                                "Claw"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 2F Monsters]^000000",
                                "2. Drainliar",
                                "A blood red bat that's much",
                                "stronger than Familiar. It also",
                                "tends to pursue any human it",
                                "finds.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 2F Monsters]^000000",
                                "3. Skel Wroker",
                                "A reanimated skeleton of a miner",
                                "that has died, but come back to",
                                "abuse its health insurance policy.",
                                "^0099FFItem Drops^000000: Iron, Lantern"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 2F Monsters]^000000",
                                "4. Myst",
                                "A strange, monster made of mist",
                                "that is attached to a phantom",
                                "window.",
                                "^0099FFItem Drops^000000: Trunk, Gas Mask"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Dead Pit 3F Monsters]^000000",
                                "1. Skel Worker",
                                "A reanimated skeleton of a miner",
                                "that has died, but returned to join",
                                "his brothers in the Miner's Union",
                                "Strike.",
                                "^0099FFItem Drops^000000: Iron, Lantern"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 3F Monsters]^000000",
                                "2. Myst",
                                "A strange, monster made of mist",
                                "that is attached to a phantom",
                                "window.",
                                "^0099FFItem Drops^000000: Trunk, Gas Mask"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Dead Pit 3F Monsters]^000000",
                                "3. Evil Druid",
                                "A flamboyantly evil druid. It's",
                                "always using a floating spellbook",
                                "that crackles with energy to cause",
                                "misery to adventurers.",
                                "^0099FFItem Drops^000000: Amulet, White Herb"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
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
                                "^FF0000[Payon Cave 1F Monsters]^000000",
                                "1. Familiar",
                                "A gray bat that's not very strong,",
                                "but really annoying because it",
                                "attacks very fast and relentlessly",
                                "pursues passerby.",
                                "^0099FFItem Drops^000000: Tooth of Batt, Fly Wing,",
                                "Grape, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 1F Monsters]^000000",
                                "2. Spore",
                                "Mushroom-like monsters that attack",
                                "with mushy headbutts. Usually live",
                                "in forests or dungeons.",
                                "^0099FFItem Drops^000000: Spore, Red Herb, Blue",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 1F Monsters]^000000",
                                "3. Zombie",
                                "Bad Case of the Dead which has been",
                                "reborn as a Walking Corpse by Black",
                                "magic. Let's lead it to Nirvana.",
                                "^0099FFItem Drops^000000: Decayed Nail, Sticky",
                                "Mucus, Horrendous Mouth"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Payon Cave 2F Monsters]^000000",
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
                                "^FF0000[Payon Cave 2F Monsters]^000000",
                                "2. Eggyra",
                                "A weird, robotic egg looking thing",
                                "that waddles when it walks. No one",
                                "knows what these things are made",
                                "out of.",
                                "^0099FFItem Drops^000000: Scell, Sticky Mucus,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 2F Monsters]^000000",
                                "3. Magnolia",
                                "Cute looking creatures that appear",
                                "as large frying pans cooking an",
                                "egg. Ironically, they're not",
                                "morning monsters.",
                                "^0099FFItem Drops^000000: Jellopy, Garlet, Scell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 2F Monsters]^000000",
                                "4. Soldier Skeleton",
                                "A skeleton soldier that wields two",
                                "swords at once. In life, they",
                                "laughed at him for being obese, but",
                                "now he will have his revenge.",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Payon Cave 3F Monsters]^000000",
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
                                "^FF0000[Payon Cave 3F Monsters]^000000",
                                "2. Soldier Skeleton",
                                "A double sword wielding skeleton.",
                                "Like all good soldiers, this",
                                "skeleton has a nice, manly cleft in",
                                "its chin.",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 3F Monsters]^000000",
                                "3. Munak",
                                "A beautiful zombie that seems to be",
                                "linked to Bongun somehow.",
                                "^0099FFItem Drops^000000: Daenggie, Munak Turban"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 3F Monsters]^000000",
                                "4. Archer Skeleton",
                                "An excellent, Undead bowman.",
                                "^0099FFItem Drops^000000: Skel-Bone, Fire Arrow",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines(args![
                                "^FF0000[Payon Cave 4F Monsters]^000000",
                                "1. Soldier Skeleton",
                                "A skeleton wielding two swords at",
                                "the same time> Wear shorts and",
                                "booties, but not socks and",
                                "underwear. A very risque monster.",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 4F Monsters]^000000",
                                "2. Archer Skeleton",
                                "Despite not having actual eyes,",
                                "Archer Skeletons have great aim.",
                                "^0099FFItem Drops^000000: Skel-Bone, Fire Arrow,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 4F Monsters]^000000",
                                "3. Sohee",
                                "A female Ghost that harbours a deep",
                                "grudge. Although she is usually",
                                "crying, she can become fierce upon",
                                "encountering the living.",
                                "^0099FFItem Drops^000000: Black Hair, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 4F Monsters]^000000",
                                "4. Horong",
                                "An eerie-looking, violent fireball.",
                                "It's useless to use hiding skills",
                                "near this vengeful spirit.",
                                "^0099FFItem Drops^000000: Stone Heart, Zargon,",
                                "Fire Arrow"
                            ])?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.lines(args![
                                "^FF0000[Payon Cave 5F Monsters]^000000",
                                "1. Soldier Skeleton",
                                "A skeleton soldier that wields two",
                                "swords at once. He might have",
                                "fought for justice at one time, but",
                                "now he's one of the undead!",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 5F Monsters]^000000",
                                "2. Archer Skeleton",
                                "Despite not having actualy eyes,",
                                "Archer Skeletons have great aim.",
                                "^0099FFItem Drops^000000: Skel-Bone, Fire Arrow,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 5F Monsters]^000000",
                                "3. Sohee",
                                "A female Ghost that harbours a deep",
                                "grudge. Although she is usually",
                                "crying, she can become fierce upon",
                                "encountering the living.",
                                "^0099FFItem Drops^000000: Black Hair, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 5F Monsters]^000000",
                                "4. Horong",
                                "An eerie-looking, violent fireball.",
                                "It's useless to use hiding skills",
                                "near this vengeful spirit.",
                                "^0099FFItem Drops^000000: Stone Heart, Zargon,",
                                "Fire Arrow"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Payon Cave 5F Monsters]^000000",
                                "5. Moonlight Flower",
                                "A wild Girl that command the 9 Tail",
                                "Foxes. She carries around a staff",
                                "topped with a Bell.",
                                "^0099FFItem Drops^000000: 9 Tails, White Herb",
                                "Topaz, Elunium"
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
                    match runtime::select_values(ctx, &[Val::from("1F:2F:3F:4F:5F:6F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Pyramid 1F Monsters]^000000",
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
                                "^FF0000[Pyramid 1F Monsters]^000000",
                                "2. Spore",
                                "Giant sized mushroom-like monsters",
                                "that might taste good on giant",
                                "sized pizza. Usually live in",
                                "forests or dungeons.",
                                "^0099FFItem Drops^000000: Spore, Red Herb, Blue",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 1F Monsters]^000000",
                                "3. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus, Garlet,",
                                "Green Herb"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Pyramid 2F Monsters]^000000",
                                "1. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus,",
                                "Garlet, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 2F Monsters]^000000",
                                "2. Drainliar",
                                "A blood red bat that's much",
                                "stronger than Familiar. It also",
                                "tends to pursue any human it",
                                "finds.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 2F Monsters]^000000",
                                "3. Soldier Skeleton",
                                "A skeleton soldier that wields two",
                                "swords at once. Doesn't have much",
                                "to do, other than attack passerby.",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 2F Monsters]^000000",
                                "4. Archer Skeleton",
                                "Despite not having actual eyes,",
                                "Archer Skeletons have great aim.",
                                "^0099FFItem Drops^000000: Skel-Bone, Fire Arrow,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Pyramid 3F Monsters]^000000",
                                "1. Drainliar",
                                "A blood red bat that's much",
                                "stronger than Familiar. It also",
                                "tends to pursue any human it",
                                "finds.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 3F Monsters]^000000",
                                "2. Soldier Skeleton",
                                "A skeleton soldier that wields two",
                                "swords at once. Surprisingly quick",
                                "for an Undead monster.",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 3F Monsters]^000000",
                                "3. Archer Skeleton",
                                "Despite not having actual eyes,",
                                "Archer Skeletons have great aim.",
                                "^0099FFItem Drops^000000: Skel-Bone, Fire Arrow,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 3F Monsters]^000000",
                                "4. Mummy",
                                "A walking corpse covered with",
                                "bandages. It probably used to be",
                                "beautiful once.",
                                "^0099FFItem Drops^000000: Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 3F Monsters]^000000",
                                "5. Verit",
                                "A mummified dog that will pick up",
                                "anything dropped to the ground.",
                                "Although it's a zombie, it seems",
                                "happy to be alive again.",
                                "^0099FFItem Drops^000000: Immortal Herat, Zargon",
                                "Rotten Bandage"
                            ])?;
                            ctx.next()?;
                        }
                        4 => {
                            ctx.lines(args![
                                "^FF0000[Pyramid 4F Monsters]^000000",
                                "1. Mummy",
                                "A walking corpse covered with",
                                "bandages. It probably used to be",
                                "beautiful once.",
                                "^0099FFItem Drops^000000: Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 4F Monsters]^000000",
                                "2. Verit",
                                "A mummified dog that will pick up",
                                "anything dropped to the ground.",
                                "Although it's a zombie, it seems",
                                "happy to be alive again.",
                                "^0099FFItem Drops^000000: Immortal Heart, Zargon,",
                                "Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 4F Monsters]^000000",
                                "3. Ghoul",
                                "Similar to a Zombie, but Ghouls are",
                                "green and much stronger. Its",
                                "retching is offensive in more ways",
                                "than one.",
                                "^0099FFItem Drops^000000: Horrendous Mouth"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 4F Monsters]^000000",
                                "4. Isis",
                                "A monster that is half serpent and",
                                "half woman, as well as one of",
                                "Osiris' trusted champions.",
                                "^0099FFItem Drops^000000: Scale Skin, Shining",
                                "Scale"
                            ])?;
                            ctx.next()?;
                        }
                        5 => {
                            ctx.lines(args![
                                "^FF0000[Pyramid 5F Monsters]^000000",
                                "1. Mummy",
                                "A walking corpse covered with",
                                "bandages. It probably used to be",
                                "beautiful once.",
                                "^0099FFItem Drops^000000: Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 5F Monsters]^000000",
                                "2. Ghoul",
                                "Similar to a Zombie, but Ghouls are",
                                "green and much stronger. Its",
                                "retching is offensive in more ways",
                                "than one.",
                                "^0099FFItem Drops^000000: Horrendous Mouth"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 5F Monsters]^000000",
                                "3. Isis",
                                "A monster that is half serpent and",
                                "half woman, as well as one of",
                                "Osiris' trusted champions.",
                                "^0099FFItem Drops^000000: Scale Skin, Shining",
                                "Scale"
                            ])?;
                            ctx.next()?;
                        }
                        6 => {
                            ctx.lines(args![
                                "^FF0000[Pyramid 6F Monsters]^000000",
                                "1. Mummy",
                                "A walking corpse covered with",
                                "bandages. It probably used to be",
                                "beautiful once.",
                                "^0099FFItem Drops^000000: Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 6F Monsters]^000000",
                                "2. Verit",
                                "A mummified dog that will pick up",
                                "anything dropped to the ground.",
                                "Although it's a zombie, it seems",
                                "happy to be alive again.",
                                "^0099FFItem Drops^000000: Immortal Heart, Zargon,",
                                "Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 6F Monsters]^000000",
                                "3. Ghoul",
                                "Similar to a Zombie, but Ghouls are",
                                "green and much stronger. Its",
                                "retching is offensive in more ways",
                                "than one.",
                                "^0099FFItem Drops^000000: Horrendous Mouth"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 6F Monsters]^000000",
                                "4. Isis",
                                "A monster that is half serpent and",
                                "half woman, as well as one of",
                                "Osiris' trusted champions.",
                                "^0099FFItem Drops^000000: Scale Skin, Shining",
                                "Scale"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Pyramid 6F Monsters]^000000",
                                "5. Osiris",
                                "The indisputable King of Mummies.",
                                "He wears a crown and rotting",
                                "bandages that are decidedly royal",
                                "purple colour.",
                                "^0099FFItem Drops^000000: Memento, Rotten",
                                "Bandage, Hand of God, Elunium"
                            ])?;
                            ctx.next()?;
                        }
                        7 => {
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

pub fn monster_encyclopedia_6pr(ctx: &Ctx) -> Script {
    monster_encyclopedia_6pr_body(ctx, Vec::new()).map(|_| ())
}

fn monster_encyclopedia_7pr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Dungeon Monster Encyclopedia]^000000",
        "This is an Encyclopedia describing",
        "Monsters living in Dungeons."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Sunken Ship near Alberta:Prontera Maze")],
        )?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
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
                                "^FF0000[Sunken Ship 1F Monsters]^000000",
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
                                "^FF0000[Sunken Ship 1F Monsters]^000000",
                                "2. Kukre",
                                "Kukre look better than Thief Bugs",
                                "but basically loot items just the",
                                "same. Luckily, they don't attack",
                                "players in a group.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Monster's Feed, Red Herb, Insect",
                                "Feeler"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 1F Monsters]^000000",
                                "3. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea that",
                                "attack using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacles, Sticky Mucus",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 1F Monsters]^000000",
                                "4. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus, Garlet",
                                "Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 1F Monsters]^000000",
                                "5. Poison Spore",
                                "A black capped mushroom. It attacks",
                                "adventurers in fear of being eaten,",
                                "despite being poisonous and not",
                                "delicious.",
                                "^0099FFItem Drops^000000: Spore, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 1F Monsters]^000000",
                                "6. Pirate Skel",
                                "A topless pirate skeleton that",
                                "skips around in purple socks.",
                                "Scourge of the seven seas.",
                                "^0099FFItem Drops^000000: Skel-Bone"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "1. Kukre",
                                "Kukre look better than Thief Bugs",
                                "but basically loot items just the",
                                "same. Luckily, they don't attack",
                                "players in a group.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Monster's Feed, Red Herb, Insect",
                                "Feeler"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "2. Hydra",
                                "Vegetable Monsters that live near",
                                "water or in the deep sea that",
                                "attack using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "3. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus,",
                                "Garlet, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "4. Thara Frog",
                                "Red Frogs that are much stronger",
                                "than the green Roda Frogs. They",
                                "also produce an annoying croaking",
                                "noise.",
                                "^0099FFItem Drops^000000: Spawn, Scell, Sticky",
                                "Webfoot"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "5. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "6. Megalodon",
                                "Skeleton Fish having spooky empty",
                                "eye-holes.",
                                "^0099FFItem Drops^000000: Stinky Scale, Skel-Bone"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "7. Pirate Skel",
                                "A topless pirate skeleton that",
                                "skips around in purple socks.",
                                "Scourge of the seven seas.",
                                "^0099FFItem Drops^000000: Skel-Bone"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "8. Marionette",
                                "A monster reborn as a cursed doll",
                                "that is bound to strings attached",
                                "to wooden sticks.",
                                "^0099FFItem Drops^000000: Golden Hair, Trunk"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Sunken Ship 2F Monsters]^000000",
                                "9. Drake",
                                "A peg-legged, ghostly pirate",
                                "captain that takes its leisurely",
                                "time to attack the living.",
                                "^0099FFItem Drops^000000: Skel-Bone, White",
                                "Herb, Elunium"
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
                    match runtime::select_values(ctx, &[Val::from("1F:3F:Cancel")])? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "1. Poring",
                                "Small, pink monsters that are made",
                                "of a living gelatinous substance.",
                                "They're cute, and move by",
                                "bouncing.",
                                "^0099FFItem Drops^000000: Jellopy, Sticky Mucus,",
                                "Apple, Empty Bottle, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "2. Lunatic",
                                "Plump and shaggy monster shaped in",
                                "a Rabbit. However it won't give you a",
                                "'Bunny Band'.",
                                "^0099FFItem Drops^000000:",
                                "Clover, Feather, Carrot, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "3. Fabre",
                                "The weak and small larva of Creamy.",
                                "Although some say it is cute, the",
                                "author must disagree.",
                                "Whole-heartedly.",
                                "^0099FFItem Drops^000000: Fluff, Feather, Green",
                                "Herb, Clover"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "4. Creamy",
                                "A flying monster with beautiful",
                                "wings. It will escape by",
                                "teleporting if it thinks that it's",
                                "in grave danger.",
                                "^0099FFItem Drops^000000: Powder of Butterfly",
                                "Honey, Butterfly Wing, Flower"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "5. Pupa",
                                "Monster that is the pupal stage of",
                                "Fabre. It doesn't attack at all, so",
                                "it's easy to kill for Novices.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "6. Poporing",
                                "A light green Poring with the",
                                "Posion property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus,",
                                "Garlet, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "7. Rocker",
                                "A lazy grasshopper that loves to",
                                "play the violin, just like in",
                                "Aesop's fable.",
                                "^0099FFItem Drops^000000: Grasshopper's Leg,",
                                "Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "8. Bigfoot",
                                "Althought its name may be",
                                "misleading, Bigfoot is actually a",
                                "large bear. It walks like it owns",
                                "the forest, and it does.",
                                "^0099FFItem Drops^000000: Bear's Foot Skin,",
                                "Animal Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "9. Smokie",
                                "A chubby little raccoon that loves",
                                "nothing better than to scamper.",
                                "It's rumoured to use a magic leaf to",
                                "become invisible!",
                                "^0099FFItem Drops^000000: Raccoon Leaf, Animal",
                                "Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "10. Snake",
                                "Green Coloured Snake living in the",
                                "Forest or Desert. Not poisonous but",
                                "be careful.",
                                "^0099FFItem Drops^000000: Snake Scale, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "11. Wolf",
                                "Wild, roving wolves with blue",
                                "manes. They tend to attack as a",
                                "pack when even one of them is",
                                "threatened.",
                                "^0099FFItem Drops^000000: Wolf Claw, Meat,",
                                "Monster's Feed, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "12. Agriope",
                                "A segmented, millipede type monster",
                                "that will attack passerby with",
                                "poison.",
                                "^0099FFItem Drops^000000: Bug Leg, Zargon, Green",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "13. Agros",
                                "A monstrous spider that will attack",
                                "adventurers on sight. It's too big",
                                "for adventurers to squish with",
                                "their feet.",
                                "^0099FFItem Drops^000000: Cobweb, Scell, Bug Leg,",
                                "Green Herb, Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "14. Chonchon",
                                "Fly monsters that move with great",
                                "speed. Amazingly, they can heal in",
                                "the presense of fecal matter.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Fly",
                                "Wing"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "15. Horn",
                                "Although it looks fierce, it's",
                                "actually a peaceful insect. It",
                                "roams around fields with a",
                                "crunching sound.",
                                "^0099FFItem Drops^000000: Horn, Shell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "16. Hunter Fly",
                                "Winged insects covered in the blood",
                                "of innocents. It's incredibly",
                                "quick, as well as strong. Novices",
                                "must flee from this monster at all",
                                "cost.",
                                "^0099FFItem Drops^000000: Solid Shell, Zargon"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "17. Mantis",
                                "It wanders about fields, waving a",
                                "tiny fan. An entire martial arts",
                                "style is based on the movements of",
                                "this insect.",
                                "^0099FFItem Drops^000000: Mantis Scythe, Scell,",
                                "Solid Shell, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "18. Stainer",
                                "Tiny little insect with a splendid,",
                                "ladybug-like shell. It can sense",
                                "magic and will attack once a spell",
                                "begins casting.",
                                "^0099FFItem Drops^000000: Rainbow Shell, Garlet,",
                                "Shell, Solid Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "19. Side Winder",
                                "A dark coloured snake that hates",
                                "people. Be careful, and kill them",
                                "before they can poison you.",
                                "^0099FFItem Drops^000000: Shining Scale, Zargon,",
                                "Poisonous Canine, Snake Scale"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "20. Yoyo",
                                "Pink coloured Monkey. Not only do",
                                "they pick up everything dropped on the",
                                "ground, outrageously, but they are",
                                "nimble and cooperative, you must be",
                                "cautious of being attacked by a",
                                "group.",
                                "^0099FFItem Drops^000000: Yoyo Tail, Banana, Yellow",
                                "Herb, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "21. Caramel",
                                "An adorable porcupine with tiny",
                                "spiky quills. However, it gets",
                                "incredibly angry when touched.",
                                "^0099FFItem Drops^000000: Porcupine Quill,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "22. Steel Chonchon",
                                "Similar to Chonchon, but is yellow",
                                "and green. It picks up everything",
                                "from the ground, so be careful not",
                                "to drop items.",
                                "^0099FFItem Drops^000000: Garlet, Shell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "23. Coco",
                                "A small Squirrel with dark,",
                                "piercing eyes. It's always holding",
                                "an Acorn, and would be cute if it",
                                "didn't always have a look of utter",
                                "contempt.",
                                "^0099FFItem Drops^000000: Acorn, Fluff, Animal",
                                "Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "24. Dustiness",
                                "This flying monster has a high",
                                "dodge rate, so if you have low",
                                "attack accuracy, you may want to",
                                "leave it alone.",
                                "^0099FFItem Drops^000000: Moth Dust, Moth Wing",
                                "Insect Feeler, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "25. Martin",
                                "An adorable mole wearing a safety",
                                "helmet. He's deathly afraid of",
                                "cave-ins and occasionally stops to",
                                "cower in fear.",
                                "^0099FFItem Drops^000000: Mole Whiskers, Mole",
                                "Claw"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 1F Monsters]^000000",
                                "26. Savage",
                                "A wild boar that walks around,",
                                "grunting restlessly. It's rough",
                                "looking tusks make it hard to",
                                "believe it was cute as a baby.",
                                "^0099FFItem Drops^000000: Mane, Animal Skin"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "1. Poring",
                                "Small, pink monsters that are made",
                                "of a living gelatinous substance.",
                                "They're cute, and move by",
                                "bouncing.",
                                "^0099FFItem Drops^000000: Jellopy, Sticky Mucus,",
                                "Apple, Empty Bottle, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "2. Lunatic",
                                "Plump and shaggy monster shaped in",
                                "a Rabbit. However it won't give you a",
                                "'Bunny Band'.",
                                "^0099FFItem Drops^000000:",
                                "Clover, Feather, Carrot, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "3. Fabre",
                                "The weak and small larva of Creamy.",
                                "Although some say it is cute, the",
                                "author must disagree.",
                                "Whole-heartedly.",
                                "^0099FFItem Drops^000000: Fluff, Feather, Green",
                                "Herb, Clover"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "4. Creamy",
                                "A flying monster with beautiful",
                                "wings. It will escape by",
                                "teleporting if it thinks that it's",
                                "in grave danger.",
                                "^0099FFItem Drops^000000: Powder of Butterfly",
                                "Honey, Butterfly Wing, Flower"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "5. Pupa",
                                "Monster that is the pupal stage of",
                                "Fabre. It doesn't attack at all, so",
                                "it's easy to kill for Novices.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "6. Poporing",
                                "A light green Poring with the",
                                "Posion property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus,",
                                "Garlet, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "7. Rocker",
                                "A lazy grasshopper that loves to",
                                "play the violin, just like in",
                                "Aesop's fable.",
                                "^0099FFItem Drops^000000: Grasshopper's Leg,",
                                "Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "8. Bigfoot",
                                "Althought its name may be",
                                "misleading, Bigfoot is actually a",
                                "large bear. It walks like it owns",
                                "the forest, and it does.",
                                "^0099FFItem Drops^000000: Bear's Foot Skin,",
                                "Animal Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "9. Smokie",
                                "A chubby little raccoon that loves",
                                "nothing better than to scamper.",
                                "It's rumoured to use a magic leaf to",
                                "become invisible!",
                                "^0099FFItem Drops^000000: Raccoon Leaf, Animal",
                                "Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "10. Snake",
                                "Green Coloured Snake living in the",
                                "Forest or Desert. Not poisonous but",
                                "be careful.",
                                "^0099FFItem Drops^000000: Snake Scale, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "11. Wolf.",
                                "Wild, roving wolves with blue",
                                "manes. They tend to attack as a",
                                "pack when even one of them is",
                                "threatened.",
                                "^0099FFItem Drops^000000: Wolf Claw, Meat,",
                                "Monster's Feed, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "12. Agriope",
                                "A segmented, millipede type monster",
                                "that will attack passerby with",
                                "poison.",
                                "^0099FFItem Drops^000000: Bug Leg, Zargon, Green",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "13. Agros",
                                "A monstrous spider that will attack",
                                "adventurers on sight. It's too big",
                                "for adventurers to squish with",
                                "their feet.",
                                "^0099FFItem Drops^000000: Cobweb, Scell, Bug Leg,",
                                "Green Herb, Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "14. Horn",
                                "Although it looks fierce, it's",
                                "actually a peaceful insect. It",
                                "roams around fields with a",
                                "crunching sound.",
                                "^0099FFItem Drops^000000: Horn, Shell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "15. Hunter Fly",
                                "Winged insects covered in the blood",
                                "of innocents. It's incredibly",
                                "quick, as well as strong. Novices",
                                "must flee from this monster at all",
                                "cost.",
                                "^0099FFItem Drops^000000: Solid Shell, Zargon"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "16. Mantis",
                                "It wanders about fields, waving a",
                                "tiny fan. An entire martial arts",
                                "style is based on the movements of",
                                "this insect.",
                                "^0099FFItem Drops^000000: Mantis Scythe, Scell,",
                                "Solid Shell, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "17. Stainer",
                                "Tiny little insect with a splendid,",
                                "ladybug-like shell. It can sense",
                                "magic and will attack once a spell",
                                "begins casting.",
                                "^0099FFItem Drops^000000: Rainbow Shell, Garlet,",
                                "Shell, Solid Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "18. Side Winder",
                                "A dark coloured snake that hates",
                                "people. Be careful, and kill them",
                                "before they can poison you.",
                                "^0099FFItem Drops^000000: Shining Scale, Zargon,",
                                "Poisonous Canine, Snake Scale"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "19. Yoyo",
                                "Pink coloured Monkey. Not only do",
                                "they pick up everything dropped on the",
                                "ground, outrageously, but they are",
                                "nimble and cooperative, you must be",
                                "cautious of being attacked by a",
                                "group.",
                                "^0099FFItem Drops^000000: Yoyo Tail, Banana, Yellow",
                                "Herb, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "20. Caramel",
                                "An adorable porcupine with tiny",
                                "spiky quills. However, it gets",
                                "incredibly angry when touched.",
                                "^0099FFItem Drops^000000: Porcupine Quill,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "21. Steel Chonchon",
                                "Similar to Chonchon, but is yellow",
                                "and green. It picks up everything",
                                "from the ground, so be careful not",
                                "to drop items.",
                                "^0099FFItem Drops^000000: Garlet, Shell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "22. Coco",
                                "A small Squirrel with dark,",
                                "piercing eyes. It's always holding",
                                "an Acorn, and would be cute if it",
                                "didn't always have a look of utter",
                                "contempt.",
                                "^0099FFItem Drops^000000: Acorn, Fluff, Animal",
                                "Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "23. Dustiness",
                                "This flying monster has a high",
                                "dodge rate, so if you have low",
                                "attack accuracy, you may want to",
                                "leave it alone.",
                                "^0099FFItem Drops^000000: Moth Dust, Moth Wing",
                                "Insect Feeler, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "24. Martin",
                                "An adorable mole wearing a safety",
                                "helmet. He's deathly afraid of",
                                "cave-ins and occasionally stops to",
                                "cower in fear.",
                                "^0099FFItem Drops^000000: Mole Whiskers, Mole",
                                "Claw"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "25. Savage",
                                "A wild boar that walks around,",
                                "grunting restlessly. It's rough",
                                "looking tusks make it hard to",
                                "believe it was cute as a baby.",
                                "^0099FFItem Drops^000000: Mane, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "26. Savage Bebe",
                                "Tiny, pink baby Savage. It's",
                                "disheartening to know that it grows",
                                "up to become ugly-looking.",
                                "^0099FFItem Drops^000000: Animal Skin, Meat,",
                                "Arrow, Feather"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "27. Mastering",
                                "A giant Poring rarely seen in the",
                                "Morocc Desert, Mt. Mjolnir or the",
                                "Prontera Maze. It may be the Master",
                                "of Porings, but... it's still a",
                                "Poring",
                                "^0099FFItem Drops^000000: Apple, Apple Juice"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "28. Eclipse",
                                "Lord and Master of all Lunatics.",
                                "Its attacks may be weak, but it",
                                "has considerable defense.",
                                "^0099FFItem Drops^000000: Carrot, Glass Bead,",
                                "Milk, Carrot Juice"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Prontera Maze 3F Monsters]^000000",
                                "29. Baphomet",
                                "A horned goat-like beast that",
                                "wields an intimidating scythe with",
                                "incredible might. Sired countless",
                                "Baphomet Jrs.",
                                "^0099FFItem Drops^000000: Evil Horn,",
                                "Yggdrasilberry, Animal Skin,",
                                "Oridecon"
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
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn monster_encyclopedia_7pr(ctx: &Ctx) -> Script {
    monster_encyclopedia_7pr_body(ctx, Vec::new()).map(|_| ())
}

fn vending_guide_pront_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Vending Guide for Dummies]^000000",
        "So you want to open your own shop",
        "so that you can sell items to other",
        "players and make zeny?"
    ])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I do!:Close the book.")])?) == 1 {
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "First, only certain job classes can",
            "open vending shops. As a Merchant,",
            "Blacksmith or Alchemist, you must",
            "first learn ^009933Level 5 Enlarge Weight^000000",
            "^009933Limit^000000, and then learn the ^009933Pushcart^000000",
            "skill."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "When you have learned the Pushcart",
            "skill, you can go ahead and rent a",
            "Cart form a Kafra Employee. Once",
            "this cart is equipped, it won't",
            "disappear as long as you don't take",
            "it off."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "However, equipping a Cart reduces",
            "your movement speed. You can",
            "recover this movement speed by",
            "continuing to add skill points to",
            "the ^009933Pushcart^000000 skill."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "Now, this next part is very",
            "important. If you want to be able",
            "to sell what is inside your cart,",
            "you must learn ^009933Level 3 Pushcart^000000 so",
            "that youc an learn ^009933Vending^000000,",
            "allowing you to sell items inside",
            "your cart."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "Remember, only items inside your",
            "Cart can be sold with the Vending",
            "skill. Press the '^009933Alt^000000' and ^009933W^000000' keys",
            "to open the Cart Window labeled",
            "'Rent a Cart Item.'"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "You can drag and drop items from",
            "your Inventory Window into this",
            "Cart Window. You can also click the",
            "'^009933items^000000' button int he Equipment",
            "Window (^009933Alt^000000 + ^009933Q^000000) to open the Cart",
            "Window."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "Now use the Vending skill. Two",
            "windows will pop up. ^009933Available^000000",
            "^009933 items for Vending^000000 and ^009933 Vend a Shop^000000."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "Drag and drop items from the",
            "^009933Available items for Vending^000000",
            "Window into the ^009933Vend a Shop^000000 Window.",
            "Then, in the ^009933Vend a Shop^000000 window,",
            "you may set prices and name your",
            "shop."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "After confirming the items and",
            "prices, click the 'OK' button. The",
            "^009933My Shop^000000 window will appear.",
            "Congratulations, you are now",
            "vending your items!"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "When you close the ^009933My Shop^000000 window,",
            "your shop will be closed. You can",
            "check the prices of your items in",
            "the ^009933My Shop^000000 window, and your sales",
            "will be recorded in the chat",
            "window."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "When everything's sold out, the",
            "shop will automatically close."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^FF0000[Vending Guide for Dummies]^000000",
            "SuperNovices can learn all the",
            "skills needed to open vending",
            "shops, but none of the Kafra",
            "Employees will rent Carts to them.",
            "If only they could find somewhere",
            "to rent a cart..."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn vending_guide_pront(ctx: &Ctx) -> Script {
    vending_guide_pront_body(ctx, Vec::new()).map(|_| ())
}

fn blacksmith_guide_pront_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Blacksmith Guide for Dummies]^000000",
        "This is a useful guide detailing",
        "the process of Ore Refining and",
        "Weapon Crafting for Blacksmith job",
        "class characters."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Ore Refining.:Weapon Craft:Cancel.")])?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "^FF0000[Ore Refining for Dummies]^000000",
                "Rough ores, like Iron Ore, and",
                "rough enchanted stones can be",
                "refined to create a higher quality",
                "metal or stone. Refining rough",
                "materials requires a ^0099FFMini Furnace^000000."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Ore Refining for Dummies]^000000",
                "Several rough ores are also needed",
                "to create just one of a higher",
                "quality. When you think you have",
                "enough rough ores or stones of the",
                "same kind, double-click the Mini",
                "Furnace item in the Inventory",
                "window."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Ore Refining for Dummies]^000000",
                "If you have the available materials",
                "and creation skills, a list of",
                "enchanted stones or metals that you",
                "can create will appear in a new",
                "window labeled ^0099FFItem List you can^000000",
                "^0099FFcraft^000000."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Ore Refining for Dummies]^000000",
                "However, if you don't have the",
                "necessary skills or materials, you",
                "will receive a message stating '^0099FFYou^000000",
                "^0099FFcan't Create Items yet^000000'."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Ore Refining for Dummies]^000000",
                "Remember that there is a",
                "possibility that the refining",
                "process may fail. Also, be aware",
                "that a Mini Furnace will be used",
                "each time you double-click it,",
                "regardless of the end result."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "To create weapons, you must first",
                "learn the appropriate smithing",
                "skills, depending on the weapon you",
                "wish to create. The following is a",
                "list of Blacksmith weapon creation",
                "skills."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "^0099FFSmith Dagger^000000",
                "^0099FFSmith Sword^000000",
                "^0099FFSmith Two-handed Sword^000000",
                "^0099FFSmith Axe^000000",
                "^0099FFSmith Mace^000000",
                "^0099FFSmith Spear^000000",
                "^0099FFSmith Knucklebrace^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "Every Weapon requires ^0099FFSteel^000000 and the",
                "consumption of one ^0099FFHammer^000000."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "When you double-click on a Hammer",
                "item in the Inventory Window, a new",
                "window labeled ^0099FFItem List you can^000000",
                "^0099FFcraft^000000 will appear."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "A list of weapons that you are",
                "currently able to craft will",
                "appear. Clicking a Weapon in that",
                "list will show the items required",
                "for creation."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "In that list window, there are 3",
                "sockets into which you can insert",
                "additional items, such as Enchanted",
                "Stones or Star Crumbs, which enable",
                "you to enhance the smithed weapon."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "Afterwards, cick the 'OK' button",
                "to confirm that you want to create",
                "the selected item. The materials",
                "required to create the weapon will",
                "be automatically consumed from your",
                "inventory."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000[Weapon Craft for Dummies]^000000",
                "The chance of smithing success will",
                "depend on your character stats and",
                "skills, and other factors. If the",
                "smithing fails, any items used to",
                "create the new weapon will still be",
                "consumed. Good luck!"
            ])?;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn blacksmith_guide_pront(ctx: &Ctx) -> Script {
    blacksmith_guide_pront_body(ctx, Vec::new()).map(|_| ())
}
