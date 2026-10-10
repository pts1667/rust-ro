use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn chief_gon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 50 {
        if ctx.var("b_sword").get()? == 0 {
            ctx.var("b_sword").set(Val::from(1))?;
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "Hmm...?",
                    "Oh, hello there~",
                    "I am Shi Yan Wen, the chief of",
                    "this village. Allow me to personally welcome you to Kunlun."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "Although our village hasn't",
                    "associated with other towns very",
                    "much, we have recently begun",
                    "to allow visitors coming from Alberta."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "I feel that this town has been",
                    "isolated for too long. Because",
                    "of that, people in this town aren't too friendly with visitors yet."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "Moreover, we've been having a recent problem with thieves",
                    "that have been enjoying themselves far too much in Kunlun...",
                    "Well, you've come to visit here",
                    "so I hope you enjoy your stay."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("b_sword").get()?.number()? < 18 {
            'b1: {
                let subject1 = ctx.var("b_sword").get()?;
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3))
                    && !subject1.loosely_equals(&Val::from(4))
                    && !subject1.loosely_equals(&Val::from(5))
                    && !subject1.loosely_equals(&Val::from(6))
                    && !subject1.loosely_equals(&Val::from(7))
                    && !subject1.loosely_equals(&Val::from(8))
                    && !subject1.loosely_equals(&Val::from(9))
                    && !subject1.loosely_equals(&Val::from(10))
                    && !subject1.loosely_equals(&Val::from(11))
                    && !subject1.loosely_equals(&Val::from(12))
                    && !subject1.loosely_equals(&Val::from(13))
                    && !subject1.loosely_equals(&Val::from(14))
                    && !subject1.loosely_equals(&Val::from(15))
                    && !subject1.loosely_equals(&Val::from(16))
                    && !subject1.loosely_equals(&Val::from(17));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Oh, hello there~",
                            "I am Shi Yan Wen, the chief of",
                            "this village. Allow me to personally welcome you to Kunlun."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Although our village hasn't",
                            "associated with other towns very",
                            "much, we have recently begun",
                            "to allow visitors coming from Alberta."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "I feel that this town has been",
                            "isolated for too long. Because",
                            "of that, people in this town aren't too friendly with visitors yet."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Moreover, we've been having a recent problem with thieves",
                            "that have been enjoying themselves far too much in Kunlun...",
                            "Well, you've come to visit here",
                            "so I hope you enjoy your stay."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Oh, it's you~",
                            "How do you like it here so far?",
                            "As you've probably noticed,",
                            "the village isn't that peaceful, huh?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("It's alright.:I heard that something was stolen...")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Shi Yan Wen",
                            args![
                                "Well, I'm glad you don't mind.",
                                "Just watch out for robbers,",
                                "and try not to act suspicious",
                                "in the village."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Hmm...you've heard of it?",
                            "It was just last night when the",
                            "robbery occurred. The rumors",
                            "are true. Sadly, there are lots of thieves out there in the village..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("......:What was stolen?:Was anyone hurt?")])? {
                        1 => {
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "I can't believe it really",
                                    "happened... Well, if you see",
                                    "any suspicious characters around,",
                                    "or find what was stolen, please let me know."
                                ],
                            )?;
                            ctx.var("b_sword").set(Val::from(3))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "Well...",
                                    "Erm...it was...",
                                    "...just an ordinary sword.",
                                    "But to us, it's been a family treasure for many generations."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "I must find this sword",
                                    "no matter what!",
                                    "...but I can't go find it just",
                                    "by myself. I'm just too busy."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "You know how busy it is to be the",
                                    "chief of a village. This is very troubling...*Sigh*"
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Hope you find it soon.:Can I find it for you?")],
                            )?) == 1
                            {
                                ctx.lines_as(
                                    "Shi Yan Wen",
                                    args!["um..Thank you.", "If you somehow come across it,", "please let me know."],
                                )?;
                                ctx.var("b_sword").set(Val::from(11))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "Oh!~ Are you serious??",
                                    "...The people in the village are",
                                    "very fearful these days because",
                                    "of the thieves. It is so difficult",
                                    "to ask them for help..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args!["If you would help me find the sword, I will surely repay you for your efforts."],
                            )?;
                            ctx.var("b_sword").set(Val::from(3))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "Fortunately, no one was hurt.",
                                    "However, the thieves took a",
                                    "valuable family treasure",
                                    "which has been passed down from",
                                    "generation to generation."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "I must find this sword",
                                    "no matter what!",
                                    "...But I can't go find it just",
                                    "by myself. I'm just too busy."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "You know how busy it is to be the",
                                    "chief of a village. This is very troubling...*Sigh*"
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Hope you find it soon.:Can I find it for you?")],
                            )?) == 1
                            {
                                ctx.lines_as(
                                    "Shi Yan Wen",
                                    args!["Um..Thank you.", "If you somehow come across it,", "please let me know."],
                                )?;
                                ctx.var("b_sword").set(Val::from(11))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args![
                                    "Oh!~ Are you serious??",
                                    "...The people in the village are",
                                    "very fearful these days because",
                                    "of the thieves. It is so difficult",
                                    "to ask them for help..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shi Yan Wen",
                                args!["If you would help me find the sword, I will surely repay you for your efforts."],
                            )?;
                            ctx.var("b_sword").set(Val::from(3))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                    matched1 = true;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(5)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Haven't found it yet?",
                            "There's no rush, take it easy.",
                            "You have to take care of",
                            "youself first before",
                            "doing favors for others."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(6)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Oh, you found a clue?",
                            "So far, it looks like",
                            "you're doing good work.",
                            "Take this old family medicine, it might be of use sometime soon."
                        ],
                    )?;
                    ctx.var("b_sword").set(Val::from(7))?;
                    ctx.call(Function::GetItem, vec![Val::from(504), Val::from(3)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(7)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "It might be helpful for you to",
                            "know that the thief sustained",
                            "an injury, so he is probably not very far from here."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(8)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "What...?",
                            "My sword has been broken!?",
                            "Unbelievable~!!",
                            "How could this happen??"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "First it's stolen, and now",
                            "it's in pieces... *Sob*",
                            "Would you please search",
                            "for the rest of my sword?",
                            "It means so much to my family..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(9)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "*Sob*...",
                            "...my sword... in pieces...",
                            "I beg of you, please find them",
                            "for me. I will give you something in return."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(10)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Oh!~",
                            "You've found the pieces for me~",
                            "I knew you could do it.",
                            "But the sword is still shattered.",
                            "What shall I do...?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "If it's okay with you,",
                            "would you repair my sword for me?",
                            "I'll repay you for your help."
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("No way.:Alright.")])?) == 1 {
                        ctx.lines_as(
                            "Shi Yan Wen",
                            args![
                                "Well, I see...you've been such a",
                                "nice person. I truly appreciate",
                                "your hard work. It would be",
                                "wonderful if you could help",
                                "me repair the sword, but I",
                                "will not force you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shi Yan Wen",
                            args![
                                "I'll find some way to repair",
                                "it. Without your help, I ",
                                "would have never found it.",
                                "Please accept this as a",
                                "token of my gratitude..."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(15))?;
                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shi Yan Wen",
                            args![
                                "If you find any information",
                                "or clues about that cursed",
                                "thief, please let me know.",
                                "I have my sword back but",
                                "there's no way I can forgive",
                                "this affront to my ancestors..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Such generosity...",
                            "I'm truly indebted to you...",
                            "I have no idea how this sword",
                            "broke into pieces, though.",
                            "You will probably need to find a",
                            "famous blacksmith to repair it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "As I already mentioned, this is",
                            "an important family treasure...",
                            "Oh! I just remembered--"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "There's a guy named ^555555Zuo Hei^000000",
                            "in the village. He has been to",
                            "many places around the world.",
                            "He may know of such a weaponsmith. Please seek this man out."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Oh, one last thing. This is",
                            "but a trinket, but please",
                            "accept this gift from me."
                        ],
                    )?;
                    ctx.var("b_sword").set(Val::from(14))?;
                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(11)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "The village is not in a good",
                            "mood these days, but there are",
                            "still lots of things to see in Kunlun."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(12)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Hmm.. no traces? No clues?",
                            "... Nothing? Well, that's",
                            "alright. Thank you for",
                            "trying to help.",
                            "Hmm...."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Shi Yan Wen", args!["Here...take this.", "Have a good time in Kunlun~"])?;
                    ctx.var("b_sword").set(Val::from(13))?;
                    ctx.call(Function::GetItem, vec![Val::from(504), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(13)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Is everything going well?",
                            "The village has not been",
                            "in a good mood, lately.",
                            "Still, please try to",
                            "enjoy yourself."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(14)) {
                    matched1 = true;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(15)) {
                    matched1 = true;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(16)) {
                    matched1 = true;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(17)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "I really appreciate what you are",
                            "doing. It's a big relief that",
                            "something is finally being done",
                            "about these thefts."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shi Yan Wen",
                        args![
                            "Perhaps now, peace will finally",
                            "come to the village, just like in the old days..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else if ctx.var("b_sword").get()?.number()? < 32 {
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "I appreciate what you are doing,",
                    "and your help has been",
                    "a relief to me. The Village",
                    "seems less tense, much like",
                    "it was in the old days."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "Although I hope you'll be",
                    "be able to repair my",
                    "sword soon, you've",
                    "already done so much for",
                    "me. I feel sorry for",
                    "asking you to do more."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("b_sword").get()? == 32 {
            if ctx.call(Function::CountItem, vec![Val::from(1123)])?.number()? < 1 {
                ctx.lines_as(
                    "Shi Yan Wen",
                    args![
                        "Hm.....",
                        "Not finished yet, huh?",
                        "Still, it's good to know",
                        "it's being repaired...",
                        "I'll be waiting, then."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "Oh, it's you~ Hello.",
                    "............",
                    "Is that my sword?",
                    "Wow, you've done it!",
                    "Good work!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shi Yan Wen",
                args!["This is a small gift for you.", "Please take it as thanks for a job well done!"],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(1123), Val::from(1)])?;
            ctx.var("b_sword").set(Val::from(33))?;
            ctx.call(Function::GetItem, vec![Val::from(2404), Val::from(1)])?;
            ctx.next()?;
            ctx.lines_as(
                "Shi Yan Wen",
                args![
                    "Thank you for all of your efforts.",
                    "With this sword repaired, I no",
                    "longer feel that I am shaming",
                    "my ancestors...",
                    "Heh heh...",
                    "Have a good time in Kunlun."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Shi Yan Wen",
            args![
                "Oh, it's you~",
                "Once again, I'd like to thank",
                "you for all of your help.",
                "Enjoy your stay in our village.",
                "Heh heh~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Shi Yan Wen",
        args![
            "Hmm...?",
            "Oh, hello there~",
            "I am Shi Yan Wen, the chief of",
            "this village. Allow me to personally welcome you to Kunlun."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shi Yan Wen",
        args![
            "Although our village hasn't",
            "associated with other towns very",
            "much, we have recently begun",
            "to allow visitors coming from Alberta."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shi Yan Wen",
        args![
            "I feel that this town has been",
            "isolated for too long. Because",
            "of that, people in this town aren't too friendly with visitors yet."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shi Yan Wen",
        args![
            "Moreover, we've been having a recent problem with thieves",
            "that have been enjoying themselves far too much in Kunlun...",
            "Well, you've come to visit here",
            "so I hope you enjoy your stay."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chief_gon(ctx: &Ctx) -> Script {
    chief_gon_body(ctx, Vec::new()).map(|_| ())
}

fn hostess_gon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 1 {
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "Oh...you're new here, right?",
                "Came from out of town?",
                "It's common to see lots of",
                "foreigners these days.",
                "It made people in the village busy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mei Yen Fang", args!["Oh, what am I saying...", "Want some wine?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes, please:No, it's okay.")])?) == 1 {
            ctx.lines_as(
                "Mei Yen Fang",
                args![
                    "Oooops~! Oh no...",
                    "A lot of customers came by earlier,",
                    "so now we're out of wine...",
                    "It's getting difficult to keep up with the increasing number of customers..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Mei Yen Fang",
            args!["Well, have fun in the village.", "Stop by again sometime."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    let subject1 = ctx.var("b_sword").get()?;
    if subject1 == 1 || subject1 == 2 {
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "Hey, you know what?",
                "The chief's house was robbed",
                "last night. I can't believe",
                "this happened...I guess this is the work of those thieves..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "How worrisome it all is...",
                "It could even happen to me!",
                "I better watch out...",
                "Oh, what am I saying?",
                "Enjoy your time in my shop...hehe~"
            ],
        )?;
        ctx.var("b_sword").set(Val::from(2))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 {
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "Oh, it's you again~",
                "I heard that you've decided",
                "to help our chief.",
                "Please do your best for him!",
                "Everyone in the village has",
                "been on edge..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "Do you see that guy over there",
                "leaning on the table?",
                "He seems to know about what",
                "happened last night, but...",
                "He's been drinking all night long."
            ],
        )?;
        ctx.var("b_sword").set(Val::from(4))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("b_sword").get()?.number()? > 3 && ctx.var("b_sword").get()?.number()? < 11) {
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "Hello there~",
                "Feeling tension in the village,",
                "huh? It's all because of those",
                "thieves... "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mei Yen Fang",
            args!["They're also making things hard for my business...", " ", "*Sigh*"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("b_sword").get()? == 11 || ctx.var("b_sword").get()? == 12) {
        ctx.lines_as(
            "Mei Yen Fang",
            args![
                "Feeling tension in the village,",
                "huh? I Hope the thief will get caught soon.",
                "..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Mei Yen Fang",
        args![
            "You caught him?!",
            "Wow, you're very brave.",
            "I should get ready to run the",
            "shop again. But I'll need to",
            "order some wine first."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mei Yen Fang",
        args![
            "Xue Bong drank all the wine in the",
            "shop, and I didn't restock any",
            "since everyone has been scared",
            "off by the thefts...",
            "But stop by next time.",
            "I'll have some wine ready."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hostess_gon(ctx: &Ctx) -> Script {
    hostess_gon_body(ctx, Vec::new()).map(|_| ())
}

fn man_in_hangover_gon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 4 {
        ctx.lines_as(
            "Xue Bong",
            args!["Ahhh.. my stomach.. my head..", "I shouldn't drink so much..", "Ehhhh...."],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()?.number()? < 6 {
        'b1: {
            let subject1 = ctx.var("b_sword").get()?;
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(4)) && !subject1.loosely_equals(&Val::from(5));
            if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.call(Function::CountItem, vec![Val::from(506)])?.number()? < 1 {
                    ctx.lines_as(
                        "Xue Bong",
                        args!["*Urk!* I feel sick...", "Can somebody bring me a potion?", "*Groan*...."],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Xue Bong",
                    args![
                        "Ohhh...my stomach...",
                        "I need something...",
                        "Moan~ Uh, hey you!",
                        "Could you give me one",
                        "of your ^00FF00Green_Potion^000000s?",
                        "I think I'm going to barf..."
                    ],
                )?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("No.:Here, drink this!")])?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as("Xue Bong", args!["Uhh...", "Are you sure?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sorry, I don't have any.:Here, you can have it!")])? {
                            1 => {
                                ctx.lines_as("Xue Bong", args!["C'mon, man...", "I...I'm in freakin' pain here..."])?;
                                ctx.var("b_sword").set(Val::from(12))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.call(Function::DelItem, vec![Val::from(506), Val::from(1)])?;
                                ctx.var("b_sword").set(Val::from(5))?;
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "Oh man...thanks.",
                                        "I thought you were teasing me.",
                                        "I feel much better now.",
                                        "So...uh, what brings you here?"
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("Heard about the thief?:Nothing.")])?) == 1 {
                                    ctx.lines_as(
                                        "Xue Bong",
                                        args![
                                            "Ah, a thief~",
                                            "Hmm...let me see..",
                                            "I went out for walk in the middle",
                                            "of the night while I was drinking.",
                                            "And I heard a noise."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Xue Bong",
                                        args![
                                            "I looked around and found that the",
                                            "area near the chief's house was",
                                            "brighter than any other area...",
                                            "It was odd...",
                                            "[Xue Bong]",
                                            "So I kept watching it and",
                                            "all of a sudden, I saw something",
                                            "moving on the rooftops..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Xue Bong",
                                        args![
                                            "It disappeared in a second.",
                                            "I was drunk, and it was dark",
                                            "outside. I have no idea",
                                            "whether it was a man,",
                                            "a poring, or if I had just",
                                            "drank too much..eheh."
                                        ],
                                    )?;
                                    ctx.var("b_sword").set(Val::from(6))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "Okay then, thanks again.",
                                        "Don't be a drinker like me,",
                                        "unless you want to suffer from",
                                        "serious hangovers. See you later~"
                                    ],
                                )?;
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
                        ctx.call(Function::DelItem, vec![Val::from(506), Val::from(1)])?;
                        ctx.var("b_sword").set(Val::from(5))?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
                        ctx.lines_as(
                            "Xue Bong",
                            args![
                                "Whew, Thanks!",
                                "I feel much better now.",
                                "Hmm...you seem new around here.",
                                "Anything you wanna know?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("About a thief...:How much have you been drinking?:Nothing~")])? {
                            1 => {
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "Ah, a thief, eh? Let's see...",
                                        "I went out for walk in the middle",
                                        "of the night while I was drinking.",
                                        "All of a sudden, I heard a strange noise..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "I looked around and found that the",
                                        "area near the chief's house was",
                                        "brighter than any other area. It was odd..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "So I kept watching it and,",
                                        "all of a sudden, I saw something moving on the rooftops..."
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("Where to?:Probably a Wild Rose.")])?) == 1 {
                                    ctx.lines_as(
                                        "Xue Bong",
                                        args!["Umm?", "Well..let me see..", "It came from... and head to...um...", "murmur.."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args!["^3355FFHe started mumbling for a bit^000000", "......"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Xue Bong",
                                        args![
                                            "Ah ha!! Right...",
                                            "A shrine, yeah, the thief was",
                                            "heading to a shrine and",
                                            "disappeared. I'm not",
                                            "sure if it was a human or an",
                                            "animal..."
                                        ],
                                    )?;
                                    ctx.var("b_sword").set(Val::from(6))?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Xue Bong",
                                        args!["Anything else I can help you with?", "I appreciate the potion."],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("Thanks for the information.:Stop drinking so much.:Okay, bye")],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                "Xue Bong",
                                                args!["I'll see you later then.", "I'm always be here drinking..eheh."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Xue Bong",
                                                args![
                                                    "Ehh..Don't mind me.",
                                                    "I'm a social drinker...",
                                                    " ",
                                                    "It's just I don't got nobody to be social with."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        3 => {
                                            ctx.lines_as(
                                                "Xue Bong",
                                                args!["Alright.", "Come again whenever you have", "any other questions."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "Yeah..maybe.",
                                        "It was so dark outside and I was",
                                        "drunk so I don't remember",
                                        "clearly. I'm pretty sure it was bigger than that, though."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "Ah..um....I..I don't remember.",
                                        "When I woke up, There were tons of",
                                        "empty bottles around me.",
                                        "..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FF.......",
                                    "For some reason, you can't discern his testimony's reliability...^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Xue Bong",
                                    args![
                                        "Alrighty then.",
                                        "Hope you don't ever drink like me",
                                        "in the future. You'll suffer",
                                        "for a long time if you do.",
                                        "You know what a hangover is, right?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(5)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Xue Bong",
                    args![
                        "Oh, it's you. Hey.",
                        "Thanks for the potion last time.",
                        "What are you up to?",
                        "Got any questions for me?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("About a thief last night...:Nope, just passing by..")])? {
                    1 => {
                        ctx.lines_as(
                            "Xue Bong",
                            args![
                                "Ah~ a thief...?",
                                "Hmm...let me see...",
                                "I went out for walk in the middle",
                                "of the night while I was drinking.",
                                "All of a sudden, I heard a strange noise..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Xue Bong",
                            args![
                                "I looked around and found that the",
                                "area near the chief's house was",
                                "brighter than any other area. It was odd..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Xue Bong",
                            args![
                                "So I kept watching it and,",
                                "all of a sudden, I saw something moving on the rooftops..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Xue Bong",
                            args![
                                "It disappeared in a second.",
                                "I was drunk, and it was dark",
                                "outside. I have no idea",
                                "whether it was a man, a poring,",
                                "or if I just had too much to drink...heheh~"
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(6))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Xue Bong",
                            args![
                                "Okay, then, thanks again.",
                                "Don't be a drinker like me,",
                                "unless you want to suffer from",
                                "serious hangovers. See you later~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    } else if ctx.var("b_sword").get()?.number()? < 11 {
        ctx.lines_as(
            "Xue Bong",
            args![
                "Well, hopefully you can find",
                "those thieves. To keep the",
                "peace in our village, we need to help out our chief..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()?.number()? < 14 {
        let subject7 = ctx.var("b_sword").get()?;
        if subject7 == 11 {
            ctx.lines_as(
                "Xue Bong",
                args!["Ahhh.. my stomach.. my head..", "I shouldn't drink so much..", "ughh...."],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject7 == 12 {
            ctx.lines_as(
                "Xue Bong",
                args![
                    "Enhhhh..go away.",
                    "You're merciless...",
                    "How could you turn a",
                    "blind eye to a",
                    "boozer's suffering?",
                    "Urk...!"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject7 == 13 {
            ctx.lines_as(
                "Xue Bong",
                args![
                    "Enhhhh...go away.",
                    "You're so coldhearted...",
                    "How could you turn away",
                    "a drunk in need...?",
                    "*Groan*..."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Xue Bong",
        args![
            "So you found the chief's",
            "belongings?! I knew it!",
            "I knew you could do it!",
            "You're brave enough",
            "to do anything!",
            "Good job!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn man_in_hangover_gon(ctx: &Ctx) -> Script {
    man_in_hangover_gon_body(ctx, Vec::new()).map(|_| ())
}

fn start01_gnbs_run(ctx: &Ctx, mut step: Start01GnbsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Start01GnbsStep::Start => {
                step = Start01GnbsStep::OnInit;
                continue 'machine;
            }
            Start01GnbsStep::OnInit => {
                ctx.call(Function::Sleep, vec![Val::from(10000)])?;
                step = Start01GnbsStep::OnCommandOn;
                continue 'machine;
            }
            Start01GnbsStep::OnCommandOn => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        ((Val::from("trace1-") + ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?)
                            + Val::from("#gnbs::OnCommandOn")),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn start01_gnbs(ctx: &Ctx) -> Script {
    start01_gnbs_run(ctx, Start01GnbsStep::Start, Vec::new()).map(|_| ())
}

pub fn start01_gnbs_oninit(ctx: &Ctx) -> Script {
    start01_gnbs_run(ctx, Start01GnbsStep::OnInit, Vec::new()).map(|_| ())
}

pub fn start01_gnbs_oncommandon(ctx: &Ctx) -> Script {
    start01_gnbs_run(ctx, Start01GnbsStep::OnCommandOn, Vec::new()).map(|_| ())
}

fn trace1_1_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 7 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-1::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace1_1_gnbs(ctx: &Ctx) -> Script {
    trace1_1_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_1_gnbs_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-1#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace1_1_gnbs_ontimer10000(ctx: &Ctx) -> Script {
    trace1_1_gnbs_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_1_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-1#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace1_1_gnbs_oninit(ctx: &Ctx) -> Script {
    trace1_1_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_1_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace1-1#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace1_1_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace1_1_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer1_1(ctx: &Ctx) -> Script {
    timer1_1_run(ctx, Timer11Step::Start, Vec::new()).map(|_| ())
}

pub fn timer1_1_oninit(ctx: &Ctx) -> Script {
    timer1_1_run(ctx, Timer11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer1_1_oncommandon(ctx: &Ctx) -> Script {
    timer1_1_run(ctx, Timer11Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer1_1_oncommandoff(ctx: &Ctx) -> Script {
    timer1_1_run(ctx, Timer11Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer1_1_ontimer3000(ctx: &Ctx) -> Script {
    timer1_1_run(ctx, Timer11Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem1_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 7 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(8))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem1_1(ctx: &Ctx) -> Script {
    getitem1_1_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-1")])?;
    return Err(Stop::End);
}

pub fn getitem1_1_oninit(ctx: &Ctx) -> Script {
    getitem1_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_1_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem1-1")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-1::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem1_1_oncommandon(ctx: &Ctx) -> Script {
    getitem1_1_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_1_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-1")])?;
    return Err(Stop::End);
}

pub fn getitem1_1_oncommandoff(ctx: &Ctx) -> Script {
    getitem1_1_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_2_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 7 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-2::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace1_2_gnbs(ctx: &Ctx) -> Script {
    trace1_2_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_2_gnbs_ontimer100000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-2#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace1_2_gnbs_ontimer100000(ctx: &Ctx) -> Script {
    trace1_2_gnbs_ontimer100000_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_2_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-2#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace1_2_gnbs_oninit(ctx: &Ctx) -> Script {
    trace1_2_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_2_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace1-2#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace1_2_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace1_2_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer1_2(ctx: &Ctx) -> Script {
    timer1_2_run(ctx, Timer12Step::Start, Vec::new()).map(|_| ())
}

pub fn timer1_2_oninit(ctx: &Ctx) -> Script {
    timer1_2_run(ctx, Timer12Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer1_2_oncommandon(ctx: &Ctx) -> Script {
    timer1_2_run(ctx, Timer12Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer1_2_oncommandoff(ctx: &Ctx) -> Script {
    timer1_2_run(ctx, Timer12Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer1_2_ontimer3000(ctx: &Ctx) -> Script {
    timer1_2_run(ctx, Timer12Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem1_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 7 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(8))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem1_2(ctx: &Ctx) -> Script {
    getitem1_2_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-2")])?;
    return Err(Stop::End);
}

pub fn getitem1_2_oninit(ctx: &Ctx) -> Script {
    getitem1_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_2_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem1-2")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-2::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem1_2_oncommandon(ctx: &Ctx) -> Script {
    getitem1_2_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_2_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-2")])?;
    return Err(Stop::End);
}

pub fn getitem1_2_oncommandoff(ctx: &Ctx) -> Script {
    getitem1_2_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_3_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 7 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-3::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace1_3_gnbs(ctx: &Ctx) -> Script {
    trace1_3_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_3_gnbs_ontimer90000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-3#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace1_3_gnbs_ontimer90000(ctx: &Ctx) -> Script {
    trace1_3_gnbs_ontimer90000_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_3_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-3#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace1_3_gnbs_oninit(ctx: &Ctx) -> Script {
    trace1_3_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_3_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace1-3#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace1_3_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace1_3_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer1_3(ctx: &Ctx) -> Script {
    timer1_3_run(ctx, Timer13Step::Start, Vec::new()).map(|_| ())
}

pub fn timer1_3_oninit(ctx: &Ctx) -> Script {
    timer1_3_run(ctx, Timer13Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer1_3_oncommandon(ctx: &Ctx) -> Script {
    timer1_3_run(ctx, Timer13Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer1_3_oncommandoff(ctx: &Ctx) -> Script {
    timer1_3_run(ctx, Timer13Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer1_3_ontimer3000(ctx: &Ctx) -> Script {
    timer1_3_run(ctx, Timer13Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem1_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 7 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(8))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem1_3(ctx: &Ctx) -> Script {
    getitem1_3_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-3")])?;
    return Err(Stop::End);
}

pub fn getitem1_3_oninit(ctx: &Ctx) -> Script {
    getitem1_3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_3_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem1-3")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-3::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem1_3_oncommandon(ctx: &Ctx) -> Script {
    getitem1_3_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_3_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-3")])?;
    return Err(Stop::End);
}

pub fn getitem1_3_oncommandoff(ctx: &Ctx) -> Script {
    getitem1_3_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_4_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 7 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-4::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace1_4_gnbs(ctx: &Ctx) -> Script {
    trace1_4_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_4_gnbs_ontimer150000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-4#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace1_4_gnbs_ontimer150000(ctx: &Ctx) -> Script {
    trace1_4_gnbs_ontimer150000_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_4_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-4#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace1_4_gnbs_oninit(ctx: &Ctx) -> Script {
    trace1_4_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_4_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace1-4#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace1_4_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace1_4_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer1_4(ctx: &Ctx) -> Script {
    timer1_4_run(ctx, Timer14Step::Start, Vec::new()).map(|_| ())
}

pub fn timer1_4_oninit(ctx: &Ctx) -> Script {
    timer1_4_run(ctx, Timer14Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer1_4_oncommandon(ctx: &Ctx) -> Script {
    timer1_4_run(ctx, Timer14Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer1_4_oncommandoff(ctx: &Ctx) -> Script {
    timer1_4_run(ctx, Timer14Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer1_4_ontimer3000(ctx: &Ctx) -> Script {
    timer1_4_run(ctx, Timer14Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem1_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 7 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(8))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem1_4(ctx: &Ctx) -> Script {
    getitem1_4_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_4_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-4")])?;
    return Err(Stop::End);
}

pub fn getitem1_4_oninit(ctx: &Ctx) -> Script {
    getitem1_4_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_4_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem1-4")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-4::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem1_4_oncommandon(ctx: &Ctx) -> Script {
    getitem1_4_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_4_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-4")])?;
    return Err(Stop::End);
}

pub fn getitem1_4_oncommandoff(ctx: &Ctx) -> Script {
    getitem1_4_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_5_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 7 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-5::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace1_5_gnbs(ctx: &Ctx) -> Script {
    trace1_5_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_5_gnbs_ontimer170000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-5#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace1_5_gnbs_ontimer170000(ctx: &Ctx) -> Script {
    trace1_5_gnbs_ontimer170000_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_5_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace1-5#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace1_5_gnbs_oninit(ctx: &Ctx) -> Script {
    trace1_5_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace1_5_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace1-5#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace1_5_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace1_5_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer1_5(ctx: &Ctx) -> Script {
    timer1_5_run(ctx, Timer15Step::Start, Vec::new()).map(|_| ())
}

pub fn timer1_5_oninit(ctx: &Ctx) -> Script {
    timer1_5_run(ctx, Timer15Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer1_5_oncommandon(ctx: &Ctx) -> Script {
    timer1_5_run(ctx, Timer15Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer1_5_oncommandoff(ctx: &Ctx) -> Script {
    timer1_5_run(ctx, Timer15Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer1_5_ontimer3000(ctx: &Ctx) -> Script {
    timer1_5_run(ctx, Timer15Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem1_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 7 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(8))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem1_5(ctx: &Ctx) -> Script {
    getitem1_5_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_5_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-5")])?;
    return Err(Stop::End);
}

pub fn getitem1_5_oninit(ctx: &Ctx) -> Script {
    getitem1_5_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_5_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem1-5")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer1-5::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start01#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem1_5_oncommandon(ctx: &Ctx) -> Script {
    getitem1_5_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem1_5_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem1-5")])?;
    return Err(Stop::End);
}

pub fn getitem1_5_oncommandoff(ctx: &Ctx) -> Script {
    getitem1_5_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn start02_gnbs_run(ctx: &Ctx, mut step: Start02GnbsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Start02GnbsStep::Start => {
                step = Start02GnbsStep::OnInit;
                continue 'machine;
            }
            Start02GnbsStep::OnInit => {
                ctx.call(Function::Sleep, vec![Val::from(10000)])?;
                step = Start02GnbsStep::OnCommandOn;
                continue 'machine;
            }
            Start02GnbsStep::OnCommandOn => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        ((Val::from("trace2-") + ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?)
                            + Val::from("#gnbs::OnCommandOn")),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn start02_gnbs(ctx: &Ctx) -> Script {
    start02_gnbs_run(ctx, Start02GnbsStep::Start, Vec::new()).map(|_| ())
}

pub fn start02_gnbs_oninit(ctx: &Ctx) -> Script {
    start02_gnbs_run(ctx, Start02GnbsStep::OnInit, Vec::new()).map(|_| ())
}

pub fn start02_gnbs_oncommandon(ctx: &Ctx) -> Script {
    start02_gnbs_run(ctx, Start02GnbsStep::OnCommandOn, Vec::new()).map(|_| ())
}

fn trace2_1_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 8 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-1::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace2_1_gnbs(ctx: &Ctx) -> Script {
    trace2_1_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_1_gnbs_ontimer80000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-1#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace2_1_gnbs_ontimer80000(ctx: &Ctx) -> Script {
    trace2_1_gnbs_ontimer80000_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_1_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-1#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace2_1_gnbs_oninit(ctx: &Ctx) -> Script {
    trace2_1_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_1_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace2-1#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace2_1_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace2_1_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer2_1(ctx: &Ctx) -> Script {
    timer2_1_run(ctx, Timer21Step::Start, Vec::new()).map(|_| ())
}

pub fn timer2_1_oninit(ctx: &Ctx) -> Script {
    timer2_1_run(ctx, Timer21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer2_1_oncommandon(ctx: &Ctx) -> Script {
    timer2_1_run(ctx, Timer21Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer2_1_oncommandoff(ctx: &Ctx) -> Script {
    timer2_1_run(ctx, Timer21Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer2_1_ontimer3000(ctx: &Ctx) -> Script {
    timer2_1_run(ctx, Timer21Step::OnTimer3000, Vec::new()).map(|_| ())
}
