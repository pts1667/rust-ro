use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn reuben_lhz_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
        ctx.lines_as(
            "Reuben",
            args![
                "Someday...",
                "Someday I just gotta",
                "become a train conductor",
                "and just get outta here!",
                "I really hate this place!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
        ctx.next()?;
        ctx.lines(args![
            "[Reuben]]",
            "Wh-whoa...!",
            "Did you just hear",
            "me talk to myself?",
            "Crud! Don't be so nosy!"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Reuben",
        args![
            "Hey. What are",
            "you doing just",
            "looking at me?",
            "I don't know you",
            "from Adam, so get lost~"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_ROCK")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn reuben_lhz_02(ctx: &Ctx) -> Script {
    reuben_lhz_02_body(ctx, Vec::new()).map(|_| ())
}

fn shengwen_zen7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Shengwen",
        args![
            "Am I just getting",
            "paranoid? I really",
            "think that some of",
            "the people I know",
            "are disappearing",
            "for no good reason!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shengwen",
        args![
            "I mean, all of my close",
            "friends are all alright,",
            "but I'm starting not to see",
            "certain acquaintances and",
            "familiar faces. Maybe I'm",
            "just thinking too much..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shengwen_zen7(ctx: &Ctx) -> Script {
    shengwen_zen7_body(ctx, Vec::new()).map(|_| ())
}

fn shayna_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Shayna", args!["^333333*Sigh...*^000000", "Oh, you poor", "darling girl..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shayna_li(ctx: &Ctx) -> Script {
    shayna_li_body(ctx, Vec::new()).map(|_| ())
}

fn cenku_dekdam_delic_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Cenku Dekdam",
        args![
            "Man, if you were",
            "gonna take this whole",
            "city and then sell it, what",
            "do you think Lighthalzen's",
            "price tag would be, eh?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cenku Dekdam",
        args![
            "I mean, this city",
            "is basically just made",
            "of money. Money is what",
            "makes this city such a nice",
            "and pleasant place to live."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cenku_dekdam_delic(ctx: &Ctx) -> Script {
    cenku_dekdam_delic_body(ctx, Vec::new()).map(|_| ())
}

fn nun_light_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Angela",
        args![
            "Greetings, adventurer.",
            "I'm Angela, a social",
            "worker for the Poor",
            "Relief Organization."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Angela",
        args![
            "I've noticed that the",
            "people living here have",
            "extremely bad health and",
            "it's not just because of",
            "their circumstances."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Angela",
        args![
            "I've filed a report",
            "to my superiors, but",
            "they haven't sent me",
            "a response yet for some",
            "reason. I'm starting to get",
            "a little worried about this..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn nun_light(ctx: &Ctx) -> Script {
    nun_light_body(ctx, Vec::new()).map(|_| ())
}

fn bankri_kun_kagun_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Bankri Kun",
        args![
            "Must work...",
            "Must focus...",
            "Resist sleepiness...",
            "Why do I keep coming",
            "here? Ugh, h-horrible."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bankri Kun",
        args![
            "Hey youngster. You wanted",
            "adventuring advice? Okay.",
            "Um. Hm. Always. Brush.",
            "Your teeth. Brush them",
            "everyday. Oh, and don't",
            "forget to floss, either."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bankri Kun",
        args![
            "Now it's time for me",
            "to head back to work.",
            "I'll see you later, kid.",
            "Sorry my advice was so",
            "lame-- I couldn't think of",
            "anything else to tell you."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bankri_kun_kagun(ctx: &Ctx) -> Script {
    bankri_kun_kagun_body(ctx, Vec::new()).map(|_| ())
}

fn enoz_oz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Enoz",
        args![
            "So, the novel I ordered from",
            "the Rune-Midgarts Kingdom",
            "just recently arrived. It's real good, by the guy who wrote",
            "''Roda Frog Adventure''",
            "years ago. Remember?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Enoz",
        args![
            "Anyway, this new book,",
            "''Where the Red Plant Grows''",
            "is up for the Yggdrasilberry",
            "Award. I... I don't know why",
            "I was compelled to share that",
            "with you. Seriously, I don't..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn enoz_oz(ctx: &Ctx) -> Script {
    enoz_oz_body(ctx, Vec::new()).map(|_| ())
}

fn ellette_tre_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Ellette", args!["..."])?;
    ctx.next()?;
    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me."])?;
    ctx.next()?;
    ctx.lines_as("Ellette", args!["...", "......"])?;
    ctx.next()?;
    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hello?"])?;
    ctx.next()?;
    ctx.lines_as("Ellette", args!["...Oh! Everyone!", "I just completed", "another one! Hooray!"])?;
    ctx.next()?;
    ctx.lines_as("All other Employees", args!["Wh-what?!", "No way, not again!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Leekal",
        args![
            "Are you even human?",
            "You must have some",
            "secret for that much",
            "productivity. It's weird..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ellette",
        args![
            "Oh, come on.",
            "Maybe I'm a little",
            "good at this, but there's",
            "no way I can beat Cenku."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ellette_tre(ctx: &Ctx) -> Script {
    ellette_tre_body(ctx, Vec::new()).map(|_| ())
}

fn dowbow_ryuei_ryusei_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Dowbow Ryuei",
        args![
            "Just out of, oh I dunno,",
            "curiosity, which word do",
            "you like better? ''Uber-Cool''",
            "or ''Reality?'' Pick one~"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Uber-Cool:Reality")])?) == 1 {
        ctx.lines_as(
            "Dowbow Ryuei",
            args![
                "Oh yeah? Me too!",
                "Yeah, we got the same",
                "outlook on life. If you don't",
                "mind, I'd like to shake",
                "your hand, adventurer."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dowbow Ryuei",
        args![
            "Reality, eh?",
            "Well, I agree that",
            "being realistic has its",
            "perks, I'm more of a dreamer."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dowbow_ryuei_ryusei(ctx: &Ctx) -> Script {
    dowbow_ryuei_ryusei_body(ctx, Vec::new()).map(|_| ())
}

fn leekal_lackee_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Leekal",
        args![
            "So... Very broke.",
            "Why did I spend so much",
            "money on wine, women and",
            "song? I regret it all, all the",
            "pleasure I've had this month.",
            "Yes, it was too much pleasure."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ninjose",
        args![
            "That's what happens",
            "when you're irresponsible",
            "with your money. You really",
            "should read this ''Anybody",
            "Can Be Rich'' book."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn leekal_lackee(ctx: &Ctx) -> Script {
    leekal_lackee_body(ctx, Vec::new()).map(|_| ())
}

fn ninjose_nina_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Ninjose",
        args![
            "At long last, I've finally",
            "bought my own home. You",
            "should invest your money for",
            "your future too! Read this,",
            "''Anybody Can Be Rich!''",
            "It's such a great book!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ninjose_nina(ctx: &Ctx) -> Script {
    ninjose_nina_body(ctx, Vec::new()).map(|_| ())
}

fn kejulle_rekenber_reken_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kejulle Rekenber",
        args![
            "Hm? Sure, my last name",
            "is Rekenber and that's the",
            "same name as our chairman,",
            "but that's just a coincidence.",
            "I'm merely a normal employee.",
            "Yeah, no special treatment..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kejulle_rekenber_reken(ctx: &Ctx) -> Script {
    kejulle_rekenber_reken_body(ctx, Vec::new()).map(|_| ())
}

fn jorjerro_fhero_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThis man here",
        "is motionless,",
        "and for all intents",
        "and purposes, is",
        "soundly asleep.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jorjerro_fhero(ctx: &Ctx) -> Script {
    jorjerro_fhero_body(ctx, Vec::new()).map(|_| ())
}

fn joshua_aya_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Joshua",
        args![
            "What am I doing here?",
            "Waiting for my dream",
            "woman to fall into my lap,",
            "what else does it look like?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Joshua",
        args![
            "Tall, blond, creamy",
            "complexion and smooth",
            "skin. That's right. Come",
            "right to Joshua, babes.",
            "I got my pheromone spray",
            "on and I'm ready to cruise~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn joshua_aya(ctx: &Ctx) -> Script {
    joshua_aya_body(ctx, Vec::new()).map(|_| ())
}

fn grinnel_zen6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Grinnel",
        args![
            "You know the men in",
            "black suits? Boy, did",
            "I get a scare! They actually",
            "tracked me down to ask me",
            "all these weird questions!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Grinnel",
        args![
            "They kept wanting to",
            "know if I had ever met",
            "anyone from the Rekenber",
            "Corporation, if I've ever been",
            "Uptown, that sort of thing. They",
            "really scared the crap out of me."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Grinnel",
        args![
            "Man, living in the",
            "slums is such a drag.",
            "Not only is life rough,",
            "but all sorts of people",
            "think they can push you",
            "around. I hate Lighthalzen..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn grinnel_zen6(ctx: &Ctx) -> Script {
    grinnel_zen6_body(ctx, Vec::new()).map(|_| ())
}

fn haggar_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Haggar", args!["Whiskey!", "I need me some", "hard liquor now!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Haggar",
        args![
            "Wha--? I didn't",
            "order this stinkin'",
            "rum! I want a man's",
            "drink! Gimme whiskey!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn haggar_zen1(ctx: &Ctx) -> Script {
    haggar_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn bartender_12_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Tony]")?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.mes("Hey man, I know this")?;
    } else {
        ctx.mes("Hey lady, I know this")?;
    }
    ctx.lines(args![
        "joint is a dive, pretty much",
        "on the verge of bein' totally",
        "ghetto, but we're proud to",
        "have the best rum in all of",
        "Midgard. It's true~"
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Tony",
        args![
            "Just a sip of this",
            "beautiful drink and",
            "you're on top of the",
            "world! But it's best",
            "for helpin' yah relax",
            "and forget your worries."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tony",
        args![
            "I don't take to bein'",
            "a poet, but I do know",
            "this. Our rum has the",
            "sweet sweet flavor of",
            "loneliness. You really",
            "oughta try it when you can."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bartender_12(ctx: &Ctx) -> Script {
    bartender_12_body(ctx, Vec::new()).map(|_| ())
}

fn bad_drunk_amano06_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Garry",
        args![
            "Hey! Hey you...!",
            "D'you wanna, you",
            "wanna hear me tell",
            "you a joke?! It goes...",
            "Um, it goes like this..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Garry",
        args![
            "Hey riddle middle,",
            "the cat and th--",
            "No! No, damn it!",
            "That's a song!",
            "No, wait, that's",
            "not a song either..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bad_drunk_amano06(ctx: &Ctx) -> Script {
    bad_drunk_amano06_body(ctx, Vec::new()).map(|_| ())
}

fn bad_drunk_12_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Bonse",
        args![
            "*Hiccup* I loooove",
            "this rum! I caught a cold",
            "once and one glass made",
            "it go away! 'Course, I slept",
            "for a week too, but that don't",
            "matter! Pshaw! Science..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bonse",
        args![
            "Oh, the flavor is just",
            "so clean, but it's also",
            "got a bit of a kick. I don't",
            "know how to describe it.",
            "Its the taste of happiness?",
            "I'm too drunk to even tell!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bad_drunk_12(ctx: &Ctx) -> Script {
    bad_drunk_12_body(ctx, Vec::new()).map(|_| ())
}

fn lab_staff_amano08_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Assam",
        args![
            "This place is nice",
            "and usually pretty quiet.",
            "I like to come here after",
            "work, have a drink and just",
            "chat with the bartender."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Assam",
        args![
            "The rum here is incredibly",
            "good too. It might even be",
            "the best in the world. I dunno",
            "why, but for some reason, its",
            "taste reminds me of teamwork~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lab_staff_amano08(ctx: &Ctx) -> Script {
    lab_staff_amano08_body(ctx, Vec::new()).map(|_| ())
}

fn city_girl_amano05_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Lanko",
        args![
            "Oh, I'm only here",
            "working as a waitress",
            "to help out my father.",
            "This job is so tiring, but",
            "it's nice to see people so",
            "relaxed and having a good time."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lanko",
        args![
            "When I get some time",
            "off, I'm going to explore",
            "Lighthalzen and see all that",
            "there is to see. But for now,",
            "it doesn't look like we've got",
            "any real shortage of drunks..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn city_girl_amano05(ctx: &Ctx) -> Script {
    city_girl_amano05_body(ctx, Vec::new()).map(|_| ())
}

fn drunken_man_amano01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Enku",
        args![
            "*Sob* I just got",
            "dumped! Yeah, I thought",
            "we were gonna get married,",
            "but obviously I was wrong!",
            "Damn it Sheryline! I loved you!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Enku",
        args![
            "I usually don't care for",
            "drinking, especially stuff",
            "like gin or rum, but today,",
            "this stuff tastes just like",
            "my misery. This is all the",
            "comfort I need, you hear?!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunken_man_amano01(ctx: &Ctx) -> Script {
    drunken_man_amano01_body(ctx, Vec::new()).map(|_| ())
}

fn drunken_man_amano02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Linus", args!["After ten years", "of marriage. My", "wife divorced me..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Linus",
        args![
            "So I guess there's no",
            "place for me but here for",
            "now. I don't know what it is,",
            "but the rum is really good",
            "today. Like, it's the flavor",
            "of relaxing, joyous relief~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn drunken_man_amano02(ctx: &Ctx) -> Script {
    drunken_man_amano02_body(ctx, Vec::new()).map(|_| ())
}

fn citizen_amano03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Mitchell",
        args![
            "You know, everyone",
            "is different, but I think",
            "humans are similar enough",
            "that we can all meaningfully",
            "connect on some level, right?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mitchell",
        args![
            "Sure, a rich person might",
            "have different problems than",
            "a poor person, but the point",
            "is, they've both got problems!",
            "Pain, pleasure, sadness, joy.",
            "Those link us all together."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mitchell",
        args![
            "So try not to be picky",
            "about who's your pal and",
            "who's not. We all need",
            "somebody to be with, right?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn citizen_amano03(ctx: &Ctx) -> Script {
    citizen_amano03_body(ctx, Vec::new()).map(|_| ())
}

fn citizen_amano04_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Dique",
        args![
            "One of the things I look",
            "forward to during my day",
            "is the drink I enjoy right",
            "after work. It's the most",
            "relaxing thing in the world."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Dique",
        args![
            "Of course, there's",
            "more to life than just",
            "hanging out in pubs and",
            "bars. The thing is, in my",
            "case, pubs and bars are",
            "all I happen to need~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn citizen_amano04(ctx: &Ctx) -> Script {
    citizen_amano04_body(ctx, Vec::new()).map(|_| ())
}

fn loudmouth_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Loudmouth",
        args![
            "Do you know who I am?!",
            "Just look at this peg leg.",
            "I was in the Comodo War,",
            "Ski Troop division! I lost my",
            "leg to earn your freedom!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Loudmouth",
        args!["H-hey! What's that", "look for? What, you", "don't believe me?!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn loudmouth(ctx: &Ctx) -> Script {
    loudmouth_body(ctx, Vec::new()).map(|_| ())
}

fn guard_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guard",
        args![
            "This is a",
            "restricted area.",
            "Please keep clear",
            "if you do not have",
            "special authorization.",
            "Thank you for your cooperating."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guard_01(ctx: &Ctx) -> Script {
    guard_01_body(ctx, Vec::new()).map(|_| ())
}

fn guide_lt0_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Lasoei",
        args![
            "Oh phooey.",
            "The same customers",
            "are always coming in,",
            "day after day. Can it",
            "get any less exciting?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Lasoei", args!["Oh...!", "W-welcome~", "C-can I help you", "with anything?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guide_lt0(ctx: &Ctx) -> Script {
    guide_lt0_body(ctx, Vec::new()).map(|_| ())
}

fn guide_lt1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Geonuii",
        args![
            "Greetings. This path",
            "leads to the Library and",
            "the Laboratory. Please be",
            "aware that these places",
            "are restricted from access",
            "by the general public."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guide_lt1(ctx: &Ctx) -> Script {
    guide_lt1_body(ctx, Vec::new()).map(|_| ())
}

fn guide_lt2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Bonnie", args!["Oh no...", "Where did I put it?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guide_lt2(ctx: &Ctx) -> Script {
    guide_lt2_body(ctx, Vec::new()).map(|_| ())
}

fn rekenber_guard_li01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
        && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
    {
        ctx.lines_as(
            "Rekenber Guard",
            args!["^3355FF(Whoa, it's a member", "of the staff!)^000000 Good day!"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(37), Val::from(225)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Rekenber Guard",
        args!["This is a restricted", "area! Please show", "some ID immediately!"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("ID?:Cancel")])?) == 1 {
        ctx.lines_as(
            "Rekenber Guard",
            args![
                "I don't know how you",
                "adventurers do things in",
                "Rune-Midgarts, but over here",
                "we have laws about trespassing!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Rekenber Guard",
        args![
            "Unless you have special",
            "authorization, nobody is",
            "allowed into the Underground",
            "Laboratory for security reasons."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rekenber_guard_li01(ctx: &Ctx) -> Script {
    rekenber_guard_li01_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RekenberGuardLi02Step {
    Start,
    OnTouch,
}

fn rekenber_guard_li02_run(ctx: &Ctx, mut step: RekenberGuardLi02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RekenberGuardLi02Step::Start => {
                step = RekenberGuardLi02Step::OnTouch;
                continue 'machine;
            }
            RekenberGuardLi02Step::OnTouch => {
                if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
                    && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
                {
                    ctx.lines_as(
                        "Rekenber Guard",
                        args![
                            "Keep your eyes open.",
                            "I've heard rumors that some",
                            "adventurers from Rune-Midgarts",
                            "are trying to sneak into here!",
                            "I know the security here is",
                            "pretty much fail sure, but..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Rekenber Guard",
                        args![
                            "This area is restricted",
                            "to the public! Who are you",
                            "and how did you get in here?!",
                            "Hey, I need backup right away!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(33), Val::from(224)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn rekenber_guard_li02(ctx: &Ctx) -> Script {
    rekenber_guard_li02_run(ctx, RekenberGuardLi02Step::Start, Vec::new()).map(|_| ())
}

pub fn rekenber_guard_li02_ontouch(ctx: &Ctx) -> Script {
    rekenber_guard_li02_run(ctx, RekenberGuardLi02Step::OnTouch, Vec::new()).map(|_| ())
}

fn repairman_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
        && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
    {
        ctx.lines_as(
            "Repairman",
            args![
                "No wonder these things",
                "break all the time! These",
                "machines have been totally",
                "abused! Ugh, there's no",
                "appreciation for all of this",
                "convenient technology..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Repairman",
            args![
                "Yeah, all of this lab",
                "equipment is really sensitive,",
                "not to mention expensive. If",
                "you ever handle this stuff, you",
                "need to be extra cautious."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Repairman",
            args!["Hey, you don't work--", "G-guards! Hurry! There's", "somebody over here!"],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(33), Val::from(224)])?;
        return Err(Stop::End);
    }
}

pub fn repairman_li_01(ctx: &Ctx) -> Script {
    repairman_li_01_body(ctx, Vec::new()).map(|_| ())
}

fn scientist_li_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
        && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
    {
        ctx.lines_as(
            "Scientist",
            args![
                "Alright. Pull one test",
                "tube out of the machine,",
                "replace the other test",
                "tube over here and then",
                "clean the first test tube?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Scientist",
            args![
                "Or do I clean the test tube,",
                "put it into the machine and",
                "then replace the other one?",
                "I'm so confused with this",
                "procedure! If only I didn't",
                "lose the instructions..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Scientist",
        args![
            "Alright. Pull one test",
            "tube out of the machine,",
            "replace th--hey. You're",
            "not Ralphie. Wait. Guaaards!",
            "Help me, there's some weirdo!"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(33), Val::from(224)])?;
    return Err(Stop::End);
}

pub fn scientist_li_02(ctx: &Ctx) -> Script {
    scientist_li_02_body(ctx, Vec::new()).map(|_| ())
}

fn scientist_li_03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
        && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
    {
        ctx.lines_as(
            "Scientist",
            args![
                "Whoa whoa~!",
                "Please! Don't",
                "touch anything!",
                "I'm dealing with highly",
                "volatile chemicals here!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Scientist",
        args![
            "Guards! Hurry,",
            "there's someone",
            "here, and I think",
            "it's one of those crazy",
            "stalkers! Why, why me?!"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(33), Val::from(224)])?;
    return Err(Stop::End);
}

pub fn scientist_li_03(ctx: &Ctx) -> Script {
    scientist_li_03_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RekenberGuardLi03Step {
    Start,
    OnTouch,
}

fn rekenber_guard_li03_run(ctx: &Ctx, mut step: RekenberGuardLi03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RekenberGuardLi03Step::Start => {
                step = RekenberGuardLi03Step::OnTouch;
                continue 'machine;
            }
            RekenberGuardLi03Step::OnTouch => {
                if (ctx.call(Function::IsEquipped, vec![Val::from(2241)])?.is_true()
                    && ctx.call(Function::IsEquipped, vec![Val::from(2243)])?.is_true())
                {
                    ctx.lines_as("Rekenber Guard", args!["......................"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Nice day, huh?:Cancel")])?) == 1 {
                        ctx.lines_as("Rekenber Guard", args!["..."])?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Rekenber Guard", args!["..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Rekenber Guard", args!["...!"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(33), Val::from(224)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn rekenber_guard_li03(ctx: &Ctx) -> Script {
    rekenber_guard_li03_run(ctx, RekenberGuardLi03Step::Start, Vec::new()).map(|_| ())
}

pub fn rekenber_guard_li03_ontouch(ctx: &Ctx) -> Script {
    rekenber_guard_li03_run(ctx, RekenberGuardLi03Step::OnTouch, Vec::new()).map(|_| ())
}

fn regenschirm_guard_40_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Regenschirm Guard]")?;
    if (runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(512))?.is_true()
        && ctx.call(Function::CountItem, vec![Val::from(2657)])?.number()? > 0)
    {
        ctx.lines(args!["Do you wish to", "go underground?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])?) == 1 {
            ctx.lines_as("Regenschirm Guard", args!["Thank you and", "have a nice day."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("lhz_dun01"), Val::from(149), Val::from(285)])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Regenschirm Guard", args!["Thank you and", "have a nice day."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "May I help you?",
        "If you would like to",
        "enter, you must first",
        "have a Laboratory Permit.",
        "Thank you for your cooperation."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn regenschirm_guard_40(ctx: &Ctx) -> Script {
    regenschirm_guard_40_body(ctx, Vec::new()).map(|_| ())
}

fn arthur_zen16_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Arthur",
        args![
            "The chairs here are",
            "so not ergonomic. And",
            "they're uncomfortable too!",
            "But it's sooo cool inside this",
            "bank and I just wanted to get",
            "get away from all this heat..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn arthur_zen16(ctx: &Ctx) -> Script {
    arthur_zen16_body(ctx, Vec::new()).map(|_| ())
}

fn helen_zen6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Helen",
        args![
            "You know, maybe when",
            "I grow up, I'll be a bank",
            "clerk. That sounds like a",
            "really nice job, don't you",
            "think? It's laid back and posh..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn helen_zen6(ctx: &Ctx) -> Script {
    helen_zen6_body(ctx, Vec::new()).map(|_| ())
}

fn tadem_zen6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Tadem",
        args![
            "I do so enjoy the",
            "architectural structure",
            "of this bank. It's quite",
            "artistic with both classical",
            "and modern elements. Would",
            "you not agree? Fascinating..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn tadem_zen6(ctx: &Ctx) -> Script {
    tadem_zen6_body(ctx, Vec::new()).map(|_| ())
}

fn gracie_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gracie",
        args![
            "Oh, it's so comfortable",
            "in here~ Though, why are",
            "we inside the bank when",
            "the bank services aren't even",
            "working? Yes, we're standing,",
            "but we're doing it in comfort."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gracie",
        args![
            "In fact, it's so",
            "comfortable here,",
            "I think I'll refuse to leave.",
            "Though, I'm willing to change",
            "my mind if you can find a place",
            "that's even more comfortable."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gracie_5(ctx: &Ctx) -> Script {
    gracie_5_body(ctx, Vec::new()).map(|_| ())
}

fn bank_clerk_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Bank Clerk",
        args![
            "Due to some critical system",
            "errors, all of the bank services",
            "have been temporarily stopped.",
            "We apologize for any inconvenience and appreciate your understanding."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bank_clerk_1(ctx: &Ctx) -> Script {
    bank_clerk_1_body(ctx, Vec::new()).map(|_| ())
}

fn togii_07_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Togii",
        args![
            "Oooh yeah...",
            "Goes down smooth.",
            "Morocc whiskey's the best!",
            "^333333*Hiccup*^000000 Whoa, this stuff",
            "really works fast! Heh heh~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn togii_07(ctx: &Ctx) -> Script {
    togii_07_body(ctx, Vec::new()).map(|_| ())
}

fn healthy_looking_guy_hol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Healthy Looking Guy",
        args![
            "Grrrrrr! Leave me alone!",
            "How many times do I have",
            "to keep telling you? I've never",
            "hoarded item upgrade materials!",
            "I swear that I'm innocent!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn healthy_looking_guy_hol(ctx: &Ctx) -> Script {
    healthy_looking_guy_hol_body(ctx, Vec::new()).map(|_| ())
}

fn hinkley_06_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hinkley",
        args![
            "Meh heh heh...",
            "^333333*Hiccup*^000000 Believe",
            "it or notsh, I'm...",
            "walkin on a... Air...",
            "Nevah thought I could",
            "b-be sho freee-eeee-eee~"
        ],
    )?;
    ctx.next()?;
    ctx.lines(args!["^3355FFThis guy", "is completely", "hammered out", "of his mind!^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hinkley_06(ctx: &Ctx) -> Script {
    hinkley_06_body(ctx, Vec::new()).map(|_| ())
}

fn millette_05_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Millette",
        args![
            "Let me go!",
            "Let me GO!!",
            "LET ME GO!!!",
            "I didn't do nuthin'",
            "wrong! I'm innocent!",
            "^333333*Hic-Hic-Hiccup...*^000000"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Millette",
        args![
            "What's wrong with",
            "drinking and singing",
            "in the street, huh?",
            "Is it a crime to have",
            "a beautiful tenor voice?!",
            "Get me outta this joint!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn millette_05(ctx: &Ctx) -> Script {
    millette_05_body(ctx, Vec::new()).map(|_| ())
}

fn officer_guo_06_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Officer Guo", args!["Tell me...!", "TELL ME...!!", "Admit you did it!!"])?;
    ctx.next()?;
    ctx.lines_as("Suspect", args!["Damn it!", "I keep telling", "you I'm not guilty!"])?;
    ctx.next()?;
    ctx.lines_as("Officer Guo", args!["^333333*Sigh...*^000000"])?;
    ctx.next()?;
    ctx.lines_as("Suspect", args!["You're wasting your", "time. Just let me go."])?;
    ctx.next()?;
    ctx.lines_as("Officer Guo", args!["So...", "How's your mother?"])?;
    ctx.next()?;
    ctx.lines_as("Suspect", args!["That's none of", "your business!", "She's fine, I guess."])?;
    ctx.next()?;
    ctx.lines_as("Officer Guo", args!["When was the last", "time you've seen her?"])?;
    ctx.next()?;
    ctx.lines_as("Suspect", args!["I just told you,", "that's none of", "your business...!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Officer Guo",
        args![
            "You know, mothers",
            "throughout the animal",
            "kingdom instinctively",
            "care for their young.",
            "Humans are no exception.",
            "Yours must be worried to death."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Suspect", args!["...", "Man...", "You're starting", "to weird me out."])?;
    ctx.next()?;
    ctx.lines_as(
        "Officer Guo",
        args![
            "Funny thing about humans,",
            "though. It seems to be their",
            "nature to lie, even when they",
            "know they'll be caught. But",
            "like all animals, they",
            "instinctively fear pain..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Suspect", args!["N-no, no...", "You gotta be...", "You're bluffing.", "Right?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Officer Guo",
        args!["NO.", "You're bluffing.", "Tell me...!", "TELL ME...!!", "Admit you did it!!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn officer_guo_06(ctx: &Ctx) -> Script {
    officer_guo_06_body(ctx, Vec::new()).map(|_| ())
}

fn banquet_staff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Banquet Staff",
        args![
            "This Banquet Hall is used",
            "to hold events such as dinner",
            "parties with partners, clients",
            "and other associates, and press",
            "conferences. Of course, there's",
            "nothing going on right now."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Banquet Staff",
        args![
            "Sometimes peace and quiet",
            "is a welcome change of pace,",
            "but right now I'm feeling quite",
            "bored. I think I would rather",
            "be busy than twiddling my",
            "thumbs, to tell the truth."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn banquet_staff(ctx: &Ctx) -> Script {
    banquet_staff_body(ctx, Vec::new()).map(|_| ())
}

fn luccet_li_party_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Luccet",
        args![
            "Shhhh! Hey, my brother's",
            "''it,'' so I gotta find a place",
            "to hide! Wait, would you just",
            "stand really still? I could",
            "just hide behind you! No?",
            "Nuts! Olly olly oxen free!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn luccet_li_party(ctx: &Ctx) -> Script {
    luccet_li_party_body(ctx, Vec::new()).map(|_| ())
}

fn hanccet_li_party_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hanccet",
        args![
            "Man... I hate being ''it!''",
            "I'm horrible at this game!",
            "Alright, okay, if I were my",
            "sister Luccet, where would",
            "I think I would not look for",
            "me? Of course...! The sewers!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hanccet_li_party(ctx: &Ctx) -> Script {
    hanccet_li_party_body(ctx, Vec::new()).map(|_| ())
}

fn annette_li_party_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Annette",
        args![
            "I've heard that the",
            "Rekenber Banquet Hall",
            "is also used to hold weddings.",
            "That must be so wonderful~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Annette",
        args![
            "Even if it is more expensive,",
            "I'd want to have my wedding",
            "here. Marriage is only once",
            "in a lifetime, ideally, so I'd",
            "want to make mine the most",
            "memorable experience."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn annette_li_party(ctx: &Ctx) -> Script {
    annette_li_party_body(ctx, Vec::new()).map(|_| ())
}

fn mereth_erem_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FF*Shhhhhhzzzz*", "*Shhhhhhzzzz*^000000"])?;
    ctx.next()?;
    ctx.lines_as("Mereth", args!["Shhhhh....", "Aaaaaaaahhh..."])?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFThe employee turned his",
        "head and peered into your",
        "eyes through the black mask",
        "on his face. Mereth stared",
        "wordlessly for a moment and",
        "then began to dance a lively,",
        "creepily jovial jig.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mereth_erem(ctx: &Ctx) -> Script {
    mereth_erem_body(ctx, Vec::new()).map(|_| ())
}

fn horri_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThis is simply a pile",
        "of files, a smattering of",
        "books and a family portrait.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn horri(ctx: &Ctx) -> Script {
    horri_body(ctx, Vec::new()).map(|_| ())
}

fn never_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThis desk is very",
        "neat and well organized",
        "in comparison to the other",
        "desks you've seen in your",
        "time. You take a moment to",
        "fully marvel at its tidiness.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn never(ctx: &Ctx) -> Script {
    never_body(ctx, Vec::new()).map(|_| ())
}

fn crazy4u_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThis desk has a bookshelf",
        "that is crammed with all sorts",
        "of books. Out of curiosity, you",
        "decide to pick one out.^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^3355FFHowever, the book you",
        "happen to touch contains",
        "an amazing amount of dark",
        "power, causing you to drop it.^000000"
    ])?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CURSEATTACK")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn crazy4u(ctx: &Ctx) -> Script {
    crazy4u_body(ctx, Vec::new()).map(|_| ())
}

fn noama_amano_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Noama",
        args![
            "Hee hee~!",
            "You wanna hear",
            "something funny?",
            "I heard there's a bar in",
            "Prontera where this guy",
            "sneaks singles into Jawa--"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mazwon",
        args!["Noama...!", "These machines are", "acting up again! Get", "over here right now!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Noama", args!["What?!", "Stop bugging me,", "I didn't do anything!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn noama_amano(ctx: &Ctx) -> Script {
    noama_amano_body(ctx, Vec::new()).map(|_| ())
}

fn mazwon_minus1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Mazwon",
        args![
            "Crap. Crap! Crap",
            "crap crap crap crap!",
            "These desk machines aren't",
            "supposed to work like this!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mazwon",
        args!["Noama...!", "These machines are", "acting up again! Get", "over here right now!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Noama", args!["What?!", "Stop bugging me,", "I didn't do anything!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mazwon_minus1(ctx: &Ctx) -> Script {
    mazwon_minus1_body(ctx, Vec::new()).map(|_| ())
}

fn mareth_seram_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Mareth", args!["Yoo hoo hoo~", "Oh, how I love", "love love chocolate!"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
    ctx.next()?;
    ctx.lines_as(
        "Mareth",
        args![
            "Eat it up...",
            "Or just melt it.",
            "Slather it all over me.",
            "Booyah. New life aspiration."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mareth_seram(ctx: &Ctx) -> Script {
    mareth_seram_body(ctx, Vec::new()).map(|_| ())
}

fn eiya_iaiai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Eiya",
        args![
            "Jorje seems so cranky",
            "recently. He's usually",
            "more laid back than this.",
            "Oh well, I hope that he",
            "feels better."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eiya",
        args![
            "Ooh, would you like",
            "to look at my miniature",
            "doll collection? I love",
            "collecting cute dolls!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn eiya_iaiai(ctx: &Ctx) -> Script {
    eiya_iaiai_body(ctx, Vec::new()).map(|_| ())
}

fn blackboard_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou found a blackboard",
        "filled with scribbling. You",
        "can only read some of the",
        "messages that have been",
        "quickly scrawled on it.^000000"
    ])?;
    ctx.next()?;
    ctx.lines(args!["''Make sure everything", "is complete by XX 00.''", "- Jorje"])?;
    ctx.next()?;
    ctx.mes("Late Fee: 59, 990 zeny")?;
    ctx.next()?;
    ctx.lines(args![
        "''I want to have''",
        "- Ellette",
        "''I want $$$ too!''",
        "- Enoz",
        "''How about @@@?''",
        "- Ellette"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "''I wanna buy [#@$].''",
        "- Ninjose",
        "''Go buy it!''",
        "- Senyu",
        "''Working hard and buying hard",
        "is the best employee attitude.''",
        "- Mazwon"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blackboard_li(ctx: &Ctx) -> Script {
    blackboard_li_body(ctx, Vec::new()).map(|_| ())
}

fn rocky_li_house_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
        ctx.lines_as("Rocky", args!["Woof woof!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Rocky", args!["Grrrrrrr..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rocky_li_house(ctx: &Ctx) -> Script {
    rocky_li_house_body(ctx, Vec::new()).map(|_| ())
}

fn jay_li_house_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jay",
        args![
            "My mommy and daddy",
            "always come home late.",
            "So I eat dinner alone.",
            "All by myself. Everyday."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jay",
        args![
            "Food doesn't taste as",
            "good when you're not",
            "eating with anybody.",
            "Maybe I'm just lonely."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jay_li_house(ctx: &Ctx) -> Script {
    jay_li_house_body(ctx, Vec::new()).map(|_| ())
}

fn housemaid_jane_li_house1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Housemaid Jane",
        args![
            "This house is enormous...",
            "It's clearly much too big",
            "for a regularly sized family.",
            "And it takes me forever to",
            "make sure that it stays clean!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Housemaid Jane",
        args![
            "It's not easy keeping",
            "things neat and tidy when",
            "you're responsible for acres",
            "of indoor living space. Being",
            "a maid can be pretty hard..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn housemaid_jane_li_house1(ctx: &Ctx) -> Script {
    housemaid_jane_li_house1_body(ctx, Vec::new()).map(|_| ())
}

fn housemaid_brenda_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Housemaid Brenda",
        args![
            "I better dust extra",
            "gently around this vase.",
            "It's worth ten million zeny",
            "and if it were to-- No. No!",
            "I'm not even going to think it!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn housemaid_brenda_li(ctx: &Ctx) -> Script {
    housemaid_brenda_li_body(ctx, Vec::new()).map(|_| ())
}

fn rekenber_employee_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Benatuth",
        args![
            "Down there, the repairman",
            "is just finishing maintenance",
            "on our chairman's private",
            "Airship. Can you imagine",
            "having one of those of your",
            "very own to fly around in?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Benatuth",
        args![
            "Yeah, the chairman of",
            "the Rekenber Corporation...",
            "He's a really powerful person.",
            "It's almost scary what he can",
            "do with his money, you know?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rekenber_employee_li(ctx: &Ctx) -> Script {
    rekenber_employee_li_body(ctx, Vec::new()).map(|_| ())
}

fn rekenber_guard_drew_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Rekenber Guard Drew",
        args![
            "Dude, check it out~",
            "Official glossy photos",
            "of the Kafra Ladies. Now...",
            "With 20% more garter belts!"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
    ctx.next()?;
    ctx.lines_as(
        "Rekenber Guard Tan",
        args![
            "So they're all wearing",
            "garter belts in these?",
            "Whoa, that means they",
            "even got the glasses chick",
            "to wear 'em too? That's the",
            "best news I've heard all day!"
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_HUK")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Rekenber Guard Tan#li")])?,
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rekenber Guard Drew",
        args![
            "Okay man, you know these",
            "are limited edition collector's",
            "items, so each one is worth",
            "300,000 zeny. I mean, I have",
            "an extra set, but I don't know",
            "if you'd wanna buy them off--"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rekenber Guard Tan",
        args![
            "I'll take them all.",
            "Wait, all of them except",
            "for that young kid. Just the",
            "idea of having her glamour",
            "photo around strikes me as...",
            "Yeah. Yeah, it's no good."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rekenber_guard_drew_li(ctx: &Ctx) -> Script {
    rekenber_guard_drew_li_body(ctx, Vec::new()).map(|_| ())
}

fn rekenber_guard_tan_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Rekenber Guard Tan",
        args![
            "Whoa, whoa. Now this...",
            "This is art. The lighting,",
            "the angle, the... the...",
            "subject matter. Oh yes."
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
    ctx.next()?;
    ctx.lines_as(
        "Rekenber Guard Drew",
        args![
            "Man, these officially licensed",
            "Kafra Lady glossies... They're",
            "worth every zeny we paid. Say",
            "goodbye Swimsuit Calendar,",
            "and hellooooo Kafra Leilah~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rekenber Guard Tan",
        args!["Leilah? Oh, you mean", "the glasses chick? Dude...", "Dude. She's my favorite too!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rekenber_guard_tan_li(ctx: &Ctx) -> Script {
    rekenber_guard_tan_li_body(ctx, Vec::new()).map(|_| ())
}
