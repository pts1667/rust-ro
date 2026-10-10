#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args, runtime};

pub fn museum_guide(ctx: &Ctx) -> Script {
    ctx.lines_as("Cenia", args!["Welcome to the Monster Museum."])?;
    ctx.next()?;
    match ctx.menu(&["Monster Museum?", "Tips"])? {
        0 => {
            ctx.lines_as(
                "Cenia",
                args![
                    "The Monster Museum was founded by",
                    "the Sages of the Schweicherbil",
                    "Magic Academy after researching",
                    "every creature dwelling in the",
                    "Midgard continent."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cenia",
                args![
                    "In this museum, you can see every",
                    "single monster in Midgard,",
                    "even the ones you hardly ever encounter."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cenia",
                args![
                    "The glass tubes holding monsters",
                    "was developed with the latest",
                    "technology as a part of the",
                    "Schwarz Project. Rest assured,",
                    "you'll be well protected."
                ],
            )?;
            ctx.next()?;
        }
        1 => {
            ctx.lines_as(
                "Cenia",
                args![
                    "Please check on the Opaque option",
                    "on your option windows by",
                    "pressing ALT + O, if you want to",
                    "see the monsters better."
                ],
            )?;
            ctx.next()?;
        }
        _ => {}
    }
    ctx.lines_as("Cenia", args!["Feel free to talk to me anytime."])?;
    ctx.close()
}

pub fn deviace_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn deviace(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", "Scientific name : Deviace", "Size : Medium", "Attribute : Water"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "This monster dwells under the deep",
        "sea and has a round body with a",
        "acetabulum on its dorsal side.",
        "Although its sharp teeth are",
        "intimidating, it has a very mellow",
        "character."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Therefore, it never initiates",
        "attack on an undersea traveler",
        "unless it's attacked first.",
        "However, once it becomes upset, it",
        "uses high level magic skills. So",
        "it's better to be careful with this monster."
    ])?;
    ctx.close()
}

pub fn seal_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn fur_seal(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", "Scientific name : Seal", "Size : Medium", "Attribute : Water"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "When you take a look at this",
        "monster carefully, you will find",
        "that it's not actually a seal but",
        "an unidentified monster hiding",
        "inside the seal-like leather",
        "clothing."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "It is rumored that the monster",
        "wears this clothing in order",
        "to protect its sensitive skin",
        "from the weather. The Seal's",
        "clothing is a very good material",
        "for people to produce winter coats."
    ])?;
    ctx.close()
}

pub fn sage_worm_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn sageworm(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", "Scientific name : Sage Worm", "Size : Small", "Attribute : Neutral"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A strange beast with the head of",
        "an old scholar and the tail of a",
        "worm. Although it is physically",
        "weak, it has the intelligence",
        "to support its comrades with",
        "magic skills."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "As its scholarly appearance",
        "indicates, it's usually seen near books or book shelves."
    ])?;
    ctx.close()
}

pub fn penomena_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn penomana(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", "Scientific name : Penomena", "Size : Medium", "Attribute : Poison"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "This monster dwells inside the",
        "deepest part of caves where there",
        "is enough moisture to keep it from",
        "getting dried up."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Unlike Hydra, a similar looking",
        "creature, it can move itself",
        "towards its enemy using many",
        "small appendixes on its acetabulum."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "The long, thin tentacles on the",
        "body shoot deadly poison which is",
        "enough to kill its enemy at once."
    ])?;
    ctx.close()
}

pub fn galapago_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn galapago(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", "Scientific name : Galapago", "Size : Small", "Attribute : Earth"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "It's a kind of bird, but sadly, its body is too heavy to fly.",
        "It's very sensitive to sunlight so it carries a water bottle and wears sunglasses all the time."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Although gluttonous, it will",
        "always cooperate to attack",
        "prey, as well as predators.",
        "Otherwise, Galapago is a",
        "generally laid back monster."
    ])?;
    ctx.close()
}

pub fn raydric_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn raydric(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", "Scientific name : Raydric", "Size : Large", "Attribute : Shadow"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A suit of armor animated by the",
        "spirit of a castle guard. The",
        "spirit is bound to this armor by",
        "a powerful curse."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Since Raydric used to be a castle",
        "guard, it possesses fast movements",
        "and powerful attack strength."
    ])?;
    ctx.close()
}

pub fn chepet_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn chepet(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Chepet", " Size : Medium", " Attribute : Fire"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "An evil creature hiding in a",
        "pretty doll. It attacks passersby",
        "by striking matchsticks held in",
        "the doll's hand. A very rare",
        "monster since it dwells in",
        "only a few places."
    ])?;
    ctx.close()
}

pub fn violy_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn violy(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Violy", " Size : Medium", " Attribute : Neutral"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A pretty doll looking creature with beautiful golden hair.",
        "Since It plays violin all the time with a peaceful look on its face, people don't realize at first that it's a monster."
    ])?;
    ctx.next()?;
    ctx.mes("Exercise extreme caution upon encountering a Violy. Otherwise, it will snatch your soul in no time with its charming song.")?;
    ctx.close()
}

