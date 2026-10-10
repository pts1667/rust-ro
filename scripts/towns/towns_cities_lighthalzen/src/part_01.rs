use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn jiwon_zen5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jiwon",
        args![
            "I think we're really",
            "fortunate to be able to",
            "live in such a beautiful",
            "and peaceful city like this."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiwon",
        args![
            "It's just so nice to",
            "have this pleasant weather,",
            "these lush gardens and to",
            "meet all of these kind people.",
            "Lighthalzen is like Asgard",
            "in Midgard, heaven on earth~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jiwon_zen5(ctx: &Ctx) -> Script {
    jiwon_zen5_body(ctx, Vec::new()).map(|_| ())
}

fn samnang_zen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Samnang",
        args![
            "^333333*Sigh...*^000000",
            "It gets harder for me",
            "to move around as I get",
            "older. That's understandable",
            "for an elderly person, right?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Samnang",
        args![
            "Just the other day, these",
            "hoodlums in black suits",
            "were yelling at me to get out",
            "of their way. But of course,",
            "I didn't move quickly enough.",
            "So what did they do to me?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Samnang",
        args![
            "They punched me.",
            "Right in the womb!",
            "I know that I'm not",
            "pregnant, but that's",
            "besides the point. Never hit",
            "a lady, especially an old one!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn samnang_zen2(ctx: &Ctx) -> Script {
    samnang_zen2_body(ctx, Vec::new()).map(|_| ())
}

fn ruth_zen4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Ruth",
        args![
            "Sweety, isn't it",
            "nice to be together",
            "under this beautiful",
            "sunlight? It's perfect",
            "for our date. Ahhhh~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ruth",
        args![
            "I'm so happy to be",
            "with you. I feel like",
            "I'm just melting with",
            "happiness. Oh, I love",
            "you so much, Oyoung."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Whoa...", "This couple is", "really headed for", "Cloud 9, aren't they?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ruth_zen4(ctx: &Ctx) -> Script {
    ruth_zen4_body(ctx, Vec::new()).map(|_| ())
}

fn oyoung_zen14_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Oyoung",
        args![
            "Girl, you look like",
            "you're comin' down with",
            "the love bug. But there's",
            "only one prescription for",
            "this ailment, ooooh yeah..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Oyoung",
        args![
            "You need yo'self",
            "your daily dose of",
            "vitamin O-YOUNG.",
            "And your lips look like",
            "they got vitamin deficiency.",
            "I better take care of that~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "Sweet Sister!",
            "I don't know what's",
            "more mind boggling--",
            "The fact that he used",
            "that line or the fact that",
            "it's actually working..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn oyoung_zen14(ctx: &Ctx) -> Script {
    oyoung_zen14_body(ctx, Vec::new()).map(|_| ())
}

fn kariya_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kariya",
        args![
            "I think ''Lighthalzen'' is",
            "supposed to mean ''crest of",
            "light,'' though I hear that this",
            "city was actually named after",
            "somebody. Who knows for sure?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kariya",
        args![
            "Still, it's a fitting",
            "name for the wealthiest",
            "and most luxurious city in",
            "all the Schwarzwald Republic.",
            "So how do you like this place?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kariya_li_01(ctx: &Ctx) -> Script {
    kariya_li_01_body(ctx, Vec::new()).map(|_| ())
}

fn sung_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sung",
        args![
            "When I grow up, I want",
            "to become such a great",
            "person that they'll make",
            "a statue of me, just like",
            "those statues over there."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sung",
        args![
            "Then people would be like,",
            "''Hey yo. That statue. That",
            "guy must have been great!''",
            "Just thinking about that",
            "makes me feel so good!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sung",
        args![
            "That's it. I'm gonna",
            "grow up as soon as I can.",
            "Ooh, and I better grow tall",
            "and handsome so my statue",
            "will be even more awesome.",
            "Yeah. Yeah, good idea, Sung..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sung_a(ctx: &Ctx) -> Script {
    sung_a_body(ctx, Vec::new()).map(|_| ())
}

fn sameer_zen15_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sameer",
        args![
            "There are too many",
            "loving couples in this city.",
            "Cuddling and kissing and",
            "hugging and necking. It's...",
            "It's utterly distasteful."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sameer",
        args![
            "I can't believe the",
            "indecency I see everyday",
            "near my own home. Don't",
            "they know better than to be",
            "so affectionate in places",
            "where the public can see them?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sameer",
        args![
            "Fortunately for the",
            "world, I'm a bulwark",
            "of morality. In fact, I have",
            "no need for a woman. All",
            "I need are my ship models,",
            "teen novels and dominoes."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sameer",
        args![
            "I'm a completely well",
            "adjusted individual, which",
            "is why the authorities should",
            "listen to me when I tell them",
            "to arrest couples for indecency!",
            "Hand holding. Have they no shame?!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sameer_zen15(ctx: &Ctx) -> Script {
    sameer_zen15_body(ctx, Vec::new()).map(|_| ())
}

fn janice_zen03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Janice",
        args![
            "Oh no, I think I got",
            "lost again. The roads",
            "here are so confusing!",
            "I've lived here for such",
            "a long time and I still",
            "can't find my way around..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn janice_zen03(ctx: &Ctx) -> Script {
    janice_zen03_body(ctx, Vec::new()).map(|_| ())
}

fn elmer_keays_li_03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Elmer Keays",
        args![
            "Walking side by side",
            "with you like this reminds",
            "me of the old days. Back",
            "then, everyone was jealous",
            "that I had such a beautiful",
            "woman by my side. Heh heh~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Elmer Keays",
        args![
            "You're still the most",
            "precious sight to these",
            "old eyes, my dear. I'm",
            "really lucky to be with you."
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_CHUP")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Margie Keays#li_02")])?,
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUPCHUP")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn elmer_keays_li_03(ctx: &Ctx) -> Script {
    elmer_keays_li_03_body(ctx, Vec::new()).map(|_| ())
}

fn margie_keays_li_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Margie Keays",
        args![
            "Oh darling, the",
            "weather is so nice",
            "and pleasant today.",
            "I'm really glad we",
            "decided to go take",
            "a walk together~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn margie_keays_li_02(ctx: &Ctx) -> Script {
    margie_keays_li_02_body(ctx, Vec::new()).map(|_| ())
}

fn maivi_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Maivi", args!["..."])?;
    ctx.next()?;
    ctx.lines_as("Maivi", args!["...", "......"])?;
    ctx.next()?;
    ctx.lines_as(
        "Maivi",
        args![
            "Ah...",
            "I just had the nicest",
            "nap. This nice weather",
            "never fails to relax me.",
            "The air here is so clean,",
            "not like that Einbroch~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Maivi",
        args![
            "This clean, pristine",
            "environment is all thanks",
            "to the Rekenber Corporation.",
            "It's incredible what they can",
            "do with technology now, isn't",
            "it? Ahhh, it's so peaceful~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maivi_zen1(ctx: &Ctx) -> Script {
    maivi_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn klaubis_zen3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Klaubis",
        args![
            "Excuse me, but are you",
            "a tourist? Well, welcome",
            "to Lighthalzen! This city",
            "has everything we need,",
            "but it can be a little too",
            "quiet and uneventful here."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from(
            "Have you lived in here long?:I agree.:Have you heard about the serial killer?",
        )],
    )? {
        1 => {
            ctx.lines_as(
                "Klaubis",
                args![
                    "Yes, our family has",
                    "lived in this city for a",
                    "long time, starting with",
                    "my great grandfather. Let's",
                    "see, my family's been here",
                    "for about two hundred years."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Klaubis",
                args![
                    "You'd be surprised how",
                    "many people stay in their",
                    "hometowns. Even if you do",
                    "leave, though, you can always",
                    "come back. It wouldn't be your hometown if you couldn't, right?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Klaubis",
                args![
                    "Yes, the atmosphere",
                    "can get pretty listless",
                    "around here. But still,",
                    "there are plenty of nice",
                    "sights to enjoy here in",
                    "Lighthalzen, so look around~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Klaubis",
                args![
                    "You mean the Serial",
                    "Axe Murderer? I thought",
                    "that was an old ghost story.",
                    "Hm. I think that lady inside",
                    "the Weapon Shop would",
                    "know more about that tale..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn klaubis_zen3(ctx: &Ctx) -> Script {
    klaubis_zen3_body(ctx, Vec::new()).map(|_| ())
}

fn sigmund_zen3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sigmund Ting",
        args![
            "You know what I noticed?",
            "The guards at the border",
            "to the slum seem distracted",
            "sometimes. I made use of one",
            "of their less attentive moments",
            "and basically jumped the fence!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sigmund Ting",
        args![
            "But once I was in the ",
            "slums, I was pretty bored.",
            "There really isn't much to",
            "do there. Which makes me",
            "wonder... Why guard it?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sigmund_zen3(ctx: &Ctx) -> Script {
    sigmund_zen3_body(ctx, Vec::new()).map(|_| ())
}

fn joyce_zen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Joyce",
        args![
            "I can sense your",
            "longing look within",
            "the depths of my heart,",
            "beating faster and faster",
            "with a feverish passion~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn joyce_zen(ctx: &Ctx) -> Script {
    joyce_zen_body(ctx, Vec::new()).map(|_| ())
}

fn dan_song_zen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Dan Song",
        args![
            "Those eyes of yours...",
            "So pure and so deep,",
            "like glimmering pools",
            "of light. So, so beautiful..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dan_song_zen2(ctx: &Ctx) -> Script {
    dan_song_zen2_body(ctx, Vec::new()).map(|_| ())
}

fn collins_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Collins",
        args![
            "I really wish that my",
            "son will be able to join",
            "the Rekenber Corporation.",
            "They certainly provide the",
            "best jobs in Lighthalzen."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Collins",
        args![
            "Although they're a large,",
            "major corporation, it's",
            "almost impossible to get",
            "employed by them. How",
            "do people get hired there",
            "in the first place anyway?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn collins_zen1(ctx: &Ctx) -> Script {
    collins_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn villagomez_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Villagomez",
        args![
            "I just step out to get",
            "a haircut and now I'm",
            "lost. Boy oh boy, I hope",
            "I don't keep my family",
            "waiting. ^333333*Sigh...*^000000"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn villagomez_li_01(ctx: &Ctx) -> Script {
    villagomez_li_01_body(ctx, Vec::new()).map(|_| ())
}

fn kemp_zen13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kemp",
        args![
            "Have you ever seen the",
            "people who work in that big",
            "corporation over there? I think",
            "their employees are all a bit",
            "off kilter for some reason."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kemp",
        args![
            "I haven't been there",
            "myself, but something",
            "strange is happening with",
            "all the people who work there."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kemp_zen13(ctx: &Ctx) -> Script {
    kemp_zen13_body(ctx, Vec::new()).map(|_| ())
}

fn mauro_zen3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Mauro",
        args![
            "The youth in this city",
            "have no appreciation for",
            "their elders. I've worked",
            "so hard to help build this",
            "city for so many years and",
            "this is the thanks I get?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mauro",
        args![
            "Bah! If it weren't for",
            "us, Lighthalzen wouldn't",
            "be as prosperous as it is",
            "today! Those kids don't",
            "know that they owe their",
            "lives of luxury to us..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mauro_zen3(ctx: &Ctx) -> Script {
    mauro_zen3_body(ctx, Vec::new()).map(|_| ())
}

fn sefith_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sefith",
        args![
            "Good looks. Intelligence.",
            "Excellent manners. A strong,",
            "manly chin and overpowering,",
            "piercing eyes. Perfectly balanced passion and charisma. All the",
            "good things that ladies want."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Sefith",
        args![
            "But enough about me. Let's",
            "discuss how sorry I should",
            "feel for any other man living",
            "in Lighthalzen. They don't hold",
            "a candle to my studliness~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sefith_li_01(ctx: &Ctx) -> Script {
    sefith_li_01_body(ctx, Vec::new()).map(|_| ())
}

fn jade_zen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jade",
        args![
            "I've heard that there's a",
            "strange kingdom out there",
            "that's basically ruled by",
            "magic and swords, where",
            "adventurers are enlisted",
            "for the greater good."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jade",
        args![
            "So are you from",
            "Rune-Midgarts?",
            "What do you think",
            "of our city with its",
            "advanced technology",
            "and economy? Huh..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jade",
        args![
            "Someday, I'd like",
            "to go visit the land",
            "where you came from.",
            "It sounds so fantastic",
            "and romantic in a way..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jade_zen2(ctx: &Ctx) -> Script {
    jade_zen2_body(ctx, Vec::new()).map(|_| ())
}

fn greedy_looking_man_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Khramptd",
        args![
            "The land around here",
            "is some pretty expensive",
            "property. Yes, it's perfect",
            "for building my awesome palace!",
            "I don't have enough funds at the",
            "moment, but the day will come~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn greedy_looking_man_li_01(ctx: &Ctx) -> Script {
    greedy_looking_man_li_01_body(ctx, Vec::new()).map(|_| ())
}

fn maggie_05_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Maggie",
        args![
            "Sure, I sell a lot",
            "of flowers here, but",
            "the lease that this city",
            "makes me pay cuts into",
            "my profits. It's almost not",
            "worth renting this property."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Maggie",
        args![
            "I pay such a ridiculous",
            "amount for the lease and the",
            "laws here won't let me raise",
            "the price of my flowers. Why",
            "are the city officials so greedy?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maggie_05(ctx: &Ctx) -> Script {
    maggie_05_body(ctx, Vec::new()).map(|_| ())
}

fn wallace_zen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Wallace",
        args![
            "......",
            "That lady, working",
            "for that one company,",
            "Kafra, Mafra or whatever.",
            "She certainly is very charming."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wallace",
        args![
            "Now, if I were",
            "thirty years younger...",
            "Wait! I'm a rich and powerful",
            "man. I could ask her out now.",
            "Hm? What's that look for?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wallace_zen2(ctx: &Ctx) -> Script {
    wallace_zen2_body(ctx, Vec::new()).map(|_| ())
}

fn lucius_zen5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    if ctx.var("Zeny").get()?.number()? < 90000 {
        ctx.lines_as(
            "Lucius",
            args!["Hello youngster~", "Would you like to", "make a donation", "to help the hungry?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:No, thanks.")])?) == 1 {
            ctx.lines_as(
                "Lucius",
                args![
                    "Now, you can donate 1 to",
                    "30,000 zeny that will be used",
                    "to support the poor and feed",
                    "starving children. If you wish",
                    "to cancel, please enter ''0.''"
                ],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_number(ctx, None, None)?;
            l_input = input;
            if (l_input.clone().number()? > 30000 || l_input.clone().number()? < 0) {
                ctx.lines_as(
                    "Lucius",
                    args![
                        "Please enter a value",
                        "from 1 to 30,000 in",
                        "order to make a donation",
                        "to the needy, youngster."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_input.clone() == 0 {
                ctx.lines_as(
                    "Lucius",
                    args![
                        "How disappointing,",
                        "but I'm sure you have",
                        "your reasons. Well, when",
                        "you can afford to give to",
                        "the needy, you're welcome",
                        "to come back at any time."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Lucius",
                args![
                    "Thank you so much",
                    ((Val::from("for your ") + l_input.clone()) + Val::from(" zeny donation.")),
                    "I promise that your money",
                    "will be put to good use in",
                    "benefiting the poor and needy."
                ],
            )?;
            ctx.next()?;
            if runtime::op(&ctx.var("Zeny").get()?, "<", &l_input.clone())?.is_true() {
                ctx.lines_as(
                    "Lucius",
                    args![
                        "Still, I'm just a little",
                        "disappointed. An adventurer",
                        "like you should be donating",
                        "as much as you possibly can..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_input.clone())?))?;
            ctx.var("$donatedzeny").set((ctx.var("$donatedzeny").get()? + l_input.clone()))?;
            ctx.lines_as(
                "Lucius",
                args![
                    "So far, I've received",
                    ((Val::from("a total of ") + ctx.var("$donatedzeny").get()?) + Val::from(" zeny in")),
                    "donations. I'm glad to see",
                    "that there are still kind and",
                    "generous people in the world."
                ],
            )?;
            if ctx.var("$donatedzeny").get()?.number()? > 260000 {
                ctx.next()?;
                ctx.lines_as(
                    "Lucius",
                    args![
                        "This should be enough",
                        "to send to the Poor Relief",
                        "Organization. Please accept",
                        "this small gift as a token of",
                        "my gratitude, adventurer. Bless",
                        "you, youngster and take care."
                    ],
                )?;
                ctx.var("$donatedzeny").set(Val::from(0))?;
                ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(12016), Val::from(1)])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Lucius",
            args![
                "I understand. Still,",
                "keep in mind that when",
                "you give from your heart,",
                "you will be rewarded tenfold.",
                "Though I admit, the benefits",
                "aren't always readily apparent."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Lucius",
        args![
            "Hello youngster~",
            "You seem to be fairly",
            "well-off. Money is good",
            "to have, but be careful not",
            "to become obsessed with it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lucius",
        args![
            "When you have the chance,",
            "please show your generosity",
            "towards others who may be",
            "much less fortunate than you."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lucius_zen5(ctx: &Ctx) -> Script {
    lucius_zen5_body(ctx, Vec::new()).map(|_| ())
}

fn laqumet_li_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Laqumet",
        args![
            "Sure, manliness is quite",
            "attractive, but I think women",
            "appreciate a guy who could",
            "sympathize and talk with them",
            "a little more. Don't you agree?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laqumet",
        args![
            "I might not be like Sefith,",
            "but I guess I've got a cute",
            "smile, a good personality and",
            "I'm a dandy to boot. Hopefully,",
            "my honesty and loyalty will",
            "help me find someone good."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn laqumet_li_02(ctx: &Ctx) -> Script {
    laqumet_li_02_body(ctx, Vec::new()).map(|_| ())
}

fn hotel_employee_zen3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hotel Employee",
        args![
            "If you are experiencing",
            "any sort of inconvenience,",
            "please do not hesitate and",
            "let us know right away."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hotel Employee",
        args![
            "Please use the stairs",
            "at the northern end to",
            "go downstairs so that you",
            "can go to the Front Desk.",
            "Thank you and I hope that",
            "you enjoy your stay here."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hotel_employee_zen3(ctx: &Ctx) -> Script {
    hotel_employee_zen3_body(ctx, Vec::new()).map(|_| ())
}

fn christopher_michael_zen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Christopher Michael",
        args![
            "OoooOoh~",
            "Soooo comfortable.",
            "Don't want to wake up.",
            "Don't want to get up.",
            "Ever again. OoOoooh..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn christopher_michael_zen(ctx: &Ctx) -> Script {
    christopher_michael_zen_body(ctx, Vec::new()).map(|_| ())
}

fn safwat_fahmy_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Safwat Fahmy",
        args![
            "This hotel is nice",
            "and comfortable, but",
            "to be quite frank, the",
            "drinks here are horrible.",
            "They're unfit for drinking",
            "men such as myself."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Safwat Fahmy",
        args![
            "If this is the best hotel,",
            "I expect them to provide me",
            "with the best alcohol. When",
            "I stay at a hotel, that's what",
            "I want. To spend the entire",
            "day not being sober."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Safwat Fahmy",
        args![
            "It looks like that",
            "today I'll be heading",
            "out to the bar again...",
            "I just wish there were",
            "someplace quieter to drink."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn safwat_fahmy(ctx: &Ctx) -> Script {
    safwat_fahmy_body(ctx, Vec::new()).map(|_| ())
}

fn hotel_employee_zen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hotel Employee",
        args![
            "This is the Couple Suite.",
            "A single can also check",
            "in here, but our hotel will",
            "prioritize couples when",
            "assigning this room."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hotel_employee_zen2(ctx: &Ctx) -> Script {
    hotel_employee_zen2_body(ctx, Vec::new()).map(|_| ())
}

fn tanoue_zen04_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Tanoue",
        args![
            "This chair looks",
            "very nice, but it really",
            "chills my bottom. Brr...!",
            "It's a might uncomfortable!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tanoue",
        args![
            "You know what the",
            "perfect chair would",
            "be like? It would be",
            "plush and have electronic",
            "massage and heating controls..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn tanoue_zen04(ctx: &Ctx) -> Script {
    tanoue_zen04_body(ctx, Vec::new()).map(|_| ())
}

fn ben_allen_zen11_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Ben Allen",
        args![
            "Aaahhh Oooooh~",
            "It's sooooo comfy~",
            "The air's so fresh and",
            "this couch is so plush...",
            "Why can't home be like this?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ben Allen",
        args![
            "I've been in those other",
            "hotels and let me tell you,",
            "this place is the best ever.",
            "After a night's sleep over",
            "here, I feel like a new man!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ben_allen_zen11(ctx: &Ctx) -> Script {
    ben_allen_zen11_body(ctx, Vec::new()).map(|_| ())
}

fn harp_zen8_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Harp",
        args![
            "Oh sweet jiminy...",
            "That Kafra Lady is so hot.",
            "What a body. And those glasses.",
            "I just gotta ask her out somehow."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Harp",
        args![
            "Hm, but what should",
            "I do? A love letter? Naw,",
            "that's kind of outdated.",
            "Argh, I can't think! Just",
            "looking at her makes me feel",
            "so happy! Praise be to Kafra!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn harp_zen8(ctx: &Ctx) -> Script {
    harp_zen8_body(ctx, Vec::new()).map(|_| ())
}

fn hotel_employee_zen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hotel Employee",
        args![
            "''Hospitality with a smile",
            "and total devotion to your",
            "comfort.'' That's our motto",
            "in the Royal Dragon Hotel.",
            "Please inquire at the front",
            "desk if you wish to check in."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hotel_employee_zen1(ctx: &Ctx) -> Script {
    hotel_employee_zen1_body(ctx, Vec::new()).map(|_| ())
}

fn hotel_employee_zen4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hotel Employee",
        args![
            "Welcome to the",
            "Royal Dragon Hotel Bar.",
            "How about a nice night",
            "cap before going to bed?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hotel Employee",
        args![
            "If you're looking",
            "for a friend, you",
            "can almost always",
            "make one in this bar.",
            "Alcohol certainly is the",
            "grease for social gears."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hotel_employee_zen4(ctx: &Ctx) -> Script {
    hotel_employee_zen4_body(ctx, Vec::new()).map(|_| ())
}

fn citizen_amano09_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hachi",
        args![
            "Oh yeah. I love-love-love",
            "bars. If I don't come here",
            "for the booze, then I'm here",
            "for all these beautiful ladies."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hachi",
        args![
            "Weird. It's the very first",
            "time I've tried this place's",
            "rum, but doesn't it taste like",
            "pure sexiness to you? Huh...",
            "Oh well, back to schmoozin'",
            "with all the hot chicks~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn citizen_amano09(ctx: &Ctx) -> Script {
    citizen_amano09_body(ctx, Vec::new()).map(|_| ())
}

fn bartender_amano07_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Duff",
        args![
            "Hey, you're from",
            "Rune-Midgarts, right?",
            "Please make yourself",
            "at home while you're here",
            "in Lighthalzen and have",
            "yourself a good time."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bartender_amano07(ctx: &Ctx) -> Script {
    bartender_amano07_body(ctx, Vec::new()).map(|_| ())
}

fn customer_amano13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Rona",
        args![
            "I hate it when guys",
            "just sidle up and sort",
            "of just skip Steps One",
            "and Two. And before you",
            "get all weird, Step Three",
            "is ''Ask for my phone number.''"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rona",
        args![
            "I would just really",
            "appreciate it if one",
            "nice boy would just",
            "talk to me for real."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn customer_amano13(ctx: &Ctx) -> Script {
    customer_amano13_body(ctx, Vec::new()).map(|_| ())
}

fn customer_amano10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Greenfield",
        args![
            "I don't believe it...",
            "This unlucky streak",
            "will never end, will it?",
            "I lost all my Apples",
            "playing Dice today.",
            "Again. Oh man..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Greenfield",
        args![
            "Okay. Okay.",
            "If I just keep",
            "playing, eventually",
            "I'll win. I mean, that's",
            "the way the odds work, right?",
            "Even when they're against me..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn customer_amano10(ctx: &Ctx) -> Script {
    customer_amano10_body(ctx, Vec::new()).map(|_| ())
}

fn customer_amano11_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Terry",
        args![
            "I'm not big on drinking,",
            "but the atmosphere in this",
            "place is really nice. The",
            "music they play is always",
            "smooth and relaxing..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Terry",
        args![
            "Yeah, this is a real cozy",
            "joint. I recommend it to",
            "all you tourists, actually.",
            "Now why don't you kick",
            "back and chill with me?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn customer_amano11(ctx: &Ctx) -> Script {
    customer_amano11_body(ctx, Vec::new()).map(|_| ())
}

fn customer_amano12_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sei",
        args![
            "You see that guy?",
            "That guy over there is",
            "always looking at me.",
            "I wonder... Does he want",
            "to ask me out or something?"
        ],
    )?;
    ctx.next()?;
    ctx.mes("[Sei]")?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.lines(args![
            "Well, if he does,",
            "shouldn't he have",
            "more guts? Or are ",
            "you boys much more",
            "shy than I think you are?"
        ])?;
    } else {
        ctx.lines(args![
            "Well, he is sort of",
            "cute. Geez, this would",
            "be so much easier if he",
            "would just come up and",
            "start talking to me..."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn customer_amano12(ctx: &Ctx) -> Script {
    customer_amano12_body(ctx, Vec::new()).map(|_| ())
}

fn merpi_zen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Merpi",
        args![
            "Isn't the weather nice",
            "today? All this sunlight",
            "will dry these clothes",
            "quickly and give them",
            "a fresh, lovely scent."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merpi",
        args![
            "Oh, an adventurer from",
            "Rune-Midgarts, are you?",
            "How do you like our city?",
            "If you have any questions,",
            "feel free to ask me anything."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("Well, I have nothing to ask...:Any news or rumors?:I like laundry too.")],
    )? {
        1 => {
            ctx.lines_as(
                "Merpi",
                args![
                    "Oh, really?",
                    "Well, if you've traveled",
                    "all over the world, maybe",
                    "you've found a place just",
                    "like Lighthalzen, so maybe",
                    "you're already comfortable?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Merpi",
                args![
                    "Well, things have",
                    "been pretty peaceful",
                    "for the past few years.",
                    "The only rumor floating",
                    "around is about some",
                    "weird axe murderer..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Merpi",
                args![
                    "Oh, that's wonderful!",
                    "I so love doing hand",
                    "laundry, though I'm not",
                    "quite sure why. Oh well~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn merpi_zen2(ctx: &Ctx) -> Script {
    merpi_zen2_body(ctx, Vec::new()).map(|_| ())
}

fn berru_lhz_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
    if subject1 == 1 {
        ctx.lines_as("Berru", args!["Daddy...! Waaaaah~!", "I wanna see my Daddy!"])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_CRY")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Berru#lhz_01")])?,
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pilia",
            args![
                "Berru, I don't ",
                "think Daddy's coming",
                "home tonight. Come on,",
                "we should go to bed."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Berru",
            args![
                "No, I'm not gonna",
                "sleep till Daddy gets",
                "home! He said he'll",
                "bring us candy tonight!",
                "You go sleep first, Pilia!"
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_ANGER")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Berru#lhz_01")])?,
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pilia",
            args![
                "^333333*Sigh...*^000000",
                "Where's our Daddy?",
                "He said he found a",
                "good job, but we haven't",
                "heard from him since then..."
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THINK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Pilia#lhz_01")])?,
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 2 {
        ctx.lines_as(
            "Pilia",
            args![
                "What's taking him",
                "so long? I hope Daddy",
                "comes back home soon.",
                "Come on, Berru, don't cry."
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THINK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Pilia#lhz_01")])?,
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Berru", args!["^333333*Sob...*^000000", "But I'm hungry", "and I miss Daddy!"])?;
        ctx.next()?;
        ctx.lines_as("Pilia", args!["Uncle Togii from", "next door hasn't", "come back either..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 {
        ctx.lines_as(
            "Pilia",
            args![
                "Hmm? Oh, I'm sorry,",
                "but my little brother",
                "just won't stop crying.",
                "I'm sorry if we're loud..."
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_QUESTION")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Pilia#lhz_01")])?,
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pilia",
            args![
                "Our daddy goes to work",
                "somewhere far away. He",
                "finally has a good job, but",
                "sometimes we don't hear",
                "from him for days. We get",
                "really worried about him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pilia",
            args![
                "My brother Berru always",
                "misses him a lot. I don't",
                "know how to make him",
                "stop crying! What do I do?"
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_PROFUSELY_SWEAT")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Pilia#lhz_01")])?,
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn berru_lhz_01(ctx: &Ctx) -> Script {
    berru_lhz_01_body(ctx, Vec::new()).map(|_| ())
}

fn beggar_lhz_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn beggar_lhz_02(ctx: &Ctx) -> Script {
    beggar_lhz_02_body(ctx, Vec::new()).map(|_| ())
}

fn beggar_lhz_02_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Beggar",
        args!["Please...", "My child is starving...", "Would you give me", "some money?"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Give him some money.:Ignore him.")])?) == 1 {
        if ctx.var("Zeny").get()?.number()? < 50 {
            ctx.lines_as(
                "Beggar",
                args![
                    "I appreciate your",
                    "kindness, but it also",
                    "looks like you're in need",
                    "of zeny, too. Would you",
                    "like to join me?"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Here you go,", "take this."],
        )?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50))?))?;
        ctx.next()?;
        ctx.lines_as(
            "Beggar",
            args![
                "Thank you so much.",
                "I have nothing to offer you",
                "in exchange, but I can share",
                "a story with you and impart",
                "some of the wisdom I've",
                "learned over the years."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
        ctx.next()?;
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject1 == 1 {
            ctx.lines_as(
                "Beggar",
                args![
                    "Everyone's been in",
                    "a situation where you",
                    "sometimes you feel that",
                    "you have to make a choice",
                    "between doing the right thing",
                    "and doing what you want, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "You may feel trapped.",
                    "Well, let me tell you, when",
                    "it comes to a problem, all",
                    "the solutions available to",
                    "you aren't always obvious.",
                    "So just calm down and think."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "What you can see and",
                    "understand might not match",
                    "with reality. Like the stars that are always there, but not visible",
                    "during the day, we'll always have hope, even if we can't see it."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.mes(". . . . . . . . . . . .")?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![". . . . . . . . . . . .", ". . . . . . . . . . . ."])?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![
                ". . . . . . . . . . . .",
                ". . . . . . . . . . . .",
                ". . . . . . . . . . . ."
            ])?;
            ctx.next()?;
            ctx.mes("[Beggar]")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
            ctx.lines(args!["Hmm...?", "You seem surprised~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 2 {
            ctx.lines_as(
                "Beggar",
                args![
                    "I sort of believe in fate and",
                    "sort of don't. Let me explain",
                    "it this way. I take life day by",
                    "day, with each day covering its",
                    "own spectrum with miracle on one end and tragedy on the other."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "So each day has the capacity",
                    "for experiences that can be",
                    "good, bad or both. I believe",
                    "each person can take an ",
                    "active role in shaping their",
                    "destiny, day by day."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "Now, there may be certain",
                    "things that you can't control,",
                    "but even a pessimist might",
                    "be able to agree that this",
                    "is a world that not only has",
                    "tragedy, but miracles as well."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "Stand up when you're down",
                    "and live your life with passion. The capacity for miracles will",
                    "always be there and know that",
                    "you can be someone else's",
                    "miracle. Isn't that wonderful?"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.mes(". . . . . . . . . . . .")?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![". . . . . . . . . . . .", ". . . . . . . . . . . ."])?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![
                ". . . . . . . . . . . .",
                ". . . . . . . . . . . .",
                ". . . . . . . . . . . ."
            ])?;
            ctx.next()?;
            ctx.mes("[Beggar]")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
            ctx.lines(args![
                "Don't believe me?",
                "Well, you'll see for",
                "yourself, youngster.",
                "There's much good in you."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 3 {
            ctx.lines_as(
                "Beggar",
                args![
                    "Anger. People deal with",
                    "it in different ways. Some",
                    "suppress it. Some relish it.",
                    "Some fear being angry. Now,",
                    "to be simple, let's say there",
                    "are two kinds of anger."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "The first is the kind that",
                    "isn't so productive. More of",
                    "a frustration that you can let",
                    "go. Someone cut you off on the",
                    "freeway or a friend innocently forgot your birthday? No biggie."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "Don't let this kind of",
                    "anger get to you or you'll",
                    "look like a loser. Think of",
                    "the big picture and if you're",
                    "still upset, vent appropriately. Be honest without hurting anyone."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "The second kind of anger",
                    "is righteous anger. You've",
                    "been wronged and need ",
                    "some form of retribution. ",
                    "Just don't misdirect your anger",
                    "and respond appropriately."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "The second kind of anger is",
                    "righteous anger. You've been",
                    "wronged and need some form",
                    "of retribution. Remember to",
                    "make appropriate confrontations",
                    "and don't misdirect your rage."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Beggar",
                args![
                    "Getting into a fight with",
                    "righteous anger, say to protect",
                    "someone dear to you, will make",
                    "you a hero. Fighting with anger",
                    "born of frustration will make you a bully. Know the difference."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.mes(". . . . . . . . . . . .")?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![". . . . . . . . . . . .", ". . . . . . . . . . . ."])?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
            ])?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![
                ". . . . . . . . . . . .",
                ". . . . . . . . . . . .",
                ". . . . . . . . . . . ."
            ])?;
            ctx.next()?;
            ctx.mes("[Beggar]")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
            ctx.lines(args!["What's wrong?", "It might be a lot", "to take in, I know."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...", "......"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn beggar_lhz_02_ontouch(ctx: &Ctx) -> Script {
    beggar_lhz_02_ontouch_body(ctx, Vec::new()).map(|_| ())
}
