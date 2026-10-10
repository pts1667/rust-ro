use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn high_priest_zhed_rachel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("ra_tem_q").get()?.number()? > 14 || runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true()) {
        ctx.var("lost_boy").set(Val::from(14))?;
    }
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
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
    if ctx.var("lost_boy").get()? == 12 {
        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "For some reason, you look",
                "familiar to me. Are you supposed",
                "to be here? Otherwise, you should",
                "leave this place if you don't have",
                "the proper authorization."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I'm here on Vincent's behalf")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "Vincent sent you? I see.",
                "You must have been looking",
                "for me. I am High Priest Zhed,",
                "and I understand that you",
                "have a package for me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "Usually, Vincent doesn't",
                "trust strangers to deliver",
                "these packages, but perhaps",
                "this was necessitated by some",
                "strange circumstance. Anyway,",
                "let's see what he sent me..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "Hm? What's this?",
                "Vincent sent a note?",
                "I suppose this must be",
                "important. Give me just a",
                "moment to read this, please..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "...............................",
                "...Hrrrmm. Vincent. Now",
                "I understand. So he was being",
                "intentionally cold towards Jenny",
                "and Phoebe so other people",
                "wouldn't get suspicious..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "Still, I don't think he",
                "had to do that--it might",
                "have been a little too",
                "much. Well, I really",
                "appreciate your help",
                "in this matter, adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "But tell me, did you only",
                "help Vincent and risk your",
                "life because you were being",
                "paid? Were you only being",
                "motivated by the money?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Actually... I was just curious.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "You were doing this just to",
                "satisfy your curiosity? Heh",
                "heh, that's very interesting.",
                "That's also the best attitude",
                "for a brave adventurer. I like",
                "that. May I have your name?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Tell him your name.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                "Yes, that's a fine name.",
                "It suits you well. I'll be sure",
                "to remember that. Again, let",
                "me thank you for risking your",
                "life to retrive my belongings."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "High Priest Zhed",
            args![
                "I shall be praying for",
                "Freya to guide and protect",
                "you in your travels. Peace",
                ((Val::from("be with you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.var("lost_boy").set(Val::from(13))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(8099)])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
        ctx.call(Function::GetExperience, vec![Val::from(900000), Val::from(0)])?;
        return Err(Stop::End);
    } else {
        if (ctx.var("lost_boy").get()? == 13 && ctx.var("ra_tem_q").get()?.number()? < 14) {
            ctx.lines_as(
                "High Priest Zhed",
                args![
                    ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                    "I appreciate that you've",
                    "managed to retrieve my",
                    "belongings for me. You are",
                    "a truly talented adventurer.",
                    "May Freya protect you..."
                ],
            )?;
        } else {
            if (ctx.var("lost_boy").get()? == 13 && ctx.var("ra_tem_q").get()? == 14) {
                ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Ah, good, I was hoping you'd",
                        "show up here sooner or later.",
                        "I have a favor to ask of you",
                        "since I'm too busy to do it",
                        "myself, and I trust you more",
                        "than any other adventurer."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "However, before I give you",
                        "the details, you must know",
                        "that this favor must be kept",
                        "secret. In other words, once",
                        "I explain the task, you must",
                        "accept my request."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Let me think about it.:Yes, sure.")])?) == 1 {
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "I understand, but if you",
                            "change your mind, please",
                            "come back and let me know.",
                            "It's very hard for me to find",
                            "someone that I can really",
                            "rely on to do this for me..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Ah, I'm relieved to see",
                        "that you accept. Don't worry,",
                        "this task isn't complicated,",
                        "nor will it require much",
                        "in the way of sacrifice."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "As you may already know,",
                        "our country of Arunafeltz",
                        "worships the goddess Freya.",
                        "There's almost no separation",
                        "between politics, society,",
                        "and our religion, really."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Now, the leader of our",
                        "national religion, our pope,",
                        "is a direct servant of Freya",
                        "and delivers her messages",
                        "to us. As such, she must live",
                        "by very strict guidelines."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Our pope is curious about",
                        "the outside world and wishes",
                        "to learn more about what lies",
                        "beyond Arunafeltz, but she",
                        "cannot come easily by this",
                        "sort of knowledge."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Firstly, she cannot leave",
                        "Rachel, the place where the",
                        "spirit of goddess Freya dwells.",
                        "Secondly, all citizens and priests are forbidden to leave the city in",
                        "order to preserve our sanctity."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "These are implicit rules that",
                        "normally aren't discussed openly,",
                        "but everyone follows them to",
                        "prevent from being stigmatized.",
                        "Frankly, I think it's somewhat shameful, but we're working on it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Furthermore, our pope must",
                        "maintain her image of aloof",
                        "piety, so it'd be inappropriate of her to openly question outside",
                        "world affairs. Although, such",
                        "knowledge would benefit her..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "That is why the high priests",
                        "decided to enlist a trustworthy",
                        "adventurer to inform our pope",
                        "about the outside world. Now",
                        "you understand why I've asked",
                        "you to keep all of this secret."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "You won't need to divulge",
                        "outrageous secrets or anything",
                        "like that. I presume mundane",
                        "details would be enough to",
                        "please her. Just let her be",
                        "able to envision your homeland."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Personally, I think it would",
                        "be refreshing for our pope to",
                        "hear a few of your stories as",
                        "she is always worshipping",
                        "our goddess on the behalf of",
                        "everyone in our glorious city."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Unfortunately, I cannot escort",
                        "you to the priestess, but I will",
                        "relate the means to which you",
                        "can gain an audience with her.",
                        "First, you will need a ^FF0000High",
                        "Priest's Recommendation^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "I will write that for you, so",
                        "there's no need to concern",
                        "yourself with that. Next, you",
                        "will need to gather ^FF000040 Glacial",
                        "Hearts^000000 as symbolic proof of",
                        "your strength and purity."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "While I finish writing this",
                        "recommendation, let me tell",
                        "you how to get to the pope's",
                        "office where you must go once",
                        "you gather 40 Glacial Hearts."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Head west from the chapel",
                        "in the center of the temple,",
                        "and then enter a door guarded",
                        "by two soldiers. Show them your",
                        "recommendation and Glacial Hearts, and they should grant you passage."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "High Priest Zhed",
                    args![
                        "Ah, here you are.",
                        "I've finished writing",
                        "your recommendation.",
                        "Well then, I guess I'll",
                        "see you after you've",
                        "completed this task."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3355FFYou received a High", "Priest's recommendation.^000000"])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.var("ra_tem_q").set(Val::from(15))?;
                ctx.var("lost_boy").set(Val::from(14))?;
                ctx.call(Function::SetQuest, vec![Val::from(8100)])?;
                return Err(Stop::End);
            } else {
                if (ctx.var("ra_tem_q").get()? == 15 || ctx.var("ra_tem_q").get()? == 16) {
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "Once you gather",
                            "^FF000040 Glacial Hearts^000000,",
                            "you can show them, along",
                            "with the recommendation I've",
                            "written for you, to the soldiers",
                            "guarding the pope's office."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "High Priest Zhed",
                        args![
                            "When you're ready to go",
                            "to the pope's office, just",
                            "head west from the chapel",
                            "to the center of the temple:",
                            "this is where the entrance is."
                        ],
                    )?;
                } else {
                    if ctx.var("ra_tem_q").get()? == 17 {
                        ctx.lines_as(
                            "High Priest Zhed",
                            args![
                                "Ah, thank you so much",
                                "for spending some time",
                                "with the pope for me.",
                                "I can tell that she really",
                                "enjoyed speaking to you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "High Priest Zhed",
                            args![
                                "I am curious, though...",
                                "Our pope is unaccustomed",
                                "to speaking with outsiders,",
                                "so I assume that you have",
                                "your own questions after",
                                "your conversation with her..."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Well, she did mention some Holy Ground...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                        ctx.lines_as("High Priest Zhed", args!["What...?!", "She mentioned", "the Holy Ground...?!"])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                        ctx.lines_as(
                            "High Priest Zhed",
                            args![
                                "The Holy Ground is a place",
                                "where all humans, including",
                                "the pope, have been forbidden.",
                                "We used to allow people to",
                                "visit in the past, but... "
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "High Priest Zhed",
                            args![
                                "Only the gods can access",
                                "that place now. Listen, if",
                                "the other priests ask about",
                                "your conversation with the",
                                "pope, please do not make any",
                                "mention of the Holy Ground."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "High Priest Zhed",
                            args![
                                "Please don't ask me",
                                "why, but believe me that",
                                "it's very important that",
                                "you feign ignorance of the",
                                "Holy Ground's existence!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "????",
                            args![
                                "Excuse me, Zhed?",
                                "May I speak with you",
                                "for a moment? I need",
                                "to talk to you about",
                                "a private matter..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                        ctx.lines_as(
                            "High Priest Zhed",
                            args![
                                "Er, yes, of course!",
                                "Excuse me for a moment...",
                                "While you're waiting for me,",
                                "why don't you relax in the",
                                "next room? Don't forget:",
                                "there is no Holy Ground."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        let choice = runtime::select_values(ctx, &[Val::from("Okay.")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.var("ra_tem_q").set(Val::from(18))?;
                        ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(297), Val::from(156)])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ra_tem_q").get()? == 18 {
                            ctx.lines_as(
                                "High Priest Zhed",
                                args![
                                    "I'm sorry, but we're not",
                                    "quite finished with our",
                                    "discussion. Why don't you",
                                    "wait and relax in the next room",
                                    "over there in the meantime?"
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(8101), Val::from(8102)])?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            ctx.call(Function::Warp, vec![Val::from("ra_temin"), Val::from(297), Val::from(156)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ra_tem_q").get()? == 19 {
                                ctx.lines_as(
                                    "High Priest Zhed",
                                    args![
                                        "I apologize for making",
                                        "you wait for so long: I had",
                                        "some crucial matters to",
                                        "discuss. Now, that look",
                                        "on your face tells me that",
                                        "you have a pressing question."
                                    ],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Ask about Bekento")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                                ctx.lines_as(
                                    "High Priest Zhed",
                                    args![
                                        "Bekento? Ah, you must",
                                        "have been speaking to",
                                        "High Priestess Niren.",
                                        "She's the only one that",
                                        "calls me that. You see, my",
                                        "full name is Zhed Bekento."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                ctx.lines_as(
                                    "High Priest Zhed",
                                    args![
                                        "Speaking of which...",
                                        "Did Niren ask you anything",
                                        "or tell you something that",
                                        "was out of the ordinary?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFAfter a little hesitation,",
                                    "you tell High Priest Zhed",
                                    "about your conversation",
                                    "with High Priestess Niren.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "High Priest Zhed",
                                    args![
                                        "Hmm, I see.",
                                        "Yes. That sounds",
                                        "like something she'd do.",
                                        "I'll explain it all later, but",
                                        "right now, I need to rest.",
                                        "I'm getting old, you know."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                                ctx.lines_as(
                                    "High Priest Zhed",
                                    args![
                                        "Ah, but before you go, let",
                                        "me warn you not to ^FF0000go near",
                                        "the Holy Ground^000000. I repeat,",
                                        "stay away from the Holy Ground."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFThe Holy Ground...",
                                    "Why is High Priest Zhed",
                                    "so adamant about protecting",
                                    "the Holy Ground? Regardless...",
                                    "If you want to go there, then",
                                    "just go there. Who'll stop you?^000000"
                                ])?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                ctx.var("ra_tem_q").set(Val::from(20))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(8103), Val::from(8104)])?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("ra_tem_q").get()?.number()? >= 20 && ctx.var("ra_tem_q").get()?.number()? < 22) {
                                    ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "High Priest Zhed",
                                        args![
                                            "Whatever you do, whatever",
                                            "journeys you take, please",
                                            "make sure that you stay",
                                            "away from the Holy Ground!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFNow, more than ever,",
                                        "you feel compelled to",
                                        "enter the Holy Ground.^000000"
                                    ])?;
                                } else {
                                    if ctx.var("ra_tem_q").get()? == 23 {
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Oh, it's you.",
                                                "Let me guess...",
                                                "After everything I said,",
                                                "you still went ahead and",
                                                "visited the Holy Ground,",
                                                "didn't you? Oh, well..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("Did something happen?")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Well, honestly, I've been",
                                                "temporarily suspended from",
                                                "work, but no need to worry.",
                                                "It's not your fault. I've been",
                                                "in conflict with the other High",
                                                "Priests for a while, anyway."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Please don't worry about it.",
                                                "If not this, they would have",
                                                "found some other way to attack",
                                                "me. It would only be a matter",
                                                "of time. They're exaggerating",
                                                "your intrusion for their benefit."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "All I want is for",
                                                "Arunafeltz to be safe",
                                                "and at peace. I hope this",
                                                "land is not twisted by the",
                                                "greedy humans. I hope all will",
                                                "work according to Freya's will."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Ah, and please do not",
                                                "blame Niren. She must have",
                                                "her own reasons for her own",
                                                "actions. She has changed",
                                                "much but in the end, I think",
                                                "that we are still friends."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "One last thing:",
                                                "try not to let the",
                                                "other priests catch you",
                                                "doing anything forbidden.",
                                                "It'd be a little embarassing",
                                                "for me, you understand."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Still, you were secretly",
                                                "invited here, so I'm sure",
                                                "that only a very few of them",
                                                "would be able to recognize",
                                                "you. In the end, you'll do",
                                                "what you will, right?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "...I think I know what",
                                                "you saw in the Holy Ground.",
                                                "I'm sure that you have much",
                                                "to ask, but now isn't the time",
                                                "to seek for answers. Please,",
                                                "you must be patient."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Until the right time",
                                                "comes, I want you not",
                                                "to tell anyone else what",
                                                "you saw in the Holy Ground."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "High Priest Zhed",
                                            args![
                                                "Well then, adventurer,",
                                                "thank you for coming by.",
                                                "I need to go lay down now,",
                                                "so if you'll let me rest..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.var("misc_quest")
                                            .set(runtime::op(&ctx.var("misc_quest").get()?, "|", &Val::from(8192))?)?;
                                        ctx.var("ra_tem_q").set(Val::from(0))?;
                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(8105)])?;
                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
                                        ctx.call(Function::GetExperience, vec![Val::from(900000), Val::from(600000)])?;
                                        return Err(Stop::End);
                                    } else {
                                        if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true() {
                                            if ctx.var("rachel_camel").get()? == 25 {
                                                if ctx.var("aru_monas").get()? == 11 {
                                                    ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                                    ctx.lines_as(
                                                        "High Priest Zhed",
                                                        args![
                                                            "Ah, it's been a while,",
                                                            "hasn't it? You look well,",
                                                            "and I'm doing fine as you",
                                                            "can see. What can I do",
                                                            ((Val::from("for you today, ")
                                                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("?"))
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    match runtime::select_values(
                                                        ctx,
                                                        &[Val::from("I just wanted to say hi.:Ask About Veins Incident")],
                                                    )? {
                                                        1 => {
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "As you well know, I'm only",
                                                                    "a High Priest in title, and",
                                                                    "lost much of my influence",
                                                                    "amongst many of the other",
                                                                    "priests for my views."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "Still, I have faith that",
                                                                    "justice and good will",
                                                                    "prevail over the corruption",
                                                                    "that plagues our priests,",
                                                                    "so long as good people",
                                                                    "are willing to act."
                                                                ],
                                                            )?;
                                                        }
                                                        2 => {
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "Veins incident?",
                                                                    "I haven't heard of",
                                                                    "anything... Well, perhaps",
                                                                    "if you'd elaborate, then",
                                                                    "I'd understand a bit more."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines(args![
                                                                "^3355FFYou told High Priest",
                                                                "Zhed about the group of",
                                                                "smugglers arrested in Veins.^000000"
                                                            ])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "Oh, I think I did hear",
                                                                    "something about that earlier.",
                                                                    "Let's see now... Ah, right.",
                                                                    "Niren mentioned something",
                                                                    "strange about them..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "The smugglers were with",
                                                                    "a high ranking official from",
                                                                    "the Rune-Midgarts Kingdom,",
                                                                    "which is why Niren was so",
                                                                    "hesitant to take any action",
                                                                    "against them for a while."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "I was too preoccupied with",
                                                                    "other matters, and I didn't",
                                                                    "think much of it at the time.",
                                                                    "Why don't you speak to Niren?",
                                                                    "I'm sure that she can tell you",
                                                                    "more about what happened."
                                                                ],
                                                            )?;
                                                            ctx.var("aru_monas").set(Val::from(12))?;
                                                            ctx.call(Function::ChangeQuest, vec![Val::from(17007), Val::from(17008)])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args!["If you do decide to", "visit Niren, please", "send her my regards."],
                                                            )?;
                                                        }
                                                        _ => {}
                                                    }
                                                } else {
                                                    if ctx.var("aru_vol").get()? == 0 {
                                                        ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Ah, it does my heart to",
                                                                "good to see you again.",
                                                                "Adventurers willing to work",
                                                                "so hard for the sake of world",
                                                                "peace like yourself are fairly",
                                                                "uncommon. I hope you know that."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Huh...?",
                                                                "Please don't flatter",
                                                                "me like that! Saying",
                                                                "it like that is kind of",
                                                                "overdoing it, isn't it?"
                                                            ],
                                                        )?;
                                                        ctx.call(
                                                            Function::Emotion,
                                                            vec![
                                                                ctx.constant("ET_OHNO")?,
                                                                Val::from(
                                                                    ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true(),
                                                                ),
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Not at all. In fact, I'm",
                                                                "sure that a place in",
                                                                "Valhalla has already",
                                                                "been secured for you",
                                                                "by the Valkyries. Now, how",
                                                                "may I help you, my friend?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        let choice = runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Mysterious Building in the Volcano")],
                                                        )?;
                                                        ctx.var("@menu").set(choice)?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Actually, I was wondering",
                                                                "if you knew of anything about",
                                                                "this facility inside Thor",
                                                                "Volcano near Veins. There",
                                                                "might be something big",
                                                                "going on over there."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args![
                                                            "^3131FFYou told High Priest",
                                                            "Zhed about the shackled",
                                                            "child in Thor Volcano, and",
                                                            "how you found the building",
                                                            "hidden there. High Priest Zhed",
                                                            "seemed disturbed by your words.^000000"
                                                        ])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "I see. I have my suspicions",
                                                                "about what is happening. Now,",
                                                                "you're aware that Arunafeltz",
                                                                "is governed by our church,",
                                                                "and that our entire nation",
                                                                "worships the goddess Freya."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "The priests that lead this",
                                                                "nation are split into two main",
                                                                "factions: I am considered part",
                                                                "of the moderate faction, which",
                                                                "usually butts heads with the",
                                                                "jingoistic hard-liner faction."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "These hard-liner priests",
                                                                "are more than willing to use",
                                                                "violent methods if they believe",
                                                                "it to be the will of Freya, and",
                                                                "are the ones that established",
                                                                "the military camp  at Thor Volcano."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Of course, it's also",
                                                                "considered a geology camp",
                                                                "since they're always monitoring",
                                                                "that place for volcanic activity,",
                                                                "but for all intents and purposes,",
                                                                "they are training men to fight."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "It scares me to think",
                                                                "about it, but I'm sure that",
                                                                "they have many military supplies",
                                                                "and machines of mass destruction",
                                                                "hidden over there in the camp."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Wait, what exactly do",
                                                                "they hope to accomplish",
                                                                "by fighting? How do they",
                                                                "intend to help the goddess",
                                                                "Freya by using violence?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "All the priests agree that",
                                                                "reassembling Ymir's Heart will",
                                                                "revive Freya. As you know, the",
                                                                "pieces are scattered all over",
                                                                "the world. Some are possessed",
                                                                "by the Rune-Midgarts Kingdom."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "In the interest of avoiding",
                                                                "conflict, the moderate faction",
                                                                "of priests decided to hire",
                                                                "scientists to reproduce Ymir's",
                                                                "Heart, resulting in an",
                                                                "imitation of that artifact."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "An imitation of Ymir's",
                                                                "Heart? I think I know of",
                                                                "a famous scientist from the",
                                                                "Schwarzwald Republic that",
                                                                "was working on that! Let's",
                                                                "see, his name was..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Isn't the scientist's name..."],
                                                        )?;
                                                        let (input, status) = runtime::input_text(ctx, None, None)?;
                                                        l_input_s = input;
                                                        ctx.lines(args![
                                                            ((Val::from("^3131FF") + l_input_s.clone()) + Val::from("^000000?"))
                                                        ])?;
                                                        ctx.next()?;
                                                        if l_input_s.clone() == "Varmunt" {
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    "Oh, you've heard of him?",
                                                                    "Yes, his name was Varmunt.",
                                                                    "Unfortunately, he deliberately",
                                                                    "destroyed the Imitation of",
                                                                    "Ymir's Heart into pieces,",
                                                                    "and then disappeared."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                        } else {
                                                            ctx.lines_as(
                                                                "High Priest Zhed",
                                                                args![
                                                                    ((Val::from("") + l_input_s.clone()) + Val::from("?")),
                                                                    "No, that wasn't him.",
                                                                    "The man that created the",
                                                                    "imitation of Ymir's Heart",
                                                                    "was named Varmunt.",
                                                                    "He was truly a genius."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                        }
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "I'm not sure what happened",
                                                                "to Varmunt. All I know was that",
                                                                "something serious happened",
                                                                "in the Schwarzwald Republic,",
                                                                "and that he disappeared in",
                                                                "the fiasco. It's strange..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Although the working",
                                                                "Ymir's Heart imitation that",
                                                                "Varmunt had created was broken",
                                                                "into pieces, other scientists",
                                                                "managed to mostly reproduce",
                                                                "his work with unstable Ymir Hearts."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Since our moderate faction,",
                                                                "was only able to provide",
                                                                "unstable Ymir Heart imitations,",
                                                                "the hard-liner faction quickly",
                                                                "gained more respectability than",
                                                                "us in the eyes of the people."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Despite our advanced weaponry",
                                                                "and armor, I'm afraid that our",
                                                                "country of Arunafeltz is not as",
                                                                "civilized and peaceful as I'd",
                                                                "like it to be, and the hard-liners",
                                                                "control our government now."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Although the hard-liners",
                                                                "allowed us to continue",
                                                                "our research in faithfully",
                                                                "reproducing Ymir's Heart,",
                                                                "they've been developing their",
                                                                "military might unopposed."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "So basically, the moderates",
                                                                "are trying to recreate Ymir's",
                                                                "Heart at research centers",
                                                                "in the Holy Ground and the",
                                                                "Schwarzwald Republic."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "The hard-liners are",
                                                                "training soldiers and",
                                                                "storing up military supplies",
                                                                "and weapons in their camp",
                                                                "in Thor Volcano. However,",
                                                                "I don't know any more details."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "This doesn't look good.",
                                                                "I mean, if they're gearing",
                                                                "up to take the Ymir Heart",
                                                                "Pieces by force, it'll be war",
                                                                "against the Schwalzvalt Republic",
                                                                "and the Rune-Midgarts Kingdom!"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "Seeing as I'm part of the",
                                                                "moderate faction, as well as",
                                                                "recently demoted, I just don't",
                                                                "have access to much information",
                                                                "about the hard-liners. However,",
                                                                "there may still be hope yet..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "War would be bad for all",
                                                                "parties involved. That is",
                                                                "why I'd like to ask you to",
                                                                "sneak into the camp at",
                                                                "Thor Volcano for the sake of",
                                                                "protecting international peace."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "I really want to know",
                                                                "more about what they're",
                                                                "planning to do, but that",
                                                                "place is heavily guarded!",
                                                                "How am I going to sneak in?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "High Priest Zhed",
                                                            args![
                                                                "You're right.",
                                                                "We'll have to think",
                                                                "of a really good way",
                                                                "for you to infiltrate",
                                                                "that place. Hmmmm..."
                                                            ],
                                                        )?;
                                                        ctx.var("aru_vol").set(Val::from(1))?;
                                                        ctx.call(Function::SetQuest, vec![Val::from(2114)])?;
                                                        ctx.close_window()?;
                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if (ctx.var("aru_vol").get()?.number()? > 0
                                                            && ctx.var("aru_vol").get()?.number()? < 5)
                                                        {
                                                            ctx.lines_as("High Priest Zhed", args!["Hmm..."])?;
                                                            ctx.next()?;
                                                            match runtime::select_values(
                                                                ctx,
                                                                &[Val::from("How to Sneak into the Camp :The Moderates:The Hard-Liners")],
                                                            )? {
                                                                1 => {
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "I still don't have any",
                                                                            "ideas for sneaking into",
                                                                            "the camp at Thor Volcano.",
                                                                            "Give me a little more time,",
                                                                            "and hopefully we can figure",
                                                                            "something out together."
                                                                        ],
                                                                    )?;
                                                                }
                                                                2 => {
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "We priests of the moderate",
                                                                            "faction are opposed to war,",
                                                                            "so we've been trying to",
                                                                            "develop imitations of Ymir's",
                                                                            "Heart for a long time. We've",
                                                                            "yet to succeed, unfortunately."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "The hard-liners actually",
                                                                            "opposed our efforts, but",
                                                                            "they've let us continue since",
                                                                            "they think they can eventually",
                                                                            "use imitation Ymir Hearts for",
                                                                            "military purposes."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "The Ymir Heart reproduction",
                                                                            "project has been frustrating",
                                                                            "the best minds in the world,",
                                                                            "and some scientists have even",
                                                                            "resorted to using forbidden",
                                                                            "methods. It's a shame, really."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "Varmunt actually succeeded",
                                                                            "in recreating Ymir's Heart,",
                                                                            "but then he destroyed it and",
                                                                            "he disappeared. He did it to",
                                                                            "prevent war, but we could",
                                                                            "really use his help now."
                                                                        ],
                                                                    )?;
                                                                }
                                                                3 => {
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "The hard-liners are especially",
                                                                            "fanatical in their devotion to",
                                                                            "Freya, and will use any means",
                                                                            "to further what they interpret",
                                                                            "as her will. They're willing to",
                                                                            "go to war for their beliefs."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "The fact that they've even",
                                                                            "built a military camp in Thor",
                                                                            "Volcano disturbs me greatly.",
                                                                            "It's almost certain that they",
                                                                            "intend to wage war against any",
                                                                            "nation with a Ymir's Heart piece."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args!["Who is in charge of that", "camp at Thor's Volcano?"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "High Priest Vildt.",
                                                                            "He's in charge of all",
                                                                            "important buildings in",
                                                                            "Arunafeltz. His room is at",
                                                                            "the other side of the building,",
                                                                            "but it's always heavily guarded."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "Vildt is one of the",
                                                                            "highest ranking members",
                                                                            "of the hard-liner priest",
                                                                            "faction. He rarely makes it",
                                                                            "his business to see anybody",
                                                                            "that he sees as beneath him."
                                                                        ],
                                                                    )?;
                                                                }
                                                                _ => {}
                                                            }
                                                            ctx.close_window()?;
                                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ctx.var("aru_vol").get()? == 5 {
                                                                ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                                                if ctx.call(Function::CountItem, vec![Val::from(7342)])?.number()? > 0 {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Would you please",
                                                                            "take a look at this",
                                                                            "file, High Priest Zhed?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args!["Of course.", "Let's see, what's", "in this File Folder?"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            ".............",
                                                                            ".................",
                                                                            "....................!"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "This is a geological",
                                                                            "report about Thor Volcano?",
                                                                            "Ah, I see. They must have",
                                                                            "someone regularly check",
                                                                            "to see if it will erupt."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "Although it's fairly dangerous",
                                                                            "to build a military camp there,",
                                                                            "some weapon materials can",
                                                                            "only be forged from the heat",
                                                                            "coming from Thor's Volcano."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "Yes, I think it's also",
                                                                            "likely that there are special",
                                                                            "weapon crafting metals and",
                                                                            "materials that can only be",
                                                                            "found on the volcano. That",
                                                                            "makes perfect sense."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "Although the volcano is",
                                                                            "dormant now, these reports",
                                                                            "claim that the volcano is",
                                                                            "either safe or ready to erupt",
                                                                            "at any time. The data here",
                                                                            "doesn't make any sense."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("High Priest Zhed", args!["Hm. This is just a hunch,", "but seeing these discrepancies", "in his reports, I really think this geologist has an agenda against", "the camp at Thor Volcano."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "You know, if that's true,",
                                                                            "he might be able to help",
                                                                            "you sneak into the camp.",
                                                                            "Don't you think it's worth",
                                                                            "checking out for now?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 2
                                                                    {
                                                                        ctx.lines_as(
                                                                            "High Priest Zhed",
                                                                            args![
                                                                                "Well, you never know",
                                                                                "until you ask. Besides,",
                                                                                "I don't know if there's",
                                                                                "any other way to sneak",
                                                                                "into the Thor Volcano Camp..."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                    }
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "I hope you'll go and talk",
                                                                            "to the geologist. He should",
                                                                            "be in Veins, the desert city",
                                                                            "south of Rachel. He's your",
                                                                            "best chance of entering the Thor",
                                                                            "Volcano camp without suspicion."
                                                                        ],
                                                                    )?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                                                                    ctx.var("aru_vol").set(Val::from(6))?;
                                                                    ctx.call(
                                                                        Function::ChangeQuest,
                                                                        vec![Val::from(2115), Val::from(2116)],
                                                                    )?;
                                                                } else {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args!["Huh...?", "Where did I put", "that File Folder?"],
                                                                    )?;
                                                                }
                                                            } else {
                                                                if ctx.var("aru_vol").get()? == 6 {
                                                                    ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                                                    ctx.lines_as(
                                                                        "High Priest Zhed",
                                                                        args![
                                                                            "I hope you'll go and talk",
                                                                            "to the geologist. He should",
                                                                            "be in Veins, the desert city",
                                                                            "south of Rachel. He's your",
                                                                            "best chance of entering the Thor",
                                                                            "Volcano camp without suspicion."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if (ctx.var("aru_vol").get()?.number()? > 6
                                                                        && ctx.var("aru_vol").get()?.number()? < 26)
                                                                    {
                                                                        ctx.call(
                                                                            Function::Cutin,
                                                                            vec![Val::from("ra_gman"), Val::from(2)],
                                                                        )?;
                                                                        ctx.lines_as(
                                                                            "High Priest Zhed",
                                                                            args![
                                                                                "I hope you'll go and talk",
                                                                                "to the geologist. He should",
                                                                                "be in Veins, the desert city",
                                                                                "south of Rachel. He's your",
                                                                                "best chance of entering the Thor",
                                                                                "Volcano camp without suspicion."
                                                                            ],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if ctx.var("aru_vol").get()? == 26 {
                                                                            ctx.call(
                                                                                Function::Cutin,
                                                                                vec![Val::from("ra_gman"), Val::from(2)],
                                                                            )?;
                                                                            ctx.lines_as(
                                                                                "High Priest Zhed",
                                                                                args![
                                                                                    "Oh! You're back!",
                                                                                    "Was that geologist able",
                                                                                    "to help you sneak into",
                                                                                    "the Thor Volcano camp?"
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                                args![
                                                                                    "More than that. We've",
                                                                                    "convinced the camp that the",
                                                                                    "volcano is going to explode,",
                                                                                    "so they're scrambling to",
                                                                                    "evacuate. They're really",
                                                                                    "panicking over there!"
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "High Priest Zhed",
                                                                                args![
                                                                                    "Hahaha! That's great!",
                                                                                    "Even if they do figure",
                                                                                    "out that they've been",
                                                                                    "fooled, it will take them",
                                                                                    "some time to realize it",
                                                                                    "in all that confusion."
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "High Priest Zhed",
                                                                                args![
                                                                                    "Yes, it might not be the",
                                                                                    "perfect time to bring down",
                                                                                    "the hard-liner faction right",
                                                                                    "now, but the opportunity",
                                                                                    "should present itself soon.",
                                                                                    "Thank you for all your help."
                                                                                ],
                                                                            )?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as(
                                                                                "High Priest Zhed",
                                                                                args![
                                                                                    "Please relax for a while,",
                                                                                    "and I'll take care of the rest.",
                                                                                    "When the time to strike comes,",
                                                                                    "I will be contacting you again.",
                                                                                    "Together, we can protect",
                                                                                    "peace between our nations."
                                                                                ],
                                                                            )?;
                                                                            ctx.var("aru_vol").set(Val::from(27))?;
                                                                            ctx.call(Function::CompleteQuest, vec![Val::from(60213)])?;
                                                                            ctx.call(
                                                                                Function::GetExperience,
                                                                                vec![Val::from(200000), Val::from(0)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            if ctx.var("aru_vol").get()?.number()? > 26 {
                                                                                if ctx.var("aru_em").get()?.number()? < 7 {
                                                                                    ctx.lines_as(
                                                                                        "High Priest Zhed",
                                                                                        args![
                                                                                            "I'll be sure to contact",
                                                                                            "you again for your help",
                                                                                            "when the time is right.",
                                                                                            "I'm sure there are others",
                                                                                            "that may need you now..."
                                                                                        ],
                                                                                    )?;
                                                                                } else {
                                                                                    if ctx.var("aru_em").get()? == 8 {
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("ra_gman"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Welcome, adventurer.",
                                                                                                "I'm glad to see that you're",
                                                                                                "here. I think... I think an",
                                                                                                "opportunity has arisen in",
                                                                                                "which I could use your",
                                                                                                "expert assistance."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("ra_gman2"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "Though, I fear that I may be",
                                                                                                "making the wrong decision.",
                                                                                                "After all, I no longer have",
                                                                                                "much influence among the",
                                                                                                "other priests... I can't help",
                                                                                                "but think I'm in the wrong."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Don't lose heart,",
                                                                                                "High Priest Zhed.",
                                                                                                "I... I believe in you!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("ra_gman"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args!["I guess you're right."],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "As you well know, the",
                                                                                                "highest authority in",
                                                                                                "Arunafeltz is our Pope,",
                                                                                                "the reincarnation of",
                                                                                                "our goddess Freya."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "Unfortunately, she is too",
                                                                                                "young and innocent, and she",
                                                                                                "doesn't yet have the experience",
                                                                                                "to assert control, and stamp",
                                                                                                "out the in-fighting and",
                                                                                                "corruption among the priests."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "In these desperate times,",
                                                                                                "we will need someone to",
                                                                                                "provide strong and confident",
                                                                                                "leadership to the people",
                                                                                                "of Arunafeltz before all",
                                                                                                "is lost to war and violence."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "^333333*Sigh...*^000000",
                                                                                                "I know it may not be",
                                                                                                "right, but someone",
                                                                                                "must do something",
                                                                                                "before the other High",
                                                                                                "Priests ruin our nation."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Emotion,
                                                                                            vec![
                                                                                                ctx.constant("ET_HUK")?,
                                                                                                Val::from(
                                                                                                    ctx.call(
                                                                                                        Function::GetCharacterId,
                                                                                                        vec![Val::from(0)],
                                                                                                    )?
                                                                                                    .is_true(),
                                                                                                ),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Y-you're not thinking",
                                                                                                "of overthrowing the",
                                                                                                "pope are you? I-isn't",
                                                                                                "that a bit extreme?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("ra_gman2"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.call(
                                                                                            Function::Emotion,
                                                                                            vec![ctx.constant("ET_SWEAT")?],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "What? No, no!",
                                                                                                "Don't jump to conclusions",
                                                                                                "like that! Don't misunderstand",
                                                                                                "me: I'm loyal to our pope."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Cutin,
                                                                                            vec![Val::from("ra_gman"), Val::from(2)],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "I actually want to work",
                                                                                                "very closely with the pope,",
                                                                                                "in essense, rule together with",
                                                                                                "her by combining my experience",
                                                                                                "and her wisdom. It's ambitious,",
                                                                                                "but what else can I do?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "You remember how you",
                                                                                                "stopped the hard-liner",
                                                                                                "priests from carrying out",
                                                                                                "their plans for war in the",
                                                                                                "Thor Volcano camp? That will",
                                                                                                "only stop them temporarily."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "They'll eventually prepare",
                                                                                                "to act out in violence again.",
                                                                                                "We need a permanent solution",
                                                                                                "to ensure that this sort of",
                                                                                                "thing doesn't happen again."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "We need to convince the",
                                                                                                "pope to finally act, and",
                                                                                                "give my policy of peace",
                                                                                                "her full support. Will",
                                                                                                "you help me to do this?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        if Val::from(runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from("Yes:No")],
                                                                                        )?) == 2
                                                                                        {
                                                                                            ctx.lines(args![
                                                                                                "Well... Asking for the",
                                                                                                "pope's support is the only",
                                                                                                "solution I've been able to",
                                                                                                "find. Do you have any ideas",
                                                                                                "for protecting peace here",
                                                                                                "in Arunafeltz?"
                                                                                            ])?;
                                                                                            ctx.next()?;
                                                                                            let (input, status) =
                                                                                                runtime::input_text(ctx, None, None)?;
                                                                                            l_input_s = input;
                                                                                            ctx.lines_as(
                                                                                                ctx.call(
                                                                                                    Function::StrCharInfo,
                                                                                                    vec![Val::from(0)],
                                                                                                )?,
                                                                                                args![
                                                                                                    ((Val::from("") + l_input_s.clone())
                                                                                                        + Val::from("?"))
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as(
                                                                                                "High Priest Zhed",
                                                                                                args![
                                                                                                    "Well, I don't know.",
                                                                                                    "It's a good idea, but",
                                                                                                    "I fear that it will only",
                                                                                                    "be effective in the short",
                                                                                                    "term. Why don't we just",
                                                                                                    "try what I've proposed first?"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                        }
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "Hmm, now how can",
                                                                                                "we convince the pope to",
                                                                                                "support us? Even as Freya's",
                                                                                                "reincarnation... She's just",
                                                                                                "a young girl. She probably",
                                                                                                "knows nothing of politics."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "It'd probably be best to",
                                                                                                "tell her exactly what is",
                                                                                                "happening. The full truth.",
                                                                                                "In my heart, I know that",
                                                                                                "would be best."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "But... Maybe it would",
                                                                                                "be better if we sugar",
                                                                                                "coated it a little bit.",
                                                                                                "She might not be too",
                                                                                                "receptive to us if we",
                                                                                                "totally shock her."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "Hmm... What do you",
                                                                                                "think? Should we stick",
                                                                                                "with the truth, or do you",
                                                                                                "think it'd be better if we",
                                                                                                "exaggerate and... lie when",
                                                                                                "we feel it's necessary?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        match runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from("Truth:Lies")],
                                                                                        )? {
                                                                                            1 => {
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "That's what I think too.",
                                                                                                        "Although I'm worried about",
                                                                                                        "how the pope will react at",
                                                                                                        "our message, I should have",
                                                                                                        "faith. I should have faith."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                            }
                                                                                            2 => {
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "I suppose that you may",
                                                                                                        "have a point. It might not",
                                                                                                        "be a good idea to totally",
                                                                                                        "expose her to the ugly",
                                                                                                        "truth. I'll try to be as",
                                                                                                        "honest as I can..."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                            }
                                                                                            _ => {}
                                                                                        }
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "Well, there's no time",
                                                                                                "to waste. The two of us may",
                                                                                                "not be enough to convince",
                                                                                                "the pope to act for peace.",
                                                                                                "I already have someone in",
                                                                                                "mind that might help us."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Emotion,
                                                                                            vec![
                                                                                                ctx.constant("ET_SPARK")?,
                                                                                                Val::from(
                                                                                                    ctx.call(
                                                                                                        Function::GetCharacterId,
                                                                                                        vec![Val::from(0)],
                                                                                                    )?
                                                                                                    .is_true(),
                                                                                                ),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args!["That sounds good.", "Who is he?"],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "My old friend, High Priest",
                                                                                                "Niren. I'm not sure if she'll",
                                                                                                "help me after all that's",
                                                                                                "happened between us. She",
                                                                                                "may even think I deserve",
                                                                                                "to be excommunicated..."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.call(
                                                                                            Function::Emotion,
                                                                                            vec![
                                                                                                ctx.constant("ET_BEST")?,
                                                                                                Val::from(
                                                                                                    ctx.call(
                                                                                                        Function::GetCharacterId,
                                                                                                        vec![Val::from(0)],
                                                                                                    )?
                                                                                                    .is_true(),
                                                                                                ),
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Come on, it wouldn't",
                                                                                                "hurt to try! Besides,",
                                                                                                "Niren isn't a bad person.",
                                                                                                "Maybe she'll understand",
                                                                                                "the way you see things",
                                                                                                "if you explain them?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "You're right, of course.",
                                                                                                "I'm just grasping at straws",
                                                                                                "here, but she and I both used",
                                                                                                "to believe that peace was",
                                                                                                "possible. I don't know why",
                                                                                                "she joined the hard-liners..."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                ((Val::from("")
                                                                                                    + ctx.call(
                                                                                                        Function::StrCharInfo,
                                                                                                        vec![Val::from(0)]
                                                                                                    )?)
                                                                                                    + Val::from("...")),
                                                                                                "Would you please talk to",
                                                                                                "her, and just see how she's",
                                                                                                "doing? It might not be a good",
                                                                                                "time to mention our plan",
                                                                                                "right away, though..."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "If you think that you",
                                                                                                "have a chance of convincing",
                                                                                                "her, then feel free and offer",
                                                                                                "my proposal. Her office is",
                                                                                                "located across from here,",
                                                                                                "but she's always very busy..."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "High Priest Zhed",
                                                                                            args![
                                                                                                "There's actually",
                                                                                                "a good chance that",
                                                                                                "she is away somewhere",
                                                                                                "on business. She is highly",
                                                                                                "loved by the people, you know."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.var("aru_em").set(Val::from(9))?;
                                                                                        ctx.call(
                                                                                            Function::ChangeQuest,
                                                                                            vec![Val::from(2131), Val::from(2132)],
                                                                                        )?;
                                                                                    } else {
                                                                                        if (ctx.var("aru_em").get()? == 9
                                                                                            || ctx.var("aru_em").get()? == 10)
                                                                                        {
                                                                                            ctx.lines_as(
                                                                                                "High Priest Zhed",
                                                                                                args![
                                                                                                    "Please speak to Niren,",
                                                                                                    "and she if she'd be willing",
                                                                                                    "to help us convince the pope",
                                                                                                    "to enforce peace in our nation."
                                                                                                ],
                                                                                            )?;
                                                                                        } else {
                                                                                            if ctx.var("aru_em").get()? == 11 {
                                                                                                ctx.call(
                                                                                                    Function::Cutin,
                                                                                                    vec![
                                                                                                        Val::from("ra_gman"),
                                                                                                        Val::from(2),
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "Welcome back.",
                                                                                                        "So did you get a chance",
                                                                                                        "to speak to Niren? How did",
                                                                                                        "she respond to my proposal?"
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines(args![
                                                                                                    "^3355FFYou explain Niren's reasons",
                                                                                                    "for her disappointment in",
                                                                                                    "the Moderate priest faction,",
                                                                                                    "and tell Zhed that she intends",
                                                                                                    "to carry out the plans for war."
                                                                                                ])?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "....................",
                                                                                                        "....................",
                                                                                                        "...................."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "Hmm... This explains why",
                                                                                                        "she and some other high",
                                                                                                        "priests started being absent",
                                                                                                        "from their offices. Many of",
                                                                                                        "them were invited over to",
                                                                                                        "the Schwarzwald Republic..."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "Regardless, I know for",
                                                                                                        "a fact that war is never",
                                                                                                        "a solution. It only brings",
                                                                                                        "death and destruction."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.next()?;
                                                                                                ctx.lines_as(
                                                                                                    "High Priest Zhed",
                                                                                                    args![
                                                                                                        "Give me a moment.",
                                                                                                        "I'm going to write",
                                                                                                        "a letter that I want",
                                                                                                        "you to give to Niren."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.var("aru_em").set(Val::from(12))?;
                                                                                            } else {
                                                                                                if ctx.var("aru_em").get()? == 12 {
                                                                                                    ctx.call(
                                                                                                        Function::Cutin,
                                                                                                        vec![
                                                                                                            Val::from("ra_gman"),
                                                                                                            Val::from(2),
                                                                                                        ],
                                                                                                    )?;
                                                                                                    ctx.lines_as(
                                                                                                        "High Priest Zhed",
                                                                                                        args![
                                                                                                            "Please bring this letter",
                                                                                                            "to Niren. I hope what",
                                                                                                            "I have to say will help",
                                                                                                            "her understand how I feel",
                                                                                                            "about these plans for war."
                                                                                                        ],
                                                                                                    )?;
                                                                                                    ctx.next()?;
                                                                                                    ctx.lines(args!["^3355FFYou received a letter for", "Niren from High Priest Zhed.^000000"])?;
                                                                                                    ctx.var("aru_em").set(Val::from(13))?;
                                                                                                    ctx.call(
                                                                                                        Function::ChangeQuest,
                                                                                                        vec![
                                                                                                            Val::from(2133),
                                                                                                            Val::from(2134),
                                                                                                        ],
                                                                                                    )?;
                                                                                                } else {
                                                                                                    if (ctx.var("aru_em").get()? == 13
                                                                                                        || ctx.var("aru_em").get()? == 14)
                                                                                                    {
                                                                                                        ctx.lines_as(
                                                                                                            "High Priest Zhed",
                                                                                                            args![
                                                                                                                "Please deliver my",
                                                                                                                "letter to Niren as",
                                                                                                                "soon as you can.",
                                                                                                                "I really appreciate",
                                                                                                                "you helping me on this."
                                                                                                            ],
                                                                                                        )?;
                                                                                                    } else {
                                                                                                        if (ctx
                                                                                                            .var("aru_em")
                                                                                                            .get()?
                                                                                                            .number()?
                                                                                                            > 14
                                                                                                            && ctx
                                                                                                                .var("aru_em")
                                                                                                                .get()?
                                                                                                                .number()?
                                                                                                                < 20)
                                                                                                        {
                                                                                                            ctx.lines_as("High Priest Zhed", args!["Great news! Niren just", "sent me a reply: I can't", "believe that she's willing", "to help me! Would you please", "give her any help that she", "may need from now on?"])?;
                                                                                                        } else if ctx.var("aru_em").get()?
                                                                                                            == 20
                                                                                                        {
                                                                                                            ctx.call(
                                                                                                                Function::Cutin,
                                                                                                                vec![
                                                                                                                    Val::from("ra_gman"),
                                                                                                                    Val::from(2),
                                                                                                                ],
                                                                                                            )?;
                                                                                                            ctx.lines_as(
                                                                                                                "High Priest Zhed",
                                                                                                                args![
                                                                                                                    "Ah, I've been",
                                                                                                                    "waiting for you."
                                                                                                                ],
                                                                                                            )?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["High Priest Zhed,", "everything's ready!", "High Priest Niren managed", "to send away all of those", "followers that were near the", "pope. You won't be interrupted."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["I hope we can get the pope", "to understand the situation", "of our nation. I only...", "Oh, Freya help us..."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as(
                                                                                                                "High Priest Zhed",
                                                                                                                args![
                                                                                                                    "..............",
                                                                                                                    "..............",
                                                                                                                    ".............."
                                                                                                                ],
                                                                                                            )?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as(
                                                                                                                "High Priest Zhed",
                                                                                                                args![
                                                                                                                    "Alright, I'm ready.",
                                                                                                                    "Let's receive an",
                                                                                                                    "audience with the",
                                                                                                                    "pope, shall we?"
                                                                                                                ],
                                                                                                            )?;
                                                                                                            ctx.var("aru_em")
                                                                                                                .set(Val::from(21))?;
                                                                                                        } else if ctx.var("aru_em").get()?
                                                                                                            == 21
                                                                                                        {
                                                                                                            ctx.lines_as("High Priest Zhed", args!["I'm a little nervous", "about talking so brazenly", "to the pope, but I'm", "ready to go meet her."])?;
                                                                                                        } else if ctx.var("aru_em").get()?
                                                                                                            == 22
                                                                                                        {
                                                                                                            ctx.lines_as("High Priest Zhed", args!["I believe that Niren", "has something important", "to tell you. Why don't", "you speak to her before", "coming back to me?"])?;
                                                                                                        } else if ctx.var("aru_em").get()?
                                                                                                            == 23
                                                                                                        {
                                                                                                            ctx.call(
                                                                                                                Function::Cutin,
                                                                                                                vec![
                                                                                                                    Val::from("ra_gman"),
                                                                                                                    Val::from(2),
                                                                                                                ],
                                                                                                            )?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["I'm glad that everything", "turned out great. Your", "help was truly a blessing.", "Although Schwarzwald will", "continue their plans without", "us, we've slowed them down."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["Yes, without our support,", "Schwarzwald won't have", "any excuse to continue", "their dangerous research", "for a little while, at least."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["Thor Volcano Camp", "should close down soon,", "now that we no longer", "have any use for it."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["I also underestimated our", "pope. She is more dignified,", "willed that I believed.", "I owe her my apologies."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["We won't give up on", "reviving our goddess Freya,", "but we will retrieve Ymir's", "Heart through diplomacy", "rather than violence.", "It won't be easy..."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["You've been instrumental", "in returning peace here in", "Arunafeltz. I've even been", "reinstated, and will be very", "busy from now on, working", "closely with the pope..."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["I know that the life of", "adventuring beckons you.", "I know she'd like to see", "you before you leave, but", "she's sleeping deeply after", "that exhausting speech..."])?;
                                                                                                            ctx.next()?;
                                                                                                            ctx.lines_as("High Priest Zhed", args!["On behalf of our pope", "and all the citizens of", "Arunafeltz, I'd like to", "thank you for your heroism.", "Your place in Valhalla is", "already assured, I'm sure."])?;
                                                                                                            ctx.var("aru_em")
                                                                                                                .set(Val::from(24))?;
                                                                                                            ctx.call(
                                                                                                                Function::CompleteQuest,
                                                                                                                vec![Val::from(2142)],
                                                                                                            )?;
                                                                                                            ctx.call(
                                                                                                                Function::GetExperience,
                                                                                                                vec![
                                                                                                                    Val::from(1500000),
                                                                                                                    Val::from(0),
                                                                                                                ],
                                                                                                            )?;
                                                                                                        } else {
                                                                                                            ctx.lines_as(
                                                                                                                "High Priest Zhed",
                                                                                                                args![
                                                                                                                    "May Freya bless you",
                                                                                                                    "on all your journeys,",
                                                                                                                    "my good friend."
                                                                                                                ],
                                                                                                            )?;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                ctx.call(
                                                                                    Function::Cutin,
                                                                                    vec![Val::from("ra_gman"), Val::from(2)],
                                                                                )?;
                                                                                ctx.lines_as(
                                                                                    "High Priest Zhed",
                                                                                    args![
                                                                                        "Ugh...!",
                                                                                        "I've got such",
                                                                                        "a horrible headache!"
                                                                                    ],
                                                                                )?;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "High Priest Zhed",
                                                    args!["I should rest for now.", "Please travel in safety."],
                                                )?;
                                            }
                                        } else {
                                            ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(2)])?;
                                            ctx.lines_as("High Priest Zhed", args!["May Freya be with you."])?;
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
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn high_priest_zhed_rachel(ctx: &Ctx) -> Script {
    high_priest_zhed_rachel_body(ctx, Vec::new()).map(|_| ())
}