pub fn alice_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn alice(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Alice ",
        " Size : Medium",
        " Attribute : Neutral"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Alice is a robot made to assist as",
        "a castle housemaid. They've been",
        "known to remain and automatically",
        "do their tasks long after the",
        "castle has been abandoned."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Without any discernable power",
        "source, how it moves and operates",
        "is still a scientific mystery."
    ])?;
    ctx.close()
}

pub fn assulter_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn assulter(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Assulter", " Size : Medium", " Attribute : Wind"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Unlike other turtles on Turtle",
        "Island, this turtle stands on two",
        "legs and attacks passersby with",
        "the other two legs, wielding a",
        "big shuriken from its back."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Interestingly, it creates a clone",
        "to do more damage when it",
        "encounters dangerous enemies.",
        "It does very powerful damage using",
        "its shuriken, but its nail attack",
        "is more threatening."
    ])?;
    ctx.close()
}

pub fn pecopeco_egg_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn thief_bug_egg_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn ant_egg_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn wanderer_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn wander_man(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Wanderer", " Size : Medium", " Attribute : Wind"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Undead warrior who came back to",
        "life through a curse. Considering",
        "its technical fencing skill, he",
        "must have been a very honorable",
        "warrior as a living human."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Wanderer can move amazingly fast",
        "and can slay enemies with a single",
        "stroke of its sword."
    ])?;
    ctx.close()
}

pub fn caterpillar_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn caterpillar(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Caterpillar",
        " Size : Small",
        " Attribute : Earth"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Although the eyes of this creature",
        "have atrophied due to living under",
        "the earth, it uses a feeler and",
        "appendices on its body to sense",
        "objects in its dark surroundings."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Caterpillar is rumored to be the",
        "larva of Creamy Fear, the advanced",
        "Creamy."
    ])?;
    ctx.close()
}

pub fn male_thiefbug_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn thief_bug(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Thief Bug",
        " ^FFFFFFScientific name :^000000 (Male)",
        " Size : Medium",
        " Attribute : Shadow"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Although it has a big blue body,",
        "it's also fast and voracious, just like other Thief Bugs."
    ])?;
    ctx.next()?;
    ctx.mes("However, it is stronger than other Thief Bugs because it's designated to protect the females and babies from danger.")?;
    ctx.close()
}

pub fn tri_joint_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn tri_joint(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Tri Joint",
        " Size : Small",
        " Attribute : Earth"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Tri Joint is a prehistoric",
        "monster that is covered with",
        "a hard shell, and uses a feeler",
        "instead of eyes so that it can",
        "live in dark places."
    ])?;
    ctx.next()?;
    ctx.mes("Recently, since many Tri Joints have been discovered inside many caves, Sages are very excited to study them to learn more about the evolution of monsters in Midgard.")?;
    ctx.close()
}

pub fn arclouz_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn arclouse(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Arclouz", " Size : Medium", " Attribute : Earth"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Hard shelled monster that coils",
        "its body to attack its enemy.",
        "Arclouz tend to stay in groups",
        "and are very aggressive",
        "creatures."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "They have incredibly fast",
        "movement speed, contrary to",
        "their looks, and are often",
        "compared to PecoPecos."
    ])?;
    ctx.close()
}

pub fn dragon_tail_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn dragon_tail(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Dragon Tail",
        " Size : Medium",
        " Attribute : Wind"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "An insect which is considered as",
        "a Libelluidae, or Dragon Fly. It",
        "uses its strong tail to suck the",
        "blood out of an enemy, or to put",
        "the enemy to sleep by shooting",
        "a sleeping poison."
    ])?;
    ctx.close()
}

pub fn owl_duke_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn owl_duke(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Owl Duke",
        " Size : Large",
        " Attribute : Neutral "
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation:",
        "An owl that wears a count costume.",
        "It's very intimidating looking",
        "with its dark, yet suave look.",
        "Owl Duke is not actually an owl,",
        "but a devil with very sharp claws",
        "on its big feet."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "It's skillful at using many",
        "lightning magic spells. When",
        "you see it attacking an enemy,",
        "you can sense the Owl Duke's",
        "aristocratic pompousness."
    ])?;
    ctx.close()
}

pub fn marine_sphere_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn marine_sphere(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Marine Sphere",
        " Size : Small",
        " Attribute : Water"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A strange creature that wanders",
        "in the deep oceans. It explodes",
        "with great power when it's",
        "touched, earning it the name",
        "'The Sea Bomb.'"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "If there is a Marine Sphere",
        "caught in the explosion of",
        "another, a trigger explosion",
        "will result, and can lead to",
        "a dangerous chain reaction."
    ])?;
    ctx.close()
}

pub fn mandragora_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn mandragora(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Mandragora",
        " Size : Medium",
        " Attribute : Earth"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "An insectivore that swallows",
        "anything alive. When it finds",
        "its prey, it strikes it first",
        "with a long tentacle to",
        "to paralyze it."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Once paralyzed, its prey is",
        "put in a large tube attached",
        "to its body where it is slowly",
        "digested. Although this tube",
        "has a skull mark, Mandragora",
        "does not actually contain any",
        "poison."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "This digestive tube",
        "apparatus also has a very unique,",
        "but disgusting smell that is far",
        "from useful in attracting prey."
    ])?;
    ctx.close()
}

