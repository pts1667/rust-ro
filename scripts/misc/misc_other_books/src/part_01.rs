use script_sdk_2::{Ctx, Script, Stop, Val, args, runtime};

fn monster_encyclopedia_prt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Monster Encyclopedia]^000000",
        "This is a Monster Encyclopedia",
        "containing information on Water,",
        "Wind and Ghost property monsters."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Water Property Monsters:Wind Property Monsters:Spritual Property Monsters:Cancel",
            )],
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
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
                                "1. Plankton",
                                "Even though they seem",
                                "insignificantly small, be careful",
                                "not to step on them. Plankton are",
                                "light and can drift on the water.",
                                "^0099FFItem Drops^000000: Single Cell, Garlet,",
                                "Sticky Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
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
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
                                "3. Hydra",
                                "Vegetable Monstesr that live near",
                                "water or in the deep sea that",
                                "attack using tentacles. As a group,",
                                "they're a pain in the ass.",
                                "^0099FFItem Drops^000000: Tentacle, Sticky Mucus,",
                                "Meat."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
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
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
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
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
                                "6. Cornutus",
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
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
                                "7. Magnolia",
                                "Cute looking creatures that appear",
                                "as large frying pans cooking an",
                                "egg. They mercilessly spank all",
                                "that oppose them.",
                                "^0099FFItem Drops^000000: Jellopy, Garlet, Scell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 1: Small Water Monsters]^000000",
                                "8. Marine Sphere",
                                "Strange, round-shaped monsters that",
                                "pulse with destructive energy.",
                                "Gathering their Detonators may be",
                                "useful for Alchemists.",
                                "^0099FFItem Drops^000000: Tendon, Detonator"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
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
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "2. Roda Frog",
                                "Amphibious frogs that have an",
                                "annoying croak. In some countries,",
                                "their legs are a delicacy.",
                                "^0099FFItem Drops^000000: Sticky Webfoot, Spawn,",
                                "Green Herb, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "3. Spore",
                                "Mushroom-like monsters that utilise",
                                "mycelial reproduction. Usually live",
                                "in forests or dungeons.",
                                "^0099FFItem Drops^000000: Spore, Red Herb, Blue",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "4. Goblin",
                                "Small, mask wearing monsters that",
                                "viciously attack passerby's. There",
                                "seem to be different types that use",
                                "different weapons.",
                                "^0099FFItem Drops^000000: Yellow Herb, Red",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "5. Thara Frog",
                                "Red Frogs that are much stronger",
                                "than the green Roda Frogs. They",
                                "also produce an annoying croaking",
                                "noise.",
                                "^0099FFItem Drops^000000: Spawn, Scell, Sticky",
                                "Webfoot"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "6. Phen",
                                "A blue fish with a pointed nose and",
                                "sad, incredibly sad, vacant eyes.",
                                "^0099FFItem Drops^000000: Fish Tail, Sharp Scale,",
                                "Meat, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "7. Marse",
                                "A miniature squid with miniature",
                                "tentacles. How it moves through",
                                "water with those tiny things is",
                                "still a scientific mystery.",
                                "^0099FFItem Drops^000000: Squid Ink, Tentacle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "8. Obeaune",
                                "A female Mermaid that attacks with",
                                "its wild, flowing hair. Whether or",
                                "not its male version is Merman is",
                                "still under debate.",
                                "^0099FFItem Drops^000000: Heart of Mermaid, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "9. Sohee",
                                "A female Ghost that harbours a deep",
                                "grudge. Although she is usually",
                                "crying, she can become fierce upon",
                                "encountering the living.",
                                "^0099FFItem Drops^000000: Black Hair, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "10. Marc",
                                "A proud looking Sea Horse that sort",
                                "of looks like a dragon.",
                                "Unfortunately, you can't ride it.",
                                "^0099FFItem Drops^000000: Gill, Fin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 2: Medium Water Monsters]^000000",
                                "11. Deviace",
                                "Fish monster with a big mouth",
                                "attached to a suction cup. Small,",
                                "strong, and sort of looks like a",
                                "watermelon.",
                                "^0099FFItem Drops^000000: Ancient Tooth, Ancient",
                                "Lips"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 3: Great Water Monsters]^000000",
                                "1. Ambernite",
                                "A snail shaped monster, it is",
                                "highly strong offense and defense.",
                                "However, it is incredibly slow like",
                                "all other snails.",
                                "^0099FFItem Drops^000000: Snail's Shell,",
                                "Garlet, Shell, Solid Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 3: Great Water Monsters]^000000",
                                "2. Sword Fish",
                                "Fish Monster with a sharp, long nose",
                                "that's just like a sword. Although it",
                                "has googly eyes, it's a dangerous",
                                "monster. Why wasn't it named Sword",
                                "Nose Fish?",
                                "^0099FFItem Drops^000000: Sharp Scale, Gill"
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
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "1. Chonchon",
                                "Fly monsters that move with great",
                                "speed. Amazingly, they can heal in",
                                "the presense of fecal matter.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Fly",
                                "Wing"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "2. Hornet",
                                "Usually benign, they will attack in",
                                "groups if one of them is harmed.",
                                "^0099FFItem Drops^000000: Bee Sting, Jellopy,",
                                "Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "3. Creamy",
                                "A flying monster with beautiful",
                                "wings. It will escape by",
                                "teleporting if it thinks that it is",
                                "in grave danger.",
                                "^0099FFItem Drops^000000: Powder of Butterfly,",
                                "Honey, Butterfly Wing, Flower"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "4. Stainer",
                                "Tiny little insect with a splended,",
                                "ladybug-like shell. It can sense",
                                "magic and will attack once a spell",
                                "begins casting.",
                                "^0099FFItem Drops^000000: Rainbow Shell, Garlet",
                                "Shell, Solid Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "5. Steel Chonchon",
                                "Similar to Chonchon, but is yellow",
                                "and green. It picks up everything",
                                "from the ground, so be careful not",
                                "to drop items.",
                                "^0099FFItem Drops^000000: Garlet, Shell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "6. Dustiness",
                                "This flying monsters has a high",
                                "dodge rate, so if you have low",
                                "attack accuracy, you may want to",
                                "leave it alone.",
                                "^0099FFItem Drops^000000: Moth Dust, Moth Wing",
                                "Insect Feeler, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 4: Small Wind Monsters]^000000",
                                "7. Hunter Fly",
                                "Winged insect covered in the blood",
                                "of innocents. It's incredibly",
                                "quick, as well as strong. Novices",
                                "must flee from this monster at all",
                                "cost.",
                                "^0099FFItem Drops^000000: Solid Shell, Zargon"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 5: Medium Wind Monsters]^000000",
                                "1. Condor",
                                "A bald, funny looking vulture-like",
                                "bird. They tend to attack in a",
                                "group if one of them is",
                                "threatened.",
                                "^0099FFItem Drops^000000: Talon, Arrow, Meat,",
                                "Feather of Birds"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 5: Medium Wind Monsters]^000000",
                                "2. Cobold the 1st",
                                "A monster looks like a baby wolf,",
                                "but it is smart enough to make and",
                                "use tools. Although Kobolds are",
                                "cute, they're actually quite",
                                "hostile.",
                                "^0099FFItem Drops^000000: Blue Hair, Zargon,",
                                "Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 5: Medium Wind Monsters]^000000",
                                "3. Petite",
                                "A tiny, cute flying Dragon. There",
                                "is another kind of Petite that",
                                "walks, but it is of the Earth",
                                "property.",
                                "^0099FFItem Drops^000000: Dragon Canine, Dragon",
                                "Tail, Zargon"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 6: Great Wind Monsters]^000000",
                                "1. Joker",
                                "A large, enchanted playing card. If",
                                "you don't have good attack",
                                "accuracy, the stakes are against",
                                "you when fighting Joker.",
                                "^0099FFItem Drops^000000: High Heels"
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
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            'l6: loop {
                if !(true) {
                    break 'l6;
                }
                'b6: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 7: Small Ghost Monsters]^000000",
                                "1. Whisper",
                                "A piece of living fabric that gives",
                                "off spooky vibes. Sometimes, it",
                                "likes to turn invisible...",
                                "^0099FFItem Drops^000000: Fabric"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 7: Small Ghost Monsters]^000000",
                                "2. Marionette",
                                "A monster reborn as a cursed doll",
                                "that is bound to strings attached",
                                "to wooden sticks.",
                                "^0099FFItem Drops^000000: Golden Hair, Trunk"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 8: Medium Ghost Monsters]^000000",
                                "1. Eggyra",
                                "A weird, robotic egg looking thing",
                                "that waddles when it walks. No one",
                                "knows where these things come",
                                "from.",
                                "^0099FFItem Drops^000000: Scell, Sticky Mucus,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 9: Great Ghost Monsters]^000000",
                                "1. Nightmare",
                                "A ghostly horse that radiates a",
                                "violet aura of evil.",
                                "^0099FFItem Drops^000000: Horseshoe, Blue Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 9: Great Ghost Monsters]^000000",
                                "2. Medusa",
                                "Monster with hair composed of",
                                "snakes. It is rumoured to turn",
                                "people into stone if they look into",
                                "her eyes.",
                                "^0099FFItem Drops^000000: Medusa Head, Horrendous",
                                "Hair, White Herb"
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

