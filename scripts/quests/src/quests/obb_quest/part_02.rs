use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn old_man_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a moment! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please enlighten your weight -",
            "- and try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("thai_head").get()?.number()? >= 11 && ctx.var("thai_head").get()?.number()? <= 15) {
        'b1: {
            let subject1 = ctx.var("thai_head").get()?;
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(11))
                && !subject1.loosely_equals(&Val::from(12))
                && !subject1.loosely_equals(&Val::from(13))
                && !subject1.loosely_equals(&Val::from(14))
                && !subject1.loosely_equals(&Val::from(15));
            if !matched1 && subject1.loosely_equals(&Val::from(11)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Elder Creek",
                    args!["Good day, adventurer.", "What kind of business", "do you have with me?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Are you seeking fortune or",
                        "mental relief? If not, do",
                        "you wish to finish something",
                        "you've started?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Elder Creek", args!["Answer me, adventurer!"])?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I seek fortune!:I seek...wisdom.:I want to finish what I've started.")],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1))
                        && !subject2.loosely_equals(&Val::from(2))
                        && !subject2.loosely_equals(&Val::from(3));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Elder Creek",
                            args![
                                "You're honest...!",
                                "However, I suggest that you try not to be enamored with wealth."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder Creek",
                            args![
                                "Do you know the exact meaning",
                                "of the old saying...",
                                "^0000FFToo much is as bad as too little^000000?",
                                "I will give you what you want if",
                                "you answer the way I expect you to."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "It's necessary to have a lot!:Too dumb to see the future.:Too much of a good thing can be bad.:Cringe to the powerful.",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as("Elder Creek", args!["...", "Get out of my sight,", "you greedy fool."])?;
                                ctx.var("thai_head").set(Val::from(12))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Bah! The more ignorant they are, the more arrogantly they act!",
                                        "Did you think that I would accept just any meaning?",
                                        "Take care, you ignorant fool."
                                    ],
                                )?;
                                ctx.var("thai_head").set(Val::from(12))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["That's right. You know it pretty", "well. That adage serves as a lesson in greed."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "There is a man I've known for a",
                                        "long time...he was so greedy for",
                                        "money and fortune, he didn't care what other people thought of him."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Consequently, people around him",
                                        "didn't like him at all. In the end, he lost his friends due to his greed."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Of course, this rule applies to",
                                        "you as well. That sort of thing is a universal principle, after all."
                                    ],
                                )?;
                                ctx.var("thai_head").set(Val::from(13))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            4 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        ".....",
                                        "Did you think that I would accept just any meaning??",
                                        "Take care, you ignorant fool."
                                    ],
                                )?;
                                ctx.var("thai_head").set(Val::from(12))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Elder Creek",
                            args![
                                "I see. I have been trying to help",
                                "other people who need advice in",
                                "order to improve their lives."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elder Creek",
                            args![
                                "Since you need my advice",
                                "as well, I will try to",
                                "impart my wisdom in a",
                                "way that you will understand."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elder Creek", args!["Now, tell me what is bothering right now."])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Boy/girlfriend issue.:Financial problem.:Bored to death.:Career issue.:I want money.:Give me items, old man.",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "I'm envious of you! Still, young",
                                        "folk like you are lucky to",
                                        "have such problems...sadly,",
                                        "no one but yourself can really",
                                        "help you in those kinds of",
                                        "situations..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "However, the advice I can give you",
                                        "is this: Be honest with your",
                                        "beloved. One lie leads to",
                                        "another lie in no time."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "If you treat the person you love",
                                        "as special to you, that person",
                                        "will eventually come to",
                                        "understand your feelings..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "And, remember this...",
                                        "Everyone has a different way of showing that they love someone.",
                                        "So...don't be discouraged if your feelings aren't returned."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["But still, I believe being honest", "is the best way to gain your", "true love."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "....well, that is all I can tell",
                                        "you. I hope you will be happy",
                                        "with that nugget of wisdom."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.var("thai_head").set(Val::from(14))?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "...yes, everybody has their own",
                                        "financial problems. However, think of it in this way:"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["Money is important to make", "a living, but it's not what life is all about."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Some may have the life goal of",
                                        "making fortunes, but I don't think it's the happiest way of living."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "As you see, there are powerful",
                                        "people in this world who can control our lives. But oftentimes..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "They are too greedy to be in that",
                                        "position and they don't have",
                                        "their priorities straight.",
                                        "Instead of improving the world, they use their power to only help themselves."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["If I were them, I wouldn't want to", "rule the world in that way."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["Don't you think we need somebody who can change this world??"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        ".....I apologize for being short",
                                        "tempered. I just wanted to let",
                                        "you know that money is not everything. But, it is something you need to live."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "I'm aware that greedy people",
                                        "ridicule this belief, saying",
                                        "that it is a poor man's effort",
                                        "to protect his pride."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "But, I still believe I am right.",
                                        "After all...I am a Sage.",
                                        "I hope you will be happy with my wise advice."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.var("thai_head").set(Val::from(14))?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Yes, we always need to be",
                                        "entertained. But...it",
                                        "seems you may not be satisfied with anything."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "It's all about the various tastes",
                                        "and interests people will have.",
                                        "Things other people enjoy might not",
                                        "displease you. Also, if your",
                                        "mind is unwilling to be happy,",
                                        "you will not enjoy life."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Here's an example of having",
                                        "the right frame of mind.",
                                        "Let's say we're cleaning",
                                        "the street for the town,",
                                        "which can be hard, grueling work..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "You're always covered in dust.",
                                        "In the summer, your body is",
                                        "sticky with sweat...",
                                        "In the winter, you'll be",
                                        "be out in the freezing cold..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["However, even so, it will be different to someone with a different perspective."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Ideally, we should try",
                                        "to find joy in making",
                                        "other people happy.",
                                        "By cleaning the street, we can",
                                        "give others a reason to smile."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Tourists passing through the street",
                                        "can enjoy its cleanliness...",
                                        "Overall, it makes the entire community look good."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "If your parents see you cleaning",
                                        "the street, they will be proud of",
                                        "their child.",
                                        "...But now, I think I'm beginning to make no sense."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Oh well, I apologize.",
                                        "Just try to think positively.",
                                        "With the right attitude, eventually you will find something to enjoy."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Haha~ I can see you're just",
                                        "clicking the 'next' button on these",
                                        "windows, because you're sick and tired of this NPC conversation."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["....anyway, that is all.", "I hope you will be happy", "with my wise advice."],
                                )?;
                                ctx.close_window()?;
                                ctx.var("thai_head").set(Val::from(14))?;
                                return Err(Stop::End);
                            }
                            4 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "That's an agonizing thought for every young person.",
                                        "But you have a lot time to think about what you want to do in the future."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Experience more in many different",
                                        "places, and read as many books",
                                        "as you can. Focus on broadening your perspective."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "Then, you will realize what you really want to do.",
                                        "....that's all I can say.",
                                        "I hope you will be happy with my wise advice."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.var("thai_head").set(Val::from(14))?;
                                return Err(Stop::End);
                            }
                            5 => {
                                ctx.lines_as(
                                    "Elder Creek",
                                    args!["....", "I see, you chose the wrong answer in the first place."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elder Creek",
                                    args![
                                        "I believe this will be a lesson to be more honest with yourself.",
                                        "Choose the right answer next time!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            6 => {
                                ctx.lines_as("Elder Creek", args!["You rascal!"])?;
                                ctx.next()?;
                                ctx.lines_as("Elder Creek", args!["Get out of my sight immediately! I don't talk to trash!"])?;
                                ctx.var("thai_head").set(Val::from(12))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as("Elder Creek", args!["Yes, you need to see things to the end."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(12)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])? == 792 {
                    ctx.lines_as("Elder Creek", args!["....*Sigh* Okay.", "I forgive you."])?;
                    ctx.var("thai_head").set(Val::from(11))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "I do not wish to continue this conversation with you.",
                        "Get out of my sight immediately!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(13)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "...I was just testing you",
                        "in order to give you something",
                        "special. I'm the only person in the world that can give you this..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["I had to check if you are the right person as I initially thought.", "....Hmm."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "But you don't expect me to give something special without any compensation, do you?",
                        "Would you bring the items I want?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Elder Creek", args!["It's not a hard thing to do.", "That is..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["10 Brigan,", "15 Clam Shell,", "10 Crab Shell", "and 50 Cyfar."],
                )?;
                ctx.next()?;
                ctx.lines_as("Elder Creek", args!["Well, I am collecting those as my hobby."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["I have no doubt that you will be able to gather them all."],
                )?;
                ctx.next()?;
                ctx.lines_as("Elder Creek", args!["Do me this favor, young man."])?;
                ctx.var("thai_head").set(Val::from(15))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(14)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(30)])? == 5 {
                    ctx.lines_as("Elder Creek", args!["....hmm. I see you want more.", "Come back later."])?;
                    ctx.var("thai_head").set(Val::from(13))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Elder Creek",
                    args!["Is there any business left between", "us? Tell me, young adventurer..."],
                )?;
                ctx.next()?;
                ctx.mes("^3355FFYou feel there may be something you missed.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(15)) {
                matched1 = true;
            }
            if matched1 {
                if (((ctx.call(Function::CountItem, vec![Val::from(7054)])?.number()? > 9
                    && ctx.call(Function::CountItem, vec![Val::from(965)])?.number()? > 14)
                    && ctx.call(Function::CountItem, vec![Val::from(964)])?.number()? > 9)
                    && ctx.call(Function::CountItem, vec![Val::from(7053)])?.number()? > 49)
                {
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "I see you know how the world",
                            "works. When you need something",
                            "from someone, you must give",
                            "in order to receive.",
                            "Thank you for your kindness."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7054), Val::from(10)])?;
                    ctx.call(Function::DelItem, vec![Val::from(965), Val::from(15)])?;
                    ctx.call(Function::DelItem, vec![Val::from(964), Val::from(10)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7053), Val::from(50)])?;
                    ctx.next()?;
                    ctx.var("thai_head").set(Val::from(16))?;
                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Please, take this first.",
                            "If you would, please go talk to my",
                            "grandson later. He seemed to witness something horrible a while ago..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "I have no idea what he saw,",
                            "because the shock took away",
                            "his ability to speak.",
                            "When he recovers, I will tell you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "In fact, the items I have asked",
                            "for will be used to cure",
                            "his condition. I don't",
                            "really collect these as a",
                            "hobby..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Elder Creek", args!["I am sorry to cause you", "much trouble for my own good."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Remember, when you want something,",
                        "especially when it's special, you",
                        "should always offer something in exchange."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Elder Creek", args!["I want..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["10 Brigan,", "15 Clam Shell,", "10 Crab Shell and", "50 Cyfar."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("thai_head").get()?.number()? > 15 {
        ctx.lines_as(
            "Elder Creek",
            args![
                "Feel free to come back anytime...",
                "And if you need advice, I am more than willing to help you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Elder Creek", args!["Do you wish for me to give you advice, adventurer?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
            ctx.lines_as("Elder Creek", args!["Now, tell me what bothers you at the moment."])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Boy/girlfriend issue.:Financial problem.:Bored to death.:Career issue.:Give me items, old man.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "I'm envious of you! Still, young",
                            "folk like you are lucky to",
                            "have such problems...sadly,",
                            "no one but yourself can really",
                            "help you in those kinds of",
                            "situations..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "However, the advice I can give you",
                            "is this: Be honest with your",
                            "beloved. One lie leads to",
                            "another lie in no time."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "If you treat the person you love",
                            "as special to you, that person",
                            "will eventually come to",
                            "understand your feelings..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "And, remember this...",
                            "Everyone has a different way of showing that they love someone.",
                            "So...don't be discouraged if your feelings aren't returned."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args!["But still, I believe being honest", "is the best way to gain your", "true love."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "....well, that is all I can tell",
                            "you. I hope you will be happy",
                            "with that nugget of wisdom."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "...yes, everybody has their own",
                            "financial problems. However, think of it in this way:"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args!["Money is important to make", "a living, but it's not what life is all about."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Some may have the life goal of",
                            "making fortunes, but I don't think it's the happiest way of living."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "As you see, there are powerful",
                            "people in this world who can control our lives. But oftentimes..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "They are too greedy to be in that",
                            "position and they don't have",
                            "their priorities straight.",
                            "Instead of improving the world, they use their power to only help themselves."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args!["If I were them, I wouldn't want to", "rule the world in that way."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args!["Don't you think we need somebody who can change this world??"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            ".....I apologize for being short",
                            "tempered. I just wanted to let",
                            "you know that money is not everything. But, it is something you need to live."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "I'm aware that greedy people",
                            "ridicule this belief, saying",
                            "that it is a poor man's effort",
                            "to protect his pride."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "But, I still believe I am right.",
                            "After all...I am a Sage.",
                            "I hope you will be happy with my wise advice."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Yes, we always need to be",
                            "entertained. But...it seems",
                            "you may not be satisfied",
                            "with anything."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "It's all about the various tastes",
                            "and interests people will have.",
                            "Things other people enjoy might",
                            "displease you. Also, if your",
                            "mind is unwilling to be happy,",
                            "you will not enjoy life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Here's an example of having",
                            "the right frame of mind.",
                            "Let's say we're cleaning",
                            "the street for the town,",
                            "which can be hard, grueling work..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "You're always covered in dust.",
                            "In the summer, your body is",
                            "sticky with sweat...",
                            "In the winter, you'll be",
                            "be out in the freezing cold..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args!["However, even so, it will be different to someone with a different perspective."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Ideally, we should try",
                            "to find joy in making",
                            "other people happy.",
                            "By cleaning the street, we can",
                            "give others a reason to smile."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Tourists passing through the",
                            "street can enjoy its",
                            "cleanliness...",
                            "Overall, it makes the entire community look good."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "If your parents see you cleaning",
                            "the street, they will be proud of",
                            "their child.",
                            "...But now, I think I'm beginning to make no sense."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Oh well, I apologize.",
                            "Just try to think positively.",
                            "With the right attitude, eventually you will find something to enjoy."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Haha~ I can see you're just",
                            "clicking the 'next' button on",
                            "these windows, because you're sick and tired of this NPC conversation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args!["....anyway, that is all.", "I hope you will be happy", "with my wise advice."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "That's an agonizing thought for every young person.",
                            "But you have a lot time to think about what you want to do in the future."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Experience more in many different",
                            "places, and read as many books",
                            "as you can. Focus on broadening your perspective."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elder Creek",
                        args![
                            "Then, you will realize what you really want to do.",
                            "....that's all I can say.",
                            "I hope you will be happy with my wise advice."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.lines_as("Elder Creek", args!["You rascal!"])?;
                    ctx.next()?;
                    ctx.lines_as("Elder Creek", args!["Get out of my sight immediately! I don't talk to trash!"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("comodo"), Val::from(196), Val::from(255)])?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        ctx.lines_as(
            "Elder Creek",
            args!["I hope you will enjoy your life. Remember, time flies and you live only once."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Elder Creek",
            args!["I have been helping people by giving them advice to improve their lives."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Elder Creek",
            args!["Since you need my help as well, I shall endeavor to impart some wisdom."],
        )?;
        ctx.next()?;
        ctx.lines_as("Elder Creek", args!["Now tell me, what bothers you right now?"])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Boy/girlfriend issue.:Financial problem.:Bored to death.:Career issue.:Give me items, old man.",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "I'm envious of you! Still, young",
                        "folk like you are lucky to",
                        "have such problems...sadly,",
                        "no one but yourself can really",
                        "help you in those kinds of",
                        "situations..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "However, the advice I can give you",
                        "is this: Be honest with your",
                        "beloved. One lie leads to",
                        "another lie in no time."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "If you treat the person you love",
                        "as special to you, that person",
                        "will eventually come to",
                        "understand your feelings..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "And, remember this...",
                        "Everyone has a different way of showing that they love someone.",
                        "So...don't be discouraged if your feelings aren't returned."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["But still, I believe being honest", "is the best way to gain your", "true love."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "....well, that is all I can tell",
                        "you. I hope you will be happy",
                        "with that nugget of wisdom."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "...yes, everybody has their own",
                        "financial problems. However, think of it in this way:"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["Money is important to make", "a living, but it's not what life is all about."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Some may have the life goal of",
                        "making fortunes, but I don't think it's the happiest way of living."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "As you see, there are powerful",
                        "people in this world who can control our lives. But oftentimes..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "They are too greedy to be in that",
                        "position and they don't have",
                        "their priorities straight.",
                        "Instead of improving the world, they use their power to only help themselves."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["If I were them, I wouldn't want to", "rule the world in that way."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["Don't you think we need somebody who can change this world??"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        ".....I apologize for being short",
                        "tempered. I just wanted to let",
                        "you know that money is not everything. But, it is something you need to live."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "I'm aware that greedy people",
                        "ridicule this belief, saying",
                        "that it is a poor man's effort",
                        "to protect his pride."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "But, I still believe I am right.",
                        "After all...I am a Sage.",
                        "I hope you will be happy with my wise advice."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Yes, we always need to be",
                        "entertained. But...it seems",
                        "you may not be satisfied",
                        "with anything."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "It's all about the various tastes",
                        "and interests people will have.",
                        "Things other people enjoy might",
                        "displease you. Also, if your",
                        "mind is unwilling to be happy,",
                        "you will not enjoy life."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Here's an example of having",
                        "the right frame of mind.",
                        "Let's say we're cleaning",
                        "the street for the town,",
                        "which can be hard, grueling work..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "You're always covered in dust.",
                        "In the summer, your body is",
                        "sticky with sweat...",
                        "In the winter, you'll be",
                        "be out in the freezing cold..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["However, even so, it will be different to someone with a different perspective."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Ideally, we should try",
                        "to find joy in making",
                        "other people happy.",
                        "By cleaning the street, we can",
                        "give others a reason to smile."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Tourists passing through the",
                        "street can enjoy its",
                        "cleanliness...",
                        "Overall, it makes the entire community look good."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "If your parents see you cleaning",
                        "the street, they will be proud of",
                        "their child.",
                        "...But now, I think I'm beginning to make no sense."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Oh well, I apologize.",
                        "Just try to think positively.",
                        "With the right attitude, eventually you will find something to enjoy."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Haha~ I can see you're just",
                        "clicking the 'next' button on",
                        "these windows, because you're sick and tired of this NPC conversation."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args!["....anyway, that is all.", "I hope you will be happy", "with my wise advice."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "That's an agonizing thought for every young person.",
                        "But you have a lot time to think about what you want to do in the future."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Experience more in many different",
                        "places, and read as many books",
                        "as you can. Focus on broadening your perspective."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elder Creek",
                    args![
                        "Then, you will realize what you really want to do.",
                        "....that's all I can say.",
                        "I hope you will be happy with my wise advice."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            5 => {
                ctx.lines_as("Elder Creek", args!["You rascal!"])?;
                ctx.next()?;
                ctx.lines_as("Elder Creek", args!["Get out of my sight immediately! I don't talk to trash!"])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("comodo"), Val::from(196), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn old_man_thai(ctx: &Ctx) -> Script {
    old_man_thai_body(ctx, Vec::new()).map(|_| ())
}

fn tommy_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("thai_head").get()?.number()? >= 6 && ctx.var("thai_head").get()?.number()? <= 11) {
        let subject1 = ctx.var("thai_head").get()?;
        if subject1 == 6 {
            ctx.lines_as(
                "Tommy",
                args!["Wahhhhhh~~~!!", "Daddy~~ let me have a Munak~~~!", "Waaaahhhh...."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tommy",
                args!["I want my Munak~~!!", "Other people have one, why can't I have one too?!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["Munak Munak Munak Munak~~~"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["Umm..Tommy..."])?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["Munak Munak Munak Munak~~~"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["Tommy, your daddy is kind of busy now..."])?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["Munak Munak Munak Munak!"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["Sigh...alright, alright..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 7 {
            ctx.lines_as(
                "Tommy",
                args![
                    "*cries*...I want Munak!....*cries*",
                    "My friends all have Munak, and I don't have it...",
                    "Waaaahhhhh~~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 8 {
            ctx.lines_as("Tommy", args!["Daddy, I hate you!!", "I hate all of you!!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Tommy",
                args![
                    "Waaaahhh~~~!",
                    "I don't care what anybody's telling",
                    "me! I'll be bad because I hate everyone!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 9 {
            ctx.lines_as(
                "Tommy",
                args!["Heh~!", "I'm so happy~", "We're gonna get a Munak~!", "Heh heh heh~!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 10 {
            ctx.lines_as(
                "Tommy",
                args!["Heh~!", "I am so happy~", "We're gonna get a Munak~!", "Heh heh heh~!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 11 {
            ctx.lines_as(
                "Tommy",
                args!["Heh~!", "I like you~ ", "Cuz now we're gonna", "get a Munak~", "So happy! *smiles*"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("thai_head").get()?.number()? > 11 {
        ctx.lines_as("Tommy", args!["Daddy...I won't ask you for something that's too hard for you."])?;
        ctx.next()?;
        ctx.lines_as(
            "Tommy",
            args![
                "I saw you're huffing and puffing when we went to catch a Munak.",
                "I'm so sorry that I gave you a hard time, daddy..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tommy",
            args!["I will be good from now on.", "I will be nice to my Daddy, to you and to anyone."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tommy",
            args!["My dad told me that I will be a good kid if I don't behave bad sometimes."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Tommy",
            args![
                "I hate to see my daddy resting in his room all day in the weekends.",
                "But I am so happy today because my daddy and I will take a walk together!!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn tommy_thai(ctx: &Ctx) -> Script {
    tommy_thai_body(ctx, Vec::new()).map(|_| ())
}