pub fn geographer_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn geographer(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Geographer",
        " Size : Medium",
        " Attribute : Earth"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "An insectivore that looks like",
        "a sunflower. It uses the petal",
        "like tentacles around its",
        "mouth to attract and snare",
        "its prey."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Unlike Mandragora, Geographer",
        "does not have a tube to",
        "store its prey. So it slowly",
        "eats it's prey, little by little."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Although Geographer has small",
        "and short roots, the roots are",
        "tough and strong enough to",
        "bear the weight of the upper body."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "The namesake of this monster, a",
        "human geographer that was",
        "promptly eaten upon discovering",
        "this species of beast, will",
        "never be forgotten..."
    ])?;
    ctx.close()
}

pub fn rafflesia_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn rafflesia(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Rafflesia",
        " Size : Small",
        " Attribute : Earth"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A puffy, leafy monster",
        "threatened with extinction.",
        "Rafflesia is the rarest",
        "monster in Midgard and",
        "is thus protected by law."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Due to this situation, most Sages",
        "are having a hard time to",
        "research this monster.",
        "However, a few Sages are",
        "currently seeking methods to",
        "cultivate and save the Rafflesias."
    ])?;
    ctx.close()
}

pub fn stem_worm_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn stem_worm(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Stem Worm",
        " Size : Medium",
        " Attribute : Wind"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A mutated Worm Tail that has a",
        "round, brownish grey body with",
        "a small head. It is covered",
        "with scales and has a long",
        "stem-like tail which is used",
        "as a whip in attacks."
    ])?;
    ctx.close()
}

pub fn blazzer_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn blazzer(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Blazzer", " Size : Medium", " Attribute : Fire"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "This is a fire ball that has been",
        "seen near volcanic zones.",
        "Because of this monster's sudden",
        "appearance, Sages believe that",
        "volcanic activity may occur",
        "sooner or later near Juno."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Blazzer blows out noxious gas",
        "which harm passersby. It is",
        "unknown whether or not these",
        "are attacks or the Blazzer's",
        "form of communication."
    ])?;
    ctx.close()
}

pub fn ride_word_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn rideword(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Ride Word",
        " Size : Small",
        " Attribute : Neutral"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "It's a cursed magic book with",
        "sharp teeth. It exists to attack any living thing nearby."
    ])?;
    ctx.close()
}

pub fn megalodon_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn megalodon(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Megalodon",
        " Size : Medium",
        " Attribute : Undead"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A skeleton fish that was brought",
        "back to life by a curse. Although",
        "It looks very threatening, it's",
        "benign and will not attack",
        "undersea travellers outright."
    ])?;
    ctx.close()
}

pub fn sleeper_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn sleeper(ctx: &Ctx) -> Script {
    ctx.lines(args![" ", " Scientific name : Sleeper", " Size : Medium", " Attribute : Earth"])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "Unidentified sand creature.",
        "Usually it stays under the earth,",
        "but when travellers step on the",
        "sand, it may abruptly",
        "appear to attack them."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "It's smaller than Sandman and can",
        "cause indirect attacks by causing a sand storm."
    ])?;
    ctx.close()
}

pub fn ancient_mummy_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn ancient_mummy(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Ancient Mummy",
        " Size : Medium",
        " Attribute : Undead"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "A mummy cursed with eternal life.",
        "Although wrapped in decaying",
        "bandages, Ancient Mummy also",
        "wears a splendid hair ornament",
        "adorned with a snake."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "This kind of head ornament",
        "indicates that the Ancient",
        "Mummy was a person of high rank",
        "while he was still alive."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Since Ancient Mummy has",
        "been wandering the underworld",
        "for a long time, it does not have",
        "any consciousness and will",
        "attack any living thing nearby."
    ])?;
    ctx.close()
}

pub fn incubus_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn incubus(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Incubus",
        " Size : Medium",
        " Attribute : Shadow"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "This demon attracts humans",
        "with its stunningly beautiful",
        "appearance. When it poses as a",
        "male human, we call it Incubus.",
        "As a female, we call it Succubus."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "This devil targets people with",
        "mental vulnerabilities so that",
        "it can eventually take them to hell."
    ])?;
    ctx.close()
}

pub fn succubus_yuno(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn succubus(ctx: &Ctx) -> Script {
    ctx.lines(args![
        " ",
        " Scientific name : Succubus",
        " Size : Medium",
        " Attribute : Shadow"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "Explanation :",
        "This demon attracts humans",
        "with its stunningly beautiful",
        "appearance. When it poses as a",
        "female human, we call it Succubus.",
        "As a male, we call it Incubus."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "This devil targets people with",
        "mental vulnerabilities so that",
        "it can eventually take them to hell."
    ])?;
    ctx.close()
}