pub fn monster_encyclopedia_prt(ctx: &Ctx) -> Script {
    monster_encyclopedia_prt_body(ctx, Vec::new()).map(|_| ())
}

fn monster_encyclopedia_2pr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Monster Encyclopedia]^000000",
        "This is a Monster Encyclopedia",
        "containing information on Earth,",
        "Fire and Neutral property",
        "monsters."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Earth Property Monsters:Fire Property Monsters:Neutral Property Monsters:Cancel",
            )],
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
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "1. Fabre",
                                "The weak and small larva of Creamy.",
                                "Although some say it is cute, the",
                                "author must disagree.",
                                "Whole-heartedly.",
                                "^0099FFItem Drops^000000: Feather, Fluff, Green",
                                "Herb, Clover"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "2. Pupa",
                                "Monster that is the pupal stage of",
                                "Fabre. It doesn't attack at all, so",
                                "it's easy to kill for Novices.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
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
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "4. Savage Bebe",
                                "Tiny, pink baby Savage. It's",
                                "disheartening to know that it grows",
                                "up to become ugly-looking.",
                                "^0099FFItem Drops^000000: Animal Skin, Meat,",
                                "Arrow, Feather"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "5. Andre",
                                "A kind of worker ant, Andres are",
                                "yellow, very diligent and gather",
                                "everything in sight for the Queen",
                                "Ant.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "6. Coco",
                                "A small Squirrel with dark",
                                "piercing eyes. It's always holding",
                                "an Acorn, and would be cute if it",
                                "didn't always have a look of utter",
                                "contempt.",
                                "^0099FFItem Drops^000000: Acorn, Fluff, Animal",
                                "Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "7. Piere",
                                "Pieres are green worker ants that",
                                "are subtly different than Andres.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "8. Smokie",
                                "A chubby little raccoon that loves",
                                "nothing better than to scamper.",
                                "It's rumored to use a magic leaf to",
                                "become invisible!",
                                "^0099FFItem Drops^000000: Raccoon Leaf, Animal",
                                "Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "9. Deniro",
                                "Red worker ants that live to serve",
                                "the Queen Ant. If there's anything",
                                "on the ground, they'll pick it up.",
                                "^0099FFItem Drops^000000: Worm Peeling, Garlet,",
                                "Sticky Mucus, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "10. Yoyo",
                                "A naughty monkey that picks up",
                                "anything from the ground. They're",
                                "very quick and will gang up on you",
                                "if you attack just one of them.",
                                "^0099FFItem Drops^000000: Yoyo Tail, Banana,",
                                "Yellow Herb, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "11. Vitata",
                                "Plump worker ants that heal the",
                                "other ants in their colony. When",
                                "killed their bodies leak... honey?",
                                "^0099FFItem Drops^000000: Worm Peeling, Scell,",
                                "Honey"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "12. Caramel",
                                "An adorable porcupine with tiny",
                                "spiky quills. However, it gets",
                                "incredibly angry when touched.",
                                "^0099FFItem Drops^000000: Porcupine Quill,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 10: Small Earth Monsters]^000000",
                                "13. Giearth",
                                "An elderly pixie that wanders caves",
                                "to gather ores. They're incredible",
                                "chain smokers.",
                                "^0099FFItem Drops^000000: Old Pixie's Moustache"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "1. Willow",
                                "Creature reborn from an old tree.",
                                "Its features and the sounds it",
                                "makes are incredibly eerie.",
                                "^0099FFItem Drops^000000: Tree Root, Trunk, Red",
                                "Herb, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "2. Rocker",
                                "A lazy grasshopper that loves to",
                                "play the violin, just like in",
                                "Aesop's fable.",
                                "^0099FFItem Drops^000000: Grasshopper's Leg,",
                                "Jellopy"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "3. Mandragora",
                                "Although it stays in the same",
                                "place, it can attack passerby from",
                                "a distance using underground",
                                "stalks.",
                                "^0099FFItem Drops^000000: Stem, Green Herb,",
                                "Shoot"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "4. Wolf",
                                "Wild, roving wolves with blue",
                                "manes. They tend to attack as a",
                                "pack when even one of them is",
                                "threatened.",
                                "^0099FFItem Drops^000000: Wolf Claw, Meat,",
                                "Monster's Feed, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "5. Snake",
                                "Green snake that lives in the",
                                "forests and deserts. They're not",
                                "poisonous, but their bites still",
                                "hurt.",
                                "^0099FFItem Drops^000000: Snake Scale, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "6. Horn",
                                "Although it looks fierce, it's",
                                "actually a peaceful insect. It",
                                "roams around fields with a",
                                "crunching sound.",
                                "^0099FFItem Drops^000000: Horn, Shell, Solid",
                                "Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "7. Orc Warrior",
                                "A warrior of the proud race of",
                                "Orcs. At one time, Orcs and humans",
                                "were allies, but now they are",
                                "bitter enemies.",
                                "^0099FFItem Drops^000000: Orcish Voucher"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "8. Hode",
                                "A huge earthworm that usually hides",
                                "under the ground. It can usually be",
                                "found in the desert.",
                                "^0099FFItem Drops^000000: Earthworm Peeling,",
                                "Sticky Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "9. Mantis",
                                "It wanders about fields, waving a",
                                "tiny fan. An entire martial arts",
                                "style is based on the movements of",
                                "this insect.",
                                "^0099FFItem Drops^000000: Mantis Scythe, Scell",
                                "Solid Shell, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "10. Savage",
                                "A wild boar that walks around,",
                                "grunting restlessly. Its rough",
                                "looking tusks make it hard to",
                                "believe it was cute as a baby.",
                                "^0099FFItem Drops^000000: Mane, Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 11: Medium Earth Monsters]^000000",
                                "11. Petite",
                                "Cute, walking Dragon. There is",
                                "another kind of Petite that flys,",
                                "but it is of the Wind property.",
                                "^0099FFItem Drops^000000: Dragon Canine, Dragon",
                                "Tail, Zargon"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 12: Great Earth Monsters]^000000",
                                "1. Worm Tail",
                                "A strange monster that uses",
                                "whiping attacks with a tail that",
                                "looks like a blade of grass.",
                                "^0099FFItem Drops^000000: Pointed Scale, Yellow",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 12: Great Earth Monsters]^000000",
                                "2. Muka",
                                "Cute Cactus commonly found in the",
                                "desert. It tries to threaten",
                                "passerby with its growls, but its",
                                "noises are too funny to be scary.",
                                "^0099FFItem Drops^000000: Cactus Needle, Empty",
                                "Bottle, Green Herb, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 12: Great Earth Monsters]^000000",
                                "3. Bigfoot",
                                "Although its name may be",
                                "misleading, Bigfoot is actually a",
                                "large bera. It walks like it owns",
                                "the forest, and it does.",
                                "^0099FFItem Drops^000000: Bear's Foot Skin,",
                                "Animal Skin, Sweet Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 12: Great Earth Monsters]^000000",
                                "4. Flora",
                                "A man-eating plant. Its parts are",
                                "considered useful for Alchemists to",
                                "make monsters of their own...",
                                "^0099FFItem Drops^000000: Maneater Blossom, Stem"
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
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 13: Small Fire Monsters]^000000",
                                "1. Picky",
                                "Cute little chick that may grow up",
                                "to be a Peco Peco, unless, of",
                                "course, you kill it.",
                                "^0099FFItem Drops^000000: Feather of Birds,",
                                "Feather, Red Herb, Milk"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 13: Small Fire Monsters]^000000",
                                "2. Baby Desert Wolf",
                                "A Baby Desert Wolf that tires to",
                                "threaten adventurers with its cute,",
                                "little yelps.",
                                "^0099FFItem Drops^000000: Animal Skin, Meat"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 13: Small Fire Monsters]^000000",
                                "3. Horong,",
                                "An eerie-looking, violent fireball.",
                                "It's useless to use hiding skills",
                                "near this vengeful spirit.",
                                "^0099FFItem Drops^000000: Stone Heart, Zargon,",
                                "Fire Arrow"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "1. Drops",
                                "The desert version of the Poring",
                                "that has a pale orange colour. It",
                                "seems a tiny bit stronger than",
                                "Poring, though.",
                                "^0099FFItem Drops^000000: Jellopy, Sticky Mucus,",
                                "Apple, Empty Bottle, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "2. Elder Willow",
                                "The elderly version of the Willow",
                                "monster. It's red and can even use",
                                "some magic.",
                                "^0099FFItem Drops^000000: Resin, Trunk, Sweet",
                                "Potato"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "3. Metaller",
                                "The evolved form of Rocker. It is",
                                "dim brown and lives in the desert.",
                                "This cricket will pick items up",
                                "from the ground.",
                                "^0099FFItem Drops^000000: Red Blood,",
                                "Grasshopper's Leg, Scell, Shell"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "4. Zerom",
                                "An undead slave. Sadly, not even",
                                "death will bring peace to the",
                                "abusive hours of labor Zerom",
                                "suffers from his Pharaoh.",
                                "^0099FFItem Drops^000000: Panties"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "5. Scorpion",
                                "Scorpions can be found in areas",
                                "where there is desert. It has a",
                                "beautiful colour, but can be",
                                "dangerous.",
                                "^0099FFItem Drops^000000: Scorpion Tail, Green",
                                "Herb, Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "6. Desert Wolf",
                                "Wolves in the desert are much",
                                "stronger than those living in the",
                                "forset. If you strike one, you'll",
                                "have to deal with the whole pack.",
                                "Item Drops; Animal Skin, Mink",
                                "Coat, Meat, Wolf Claw"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "7. Frilldora",
                                "Lizard with a frilly, fan-like",
                                "neck. Although it looks rediculous,",
                                "it's actually pretty strong.",
                                "^0099FFItem Drops^000000: Frill, Reptile Tongue,",
                                "Red Herb, Zargon"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "8. Cobold the 3rd",
                                "A blue, wolf-like monster that is",
                                "amazingly cuddly. However, all",
                                "Kobolds have sworn to hate humans.",
                                "There are different kinds of",
                                "Kobolds that use different",
                                "weapons.",
                                "^0099FFItem Drops^000000: Blue Hair, Zargon,",
                                "Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 14: Medium Fire Monsters]^000000",
                                "9. Jakk",
                                "A spooky, Pumpkin-headed monster",
                                "that dresses in a slick formal",
                                "suit. It's been known to invade",
                                "Prontera on St. Hallow's Eve in the",
                                "past.",
                                "^0099FFItem Drops^000000: Jack'o'Pumpkin, Zargon"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 15: Great Fire Monsters]^000000",
                                "1. Peco Peco",
                                "Nowadays peco peco is popular as a",
                                "vehicle for Knights and Crusaders.",
                                "They live in the Desert or Forest",
                                "and will also attack in packs if",
                                "one of them is threatened.",
                                "^0099FFItem Drops^000000: Bill of Birds, Yellow",
                                "Herb, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 15: Great Fire Monsters]^000000",
                                "2. Marduk",
                                "A gangly wizard of darkness. Look",
                                "out, it knows magic!",
                                "^0099FFItem Drops^000000: Flame Heart"
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
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            'l6: loop {
                if !(true) {
                    break 'l6;
                }
                'b6: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 16: Small Neutral Monsters]^000000",
                                "1. Lunatic",
                                "A shaggy monster that looks kind of",
                                "like a rabbit. Although some may",
                                "think it's cute, the author",
                                "believes it to be absolutely",
                                "hideous. Perhaps that is because of",
                                "his alleriges.",
                                "^0099FFItem Drops^000000: Clover, Feather,",
                                "Carrot, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 16: Small Neutral Monsters]^000000",
                                "2. Peco Peco Egg",
                                "The egg of a Peco Peco. It's small,",
                                "and defenseless, making it a",
                                "perfect target for Novices.",
                                "^0099FFItem Drops^000000: Shell, Red Herb,",
                                "Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 16: Small Neutral Monsters]^000000",
                                "3. Ant Egg",
                                "An Ant Egg that is also",
                                "defenseless. Some of them actually",
                                "hatch, though.",
                                "^0099FFItem Drops^000000: Shell, Jellopy, Sticky",
                                "Mucus, Empty Bottle"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 16: Small Neutral Monsters]^000000",
                                "4. Baby Thief Bug",
                                "Dirty, filthy Creatures that tend",
                                "to work in groups. Whatever you do,",
                                "don't let them pollute the Prontera",
                                "Culvert, it'd be a disaster!",
                                "^0099FFItem Drops^000000: Worm Peeling, Red Herb,",
                                "Jellopy"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args!["^FF0000[Vol. 17: Medium Neutral Monsters]^000000", "^0099FF...^000000"])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 17: Medium Neutral Monsters]^000000",
                                "^0099FF...^000000",
                                "^0099FF......^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^0099FFSome pages seem to have been ripped^000000",
                                "^0099FFout of this book, and replaced with^000000",
                                "^0099FFpinups of the Isis monster. It^000000",
                                "^0099FFlooks like you'll need to learn^000000",
                                "^0099FFabout Neutral, medium sizd^000000",
                                "^0099FFmonsters on your own.^000000"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 18: Great Neutral Monsters]^000000",
                                "1. Golem",
                                "A being of living stone that has",
                                "been enchanted with black magic. It",
                                "can recognise spell casting, but",
                                "moves incredibly slowly due to its",
                                "weight problem.",
                                "^0099FFItem Drops^000000: Scell"
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

