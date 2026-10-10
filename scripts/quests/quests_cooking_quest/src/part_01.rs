use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum CharlesOrleansCookStep {
    Start,
    LEnd,
}

fn charles_orleans_cook_run(ctx: &Ctx, mut step: CharlesOrleansCookStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cook_m1 = Val::from(0);
    let mut l_new_book = Val::from(0);
    let mut l_old_book = Val::from(0);
    let mut l_talk_j = Val::from(0);
    'machine: loop {
        match step {
            CharlesOrleansCookStep::Start => {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                    ctx.lines_as(
                        "Charles Orleans",
                        args![
                            "Just one second.",
                            "You're carrying too",
                            "many items with you",
                            "right now, so you better",
                            "place some of your things",
                            "into Kafra Storage, yes?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Excuse me, monsieur?",
                                "Yes, you. If you're not",
                                "here as hired help for the",
                                "kitchen, then I'd like to",
                                "ask you to leave now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Please don't be",
                                "offended, but I can't",
                                "concentrate on my ",
                                "cooking when Novices",
                                "like yourself are running",
                                "around here like children."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                    ctx.call(Function::Cutin, vec![Val::from("orleans_1"), Val::from(0)])?;
                    ctx.lines_as(
                        "Charles Orleans",
                        args![
                            "Mademoiselle, what",
                            "are you doing in this",
                            "area of the castle?",
                            "Oh, you must be lost~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("orleans_2"), Val::from(0)])?;
                    ctx.lines_as(
                        "Charles Orleans",
                        args![
                            "Please, use the stairs",
                            "to the right to exit into the",
                            "main structure. My dear,",
                            "be careful and watch your",
                            "step when you climb up",
                            "the stairs for me, alright?"
                        ],
                    )?;
                    step = CharlesOrleansCookStep::LEnd;
                    continue 'machine;
                } else {
                    if ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_HEAD_TOP")?])? != 5026 {
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Monsieur, why you look",
                                    "at me so? Is it the Morocc",
                                    "silk shirt I am wearing, my",
                                    "hair styled by Madam Veronica,",
                                    "or my brand name muffler",
                                    "refined by Monsieur Antonio?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Perhaps you are in awe",
                                    "of the latest, fashionable",
                                    "spectacles that was designed",
                                    "by the artists from the Rekenber Corporation? Please, do tell~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("orleans_2"), Val::from(0)])?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Wait, wait just a",
                                    "moment. Do you know",
                                    "anything about the latest",
                                    "trends? You don't seem",
                                    "to be very fashionable..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                            ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Ugh, if I can avoid it,",
                                    "I usually prefer not to",
                                    "associate with ruffians.",
                                    "But I do find that you",
                                    "adventurers do have",
                                    "your strong points..."
                                ],
                            )?;
                            step = CharlesOrleansCookStep::LEnd;
                            continue 'machine;
                        }
                        ctx.call(Function::Cutin, vec![Val::from("orleans_1"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Oh! Pardon the squalor",
                                "of my humble kitchen,",
                                "Mademoiselle. But even",
                                "the splendor of the Prontera",
                                "Castle pales to the radiance",
                                "of your captivating beauty."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("orleans_2"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Tell me, who is the",
                                "lovely child holding",
                                "the cat right next to you?",
                                "I know it is rude to ask,",
                                "but I am emboldened by",
                                "my, shall we say, curiosity."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("......?:She is my sister:Actually, I don't know her.")])? {
                            1 => {
                                ctx.call(Function::Cutin, vec![Val::from("nyuang_3"), Val::from(2)])?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_DELIGHT")?,
                                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Child with Cat#cook")])?,
                                    ],
                                )?;
                                ctx.lines_as("The kid with a cat", args!["Nyahahaha,", "Nyahahaha~", "Meow~ Meow~"])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("orleans_2"), Val::from(0)])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                ctx.lines_as("Charles Orleans", args!["What a lovely child.", "Be quiet like a good girl."])?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.call(Function::Cutin, vec![Val::from("nyuang_1"), Val::from(2)])?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_QUESTION")?,
                                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Child with Cat#cook")])?,
                                    ],
                                )?;
                                ctx.lines_as("Child with Cat", args!["...Nya?", "...Meow?"])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("orleans_2"), Val::from(0)])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "She is lovely and",
                                        "quite although she",
                                        "doesn't look like you.",
                                        "Even her cat looks adorable."
                                    ],
                                )?;
                            }
                            3 => {
                                ctx.call(Function::Cutin, vec![Val::from("nyuang_2"), Val::from(2)])?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_HNG")?,
                                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Child with Cat#cook")])?,
                                    ],
                                )?;
                                ctx.lines_as("Child with Cat", args!["Nyahahaha,", "Nyahahaha~", "Meow, meow~"])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Ah, again, let me",
                                        "apologize. I had believed",
                                        "that this child was fortunate",
                                        "enough to be a companion",
                                        "of the mademoiselle."
                                    ],
                                )?;
                            }
                            _ => {}
                        }
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("orleans_1"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Allow me to introduce",
                                "myself to you, amour.",
                                "I am your ever faithful",
                                "servant whose heart is",
                                "enraptured by your gaze.",
                                "My name is Charles Orleans."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Yet there is one thing that",
                                "anguishes me. Ever since his",
                                "highness, King Tristram III,",
                                "has vanished, I have found no",
                                "one worthy of tasting my wares.",
                                "My life now lacks meaning..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Alas, recently I have been",
                                "reduced to teaching mere",
                                "apprentices, tyros in the",
                                "culinary arts, my skills.",
                                "It is frustrating--many of",
                                "them do not have any talent!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Ah, forgive me, dear",
                                "Mademoiselle. I hope you",
                                "understand the difficulty",
                                "I am forced to suffer. When",
                                "next we meet, I would very much like to give you a sweet dessert."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("orleans_1"), Val::from(0)])?;
                        ctx.lines_as(
                            "Charles Orleans",
                            args![
                                "Yes, women with your",
                                "beauty definitely deserve",
                                "the luscious flavors of the",
                                "treats that only I can offer.",
                                "Until that day comes, I shall",
                                "reluctantly bid you adieu."
                            ],
                        )?;
                        step = CharlesOrleansCookStep::LEnd;
                        continue 'machine;
                    } else {
                        if ctx.var("cooking_q").get()? == 0 {
                            ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Oh, have you come here",
                                    "to learn cooking? ^333333*Sigh*^000000",
                                    "I don't feel like teaching",
                                    "anything today--in fact,",
                                    "I think teaching is a waste",
                                    "of my time! ^333333*Sigh*^000000 However..."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[Val::from("Um, are you talking to me?:Wait, don't you remember me?:Sir Orleans?")],
                            )? {
                                1 => {
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Who else would",
                                            "I be talking to?",
                                            "To Madeleine over",
                                            "there? Or that child",
                                            "holding that mangy",
                                            "cat? Sacrebleu!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("nyuang_4"), Val::from(2)])?;
                                    ctx.lines_as("Child with Cat", args!["Grrrrrrr!", "Rrrroreow!"])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args!["Goodness, you scared me!", "What an ill natured kid!", "Who brought this kid in?"],
                                    )?;
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "I can't remember every",
                                            "tyro who's begged me for",
                                            "instruction in the culinary",
                                            "arts. I could swear you've",
                                            "come here months ago, it's",
                                            "just--what was your name...?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args!["Bah! No matter.", "I suppose that's", "of no importance", "at the moment."],
                                    )?;
                                }
                                3 => {
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "E-excuse me? I may be",
                                            "your instructor, but you",
                                            "can call me by my first name.",
                                            "I know that I can be strict,",
                                            "but please: in the end, we are colleagues, even if I am superior."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_1"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Fine, fine...",
                                            "If you insist on your",
                                            "modicum of expressed",
                                            "respect, then you may",
                                            "call me ''Sir Charles.''"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Ah... That does have",
                                            "a rather fine ring to it.",
                                            "I actually earned that title",
                                            "from the king himself, even",
                                            "if I'm a knight only in title and manner, rather than strength."
                                        ],
                                    )?;
                                }
                                _ => {}
                            }
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Well then, let's get started",
                                    "today with making a simple",
                                    "dish. Okay, ^FF0000I don't teach recipes",
                                    "more than once^000000, ^FF0000so make sure",
                                    "that you write this down^000000. Now",
                                    "then, what shall we cook?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                            l_cook_m1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                            if l_cook_m1.clone() == 1 {
                                ctx.var("cooking_q").set(Val::from(1))?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Ahhh, how about",
                                        "'Fried Grasshopper Legs?'",
                                        "To the uninitiated, it may",
                                        "seem to be a disgusting dish,",
                                        "but trust me, its exquisite taste is pure pleasure for your palate."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Now, please bring",
                                        "^4D4DFF5 Grasshopper Legs^000000,",
                                        "^4D4DFF1 Cooking Oil^000000, and",
                                        "^4D4DFF1 Old Frying Pan^000000.",
                                        "Then, we can begin."
                                    ],
                                )?;
                                step = CharlesOrleansCookStep::LEnd;
                                continue 'machine;
                            } else if l_cook_m1.clone() == 2 {
                                ctx.var("cooking_q").set(Val::from(2))?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Ah, I've got it!",
                                        "Let's make ''Grape Juice",
                                        "Herbal Tea.'' The weather",
                                        "is perfect right now for",
                                        "a cool, refreshing drink."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Please bring",
                                        "^4D4DFF3 Grapes^000000, and",
                                        "^4D4DFF2 Red Potions^000000",
                                        "so that we can",
                                        "begin the lesson~"
                                    ],
                                )?;
                                step = CharlesOrleansCookStep::LEnd;
                                continue 'machine;
                            } else if l_cook_m1.clone() == 3 {
                                ctx.var("cooking_q").set(Val::from(3))?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "I've got it~",
                                        "We can make",
                                        "''Honey Grape Juice.''",
                                        "Please bring me the",
                                        "following ingredients so",
                                        "that we can begin the lesson."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "^4D4DFF1 Honey^000000,",
                                        "^4D4DFF2 Grapes^000000, and",
                                        "^4D4DFF1 Red Potion^000000."
                                    ],
                                )?;
                                step = CharlesOrleansCookStep::LEnd;
                                continue 'machine;
                            } else if l_cook_m1.clone() == 4 {
                                ctx.var("cooking_q").set(Val::from(4))?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Mmm, why don't we",
                                        "make ''Frog Egg and",
                                        "Squid Ink Soup?'' Those",
                                        "bereft of gourmet taste may",
                                        "think it's disgusting, but it's",
                                        "actually quite scrumptious."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Well then,",
                                        "please bring me",
                                        "^4D4DFF1 Bag of Grain^000000,",
                                        "^4D4DFF10 Spawns^000000, and",
                                        "^4D4DFF1 Squid Ink^000000."
                                    ],
                                )?;
                                step = CharlesOrleansCookStep::LEnd;
                                continue 'machine;
                            } else if l_cook_m1.clone() == 5 {
                                ctx.var("cooking_q").set(Val::from(5))?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Ah, I know what",
                                        "would be perfect right",
                                        "now. ''Steamed Crab",
                                        "Nippers.'' Now, please",
                                        "bring these ingredients",
                                        "so we can make this soup."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "We'll need",
                                        "^4D4DFF10 Green Herbs^000000,",
                                        "^4D4DFF10 Nippers^000000, and",
                                        "^4D4DFF1 Yellow Potion^000000."
                                    ],
                                )?;
                                step = CharlesOrleansCookStep::LEnd;
                                continue 'machine;
                            }
                            ctx.var("cooking_q").set(Val::from(6))?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "Ooh, you know what",
                                    "would be scrumptious?",
                                    "''Fried Monkey Tails.'' Yes,",
                                    "that sounds perfect! Please",
                                    "bring these ingredients so",
                                    "that I can teach you this dish."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Charles Orleans",
                                args![
                                    "We'll need",
                                    "^4D4DFF1 Frying Pan^000000,",
                                    "^4D4DFF5 Yoyo Tails^000000, and",
                                    "^4D4DFF1 Cooking Oil^000000."
                                ],
                            )?;
                            step = CharlesOrleansCookStep::LEnd;
                            continue 'machine;
                        } else {
                            if ctx.var("cooking_q").get()? == 1 {
                                if ((ctx.call(Function::CountItem, vec![Val::from(940)])?.number()? > 4
                                    && ctx.call(Function::CountItem, vec![Val::from(7031)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(7457)])?.number()? > 0)
                                {
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Finally, you're here! Never",
                                            "forget: your ingredients must",
                                            "always be as fresh as possible.",
                                            "If not, your cuisine will be much poorer in quality. Now, let me",
                                            "explain how to make this dish."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Scrub the Grasshopper Legs",
                                            "as cleanly as you can before",
                                            "placing them in the Frying Pan.",
                                            "Afterwards, pour half a bottle of Cooking Oil and fry the legs at",
                                            "high heat for about 20 minutes."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Now, follow the instructions",
                                            "that I've just given you to the",
                                            "letter! Hmmm... Good, good.",
                                            "That's not bad at all. Alright,",
                                            "you're almost there..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "There, you're done!",
                                            "The presentation can use",
                                            "a little work, but at least you",
                                            "know this recipe now. That's",
                                            "all for today, so please go",
                                            "and practice on your own now."
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(940), Val::from(5)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7031), Val::from(1)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7457), Val::from(1)])?;
                                    ctx.var("cooking_q").set(Val::from(7))?;
                                    ctx.call(Function::GetItem, vec![Val::from(12041), Val::from(1)])?;
                                    step = CharlesOrleansCookStep::LEnd;
                                    continue 'machine;
                                }
                                ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                if l_talk_j.clone() == 1 {
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Hurry and bring me",
                                            "the ingredients to make",
                                            "Fried Grasshopper Legs!",
                                            "If a restaurant patron had",
                                            "ordered this, then you'd",
                                            "already be making him wait!"
                                        ],
                                    )?;
                                    step = CharlesOrleansCookStep::LEnd;
                                    continue 'machine;
                                }
                                ctx.lines_as(
                                    "Charles Orleans",
                                    args![
                                        "Just go and ambush those",
                                        "happy-go-lucky grasshoppers",
                                        "just playing in the fields. Hurry and smash them, then rip their",
                                        "legs off--but be humane about it! "
                                    ],
                                )?;
                                step = CharlesOrleansCookStep::LEnd;
                                continue 'machine;
                            } else {
                                if ctx.var("cooking_q").get()? == 2 {
                                    if (ctx.call(Function::CountItem, vec![Val::from(514)])?.number()? > 2
                                        && ctx.call(Function::CountItem, vec![Val::from(501)])?.number()? > 1)
                                    {
                                        ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "Finally, you're here! Never",
                                                "forget: your ingredients must",
                                                "always be as fresh as possible.",
                                                "If not, your cuisine will be much poorer in quality. Now, let me",
                                                "explain how to make this dish."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "You extract the juice",
                                                "from the Grapes like this--",
                                                "we can't use pre-made Grape",
                                                "Juice for the sake of freshness. Then, you need to boil the Red",
                                                "Potions in a bain-marie..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "When the Red Potions",
                                                "reach the right consistency,",
                                                "gently stir in the juice that you just squeezed from the Grapes.",
                                                "Now, I want you to try it. Hmm... That's not bad... Good, good..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "There, you're done!",
                                                "The presentation can use",
                                                "a little work, but at least you",
                                                "know this recipe now. That's",
                                                "all for today, so please go",
                                                "and practice on your own now."
                                            ],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(514), Val::from(3)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(501), Val::from(2)])?;
                                        ctx.var("cooking_q").set(Val::from(7))?;
                                        ctx.call(Function::GetItem, vec![Val::from(12046), Val::from(1)])?;
                                        step = CharlesOrleansCookStep::LEnd;
                                        continue 'machine;
                                    }
                                    ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                    l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                    if l_talk_j.clone() == 1 {
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "What are you doing?",
                                                "Hurry and bring me the",
                                                "ingredients for Grape Juice",
                                                "Herbal Tea! At a real restaurant, you'd never be able to take your",
                                                "time like this! Quickly, now!"
                                            ],
                                        )?;
                                        step = CharlesOrleansCookStep::LEnd;
                                        continue 'machine;
                                    }
                                    ctx.lines_as(
                                        "Charles Orleans",
                                        args![
                                            "Having trouble finding",
                                            "Grapes? Just pop open",
                                            "those cute little Poporings...",
                                            "Of course, you should try to",
                                            "be humane when you hunt them..."
                                        ],
                                    )?;
                                    step = CharlesOrleansCookStep::LEnd;
                                    continue 'machine;
                                } else {
                                    if ctx.var("cooking_q").get()? == 3 {
                                        if ((ctx.call(Function::CountItem, vec![Val::from(518)])?.number()? > 0
                                            && ctx.call(Function::CountItem, vec![Val::from(514)])?.number()? > 1)
                                            && ctx.call(Function::CountItem, vec![Val::from(501)])?.number()? > 0)
                                        {
                                            ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                            ctx.lines_as(
                                                "Charles Orleans",
                                                args![
                                                    "Finally, you're here! Never",
                                                    "forget: your ingredients must",
                                                    "always be as fresh as possible.",
                                                    "If not, your cuisine will be much poorer in quality. Now, let me",
                                                    "explain how to make this dish."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                            ctx.lines_as(
                                                "Charles Orleans",
                                                args![
                                                    "First, skin the Grapes",
                                                    "and extract the seeds. Then,",
                                                    "blend the Grapes with the",
                                                    "Honey. Take this blended",
                                                    "mixture and carefully stir",
                                                    "it into the Red Potion..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Charles Orleans",
                                                args![
                                                    "When the pulp is fully",
                                                    "mixed into the Red Potion,",
                                                    "you'll be finished. Now, go",
                                                    "and try making it yourself.",
                                                    "Right, that's good. Yes...",
                                                    "Wait, wait! Okay, there you go~"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                            ctx.lines_as(
                                                "Charles Orleans",
                                                args![
                                                    "There, you're done!",
                                                    "The presentation can use",
                                                    "a little work, but at least you",
                                                    "know this recipe now. That's",
                                                    "all for today, so please go",
                                                    "and practice on your own now."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(518), Val::from(1)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(514), Val::from(2)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(501), Val::from(1)])?;
                                            ctx.var("cooking_q").set(Val::from(7))?;
                                            ctx.call(Function::GetItem, vec![Val::from(12061), Val::from(1)])?;
                                            step = CharlesOrleansCookStep::LEnd;
                                            continue 'machine;
                                        }
                                        ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                        l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                        if l_talk_j.clone() == 1 {
                                            ctx.lines_as(
                                                "Charles Orleans",
                                                args![
                                                    "What's taking you so",
                                                    "long? You should have",
                                                    "brought me the ingredients",
                                                    "to make Honey Grape Juice",
                                                    "a while ago. Hurry it up!"
                                                ],
                                            )?;
                                            step = CharlesOrleansCookStep::LEnd;
                                            continue 'machine;
                                        }
                                        ctx.lines_as(
                                            "Charles Orleans",
                                            args![
                                                "You're having trouble",
                                                "finding some Honey for the",
                                                "Honey Grape Juice, aren't you?",
                                                "Just go and hunt some bears,",
                                                "they're always carrying some",
                                                "of that Honey around."
                                            ],
                                        )?;
                                        step = CharlesOrleansCookStep::LEnd;
                                        continue 'machine;
                                    } else {
                                        if ctx.var("cooking_q").get()? == 4 {
                                            if ((ctx.call(Function::CountItem, vec![Val::from(577)])?.number()? > 0
                                                && ctx.call(Function::CountItem, vec![Val::from(908)])?.number()? > 9)
                                                && ctx.call(Function::CountItem, vec![Val::from(1024)])?.number()? > 0)
                                            {
                                                ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "Charles Orleans",
                                                    args![
                                                        "Finally, you're here! Never",
                                                        "forget: your ingredients must",
                                                        "always be as fresh as possible.",
                                                        "If not, your cuisine will be much poorer in quality. Now, let me",
                                                        "explain how to make this dish."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "Charles Orleans",
                                                    args![
                                                        "Mill the grain until",
                                                        "it's a fine flour, then",
                                                        "boil the Squid Ink at",
                                                        "medium heat. Once it",
                                                        "bubbles, pour in the flour."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Charles Orleans",
                                                    args![
                                                        "Keep stirring, slowly",
                                                        "adding the Spawns. When",
                                                        "it all boils again, reduce the",
                                                        "heat and simmer for about 10",
                                                        "minutes. Okay, now you try it.",
                                                        "That's good, good... Alright~"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                                ctx.lines_as(
                                                    "Charles Orleans",
                                                    args![
                                                        "There, you're done!",
                                                        "The presentation can use",
                                                        "a little work, but at least you",
                                                        "know this recipe now. That's",
                                                        "all for today, so please go",
                                                        "and practice on your own now."
                                                    ],
                                                )?;
                                                ctx.call(Function::DelItem, vec![Val::from(577), Val::from(1)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(908), Val::from(10)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(1024), Val::from(1)])?;
                                                ctx.var("cooking_q").set(Val::from(7))?;
                                                ctx.call(Function::GetItem, vec![Val::from(12056), Val::from(1)])?;
                                                step = CharlesOrleansCookStep::LEnd;
                                                continue 'machine;
                                            }
                                            ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                            l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                            if l_talk_j.clone() == 1 {
                                                ctx.lines_as(
                                                    "Charles Orleans",
                                                    args![
                                                        "What are you doing?",
                                                        "You're supposed to be",
                                                        "preparing ingredients",
                                                        "for Frog Egg and Squid",
                                                        "Ink Soup right now!"
                                                    ],
                                                )?;
                                                step = CharlesOrleansCookStep::LEnd;
                                                continue 'machine;
                                            }
                                            ctx.lines_as(
                                                "Charles Orleans",
                                                args![
                                                    "You have to be careful",
                                                    "when you're handling Frog",
                                                    "Eggs. If you feed them raw to",
                                                    "somebody, well, their flavor is",
                                                    "decidely less than magnifique."
                                                ],
                                            )?;
                                            step = CharlesOrleansCookStep::LEnd;
                                            continue 'machine;
                                        } else {
                                            if ctx.var("cooking_q").get()? == 5 {
                                                if ((ctx.call(Function::CountItem, vec![Val::from(960)])?.number()? > 9
                                                    && ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 9)
                                                    && ctx.call(Function::CountItem, vec![Val::from(503)])?.number()? > 0)
                                                {
                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "Charles Orleans",
                                                        args![
                                                            "Finally, you're here! Never",
                                                            "forget: your ingredients must",
                                                            "always be as fresh as possible.",
                                                            "If not, your cuisine will be much poorer in quality. Now, let me",
                                                            "explain how to make this dish."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "Charles Orleans",
                                                        args![
                                                            "Boil the Nippers in Yellow",
                                                            "Potion on low heat for about",
                                                            "30 minutes. Then, bring it down",
                                                            "to a simmer and carefully stir",
                                                            "in the Green Herbs one by one."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Charles Orleans",
                                                        args![
                                                            "This is an easy recipe, but",
                                                            "I still want you to demonstrate",
                                                            "for me. Okay, let's see now...",
                                                            "You're doing fine. Now, wait...",
                                                            "Good, good, okay, it's ready",
                                                            "for the Green Herbs now..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                                    ctx.lines_as(
                                                        "Charles Orleans",
                                                        args![
                                                            "There, you're done!",
                                                            "The presentation can use",
                                                            "a little work, but at least you",
                                                            "know this recipe now. That's",
                                                            "all for today, so please go",
                                                            "and practice on your own now."
                                                        ],
                                                    )?;
                                                    ctx.call(Function::DelItem, vec![Val::from(960), Val::from(10)])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(511), Val::from(10)])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(503), Val::from(1)])?;
                                                    ctx.var("cooking_q").set(Val::from(7))?;
                                                    ctx.call(Function::GetItem, vec![Val::from(12051), Val::from(1)])?;
                                                    step = CharlesOrleansCookStep::LEnd;
                                                    continue 'machine;
                                                }
                                                ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                                if l_talk_j.clone() == 1 {
                                                    ctx.lines_as(
                                                        "Charles Orleans",
                                                        args![
                                                            "Shouldn't you be",
                                                            "preparing all of the",
                                                            "ingredients for Steamed",
                                                            "Crab Nippers? You need",
                                                            "to work quickly for those",
                                                            "hungry restaurant patrons!"
                                                        ],
                                                    )?;
                                                    step = CharlesOrleansCookStep::LEnd;
                                                    continue 'machine;
                                                }
                                                ctx.lines_as(
                                                    "Charles Orleans",
                                                    args![
                                                        "It shouldn't be too hard",
                                                        "to gather Nippers. Just",
                                                        "find some Vadons and crush",
                                                        "them, making sure to rip off",
                                                        "their Nippers. That sounds strange, I know, but just do it."
                                                    ],
                                                )?;
                                                step = CharlesOrleansCookStep::LEnd;
                                                continue 'machine;
                                            } else {
                                                if ctx.var("cooking_q").get()? == 6 {
                                                    if ((ctx.call(Function::CountItem, vec![Val::from(942)])?.number()? > 4
                                                        && ctx.call(Function::CountItem, vec![Val::from(7031)])?.number()? > 0)
                                                        && ctx.call(Function::CountItem, vec![Val::from(7457)])?.number()? > 0)
                                                    {
                                                        ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "Finally, you're here! Never",
                                                                "forget: your ingredients must",
                                                                "always be as fresh as possible.",
                                                                "If not, your cuisine will be much poorer in quality. Now, let me",
                                                                "explain how to make this dish."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "Pluck the hair from the",
                                                                "tails and rinse them well",
                                                                "under cold water. Pour half",
                                                                "a bottle of Cooking Oil unto",
                                                                "a preheated pan, and then",
                                                                "quickly fry the tails."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "The trick is to fry the",
                                                                "tails quickly without burning",
                                                                "them, so you'll probably want",
                                                                "to cook using medium-high heat.",
                                                                "Show me what you've learned now... Alright, that's not bad... Hmmm..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "There, you're done!",
                                                                "The presentation can use",
                                                                "a little work, but at least you",
                                                                "know this recipe now. That's",
                                                                "all for today, so please go",
                                                                "and practice on your own now."
                                                            ],
                                                        )?;
                                                        ctx.call(Function::DelItem, vec![Val::from(942), Val::from(5)])?;
                                                        ctx.call(Function::DelItem, vec![Val::from(7031), Val::from(1)])?;
                                                        ctx.call(Function::DelItem, vec![Val::from(7457), Val::from(1)])?;
                                                        ctx.var("cooking_q").set(Val::from(7))?;
                                                        ctx.call(Function::GetItem, vec![Val::from(12066), Val::from(1)])?;
                                                        step = CharlesOrleansCookStep::LEnd;
                                                        continue 'machine;
                                                    }
                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                    l_talk_j = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                                                    if l_talk_j.clone() == 1 {
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "You better go prepare",
                                                                "those ingredients for Fried",
                                                                "Monkey Tails are quickly as",
                                                                "you can. In a real restaurant,",
                                                                "you'd never able to take your",
                                                                "time like this. Toute allure!"
                                                            ],
                                                        )?;
                                                        step = CharlesOrleansCookStep::LEnd;
                                                        continue 'machine;
                                                    }
                                                    ctx.lines_as(
                                                        "Charles Orleans",
                                                        args![
                                                            "You need more tails?",
                                                            "Just sneak up on some",
                                                            "Yoyos, swiftly kill them,",
                                                            "and then slice off their tails.",
                                                            "You're a beginner, but I won't",
                                                            "allow you to be inhumane!"
                                                        ],
                                                    )?;
                                                    step = CharlesOrleansCookStep::LEnd;
                                                    continue 'machine;
                                                } else {
                                                    if ctx.var("cooking_q").get()? == 7 {
                                                        ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "Oh... I'm so exhausted!",
                                                                "I have too much wisdom and",
                                                                "skills to pass on! Teaching is",
                                                                "not an endeavor I enjoy, but",
                                                                "I do realize it is necessary",
                                                                "for my cuisine to survive me..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::Cutin, vec![Val::from("orleans_1"), Val::from(0)])?;
                                                        ctx.lines_as(
                                                            "Charles Orleans",
                                                            args![
                                                                "Peser le bien et le mal...",
                                                                "Even though it pains me,",
                                                                "I suppose I have to continue",
                                                                "teaching until one of you can",
                                                                "become a worthy successor.",
                                                                "It will take some time..."
                                                            ],
                                                        )?;
                                                        step = CharlesOrleansCookStep::LEnd;
                                                        continue 'machine;
                                                    } else {
                                                        if ctx.var("cooking_q").get()? == 8 {
                                                            ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                                                            ctx.lines_as(
                                                                "Charles Orleans",
                                                                args![
                                                                    "Strange, strange...",
                                                                    "What is that kid and",
                                                                    "that cat doing here?",
                                                                    "The kitchen is no place",
                                                                    "for them--at the very least,",
                                                                    "not for pets, you know."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                            ctx.lines_as(
                                                                "Charles Orleans",
                                                                args![
                                                                    "Pardon moi, child,",
                                                                    "but would you remove",
                                                                    "yourself and your cat",
                                                                    "from the premises? This",
                                                                    "is a kitchen, and everything",
                                                                    "here needs to be clean!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.call(Function::Cutin, vec![Val::from("nyuang_1"), Val::from(2)])?;
                                                            ctx.lines_as("Child with Cat", args!["...Nyaaa?", "...Meow?"])?;
                                                            ctx.next()?;
                                                            match runtime::select_values(
                                                                ctx,
                                                                &[Val::from(
                                                                    "What's your name, kid?:Do you want to eat something?:Here, kitty~:Get out!",
                                                                )],
                                                            )? {
                                                                1 => {
                                                                    ctx.call(
                                                                        Function::Emotion,
                                                                        vec![
                                                                            ctx.constant("ET_OK")?,
                                                                            ctx.call(
                                                                                Function::GetNpcId,
                                                                                vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                            )?,
                                                                        ],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Child with Cat",
                                                                        args!["Nyaaa~", "nyaaa~", "(Purrrrrr)"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            ".....Who told you",
                                                                            "to ask her name?",
                                                                            "You don't even understand what she is saying."
                                                                        ],
                                                                    )?;
                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                    continue 'machine;
                                                                }
                                                                2 => {
                                                                    ctx.call(Function::Cutin, vec![Val::from("nyuang_2"), Val::from(2)])?;
                                                                    ctx.call(
                                                                        Function::Emotion,
                                                                        vec![
                                                                            ctx.constant("ET_HNG")?,
                                                                            ctx.call(
                                                                                Function::GetNpcId,
                                                                                vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                            )?,
                                                                        ],
                                                                    )?;
                                                                    ctx.lines_as("Child with Cat", args!["Nyaaaa~", "(Purrrrrr~)"])?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "That was a good idea...",
                                                                            "Offering them food to get",
                                                                            "them to leave. Mon dieu, if",
                                                                            "the child won't talk to us...",
                                                                            "Still, we need to get those",
                                                                            "two out of the kitchen."
                                                                        ],
                                                                    )?;
                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                    continue 'machine;
                                                                }
                                                                3 => {
                                                                    ctx.call(Function::Cutin, vec![Val::from("nyuang_3"), Val::from(2)])?;
                                                                    ctx.call(
                                                                        Function::Emotion,
                                                                        vec![
                                                                            ctx.constant("ET_SMILE")?,
                                                                            ctx.call(
                                                                                Function::GetNpcId,
                                                                                vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                            )?,
                                                                        ],
                                                                    )?;
                                                                    ctx.lines_as(
                                                                        "Child with Cat",
                                                                        args!["Nyahahaha~", "Nyhhahaha~", "(Meow, meow~)"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "...Are you playing with that kid...?",
                                                                            "...Then I don't need you to be here."
                                                                        ],
                                                                    )?;
                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                    continue 'machine;
                                                                }
                                                                4 => {
                                                                    ctx.call(
                                                                        Function::Emotion,
                                                                        vec![
                                                                            ctx.constant("ET_THINK")?,
                                                                            ctx.call(
                                                                                Function::GetNpcId,
                                                                                vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                            )?,
                                                                        ],
                                                                    )?;
                                                                    ctx.lines_as("Child with Cat", args!["...", "(Meow?)"])?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("nyuang_4"), Val::from(2)])?;
                                                                    ctx.call(
                                                                        Function::Emotion,
                                                                        vec![
                                                                            ctx.constant("ET_FRET")?,
                                                                            ctx.call(
                                                                                Function::GetNpcId,
                                                                                vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                            )?,
                                                                        ],
                                                                    )?;
                                                                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                                    ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                                                                    ctx.lines_as("Child with Cat", args!["Grrrrrrr!", "(RRRrrreow!)"])?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "Oh, look out!",
                                                                            "You should have",
                                                                            "been more careful",
                                                                            "handling that cat..."
                                                                        ],
                                                                    )?;
                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                    continue 'machine;
                                                                }
                                                                _ => {}
                                                            }
                                                        } else {
                                                            if ctx.var("cooking_q").get()? == 9 {
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                                                                ctx.lines_as(
                                                                    "Charles Orleans",
                                                                    args![
                                                                        "Alright, enough",
                                                                        "is enough. We can't",
                                                                        "continue to cook if we",
                                                                        "have live animals in the",
                                                                        "kitchen. It's a violation",
                                                                        "of our sanitary standards!"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                                ctx.lines_as(
                                                                    "Charles Orleans",
                                                                    args![
                                                                        "I'm sorry, mon chere,",
                                                                        "but you have to leave.",
                                                                        "Child, please take your",
                                                                        "cat and head out the door",
                                                                        "before your feline can touch",
                                                                        "or shed on any of the food!"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("nyuang_4"), Val::from(2)])?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_FRET")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.lines_as("Child with Cat", args!["Grrrrrrr!", "RRRRreow!"])?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::DoNpcEvent,
                                                                    vec![Val::from("Wickebine#cook::OnEnable")],
                                                                )?;
                                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_DELIGHT")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("job_black_hucke01"), Val::from(1)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Wickebine",
                                                                    args![
                                                                        "Oh...!",
                                                                        "Nyuyang, there",
                                                                        "you are! What are",
                                                                        "you doing here in",
                                                                        "Charles's kitchen?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                                                                ctx.lines_as(
                                                                    "Charles Orleans",
                                                                    args![
                                                                        "M-Madam Wickebine...!",
                                                                        "Forgive me, you surprised",
                                                                        "me by appearing from out",
                                                                        "of nowhere. Do you happen",
                                                                        "to know this young child?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("job_black_hucke02"), Val::from(1)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Wickebine",
                                                                    args![
                                                                        "Oh, Nyuyang here is my",
                                                                        "little sister. It may be hard",
                                                                        "to see the resemblance...",
                                                                        "Anyway, I've been looking",
                                                                        "all over for her. Are you",
                                                                        "bothering Charles, Nyuyang?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("nyuang_3"), Val::from(2)])?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_SMILE")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Child with Cat",
                                                                    args!["Nyuuuunyuuu~", "nyuuuunyuuu~", "Meooooow~"],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                                                                ctx.lines_as("Charles Orleans", args!["...!!!"])?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("job_black_hucke03"), Val::from(1)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Wickebine",
                                                                    args![
                                                                        "Oh, so you have",
                                                                        "been bothering him!",
                                                                        "You think Charles",
                                                                        "wants you to leave?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                                                                ctx.lines_as(
                                                                    "Charles Orleans",
                                                                    args![
                                                                        "Hahahahah, what",
                                                                        "are you talking about!",
                                                                        "Nonsense! How can ",
                                                                        "such a cute little belle",
                                                                        "be of any trouble to me?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_7"), Val::from(0)])?;
                                                                ctx.lines_as(
                                                                    "Charles Orleans",
                                                                    args![
                                                                        "As a matter of fact,",
                                                                        "I was just about to treat",
                                                                        "this precious petit and",
                                                                        "her little cat to some",
                                                                        "of my delicious cuisine.",
                                                                        "So do not worry, Madam~"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("job_black_hucke01"), Val::from(1)],
                                                                )?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_QUESTION")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Wickebine#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Wickebine",
                                                                    args![
                                                                        "Are you sure, Charles?",
                                                                        "I know how serious you",
                                                                        "are about your cooking,",
                                                                        "and I don't want Nyuyang",
                                                                        "to disturb you in any way..."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("job_black_hucke02"), Val::from(1)],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Wickebine",
                                                                    args![
                                                                        "Oh, Charles, you've",
                                                                        "been nothing but kind",
                                                                        "to me. I'm glad that you're",
                                                                        "also taking care of Nyuyang.",
                                                                        "Well then, take care~"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("nyuang_3"), Val::from(2)])?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_SMILE")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.lines_as(
                                                                    "Child with Cat",
                                                                    args!["Nyahahaha~", "Nyahahaha~", "(Meow, meow~)"],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(
                                                                    Function::Cutin,
                                                                    vec![Val::from("job_black_hucke02"), Val::from(255)],
                                                                )?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_DELIGHT")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Wickebine#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.call(
                                                                    Function::DoNpcEvent,
                                                                    vec![Val::from("Wickebine#cook::OnDisable")],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                                                ctx.call(
                                                                    Function::Emotion,
                                                                    vec![
                                                                        ctx.constant("ET_THINK")?,
                                                                        ctx.call(
                                                                            Function::GetNpcId,
                                                                            vec![Val::from(0), Val::from("Child with Cat#cook")],
                                                                        )?,
                                                                    ],
                                                                )?;
                                                                ctx.lines_as("Charles Orleans", args!["......"])?;
                                                                ctx.next()?;
                                                                ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                                ctx.lines_as(
                                                                    "Charles Orleans",
                                                                    args!["Ah...", "There goes a true", "lady... Madam Wickebine..."],
                                                                )?;
                                                                step = CharlesOrleansCookStep::LEnd;
                                                                continue 'machine;
                                                            } else {
                                                                if ctx.var("cooking_q").get()? == 10 {
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_5"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "It's been bothering",
                                                                            "me that his highness",
                                                                            "has been missing for",
                                                                            "a while. Why doesn't",
                                                                            "anybody know where he is?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_4"), Val::from(0)])?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_3"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "Wise and benevolent",
                                                                            "King Tristram III would",
                                                                            "never abandon his subjects.",
                                                                            "I dearly hope that nothing",
                                                                            "serious has happened to him..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from("orleans_6"), Val::from(0)])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "Can it be possible that",
                                                                            "our beloved king would",
                                                                            "have enemies? He's done",
                                                                            "nothing but good for the",
                                                                            "Rune-Midgarts Kingdom",
                                                                            "and the rest of the world!"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "I can't imagine a great",
                                                                            "man like him to be in any",
                                                                            "sort of trouble. It makes me",
                                                                            "me laugh whenever anyone",
                                                                            "suggests that he is hiding",
                                                                            "in the Schwarzwald Republic..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_GO")?])?;
                                                                    ctx.lines_as(
                                                                        "Charles Orleans",
                                                                        args![
                                                                            "Praise the glories of the",
                                                                            "Rune-Midgarts Kingdom!",
                                                                            "Long live King Tristram III!"
                                                                        ],
                                                                    )?;
                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                    continue 'machine;
                                                                } else {
                                                                    if ctx.var("cooking_q").get()? == 11 {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("orleans_5"), Val::from(0)],
                                                                        )?;
                                                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                                                        ctx.lines_as(
                                                                            "Charles Orleans",
                                                                            args![
                                                                                "Oh, I'm in great need of",
                                                                                "some rest. Unless you ",
                                                                                "have something incredibly",
                                                                                "important to ask of me,",
                                                                                "please let me take a break~"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        match runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from(
                                                                                "I want to learn more recipes.:For whom do you cook?:I'm sorry to bother you...",
                                                                            )],
                                                                        )? {
                                                                            1 => {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("orleans_7"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "You want to learn more",
                                                                                        "recipes? I suppose that",
                                                                                        "you should borrow another",
                                                                                        "cookbook then. Before that,",
                                                                                        "please return the cookbook",
                                                                                        "that you were studying, okay?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "Now, choose the cookbook",
                                                                                        "that you want to borrow by",
                                                                                        "entering a level from 1 to 5.",
                                                                                        "There are more advanced books,",
                                                                                        "but I'm not lending those out.",
                                                                                        "Oh, and enter 0 to cancel."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                let (input, status) =
                                                                                    runtime::input_number(ctx, None, None)?;
                                                                                l_new_book = input;
                                                                                if (l_new_book.clone().number()? < 0
                                                                                    || l_new_book.clone().number()? > 5)
                                                                                {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_5"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Hmm...",
                                                                                            "I asked to you to",
                                                                                            "enter a level from",
                                                                                            "1 to 5. Those are the",
                                                                                            "only cookbooks that I will",
                                                                                            "lend out to my students."
                                                                                        ],
                                                                                    )?;
                                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                                    continue 'machine;
                                                                                } else if l_new_book.clone() == 0 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_5"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "So you changed your mind?",
                                                                                            "It would be a good idea to",
                                                                                            "study the recipes that you",
                                                                                            "have right now before trying",
                                                                                            "something new, I suppose."
                                                                                        ],
                                                                                    )?;
                                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                                    continue 'machine;
                                                                                }
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "So you wanted to borrow a",
                                                                                        ((Val::from("Level ") + l_new_book.clone())
                                                                                            + Val::from(" Cookbook, eh?")),
                                                                                        "Oh, would you please tell",
                                                                                        "me the level of the cookbook",
                                                                                        "that you are returning to me?"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                let (input, status) =
                                                                                    runtime::input_number(ctx, None, None)?;
                                                                                l_old_book = input;
                                                                                if (l_old_book.clone().number()? < 0
                                                                                    || l_old_book.clone().number()? > 5)
                                                                                {
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "There must be some",
                                                                                            "kind of mistake-- I only",
                                                                                            "lend out cookbooks from",
                                                                                            "levels 1 to 5. Hmm, well, ask",
                                                                                            "me again when you remember",
                                                                                            "which cookbook you have, okay?"
                                                                                        ],
                                                                                    )?;
                                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                                    continue 'machine;
                                                                                } else if l_old_book.clone() == 0 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_5"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "So you changed your mind?",
                                                                                            "It would be a good idea to",
                                                                                            "study the recipes that you",
                                                                                            "have right now before trying",
                                                                                            "something new, I suppose."
                                                                                        ],
                                                                                    )?;
                                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                                    continue 'machine;
                                                                                } else if l_old_book
                                                                                    .clone()
                                                                                    .loosely_equals(&l_new_book.clone())
                                                                                {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_4"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Wait, wait...",
                                                                                            "Why do you want to",
                                                                                            "borrow a copy of the",
                                                                                            "cookbook that you already",
                                                                                            "have? I guess you made",
                                                                                            "some sort of mistake?"
                                                                                        ],
                                                                                    )?;
                                                                                    step = CharlesOrleansCookStep::LEnd;
                                                                                    continue 'machine;
                                                                                } else {
                                                                                    if ctx
                                                                                        .call(
                                                                                            Function::CountItem,
                                                                                            vec![(Val::from(7471) + l_old_book.clone())],
                                                                                        )?
                                                                                        .number()?
                                                                                        < 1
                                                                                    {
                                                                                        ctx.lines_as(
                                                                                            "Charles Orleans",
                                                                                            args![
                                                                                                "Wait, wait...",
                                                                                                "Why don't you have",
                                                                                                "the book that you said",
                                                                                                "that you'd return to me?",
                                                                                                "Find it first, and then I can",
                                                                                                "lend another cookbook to you."
                                                                                            ],
                                                                                        )?;
                                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                                        continue 'machine;
                                                                                    }
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_2"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.mes("[Charles Orleans]")?;
                                                                                    let subject5 = l_old_book.clone();
                                                                                    if subject5 == 1 {
                                                                                        ctx.lines(args![
                                                                                            "Ah, so you're done",
                                                                                            "with the Level 1 Cookbook.",
                                                                                            "That's good, that means you're",
                                                                                            "ready to graduate from the most",
                                                                                            "basic of basics. From now on,",
                                                                                            "the recipes will be harder..."
                                                                                        ])?;
                                                                                    } else if subject5 == 2 {
                                                                                        ctx.lines(args![
                                                                                            "Ah, so what did you",
                                                                                            "think of the recipes in",
                                                                                            "the Level 2 Cookbook?",
                                                                                            "Homestyle cooking may be",
                                                                                            "simple, but it should never",
                                                                                            "be neglected by chefs."
                                                                                        ])?;
                                                                                    } else if subject5 == 3 {
                                                                                        ctx.lines(args!["Ah, done with the Level 3", "Cookbook already? The recipes", "in there are really good when you're cooking romantic dinners.", "They'll come in handy someday,", "if you know what I mean."])?;
                                                                                    } else if subject5 == 4 {
                                                                                        ctx.lines(args![
                                                                                            "So you've finished the",
                                                                                            "Level 4 Cookbook. That's",
                                                                                            "no small feat! You've got to",
                                                                                            "use very strange ingredients",
                                                                                            "to create delicious cuisine!"
                                                                                        ])?;
                                                                                    } else if subject5 == 5 {
                                                                                        ctx.lines(args![
                                                                                            "You're done with the",
                                                                                            "Level 5 Cookbook? Good",
                                                                                            "work: most beginners don't",
                                                                                            "even get this far. I suppose",
                                                                                            "you'll want to review some",
                                                                                            "of the easier recipes now~"
                                                                                        ])?;
                                                                                    }
                                                                                    ctx.next()?;
                                                                                }
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("orleans_1"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "Now, before I let you",
                                                                                        "borrow one of my beloved",
                                                                                        "cookbooks, I have a small",
                                                                                        "condition that you must fulfill. "
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                if l_new_book.clone() == 1 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_7"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "When I was a young child,",
                                                                                            "my family was destitute to",
                                                                                            "the point where we live off",
                                                                                            "leftover vegetables. Even",
                                                                                            "Monster's Feed was a prime",
                                                                                            "delicacy back in those days."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Back then, my father would",
                                                                                            "always serve us Pumpkin.",
                                                                                            "I grew sick of it as a boy, but",
                                                                                            "now it brings back memories",
                                                                                            "of those days of innocence."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Why don't we do this?",
                                                                                            "If you bring me 10 Pumpkins,",
                                                                                            "I will let you borrow one of",
                                                                                            "my Level 1 Cookbooks."
                                                                                        ],
                                                                                    )?;
                                                                                    if ctx
                                                                                        .call(Function::CountItem, vec![Val::from(535)])?
                                                                                        .number()?
                                                                                        > 9
                                                                                    {
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from(
                                                                                                "Give 10 Pumpkins and Current Cookbook:Cancel",
                                                                                            )],
                                                                                        )?) == 1
                                                                                        {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from("orleans_6"), Val::from(0)],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Perfect, you've brought",
                                                                                                    "me 10 Pumpkins! I can't",
                                                                                                    "want to taste these flavors",
                                                                                                    "that I used to experience",
                                                                                                    "everyday in my childhood."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Oh, how I miss the",
                                                                                                    "taste of Pumpkins!",
                                                                                                    "Ahhh, how nostalgic~"
                                                                                                ],
                                                                                            )?;
                                                                                            step = CharlesOrleansCookStep::LEnd;
                                                                                            continue 'machine;
                                                                                        }
                                                                                    } else {
                                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                                        continue 'machine;
                                                                                    }
                                                                                } else if l_new_book.clone() == 2 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_7"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Today, I have a craving",
                                                                                            "for a cup of tea. Of course,",
                                                                                            "you cannot enjoy tea without",
                                                                                            "crackers or cookies. Please",
                                                                                            "bring me 5 Well-Baked Cookies",
                                                                                            "to borrow my Level 2 Cookbook."
                                                                                        ],
                                                                                    )?;
                                                                                    if ctx
                                                                                        .call(Function::CountItem, vec![Val::from(538)])?
                                                                                        .number()?
                                                                                        > 4
                                                                                    {
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from(
                                                                                                "Give Cookies and Current Cookbook:Cancel",
                                                                                            )],
                                                                                        )?) == 1
                                                                                        {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from("orleans_6"), Val::from(0)],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Oh, you brought these",
                                                                                                    "cookies much quicker",
                                                                                                    "than I had expected!",
                                                                                                    "Great, now I can put",
                                                                                                    "the tea on, relax, then",
                                                                                                    "enjoy a delicious snack~"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Ohh...",
                                                                                                    "I must have some tea",
                                                                                                    "soon... But the experience",
                                                                                                    "isn't complete without any",
                                                                                                    "Well-Baked Cookies to munch~"
                                                                                                ],
                                                                                            )?;
                                                                                            step = CharlesOrleansCookStep::LEnd;
                                                                                            continue 'machine;
                                                                                        }
                                                                                    } else {
                                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                                        continue 'machine;
                                                                                    }
                                                                                } else if l_new_book.clone() == 3 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_7"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "You know that specialty",
                                                                                            "dish from Amatsu? I've",
                                                                                            "been craving that lately.",
                                                                                            "Please bring me 5 Sushi,",
                                                                                            "and I'll let you borrow a",
                                                                                            "Level 3 Cookbook, okay?"
                                                                                        ],
                                                                                    )?;
                                                                                    if ctx
                                                                                        .call(Function::CountItem, vec![Val::from(551)])?
                                                                                        .number()?
                                                                                        > 4
                                                                                    {
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from(
                                                                                                "Give Sushi and Current Cookbook:Cancel",
                                                                                            )],
                                                                                        )?)
                                                                                        .is_true()
                                                                                        {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from("orleans_6"), Val::from(0)],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Ooh, these look so fresh!",
                                                                                                    "And the presentation is also",
                                                                                                    "wonderful! These must have",
                                                                                                    "been prepared by a skilled chef! "
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Ahhh, Sushi...",
                                                                                                    "It's one of the few",
                                                                                                    "things I don't know",
                                                                                                    "how to make extremely",
                                                                                                    "well. Can you believe that?"
                                                                                                ],
                                                                                            )?;
                                                                                            step = CharlesOrleansCookStep::LEnd;
                                                                                            continue 'machine;
                                                                                        }
                                                                                    } else {
                                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                                        continue 'machine;
                                                                                    }
                                                                                } else if l_new_book.clone() == 4 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_7"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Oh, I'm in the mood for",
                                                                                            "some cuisine from Kunlun.",
                                                                                            "Would you bring me some of",
                                                                                            "that delicious Bao? 5 would",
                                                                                            "be perfect. Then, I'll let you",
                                                                                            "borrow my Level 4 Cookbook."
                                                                                        ],
                                                                                    )?;
                                                                                    if ctx
                                                                                        .call(Function::CountItem, vec![Val::from(553)])?
                                                                                        .number()?
                                                                                        > 4
                                                                                    {
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from(
                                                                                                "Give Bao and Current Cookbook:Cancel",
                                                                                            )],
                                                                                        )?) == 1
                                                                                        {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from("orleans_6"), Val::from(0)],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Great, you actually",
                                                                                                    "brought them! These",
                                                                                                    "Bao look especially",
                                                                                                    "scrumptious! I can't",
                                                                                                    "wait to have a taste!"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Oh...",
                                                                                                    "It's been so long",
                                                                                                    "since I've had a taste",
                                                                                                    "of that delicious Bao.",
                                                                                                    "I'd cook it myself, but",
                                                                                                    "I don't know the secret!"
                                                                                                ],
                                                                                            )?;
                                                                                            step = CharlesOrleansCookStep::LEnd;
                                                                                            continue 'machine;
                                                                                        }
                                                                                    } else {
                                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                                        continue 'machine;
                                                                                    }
                                                                                } else if l_new_book.clone() == 5 {
                                                                                    ctx.call(
                                                                                        Function::Cutin,
                                                                                        vec![Val::from("orleans_7"), Val::from(0)],
                                                                                    )?;
                                                                                    ctx.lines_as(
                                                                                        "Charles Orleans",
                                                                                        args![
                                                                                            "Lately, my pantry has been",
                                                                                            "in some dire need of Shoots.",
                                                                                            "They're a tasty ingredient with",
                                                                                            "unignorable health value. Bring",
                                                                                            "me 10 of those, and you can",
                                                                                            "borrow a Level 5 Cookbook."
                                                                                        ],
                                                                                    )?;
                                                                                    if ctx
                                                                                        .call(Function::CountItem, vec![Val::from(711)])?
                                                                                        .number()?
                                                                                        > 9
                                                                                    {
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from(
                                                                                                "Give Shoots and Current Cookbook:Quit",
                                                                                            )],
                                                                                        )?) == 1
                                                                                        {
                                                                                            ctx.call(
                                                                                                Function::Cutin,
                                                                                                vec![Val::from("orleans_6"), Val::from(0)],
                                                                                            )?;
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "Goodness, these are",
                                                                                                    "some high quality Shoots!",
                                                                                                    "These look so good, I'm",
                                                                                                    "sure that you you can",
                                                                                                    "even eat them raw!"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Charles Orleans",
                                                                                                args![
                                                                                                    "I'm going to need to",
                                                                                                    "cook with those Shoots",
                                                                                                    "soon, so I'd appreciate it",
                                                                                                    "if you'd do this little favor~"
                                                                                                ],
                                                                                            )?;
                                                                                            step = CharlesOrleansCookStep::LEnd;
                                                                                            continue 'machine;
                                                                                        }
                                                                                    } else {
                                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                                        continue 'machine;
                                                                                    }
                                                                                }
                                                                                if l_old_book.clone() == 1 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(7472), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_old_book.clone() == 2 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(7473), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_old_book.clone() == 3 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(7474), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_old_book.clone() == 4 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(7475), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_old_book.clone() == 5 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(7476), Val::from(1)],
                                                                                    )?;
                                                                                }
                                                                                if l_new_book.clone() == 1 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(535), Val::from(10)],
                                                                                    )?;
                                                                                    ctx.call(
                                                                                        Function::GetItem,
                                                                                        vec![Val::from(7472), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_new_book.clone() == 2 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(538), Val::from(5)],
                                                                                    )?;
                                                                                    ctx.call(
                                                                                        Function::GetItem,
                                                                                        vec![Val::from(7473), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_new_book.clone() == 3 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(551), Val::from(5)],
                                                                                    )?;
                                                                                    ctx.call(
                                                                                        Function::GetItem,
                                                                                        vec![Val::from(7474), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_new_book.clone() == 4 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(553), Val::from(5)],
                                                                                    )?;
                                                                                    ctx.call(
                                                                                        Function::GetItem,
                                                                                        vec![Val::from(7475), Val::from(1)],
                                                                                    )?;
                                                                                } else if l_new_book.clone() == 5 {
                                                                                    ctx.call(
                                                                                        Function::DelItem,
                                                                                        vec![Val::from(711), Val::from(10)],
                                                                                    )?;
                                                                                    ctx.call(
                                                                                        Function::GetItem,
                                                                                        vec![Val::from(7476), Val::from(1)],
                                                                                    )?;
                                                                                }
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "Well, as promised,",
                                                                                        "here's the cookbook",
                                                                                        "that you asked for. Take",
                                                                                        "good care of it--don't sell",
                                                                                        "it or lose it or anything like",
                                                                                        "that. Good luck cooking now~"
                                                                                    ],
                                                                                )?;
                                                                                step = CharlesOrleansCookStep::LEnd;
                                                                                continue 'machine;
                                                                            }
                                                                            2 => {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("orleans_3"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "What do you mean,",
                                                                                        "''Who do I cook for?''",
                                                                                        "That's a strange question",
                                                                                        "with a simple answer. I'm",
                                                                                        "an artiste that must bring",
                                                                                        "more of my art into the world."
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("orleans_3"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "Wait, wait...",
                                                                                        "Have you been speaking",
                                                                                        "to Madeleine Chu? She",
                                                                                        "didn't say anything out",
                                                                                        "of the ordinary did she?",
                                                                                        "Because if she did, ignore her!"
                                                                                    ],
                                                                                )?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("orleans_6"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "No. There is",
                                                                                        "no special reason",
                                                                                        "why my spirit to create",
                                                                                        "culinary masterpieces has",
                                                                                        "been reinvigorated lately..."
                                                                                    ],
                                                                                )?;
                                                                                step = CharlesOrleansCookStep::LEnd;
                                                                                continue 'machine;
                                                                            }
                                                                            3 => {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("orleans_5"), Val::from(0)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "Charles Orleans",
                                                                                    args![
                                                                                        "Please, do not worry",
                                                                                        "yourself about it. Just",
                                                                                        "let me rest for now~"
                                                                                    ],
                                                                                )?;
                                                                                step = CharlesOrleansCookStep::LEnd;
                                                                                continue 'machine;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                    } else {
                                                                        ctx.lines_as(
                                                                            "Charles Orleans",
                                                                            args!["Mon dieu!", "An error has", "occurred!"],
                                                                        )?;
                                                                        step = CharlesOrleansCookStep::LEnd;
                                                                        continue 'machine;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                step = CharlesOrleansCookStep::LEnd;
                continue 'machine;
            }
            CharlesOrleansCookStep::LEnd => {
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn charles_orleans_cook(ctx: &Ctx) -> Script {
    charles_orleans_cook_run(ctx, CharlesOrleansCookStep::Start, Vec::new()).map(|_| ())
}
