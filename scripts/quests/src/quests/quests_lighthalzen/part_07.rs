use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MrPresidentStep {
    Start,
    LMission,
    HoistEnd1,
    HoistEnd2,
}

fn mr_president_run(ctx: &Ctx, mut step: MrPresidentStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    'machine: loop {
        match step {
            MrPresidentStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7342), Val::from(1)])? != 1 {
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
                ctx.call(Function::Cutin, vec![Val::from("lhz_karl.bmp"), Val::from(2)])?;
                if ctx.var("lhz_boss").get()?.number()? < 11 {
                    {
                        ctx.lines_as(
                            "Karl",
                            args!["How did you get in here?", "Please leave this place", "immediately! Security...!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(83), Val::from(22)])?;
                    }
                } else {
                    if ctx.var("lhz_boss").get()? == 14 {
                        ctx.lines_as(
                            "Karl",
                            args![
                                "Ah, welcome~",
                                "I understand that you",
                                "must have had a lot of",
                                "trouble coming here.",
                                "It's a pleasure to",
                                "finally meet you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "As I'm sure Ghalstein",
                                "has told you, ''Secret Wing''",
                                "was formed to overthrow the",
                                "evil Rekenber Corporation.",
                                "Now, I'm sure that you must",
                                "have some questions for me..."
                            ],
                        )?;
                        ctx.next()?;
                        step = MrPresidentStep::LMission;
                        continue 'machine;
                    } else {
                        if ctx.var("lhz_boss").get()? == 15 {
                            ctx.lines_as(
                                "Karl",
                                args![
                                    "Now, you must first visit",
                                    "the Kafra Headquarters in",
                                    "Al De Baran and meet someone",
                                    "named ^FF0000Benith^000000. She will tell you",
                                    "all about your next mission."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Karl",
                                args![
                                    "Be careful and do not",
                                    "freely mention anything",
                                    "about ''Secret Wing'' or",
                                    "your cooperation with us as",
                                    "it may jeopardize our goals.",
                                    "Hurry, time is of the essence."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Karl",
                                args![
                                    "Ah, before I forget,",
                                    "please bring this file",
                                    "folder to ^FF0000Ms. Hes O'Neil^000000",
                                    "for me before you leave.",
                                    "Thank you very much."
                                ],
                            )?;
                            ctx.var("lhz_boss").set(Val::from(16))?;
                            ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(12018), Val::from(12019)])?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("lhz_boss").get()?.number()? < 21 {
                                ctx.lines_as(
                                    "Karl",
                                    args![
                                        "Hurry and complete",
                                        "the mission given to",
                                        "you by Benith, who is",
                                        "in the Kafra Headquarters",
                                        "in Al De Baran. It is a very",
                                        "urgent, high priority task..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("lhz_boss").get()? == 21 {
                                    if !(ctx.call(Function::CountItem, vec![Val::from(7343)])?.is_true()) {
                                        ctx.lines_as(
                                            "Karl",
                                            args![
                                                "Hm? I was expecting",
                                                "for you to bring me",
                                                "back an important file.",
                                                "Please hurry, our entire",
                                                "organization is at stake!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Karl",
                                        args![
                                            "Ah, you're here.",
                                            "Good work, I hear that",
                                            "the mission was a success.",
                                            "An official success anyway.",
                                            "..............................."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Karl",
                                        args![
                                            "Poor, dear sweet",
                                            "Jargeah. Why him?",
                                            "Must fate be so cruel",
                                            "and harsh? I still can't",
                                            "believe he's gone. Just",
                                            "last week, we were..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Karl",
                                        args![
                                            "I'm sorry. You didn't",
                                            "know him that well, but",
                                            "all those who did know",
                                            "him, loved him. And I can",
                                            "say that with pride. L-let",
                                            "me check the files a minute..."
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7343), Val::from(1)])?;
                                    ctx.var("lhz_boss").set(Val::from(22))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("lhz_boss").get()? == 22 {
                                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 3 {
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "Hm. After completely",
                                                    "reviewing this file, I've",
                                                    "found that it offers some",
                                                    "great leads, but it's still not",
                                                    "enough hard evidence to",
                                                    "really hurt Rekenber."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "However, according to",
                                                    "this, there's a researcher",
                                                    "from Rekenber named",
                                                    "Shinokas who vanished all",
                                                    "of a sudden. Apparently, he",
                                                    "knew some kind of secret."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "Luckily, my men have",
                                                    "reported sightings of",
                                                    "Shinokas somewhere in",
                                                    "Einbroch. He's certainly",
                                                    "out there. But what does",
                                                    "he have to hide?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "We must learn his",
                                                    "secret if we hope to",
                                                    "do any lasting damage to",
                                                    "the Rekenber Corporation.",
                                                    "Your next mission is to",
                                                    "find Shinokas. Good luck."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "And please hurry.",
                                                    "I don't want our enemies",
                                                    "to discover Shinokas before",
                                                    "we can get him to talk."
                                                ],
                                            )?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(12022), Val::from(12023)])?;
                                            if ctx.var("shinokas_quest").get()? == 11 {
                                                ctx.var("lhz_boss").set(Val::from(24))?;
                                            } else {
                                                ctx.var("lhz_boss").set(Val::from(23))?;
                                            }
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Karl",
                                            args![
                                                "I'm sorry, but it's",
                                                "taking me a long time",
                                                "to analyze the information",
                                                "in this file. A-and Jargeah",
                                                "sacrificed himself to get",
                                                "it for us. ^333333*Sigh*^000000 Jargeah..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Karl",
                                            args![
                                                "Why don't you relax",
                                                "while I peruse this",
                                                "file? I should be done",
                                                "with this soon, hopefully..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("lhz_boss").get()? == 23 {
                                            if ctx.var("shinokas_quest").get()? == 11 {
                                                ctx.lines_as(
                                                    "Karl",
                                                    args![
                                                        "Ymir's Heart Pieces?",
                                                        "Is that what they're",
                                                        "trying to collect?",
                                                        "What are they going",
                                                        "to do with something",
                                                        "so incredibly dangerous?!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Karl",
                                                    args![
                                                        "Huh. This new information",
                                                        "raises some new questions.",
                                                        "Fortunately, we have a new",
                                                        "lead from the spy who got",
                                                        "us the info on Shinokas."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Karl",
                                                    args![
                                                        "We've recently learned that",
                                                        "Shinokas managed to access",
                                                        "the Regenschirm Secret Archive, a lab affiliated with Rekenber.",
                                                        "That's how Shinokas was able",
                                                        "to learn so much about them."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Karl",
                                                    args![
                                                        "It will be difficult",
                                                        "and very dangerous to",
                                                        "access this archive, so",
                                                        "I want you to meet with",
                                                        "someone from the Kafra",
                                                        "Corporation near Lighthalzen."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Karl",
                                                    args![
                                                        "And don't worry...",
                                                        "She'll know exactly",
                                                        "who you are and",
                                                        "how to help you."
                                                    ],
                                                )?;
                                                ctx.var("lhz_boss").set(Val::from(26))?;
                                                ctx.call(Function::CompleteQuest, vec![Val::from(12023)])?;
                                                ctx.call(Function::SetQuest, vec![Val::from(12024)])?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "We don't know how",
                                                    "helpful Shinokas's",
                                                    "information may be for",
                                                    "us, but we must learn",
                                                    "anything we can about",
                                                    "the Rekenber Corporation."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "There is so much we",
                                                    "don't know about them.",
                                                    "Their motives, their methods,",
                                                    "almost all of the important",
                                                    "details are still in the dark."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Karl",
                                                args![
                                                    "By all means, you",
                                                    "must find Shinokas and",
                                                    "see what you can learn!",
                                                    "Your efforts will not go",
                                                    ((Val::from("unrecognized, ")
                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("."))
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("lhz_boss").get()? == 24 {
                                                ctx.lines_as(
                                                    "Karl",
                                                    args![
                                                        "Hm? What's wrong?",
                                                        "Now that I think about",
                                                        "it, you reacted as if you",
                                                        "recognized his name back",
                                                        "when I mentioned Shinokas."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Oh, no. Not really."), Val::from("Actually, Shinokas is...")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "Then it must be",
                                                                "my imagination.",
                                                                "Okay then, go and",
                                                                "find Shinokas, he",
                                                                "may be in grave",
                                                                "danger as we speak!"
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "Oh...",
                                                                "You've already met",
                                                                "Shinokas, did you?",
                                                                "You witnessed his ",
                                                                "death?! This is a very",
                                                                "strange coincidence."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "I see, there must be",
                                                                "no need to investigate",
                                                                "Shinokas now. And you",
                                                                "did learn the secret for",
                                                                "which he was hunted..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Guard",
                                                            args!["Mr. President,", "Ms. Hes O' Neil has an", "urgent message for you."],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Karl", args!["Hmm...", "Then please,", "let her in."])?;
                                                        ctx.next()?;
                                                        ctx.call(Function::EnableNpc, vec![Val::from("Secretary#2")])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hes O'Neil",
                                                            args![
                                                                "Sir, I'm sorry",
                                                                "for interrupting",
                                                                "you, but there's",
                                                                "something I need to",
                                                                "show you immediately!"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "Alright, let's",
                                                                "have it. Let me",
                                                                "see those new files.",
                                                                "Oh, these are--! Thank",
                                                                "you, Ms. O' Neil, you",
                                                                "may leave now."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hes O'Neil",
                                                            args![
                                                                "Thank you sir.",
                                                                "Once again, let me",
                                                                "apologize for disturbing",
                                                                "your private conference."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.call(Function::DisableNpc, vec![Val::from("Secretary#2")])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "Alright. Please let me",
                                                                "review these documents",
                                                                "to see if there are any",
                                                                "new developments in the",
                                                                "investigation involving",
                                                                "Shinokas and his secret..."
                                                            ],
                                                        )?;
                                                        ctx.var("lhz_boss").set(Val::from(25))?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            } else {
                                                if ctx.var("lhz_boss").get()? == 25 {
                                                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 8 {
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "Alright, I just",
                                                                "completed reading",
                                                                "all of these new reports",
                                                                "from our field agents.",
                                                                "Thank you for waiting."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "We've recently learned that",
                                                                "Shinokas managed to access",
                                                                "the Regenschirm  Secret Archive,a lab affiliated with Rekenber.",
                                                                "That's how Shinokas was able",
                                                                "to learn so much about them."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "It will be difficult",
                                                                "and very dangerous to",
                                                                "access this archive, so",
                                                                "I want you to meet with",
                                                                "someone from the Kafra",
                                                                "Corporation near Lighthalzen."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "If you keep looking around",
                                                                "the ^3355FFfields just outside of",
                                                                "Lighthalzen^000000, you'll certainly",
                                                                "find her. I'm sorry that I can't",
                                                                "tell you more, but we've got",
                                                                "to protect our security..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "And don't worry...",
                                                                "She'll know exactly",
                                                                "who you are and",
                                                                "how to help you."
                                                            ],
                                                        )?;
                                                        ctx.var("lhz_boss").set(Val::from(26))?;
                                                        ctx.call(Function::CompleteQuest, vec![Val::from(12023)])?;
                                                        ctx.call(Function::SetQuest, vec![Val::from(12024)])?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Karl",
                                                        args![
                                                            "I apologize, but it's",
                                                            "taking me a long time",
                                                            "to go through all of the",
                                                            "reports in these files.",
                                                            "Please give me a little more",
                                                            "time to make sense of them."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("lhz_boss").get()?.number()? < 37 {
                                                        ctx.lines_as(
                                                            "Karl",
                                                            args![
                                                                "I hope you continue",
                                                                "to keep up the good",
                                                                "work, not only for the",
                                                                "sake of the Schwarzwald",
                                                                "Republic, but for peace",
                                                                "on all of Midgard."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("lhz_boss").get()? == 37 {
                                                            if !(ctx.call(Function::CountItem, vec![Val::from(7344)])?.is_true()) {
                                                                ctx.lines_as(
                                                                    "Karl",
                                                                    args![
                                                                        "Did you find any",
                                                                        "of that evidence in",
                                                                        "the Secret Archive yet?",
                                                                        "Hurry! The movements in",
                                                                        "Rekenber Corporation are",
                                                                        "making me feel really uneasy."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                return Err(Stop::End);
                                                            }
                                                            ctx.lines_as(
                                                                "Karl",
                                                                args![
                                                                    "Ah, you're here.",
                                                                    "I've already received a",
                                                                    "message from Esuna",
                                                                    "about your success.",
                                                                    "If you would, please",
                                                                    "let me read the file..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Karl",
                                                                args![
                                                                    "Ah, now everything is",
                                                                    "clearer. Shinokas and his",
                                                                    "friends were killed over",
                                                                    "that piece of Ymir's Heart.",
                                                                    "That's what the Rekenber",
                                                                    "Corporation ultimately wants."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Karl",
                                                                args![
                                                                    "But this raises some",
                                                                    "new questions. What",
                                                                    "are they planning to",
                                                                    "do with Ymir's Heart?",
                                                                    "It must be more powerful",
                                                                    "than we had thought..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Karl",
                                                                args![
                                                                    "We'll need even more",
                                                                    "information. For now,",
                                                                    "please go back to Esuna.",
                                                                    "It seems that she just",
                                                                    "received some critically",
                                                                    "important intel to give you."
                                                                ],
                                                            )?;
                                                            ctx.call(Function::DelItem, vec![Val::from(7344), Val::from(1)])?;
                                                            ctx.var("lhz_boss").set(Val::from(38))?;
                                                            ctx.close_window()?;
                                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ctx.var("lhz_boss").get()? == 38 {
                                                                ctx.lines_as(
                                                                    "Karl",
                                                                    args![
                                                                        "Please hurry and meet",
                                                                        "Esuna just outside of",
                                                                        "Lighthalzen. Perhaps I am",
                                                                        "getting paranoid, but I keep",
                                                                        "getting the feeling that",
                                                                        "Rekenber is on to us..."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if ctx.var("lhz_boss").get()? == 39 {
                                                                    if !(ctx.call(Function::CountItem, vec![Val::from(7343)])?.is_true()) {
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "You don't have the",
                                                                                "file? Retrieve it for",
                                                                                "me as soon as you can.",
                                                                                "It's imperative that I read",
                                                                                "what Esuna has to report!"
                                                                            ],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    ctx.lines_as(
                                                                        "Karl",
                                                                        args![
                                                                            "Ah, good to see you",
                                                                            "again. I hear you have",
                                                                            "some important news for",
                                                                            "me, something of the utmost",
                                                                            "urgency. But I fear the worst."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    match runtime::select_values(
                                                                        ctx,
                                                                        &[Val::from("Give the file from Esuna.")],
                                                                    )? {
                                                                        1 => {}
                                                                        _ => {}
                                                                    }
                                                                    ctx.lines_as(
                                                                        "Karl",
                                                                        args![
                                                                            "What...?",
                                                                            "I can't believe this!",
                                                                            "How can our security",
                                                                            "be breached like this?!",
                                                                            "Only someone from",
                                                                            "really deep inside could..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Karl",
                                                                        args![
                                                                            "Please...",
                                                                            "Would you go and",
                                                                            "speak to Esuna one",
                                                                            "more time? I need to",
                                                                            "know more about how",
                                                                            "all of this happened..."
                                                                        ],
                                                                    )?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(7343), Val::from(1)])?;
                                                                    ctx.var("lhz_boss").set(Val::from(40))?;
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if ctx.var("lhz_boss").get()? == 41 {
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "No...",
                                                                                "I can't...",
                                                                                "Kurelle? We've",
                                                                                "worked together,",
                                                                                "trusted each other",
                                                                                "for years. No, it's not...."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "If it's true, then",
                                                                                "that means I've been",
                                                                                "playing into the enemy's",
                                                                                "hands this whole time.",
                                                                                "I've... I've got to know",
                                                                                "and ask Kurelle myself."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args!["O'Neil!", "Bring Kurelle in", "here, right now!"],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Hes O'Neil", args!["...Yes, sir.", "Right away."])?;
                                                                        ctx.next()?;
                                                                        ctx.var("lhz_boss").set(Val::from(42))?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    } else if ctx.var("lhz_boss").get()? == 42 {
                                                                        ctx.lines_as("Guard", args!["Advisor Kurelle", "is here now."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["Let him in!", "..................."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Guard", args!["...............?", "Yes, sir..."])?;
                                                                        ctx.next()?;
                                                                        ctx.call(Function::EnableNpc, vec![Val::from("A Neat Gentleman")])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Kurelle",
                                                                            args![
                                                                                "Hello, Mr. President.",
                                                                                "What exactly did you",
                                                                                "need from me today?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["...........", ".......", "..."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Kurelle", args!["???", "......."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "Did you...",
                                                                                "Why did you betray us?!",
                                                                                "After all this time, why now?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Kurelle",
                                                                            args![
                                                                                "...",
                                                                                "......",
                                                                                "So you know.",
                                                                                "Well, you're smarter",
                                                                                "than I gave you credit for."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Kurelle",
                                                                            args![
                                                                                "Ha ha ha~",
                                                                                "Right, it was me.",
                                                                                "Your right hand man.",
                                                                                "I reported everything",
                                                                                "you were doing to the",
                                                                                "Rekenber Corporation."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "But why...?",
                                                                                "I thought we were",
                                                                                "working together for",
                                                                                "the greater good, to",
                                                                                "do the right thing?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Kurelle", args!["Forget that! I'm sick", "of being second place", "to you. All our lives, you've", "always been on top. School,", "athletics, politics. Well, here's my chance to finally beat you!"])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "I can't believe this. All",
                                                                                "those years of friendship",
                                                                                "were all a lie? We even",
                                                                                "joined Secret Wing together.",
                                                                                "This whole time, you were",
                                                                                "harboring a silly grudge..."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Kurelle",
                                                                            args![
                                                                                "Shut up! It's not a",
                                                                                "silly grudge! ...You there.",
                                                                                "Adventurer. Can't you see",
                                                                                "this man is finished?! But",
                                                                                "it's not too late. Join us.",
                                                                                "Rekenber could use you."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Kurelle",
                                                                            args![
                                                                                "Now that I've helped",
                                                                                "the Rekenber Corporation,",
                                                                                "I'll get their support in the",
                                                                                "next presidential election.",
                                                                                "I'll beat you for sure, Karl!",
                                                                                "Bwahahahahahaahahahhaah~!"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.call(
                                                                            Function::DisableNpc,
                                                                            vec![Val::from("A Neat Gentleman")],
                                                                        )?;
                                                                        ctx.lines(args![".......", ".........", "..........."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "It's all over for",
                                                                                "now. This isn't good",
                                                                                "at all. You're finished",
                                                                                "here too. Go and talk to",
                                                                                "^3355FFGhalstein^000000 again. It's time",
                                                                                "we let you loose, adventurer."
                                                                            ],
                                                                        )?;
                                                                        ctx.var("lhz_boss").set(Val::from(43))?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    } else if ctx.var("lght_boss").get()? == 43 {
                                                                        ctx.lines_as("Karl", args![".........", "............"])?;
                                                                        l_i = Val::from(12015);
                                                                        'l3: loop {
                                                                            if !(l_i.clone().number()? < 12028) {
                                                                                break 'l3;
                                                                            }
                                                                            'b3: {
                                                                                if (ctx
                                                                                    .call(Function::CheckQuest, vec![l_i.clone()])?
                                                                                    .number()?
                                                                                    >= 0
                                                                                    && ctx
                                                                                        .call(Function::CheckQuest, vec![l_i.clone()])?
                                                                                        .number()?
                                                                                        < 2)
                                                                                {
                                                                                    ctx.call(Function::CompleteQuest, vec![l_i.clone()])?;
                                                                                }
                                                                            }
                                                                            l_i = (l_i.clone() + Val::from(1));
                                                                        }
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else if ctx.var("hg_tre").get()? == 56 {
                                                                        if !(ctx
                                                                            .call(Function::CountItem, vec![Val::from(7342)])?
                                                                            .is_true())
                                                                        {
                                                                            ctx.lines_as(
                                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                                args!["Ah, where did I put the record?"],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args!["It's you.....!", "It's really been a long time."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                            args![
                                                                                "So, are you still having the idea to go against Rekenber?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["..................."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["I'm not sure.", "The Secret Wing has been disbanded and I am just another puppet they have, just like the previous presidents."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Not long ago, the scientists of Regenschrim have stolen the research record and destroyed the machines that they used for research purposes."])?;
                                                                        ctx.next()?;
                                                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                                                                        ctx.lines_as("Karl", args!["...!", "Did you do that? ", "I still fail to move on after the incident of my friends' betrayal and the disband of the Secret Wing...", "You are really a great friend."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "Didn't you speak about the research record just now?",
                                                                                "Oh! Can you give me the record?",
                                                                                "It will be a big trouble to them."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["If you insist to go against them, I will give you the record."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["I am the president of this country.", "I have failed before but I won't stop trying when there is still an opportunity."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["You remind me of the day I made up my mind to rebuild this country.", "How could I forgot such important responsibilities of mine."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                            args![
                                                                                "That's great.",
                                                                                "Here's the record.",
                                                                                "I hope you will make good use of it. "
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["Let's see..."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args!["Oh! That's some interesting information."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I have another good news. The Secret Wing has not given up yet. They are still working on the project and I hope that you won't be giving up on it too."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![".....I feel so sorry for what I did."],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["Thank you for bringing this great gift. I will stay strong and work hard to achieve my target."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["I will recruit the new batch of members and use them to let the world know about what the corporation had really done."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Karl",
                                                                            args![
                                                                                "Thank you so much. ",
                                                                                "You can count on me to right what was wrong."
                                                                            ],
                                                                        )?;
                                                                        ctx.var("hg_tre").set(Val::from(57))?;
                                                                        ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                                                                        ctx.call(
                                                                            Function::GetExperience,
                                                                            vec![Val::from(2000000), Val::from(0)],
                                                                        )?;
                                                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    } else if ctx.var("hg_tre").get()? == 57 {
                                                                        ctx.lines_as("Karl", args!["I am trying to recruit capable people who can help me to bring down Rekenber Corporation.", "Thanks to you, I have enough information as a good beginning to nail them now down."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Karl", args!["I have had my men to gather some information and learned that Regenschrim had stopped operating now.", "You have done well for the Schwarzwald Republic."])?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        ctx.lines_as("Karl", args![".........", "............"])?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
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
                    step = MrPresidentStep::HoistEnd1;
                    continue 'machine;
                    step = MrPresidentStep::LMission;
                    continue 'machine;
                }
                step = MrPresidentStep::HoistEnd2;
                continue 'machine;
            }
            MrPresidentStep::LMission => {
                match runtime::select_values(
                    ctx,
                    &[
                        Val::from("Secret Wing's Background"),
                        Val::from("Rekenber's Purpose"),
                        Val::from("Secret Wing's Goal"),
                        Val::from("Details about my mission"),
                        Val::from("I'm ready for my mission."),
                    ],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Karl",
                            args![
                                "Rekenber has been",
                                "unopposed for a very long",
                                "time. Our nation is unhappy",
                                "with their rule, but since our",
                                "country lacks solidarity, the",
                                "people can do nothing."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "But one day, I was contacted",
                                "by some people who claimed to",
                                "share my sentiments against",
                                "the Rekenber Corporation.",
                                "Later, I learned that they were",
                                "from the Kafra Corporation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "Apparently, they were",
                                "facing some aggressive",
                                "competition from Cool",
                                "Event Corp, which is",
                                "actually backed by the",
                                "Rekenber Corporation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "We decided to pool our",
                                "resources to deal with what",
                                "we perceived as a common",
                                "enemy. Before long, we gathered",
                                "more devotees to our cause and",
                                "formed the ''Secret Wing.''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "It may be helpful for",
                                "you to know that our",
                                "direct contact to the",
                                "Kafra Corporation is the",
                                "^FF00003rd Security Team^000000. Oh, did",
                                "you have any other questions?"
                            ],
                        )?;
                        ctx.next()?;
                        step = MrPresidentStep::LMission;
                        continue 'machine;
                    }
                    2 => {
                        ctx.lines_as(
                            "Karl",
                            args![
                                "It's true that companies",
                                "exist to create money, but",
                                "the Rekenber Corporation",
                                "is much more nefarious.",
                                "They actually want to dominate the entire Midgard continent."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "In fact, the chairman of",
                                "that company is shrouded",
                                "in mystery. Although I'm",
                                "the president, I go through",
                                "many difficulties just to",
                                "send a message to him."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "It's disheartening,",
                                "but we really have no",
                                "idea what their true goals",
                                "and plans might be. Now, did",
                                "you have any other questions?"
                            ],
                        )?;
                        ctx.next()?;
                        step = MrPresidentStep::LMission;
                        continue 'machine;
                    }
                    3 => {
                        ctx.lines_as(
                            "Karl",
                            args![
                                "''Secret Wing's'' only",
                                "goal is to destroy the",
                                "Rekenber Corporation in",
                                "order to break the Schwarzwald Republic free from its oppression."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Karl",
                            args![
                                "Of course, we realize",
                                "that it will take time and",
                                "a lot of sacrifice to make",
                                "this a reality. Now, do you",
                                "have anything else to ask?"
                            ],
                        )?;
                        ctx.next()?;
                        step = MrPresidentStep::LMission;
                        continue 'machine;
                    }
                    4 => {
                        ctx.lines_as(
                            "Karl",
                            args![
                                "Ah, your next mission.",
                                "I'm ready to give you",
                                "some of the details if",
                                "you no longer have any",
                                "questions to ask. Let me",
                                "know when you are ready."
                            ],
                        )?;
                        ctx.next()?;
                        step = MrPresidentStep::LMission;
                        continue 'machine;
                    }
                    5 => {
                        ctx.lines_as(
                            "Karl",
                            args![
                                "Very well, then.",
                                "Your mission will not",
                                "be too difficult, but it does",
                                "have great urgency so you",
                                "must accomplish it as soon",
                                "as you can. Understood?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("Wait, I have one more question!"), Val::from("Yes sir, Mr. President.")],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Karl",
                                    args![
                                        "Hm? I thought you",
                                        "didn't have any more",
                                        "questions. However, I still",
                                        "have the luxury to give you",
                                        "any answers that I can provide."
                                    ],
                                )?;
                                ctx.next()?;
                                step = MrPresidentStep::LMission;
                                continue 'machine;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Karl",
                                    args![
                                        "I'm glad to hear that.",
                                        "Alright, give me a second",
                                        "to search for this file before",
                                        "I explain the mission."
                                    ],
                                )?;
                                ctx.var("lhz_boss").set(Val::from(15))?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                step = MrPresidentStep::HoistEnd1;
                continue 'machine;
            }
            MrPresidentStep::HoistEnd1 => {
                step = MrPresidentStep::HoistEnd2;
                continue 'machine;
            }
            MrPresidentStep::HoistEnd2 => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn mr_president(ctx: &Ctx) -> Script {
    mr_president_run(ctx, MrPresidentStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Secretary2Step {
    Start,
    OnInit,
}

fn secretary_2_run(ctx: &Ctx, mut step: Secretary2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Secretary2Step::Start => {
                step = Secretary2Step::OnInit;
                continue 'machine;
            }
            Secretary2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Secretary#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn secretary_2(ctx: &Ctx) -> Script {
    secretary_2_run(ctx, Secretary2Step::Start, Vec::new()).map(|_| ())
}

pub fn secretary_2_oninit(ctx: &Ctx) -> Script {
    secretary_2_run(ctx, Secretary2Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ANeatGentlemanStep {
    Start,
    OnInit,
}

fn a_neat_gentleman_run(ctx: &Ctx, mut step: ANeatGentlemanStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ANeatGentlemanStep::Start => {
                step = ANeatGentlemanStep::OnInit;
                continue 'machine;
            }
            ANeatGentlemanStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("A Neat Gentleman")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn a_neat_gentleman(ctx: &Ctx) -> Script {
    a_neat_gentleman_run(ctx, ANeatGentlemanStep::Start, Vec::new()).map(|_| ())
}

pub fn a_neat_gentleman_oninit(ctx: &Ctx) -> Script {
    a_neat_gentleman_run(ctx, ANeatGentlemanStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EavesdropStep {
    Start,
    OnTouch,
}

fn eavesdrop_run(ctx: &Ctx, mut step: EavesdropStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EavesdropStep::Start => {
                step = EavesdropStep::OnTouch;
                continue 'machine;
            }
            EavesdropStep::OnTouch => {
                if ctx.var("lhz_boss").get()? == 16 {
                    ctx.lines(args![
                        "^3355FFWhat the...?",
                        "You can hear",
                        "whispers coming",
                        "from the window.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args!["^333333That's expected...", "But... Why did...", "... the president...^000000"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "?????",
                        args![
                            "^666666.......So...................",
                            "...their investigation...",
                            "............of course.........",
                            "...just bait...................^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "^333333...Next election.....",
                            "..............................",
                            "....you'll be................",
                            "...............Can't stop us.^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe whispers grow",
                        "lower and lower until",
                        "you can no longer hear",
                        "anything. One of those",
                        "voices seemed so familiar...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn eavesdrop(ctx: &Ctx) -> Script {
    eavesdrop_run(ctx, EavesdropStep::Start, Vec::new()).map(|_| ())
}

pub fn eavesdrop_ontouch(ctx: &Ctx) -> Script {
    eavesdrop_run(ctx, EavesdropStep::OnTouch, Vec::new()).map(|_| ())
}

fn kafra_employee_l1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()? != 17 {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Welcome to the",
                "Kafra Headquarters.",
                "Here in the heart of",
                "Kafra's operations, you",
                "can be provided with special",
                "services offered nowhere else!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "If you need anything,",
                "please inquire the Kafra",
                "Employees inside the building.",
                "Thank you and have a good day~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Good day~",
            "The Kafra Corporation is",
            "always working to ensure",
            "our customers' satisfaction.",
            "How may I help you today?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("I have a question."), Val::from("I'm fine, thanks.")])? {
        1 => {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "Sure, I'll answer your",
                    "question to the best of",
                    "my ability. However, I may",
                    "need to reference you to",
                    "another employee for",
                    "specialized information."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Do you know where I can find Benith?")])? {
                1 => {}
                _ => {}
            }
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "Oh, Benith? Sure, you",
                    "can find her here inside",
                    "Kafra Headquarters to the",
                    "right somewhere. She wears",
                    "a special uniform, so you",
                    "can spot her easily."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "Okay then,",
                    "have a good day!",
                    "Always remember",
                    "that the Kafra Service",
                    "will be on your side~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "Feel free to ask any",
                    "one of our conveniently",
                    "located employees if you",
                    "ever have need of Kafra's",
                    "special services. Thank",
                    "you and have a nice day~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn kafra_employee_l1(ctx: &Ctx) -> Script {
    kafra_employee_l1_body(ctx, Vec::new()).map(|_| ())
}

fn kafra_employee_l2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7343), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lhz_boss").get()?.number()? < 17 {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Welcome to",
                "Kafra Headquarters.",
                "What's new with Kafra?",
                "Glad you asked. Right now,",
                "we're developing a brand new",
                "program with Cool Event Corp."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "This new program will",
                "provide a direct teleport",
                "service to dungeons for",
                "the convenience of our",
                "valued customers. Is",
                "that not... exciting?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Due to technical issues,",
                "Kafra Corp and Cool Event",
                "Corp cannot provide teleport",
                "services to the same dungeon,",
                "so one common teleport service",
                "provider will be selected."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Therefore, elections will",
                "be held to determine which",
                "company will provide this",
                "Dungeon Teleport Service.",
                "Please check the eligibility",
                "requirements before voting."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 17 {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Welcome to",
                "Kafra Headquarters.",
                "What's new with Kafra?",
                "Glad you asked. Right now,",
                "we're developing a brand new",
                "program with Cool Event Corp."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "This new program will",
                "provide a direct teleport",
                "service to dungeons for",
                "the convenience of our",
                "valued customers. Is",
                "that not... exciting?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Due to technical issues,",
                "Kafra Corp and Cool Event",
                "Corp cannot provide teleport",
                "services to the same dungeon,",
                "so one common teleport service",
                "provider will be selected."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "We'd like to inform",
                "you that the customers",
                "will decide the teleport",
                "service provider through",
                "an election. Your vote will",
                "be much appreciated."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Remember to take part",
                "in the polls that will be",
                "taking place in the cities",
                "of Prontera and Juno.",
                "Happy voting."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Okay."), Val::from("I can't wait!")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "Uh oh...",
                        "Did you have a question?",
                        "In all honesty, I don't know",
                        "very much about the services",
                        "Kafra offers. My work is...",
                        "I'm in a different department."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Oh, that's okay then."), Val::from("Are you Benith..?")])? {
                    1 => {
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "*Whew!*",
                                "Oh good. Well, if you",
                                "do have any questions",
                                "about the Kafra Services,",
                                "please ask one of the regular",
                                "Kafra Employees. Thank you."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Benith",
                            args!["Yes, that's me.", "Is there anything", "that I can do for you?"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("No, not really."), Val::from("Show Membership Card.")])? {
                            1 => {
                                ctx.lines_as("Benith", args!["Really?", "Okay, then."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                if !(ctx.call(Function::CountItem, vec![Val::from(7348)])?.is_true()) {
                                    ctx.lines(args![
                                        "^3355FFWait...",
                                        "You can't show",
                                        "your ''Secret Wing''",
                                        "Membership Card",
                                        "if you don't have it!^000000"
                                    ])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Benith",
                                    args![
                                        "Oh, I've been waiting",
                                        "for you. Finally, I can",
                                        "drop this promotional",
                                        "pretense and get down",
                                        "to business and tell you",
                                        "about your mission."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Benith",
                                    args![
                                        "As an experienced",
                                        "adventurer, your specialty",
                                        "is in retrieving items and",
                                        "fighting against monsters.",
                                        "We have a rescue mission",
                                        "that suits your expertise."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Benith",
                                    args![
                                        "One of our special",
                                        "agents got into an",
                                        "accident and is stuck",
                                        "in ^FF0000Grim Reaper's Valley^000000,",
                                        "located somewhere between",
                                        "Einbroch and Lighthalzen."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Benith",
                                    args![
                                        "Our agent, Jargeah,",
                                        "is reported to be hiding",
                                        "near a broken bridge there.",
                                        "All of our other agents are",
                                        "assigned on other missions,",
                                        "so you're all he has right now."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Benith",
                                    args!["He's seriously wounded,", "so please hurry before", "the enemy can get to him..."],
                                )?;
                                ctx.var("lhz_boss").set(Val::from(18))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(12019), Val::from(12020)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "That makes one of u--",
                        "I mean, your participation",
                        "is very much appreciated.",
                        "Remember that Kafra is",
                        "always on your side."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("lhz_boss").get()? == 18 {
        ctx.lines_as(
            "Benith",
            args![
                "Please hurry and save",
                "Jargeah. If the enemy",
                "gets to him before we",
                "do, all his efforts, as well",
                "as his life, may be forfeit."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benith",
            args![
                "You should be able to",
                "find him near a broken",
                "bridge in Grim Reaper's",
                "Valley, which is located",
                "somewhere between",
                "Einbroch and Lighthalzen."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 19 {
        if !(ctx.call(Function::CountItem, vec![Val::from(7343)])?.is_true()) {
            ctx.lines_as(
                "Benith",
                args![
                    "Please hurry!",
                    "I don't want the enemy",
                    "to find Jargeah before",
                    "we do! His life and the",
                    "Secret Wing are at stake!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Benith",
            args![
                "Great, you're back,",
                "and you even have the",
                "information that Jargeah",
                "managed to obtain. But...",
                "Where's Jargeah? Is he...?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("He's in a better place.")])? {
            1 => {}
            _ => {}
        }
        ctx.lines_as("Benith", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Benith",
            args![
                "What...?! Noooo!",
                "Comrade Jargeah!",
                "I swear to you your",
                "death won't be in vain!",
                "Why did another good man",
                "have to die? Answer me!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benith",
            args![
                "I can't... No. We must",
                "first honor Jargeah's noble",
                "sacrifice before we can allow ourselves the luxury of mourning",
                "our loss. Let me read these files first before you deliver them..."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7343), Val::from(1)])?;
        ctx.var("lhz_boss").set(Val::from(20))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 20 {
        ctx.lines_as(
            "Benith",
            args![
                "Alright. Please take",
                "this file to ^FF0000him^000000 now.",
                "I believe you know",
                "whom I am talking about.",
                "The Secret Wing is counting",
                "on you, brave adventurer."
            ],
        )?;
        ctx.var("lhz_boss").set(Val::from(21))?;
        ctx.call(Function::GetItem, vec![Val::from(7343), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12021), Val::from(12022)])?;
        ctx.next()?;
        ctx.lines_as(
            "Benith",
            args![
                "I swear by my father's",
                "grave that the tears I shed",
                "for Jargeah will only be",
                "matched by the blood I will",
                "spill in holy retribution.",
                "Jargeah, watch over me!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Benith",
            args![
                "W-welcome to the",
                "Kafra Headquarters.",
                "What's new with Kafra?",
                "^333333Glad... You... Asked...^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn kafra_employee_l2(ctx: &Ctx) -> Script {
    kafra_employee_l2_body(ctx, Vec::new()).map(|_| ())
}