pub fn monster_encyclopedia_2pr(ctx: &Ctx) -> Script {
    monster_encyclopedia_2pr_body(ctx, Vec::new()).map(|_| ())
}

fn monster_encyclopedia_3pr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FF0000[Monster Encyclopedia]^000000",
        "This is a Monster Encyclopedia",
        "containing information on Dark,",
        "Poison and Undead monsters."
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Dark Property Monsters:Poison Property Monsters:Undead Property Monsters:Cancel",
            )],
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
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 19: Small Dark Monsters]^000000",
                                "1. Thief Bug Egg",
                                "A filthy egg from a filthy Thief",
                                "Bug. They make horrible omelets.",
                                "^0099FFItem Drops^000000: Chrysalis, Sticky",
                                "Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 19: Small Dark Monsters]^000000",
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
                                "^FF0000[Vol. 19: Small Dark Monsters]^000000",
                                "3. Tarou",
                                "A tiny, little white mouse. Its",
                                "squeaks can be heard in the Dead",
                                "Pit and the Prontera Culvert.",
                                "^0099FFItem Drops^000000: Rat Tail, Animal",
                                "Skin, Feather, Monster's Feed"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 19: Small Dark Monsters]^000000",
                                "4. Drainliar",
                                "A blood red bat that's much",
                                "stronger than Familiar. It also",
                                "tends to pursue any human it",
                                "finds.",
                                "^0099FFItem Drops^000000: Tooth of Bat, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 19: Small Dark Monsters]^000000",
                                "5. Dokkaebi",
                                "A traditional Korean demon with the",
                                "power to generate wealth. Using",
                                "Mammonite is no big deal to them.",
                                "^0099FFItem Drops^000000: Dokkaebi Horn"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 19: Small Dark Monsters]^000000",
                                "6. Deviruchi",
                                "A miniature demon that repeatedly",
                                "stabs umans with its pitchfork.",
                                "It's cute, but nonetheless a true",
                                "fiend of darkness.",
                                "^0099FFItem Drops^000000: Little Evil Horn,",
                                "Little Evil Wing, Zargon"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 20: Medium Dark Monsters]^000000",
                                "1. Female Thief Bug",
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
                                "^FF0000[Vol. 20: Medium Dark Monsters]^000000",
                                "2. Male Thief Bug",
                                "A large, green insect, the Male",
                                "Thief Bug is considerably powerful.",
                                "They're also very aggressive",
                                "towards humans.",
                                "^0099FFItem Drops^000000: Worm Peeling, Red Herb,",
                                "Jellopy, Garlet, Insect Feeler,",
                                "Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 20: Medium Dark Monsters]^000000",
                                "3. Matyr",
                                "A hound saturated with evil. It's",
                                "always sleeping, but springs to",
                                "action after smelling an",
                                "adventurer.",
                                "^0099FFItem Drops^000000: Monster's Feed,",
                                "Animal Skin"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 20: Medium Dark Monsters]^000000",
                                "4. Zenorc",
                                "A dishonourable Orc whose body has",
                                "been cursed. They continue their",
                                "shameful ways by looting items that",
                                "have been dropped to the ground.",
                                "^0099FFItem Drops^000000: Zenorc's Fang, Sticky",
                                "Mucus, Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 20: Medium Dark Monsters]^000000",
                                "5. Requiem",
                                "An ancient slave that carries a",
                                "heavy coffin on its back. Weary",
                                "from its labour, Requiem simply",
                                "collapses, hoping the coffing will",
                                "hit its mark, when attacking",
                                "^0099FFItem Drops^000000: Old Blue Box"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 20: Medium Dark Monsters]^000000",
                                "6. Bathory",
                                "A wart-nosed Witch wearing bunny",
                                "boxers that will attack anything",
                                "prettier that her. In other words,",
                                "she attacks everyone.",
                                "^0099FFItem Drops^000000: Witch Starsand"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 21: Great Dark Monsters]^000000",
                                "1. Isis",
                                "A monster that is half serpent and",
                                "half woman, as well as one of",
                                "Osiris' trusted champions.",
                                "^0099FFItem Drops^000000: Scale Skin, Shining",
                                "Scale"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 21: Great Dark Monsters]^000000",
                                "2. Raydric",
                                "The soul of a castle guard bound to",
                                "a living suit of armour through a",
                                "curse.",
                                "^0099FFItem Drops^000000: Elunium, Chivalry",
                                "Emblem"
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
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args!["^FF0000[Vol. 22: Small Poison Monsters]^000000", "^0099FF...^000000"])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 22: Small Posion Monsters]^000000",
                                "^0099FF...^000000",
                                "^0099FF......^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^0099FFThere are^000000",
                                "^0099FFPoring stickers^000000",
                                "^0099FFall over these pages!^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^0099FFIt looks like you'll have to learn^000000",
                                "^0099FFabout small, poisonous monsters all^000000",
                                "^0099FFon your own.^000000"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 23: Medium Posion Monsters]^000000",
                                "1. Poporing",
                                "A light green Poring with the",
                                "Poison property. It's much stronger",
                                "than Poring, but still moves by",
                                "means of bouncing.",
                                "^0099FFItem Drops^000000: Sticky Mucus, Garlet,",
                                "Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 23: Medium Posion Monsters]^000000",
                                "2. Poison Spore",
                                "A black capped mushroom. It attacks",
                                "adventurers in fear of being eaten,",
                                "despite being poisonous and not",
                                "delicious.",
                                "^0099FFItem Drops^000000: Spore, Green Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 23: Medium Posion Monsters]^000000",
                                "3. Cobold the 2nd",
                                "A small, wolf-like monster that's",
                                "intelligent enough to use weapons.",
                                "Look out, though, he's pretty mean",
                                "for a little guy.",
                                "^0099FFItem Drops^000000: Blue Hair, Zargon,",
                                "Yellow Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 23: Medium Posion Monsters]^000000",
                                "4. Side Winder",
                                "A dark coloured snake that hates",
                                "people. Be careful, and kill them",
                                "before they can poison you.",
                                "^0099FFItem Drops^000000: Shining Scale, Zargon,",
                                "Poisonous Canine, Snake Scale"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 24: Great Poison Monsters]^000000",
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
                                "^FF0000[Vol. 24: Great Poison Monsters]^000000",
                                "2. Argriope",
                                "A segmented, millipede type monster",
                                "that will attack passerby with",
                                "poison.",
                                "^0099FFItem Drops^000000: Bug Leg, Zargon, Green",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 24: Great Poison Monsters]^000000",
                                "3. Myst",
                                "A strange, monster made of mist",
                                "that is attached to a phantom",
                                "window.",
                                "^0099FFItem Drops^000000: Trunk, Gas Mask"
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
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            'l6: loop {
                if !(true) {
                    break 'l6;
                }
                'b6: {
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Small Sized Monsters:Medium Sized Monsters:Great Sized Monsters:Cancel")],
                    )? {
                        1 => {
                            ctx.lines(args!["^FF0000[Vol. 25: Small Undead Monsters]^000000", "^0099FF...^000000"])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 25: Small Undead Monsters]^000000",
                                "^0099FF...^000000",
                                "^0099FF......^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^0099FF...!^000000",
                                "^0099FFSome stupid kid scribbled pictures^000000",
                                "^0099FFall over this chapter! It looks^000000",
                                "^0099FFlike you'll have to learn about^000000",
                                "^0099FFsmall Undead monsters^000000",
                                "^0099FFon your very own.^000000"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "1. Zombie",
                                "An innocent human that has been",
                                "raised from the dead through black",
                                "magic.",
                                "^0099FFItem Drops^000000: Decayed Nail, Sticky",
                                "Mucus, Horrendous Mouth"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "2. Megalodon",
                                "An animated fish skeleton that",
                                "roams the seas. Although it looks",
                                "scary, it's actually benign.",
                                "^0099FFItem Drops^000000: Stinky Scale,",
                                "Skel-Bone"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "3. Orc Zombie",
                                "Orcs that have risen back from the",
                                "dead. The honourable fighting spirit",
                                "of the Orc Warrior never dies!",
                                "^0099FFItem Drops^000000: Orc Claw, Sticky Mucus"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "4. Pirate Skel",
                                "A topless pirate skeleton that",
                                "skips around in purple socks.",
                                "Scourge of the seven seas.",
                                "^0099FFItem Drops^000000: Skel-Bone"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "5. Orc Skeleton",
                                "The skeleton of an Orc that has",
                                "been brought back to life. Even in",
                                "death, Orcs continue to do battle.",
                                "^0099FFItem Drops^000000: Orc's Fang, Green",
                                "Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "6. Soldier Skeleton",
                                "A skeleton soldier that wields two",
                                "swords at once. He must have been a",
                                "badass when he was alive.",
                                "^0099FFItem Drops^000000: Skel-Bone, Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "7. Munak",
                                "A beautiful zombie that seems to be",
                                "linked to Bongun somehow.",
                                "^0099FFItem Drops^000000: Daenggie, Munak Turban"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "8. Skel Worker",
                                "A reanimated skeleton of a miner",
                                "that has died without receiving its",
                                "severance pay.",
                                "^0099FFItem Drops^000000: Iron, Lantern"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "9. Archer Skeleton",
                                "Despite not having actual eyes,",
                                "Archer Skeletons have great aim.",
                                "^0099FFItem Drops^000000: Skel-Bone, Fire Arrow,",
                                "Red Herb"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "10. Mummy",
                                "A walking corpse covered with",
                                "bandages. It probably used to be",
                                "beautiful once.",
                                "^0099FFItem Drops^000000: Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "11. Verit",
                                "A mummified dog that will pick up",
                                "anything dropped to the ground.",
                                "Although it's a zombie, it seems",
                                "happy to be alive again.",
                                "^0099FFItem Drops^000000: Immortal Heart, Zargon",
                                "Rotten Bandage"
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^FF0000[Vol. 26: Medium Undead Monsters]^000000",
                                "12. Ghoul",
                                "Similar to a Zombie, but Ghouls are",
                                "green and much stronger. Its",
                                "retching is offensive in more ways",
                                "than one.",
                                "^0099FFItem Drops^000000: Horrendous Mouth"
                            ])?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines(args![
                                "^FF0000[Vol. 27: Great Undead Monsters]^000000",
                                "1. Evil Druid",
                                "A flamboyantly evil druid. It's",
                                "always using a floating spellbook",
                                "that crackles with energy to cause",
                                "misery to adventurers",
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

pub fn monster_encyclopedia_3pr(ctx: &Ctx) -> Script {
    monster_encyclopedia_3pr_body(ctx, Vec::new()).map(|_| ())
}
