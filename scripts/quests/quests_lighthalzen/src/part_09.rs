use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn young_man_reken_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_rekenber").get()?.number()? > 21 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
        ctx.lines_as(
            "Kazien",
            args!["Just...", "Leave me alone.", "I feel nothing but", "guilt when I see you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kazien",
            args![
                "Don't take it the wrong",
                "way. I mean, it's not like",
                "you did nothing wrong. I'm",
                "the one who's... Geez. I wish",
                "I could live the way you do.",
                "Someday I'll be strong enough..."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("lhz_rekenber").get()? == 21 {
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz03"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Hey, you're back. As usual,",
                    "you've done a good job. You",
                    "look exhausted: did you run",
                    "into those thugs again?",
                    "Why don't you take a rest?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Kazien...",
                    "While I was fighting",
                    "those thugs, one of the",
                    "packages was accidentally",
                    "opened, and I saw what was",
                    "inside of those packages."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz06"), Val::from(2)])?;
            ctx.next()?;
            ctx.lines_as("Kazien", args!["......", ".........", "............"])?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz08"), Val::from(2)])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Wh-why are you supplying",
                    "those things? If we let those",
                    "packages get imported by",
                    "other countries, it can",
                    "cause a lot of trouble...!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args!["Stop. Please.", "J-just stop it.", "I don't want to", "hear anymore."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Hold it, I deserve an",
                    "answer! How can you turn",
                    "a blind eye and provide just",
                    "anyone with ^FF0000hi-tech weapons",
                    "and guardians^000000? It's like you're",
                    "promoting war and violence!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "And what about your little",
                    "brother Lyozien? He has",
                    "no idea what he's doing!",
                    "Don't we have a responsibility",
                    "to the world to make sure these",
                    "weapons aren't distributed?"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz11"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Shut up! You don't",
                    "know anything! Just",
                    "shut up! I'm doing this",
                    "for the sake of my family!",
                    "You don't know what it's like",
                    "to live in Lighthalzen's slums!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["................."])?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "There'd be days when my",
                    "brother and I'd have nothing",
                    "to eat. So when I heard about",
                    "this job, I took it. What good",
                    "is world peace if I'm not even",
                    "alive to enjoy it, huh?"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz11"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Now, my brother Lyozien is",
                    "a gentle soul, has nothing",
                    "but love for everybody. So, of",
                    "course I can't tell him what",
                    "I'm really doing--he'd never",
                    "agree to it, believe me."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "I hate this job and what",
                    "I'm doing and I want to quit.",
                    "But then what? Go back to the",
                    "slums? Forget it. As long as",
                    "Lyozien is happy, I don't mind if I have to do the devil's work."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz11"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "At least this way, keeping",
                    "it all secret, I can protect",
                    "Lyozien from the ugly nature",
                    "of this job, even if I'm dirtying my hands, making money",
                    "off of other people's deaths."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "But that still isn't right.",
                    "You're selling weapons so",
                    "that people can kill each other! Even if it's for the sake of",
                    "providing for your family..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz05"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Look man, this is what",
                    "I decided. I don't care",
                    "what other people'll think.",
                    "I might go to hell when",
                    "I die, but that's my problem."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Besides, you adventurers",
                    "are always running around",
                    "with your swords and magic spells... Isn't that just as bad?",
                    "It's not the weapons or the power that's bad: it's how they're used."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz11"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Granted, most of my clients",
                    "are pretty questionable, and",
                    "you adventurers usually use",
                    "your powers for good, but...",
                    "Damn it! Just... Don't come",
                    "back. I can't work like this..."
                ],
            )?;
            ctx.next()?;
            ctx.var("lhz_rekenber").set(Val::from(22))?;
            ctx.call(Function::GetExperience, vec![Val::from(550000), Val::from(0)])?;
            ctx.call(Function::CompleteQuest, vec![Val::from(12013)])?;
            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz01"), Val::from(2)])?;
            ctx.lines_as(
                "Kazien",
                args![
                    "Look, I can't have",
                    "you working with me",
                    "and Lyozien anymore.",
                    "Sorry, but it's for Lyozien's",
                    "own good. That, and you",
                    "make me feel guilty..."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else {
            if ctx.var("lhz_rekenber").get()?.number()? > 16 {
                ctx.call(Function::Cutin, vec![Val::from("lhz_kaz02"), Val::from(2)])?;
                ctx.lines_as(
                    "Kazien",
                    args![
                        "Hey now, you better",
                        "get a move on. You gotta",
                        "assist another delivery to",
                        "the Rune-Midgarts Kingdom."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            } else {
                if ctx.var("lhz_rekenber").get()? == 16 {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz03"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Ah, I heard from Lyozien",
                            "that you guys finished your",
                            "delivery. You're probably",
                            "the best part-timer that I've",
                            "had in a long, long while."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Anyway, we've got yet",
                            "another delivery for the",
                            "Rune-Midgarts Kingdom. It's",
                            "weird that we're getting more",
                            "orders from there, but orders from other countries are decreasing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz07"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Eh, I don't have the time",
                            "to wonder about stuff like",
                            "that. Lyozien's waiting for",
                            "you, so get to it, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "W-wait! During the last",
                            "delivery, I was attacked",
                            "by a group of thugs that",
                            "wanted to destroy the",
                            "packages? Why would",
                            "they want to do that?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Look... You're better",
                            "off not knowing. Or are",
                            "you asking me to pay you",
                            "more for this job since",
                            "you're risking your life?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I understand that the",
                            "customer's confidentiality",
                            "is important, but I'd feel a",
                            "lot better if I knew what was",
                            "in those packages, and why me",
                            "and Lyozien are being attacked."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz08"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Listen, I'm not obligated--",
                            "I can't tell you. Heck, I can't",
                            "even tell my own brother what's",
                            "in those packages. You can see",
                            "that, can't you? Anyway, you",
                            "can handle those thugs, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("lhz_rekenber").set(Val::from(17))?;
                    ctx.call(Function::GetExperience, vec![Val::from(450000), Val::from(0)])?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kazien",
                        args![
                            "Right. Now get back to",
                            "the Airship and talk to",
                            "Lyozien again. Don't give",
                            "him any trouble and make",
                            "sure you protect him."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("lhz_rekenber").get()?.number()? > 11 {
                        ctx.call(Function::Cutin, vec![Val::from("lhz_kaz08"), Val::from(2)])?;
                        ctx.lines_as(
                            "Kazien",
                            args![
                                "What are you doing",
                                "waiting around here",
                                "for? You've got a job to",
                                "do, so hurry up and do it~"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("lhz_rekenber").get()? == 11 {
                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz05"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "Hey, you're back. I got a",
                                    "message from Rune-Midgarts,",
                                    "telling us they received their",
                                    "order. Good work! So how do",
                                    "you like working with Lyozien?",
                                    "He's one of my best men."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Wait, aren't you and", "Lyozien supposed", "to be brothers?"],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "Whoa, he told you that?",
                                    "I guess he feels that he",
                                    "can trust you enough with",
                                    "that kind of personal talk...",
                                    "Yeah, he's my little brother.",
                                    "And a better man than me..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "I've gotten my hands",
                                    "pretty dirty doing this...",
                                    "Oh, forget it. You came ",
                                    "here for a job, right?",
                                    "Luckily, I got another",
                                    "delivery for you to work on..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Actually, Kazien...",
                                    "I was curious. What",
                                    "exactly are we delivering?",
                                    "I mean, not even Lyozien",
                                    "doesn't know exactly what",
                                    "is in those packages."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz06"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "Huh... Does that mean",
                                    "you can't work with us if",
                                    "you don't know exactly what",
                                    "you're doing?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kazien",
                                args![" Come on, I told", "you before--absolute secrecy.", "It goes both ways, you know."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "Look, for your own good,",
                                    "quit asking. Knowing what",
                                    "you're delivering wouldn't",
                                    "change a thing. Trust me."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "Anyway, this next job is",
                                    "more of the same. Meet",
                                    "Lyozien in the international",
                                    "flight Airship and protect",
                                    "another package destined",
                                    "for the Rune-Midgarts Kingdom."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.var("lhz_rekenber").set(Val::from(12))?;
                            ctx.call(Function::GetExperience, vec![Val::from(400000), Val::from(0)])?;
                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Kazien",
                                args![
                                    "Alright, I'll see you",
                                    "later. The important",
                                    "thing is that you do the",
                                    "best job that you can.",
                                    "And don't give Lyozien",
                                    "any trouble: that's my job!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("lhz_rekenber").get()?.number()? > 7 {
                                ctx.call(Function::Cutin, vec![Val::from("lhz_kaz08"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Kazien",
                                    args![
                                        "Lyozien is waiting for",
                                        "you on the international",
                                        "flight Airship, so go and",
                                        "meet him there as soon",
                                        "as you can. Alright then,",
                                        "I'll see you later."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("lhz_rekenber").get()? == 7 {
                                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz05"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Kazien",
                                        args![
                                            "Hey, you're back~",
                                            "Great, I guess that means",
                                            "that you've decided to work",
                                            "for us! Alright, let me tell you about your first real job. As",
                                            "always: ^FF0000keep it on the down-low^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Kazien",
                                        args![
                                            "Go to the Airship for the",
                                            "international flights, not",
                                            "the domestic ones, and meet",
                                            "a man named ^FF0000Lyozien^000000 inside.",
                                            "He's our courier that'll provide you with further instructions."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz06"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Kazien",
                                        args![
                                            "Oh. You can talk to Lyozien",
                                            "about the job, but definitely",
                                            "not to anybody else. Anyway,",
                                            "when you're done with what",
                                            "he asks you to do, come back",
                                            "to me for another job, okay?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.var("lhz_rekenber").set(Val::from(8))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(12010), Val::from(12011)])?;
                                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz01"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Kazien",
                                        args![
                                            "Don't forget...",
                                            "Talk to ^FF0000Lyozien^000000, our",
                                            "courier, on the Airship",
                                            "for the international flights."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("lhz_rekenber").get()? == 6 {
                                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(15)])? == 7 {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Oh, hey, it's you again.",
                                                    "Wait. No. You only remind",
                                                    "me of someone I've met. Um,",
                                                    "have we met before? I don't",
                                                    "remember at all. Oooh, I hate",
                                                    "being this busy, I can't focus!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.var("lhz_rekenber").set(Val::from(0))?;
                                            ctx.call(Function::EraseQuest, vec![Val::from(12009)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Arrgh, this is not good.",
                                                    "We are this busy but we don't have enough people,",
                                                    "yet it is not that extrememly bad",
                                                    "to a point that we need to hire more people."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz01"), Val::from(2)])?;
                                            ctx.lines_as("Kazien", args!["Will you step back? You are hindering my vision."])?;
                                            ctx.close_window()?;
                                        } else {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Oh, hey, it's you again.",
                                                    "Listen, you didn't come here",
                                                    "looking for a part time job,",
                                                    "did you? I already told you",
                                                    "that I can't bring myself",
                                                    "to trust you, you know?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Look, you're not a bad guy,",
                                                    "so I'm sure you'd be perfect",
                                                    "for some other employer.",
                                                    "Don't feel bad... Um, what",
                                                    "was your name again? Wait,",
                                                    "did you even give it to me...?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                        }
                                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("lhz_rekenber").get()? == 5 {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args!["So, were you able to", "contact Garins? Or did", "you encounter any problems?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Well, I came back to",
                                                    "let you know that I haven't",
                                                    "been able to find a way inside",
                                                    "the Einbroch Laboratory. There",
                                                    "was the guard, but I couldn't",
                                                    "really tell him anything."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Whaaaat~?",
                                                    "I'm sure he would",
                                                    "have let you in if you",
                                                    "told him that you had",
                                                    "to talk to Garins, right?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "But...",
                                                    "I thought you said",
                                                    "I'm not supposed to",
                                                    "tell anyone the details",
                                                    "of my assignment?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz06"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Heh... Yeah.",
                                                    "Yeah, that's right.",
                                                    "Heh heh heh! Hahahaha!",
                                                    "Great! I'm happy to say,",
                                                    "buddy, you passed the test!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "H-huh? But I never",
                                                    "even got to see Garins...",
                                                    "I didn't finish the task",
                                                    "that you assigned to me."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz09"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Garins is just some",
                                                    "cool name I made up.",
                                                    "He doesn't really exist.",
                                                    "I just wanted to test your",
                                                    "trustworthiness, is all.",
                                                    "Now do you understand?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "I... I guess.",
                                                    "Still, you just tricked",
                                                    "me! How am I supposed",
                                                    "to trust you now?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.var("lhz_rekenber").set(Val::from(7))?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz06"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Oh... You...",
                                                    "You got a point, there.",
                                                    "Huh, now isn't that ironic? I'm sorry, pal, let me apologize.",
                                                    "Take some time, consider working for me, and then come back, okay?"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else if ctx.var("lhz_rekenber").get()? == 4 {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args!["So, were you able to", "contact Garins? Or did", "you encounter any problems?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Well, I wasn't able to",
                                                    "find him. In fact, I don't",
                                                    "think that Garins even works",
                                                    "at the Einbroch Laboratory."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz08"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args!["Whaaaat~?", "That can't be true.", "Well, how'd you", "find that out?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["I happened to ask the", "Laboratory Guard, but", "he wouldn't even let me in."],
                                            )?;
                                            ctx.next()?;
                                            ctx.var("lhz_rekenber").set(Val::from(6))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(12008), Val::from(12009)])?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args!["Uh oh...", "I thought so.", "I'm sorry, pal, but", "you failed the test."],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["H-huh? What?", "What do you mean?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "You told that to the guard,",
                                                    "but you weren't supposed",
                                                    "to let anyone know any detail",
                                                    "about your assignment. Yeah...",
                                                    "Garins is just a name I made",
                                                    "up. He doesn't really exist."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "I know you meant well,",
                                                    "you know, doing whatever",
                                                    "you can to finish whatever",
                                                    "goal you have, but you can't",
                                                    "forget the details. Anyway,",
                                                    "sorry, but we can't use you..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else if ctx.var("lhz_rekenber").get()? == 3 {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz01"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Remember, visit ^FF0000Garins^000000",
                                                    "in the ^FF0000Einbroch Laboratory^000000",
                                                    "and ^FF0000confirm that he received",
                                                    "his order^000000. We pride ourselves",
                                                    "in our clients' confidentiality,^FFFFFF  ^000000 so keep it secret, got it?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "I'll just be waiting",
                                                    "around over here, so",
                                                    "once you're done with",
                                                    "that, come back to me."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else if ctx.var("lhz_rekenber").get()? == 2 {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Okay, I got it! Your first",
                                                    "assignment for us is pretty",
                                                    "simple, but think of it as",
                                                    "something of a trial run.",
                                                    "You know, for us to see",
                                                    "how reliable you are."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "All you gotta do is head",
                                                    "to Einbroch, find the Lab",
                                                    "there, and find a researcher",
                                                    "named Garins. You need to",
                                                    "confirm whether he safely",
                                                    "received his order from us."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Simple stuff, yeah?",
                                                    "Now, you can't let anyone",
                                                    "know about your assignment.",
                                                    "Otherwise, we can't trust you",
                                                    "for more important stuff. And",
                                                    "I really wanna trust you."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz05"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Remember, visit ^FF0000Garins^000000",
                                                    "in the ^FF0000Einbroch Laboratory^000000",
                                                    "and ^FF0000confirm that he received",
                                                    "his order^000000. We pride ourselves",
                                                    "in our clients' confidentiality, so keep it secret, got it?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.var("lhz_rekenber").set(Val::from(3))?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(12007), Val::from(12008)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "I'll just be waiting",
                                                    "around over here, so",
                                                    "once you're done with",
                                                    "that, come back to me."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                            return Err(Stop::End);
                                        } else if ctx.var("lhz_rekenber").get()? == 1 {
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz01"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "We're so busy, we barely",
                                                    "have enough people to cover",
                                                    "our workload right now. Still,",
                                                    "it's not so bad that we gotta",
                                                    "invest in some new hires."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Oh hey, sorry buddy,",
                                                    "but you mind stepping",
                                                    "back? It's just that you're",
                                                    "blocking my view is all."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Wait, did you just say",
                                                    "that you need help?",
                                                    "I'd like to help solve",
                                                    "your problem, er, for",
                                                    "a nominal fee or some",
                                                    "kind of reward. You know..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.call(Function::Cutin, vec![Val::from("lhz_kaz10"), Val::from(2)])?;
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Hey, alright~",
                                                    "I could use an extra",
                                                    "hand if you're willing",
                                                    "to work part time. Plus,",
                                                    "you're a straight shooter.",
                                                    "I like that. Let's see now..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            if ctx.var("BaseLevel").get()?.number()? < 70 {
                                                ctx.lines_as(
                                                    "Kazien",
                                                    args![
                                                        "Awww, I'm sorry, pal.",
                                                        "I know you mean well, but",
                                                        "to put it bluntly, you're not",
                                                        "not strong enough for this",
                                                        "kinda work. Hey, but if you put on",
                                                        "some muscle, ask me again, okay?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Kazien",
                                                args![
                                                    "Yeah, okay. You look like",
                                                    "you can handle this. But",
                                                    "are you the type of person",
                                                    "I can trust? Hey, you can",
                                                    "keep confidential information",
                                                    "without telling anyone, right?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            match runtime::select_values(
                                                ctx,
                                                &[Val::from("Yes, of course."), Val::from("Nope, I love giving away secrets.")],
                                            )? {
                                                1 => {
                                                    ctx.var("lhz_rekenber").set(Val::from(2))?;
                                                    ctx.lines_as(
                                                        "Kazien",
                                                        args![
                                                            "Great, great.",
                                                            "I guess we can just",
                                                            "get down to business,",
                                                            "then. Let me think. First,",
                                                            "I should give you something",
                                                            "easy to do to test you out..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
                                                    ctx.lines_as(
                                                        "Kazien",
                                                        args![
                                                            "Awww, man.",
                                                            "I can't hire you",
                                                            "if you're gonna blab",
                                                            "your mouth. Sorry buddy,",
                                                            "but I can't afford to take",
                                                            "any risks. You understand..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
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
    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz01"), Val::from(2)])?;
    ctx.lines_as(
        "Kazien",
        args![
            "We're so busy, we barely",
            "have enough people to cover",
            "our workload right now. Still,",
            "it's not so bad that we gotta",
            "invest in some new hires."
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Cutin, vec![Val::from("lhz_kaz04"), Val::from(2)])?;
    ctx.lines_as(
        "Kazien",
        args![
            "Oh hey, sorry buddy,",
            "but you mind stepping",
            "back? It's just that you're",
            "blocking my view is all."
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn young_man_reken(ctx: &Ctx) -> Script {
    young_man_reken_body(ctx, Vec::new()).map(|_| ())
}

fn old_man_reken_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Old Man",
        args![
            "Lately, Kazien seems",
            "to be having a hard time",
            "managing his business.",
            "Always complaining that",
            "he lacks the manpower..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Old Man",
        args![
            "I don't know what kind of",
            "business he's conducting,",
            "but why don't you help him",
            "out? I don't believe that any",
            "particularly special skills are",
            "required for some positions."
        ],
    )?;
    ctx.next()?;
    if !(ctx.var("lhz_rekenber").get()?.is_true()) {
        ctx.var("lhz_rekenber").set(Val::from(1))?;
        ctx.call(Function::SetQuest, vec![Val::from(12007)])?;
    }
    ctx.lines_as(
        "Old Man",
        args![
            "Well, if you're interested,",
            "you can find Kazien inside",
            "the corporation building.",
            "Young people like him should",
            "never be too proud to ask for help. He still needs to learn..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn old_man_reken(ctx: &Ctx) -> Script {
    old_man_reken_body(ctx, Vec::new()).map(|_| ())
}

fn laboratory_guard_reken_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_rekenber").get()? == 3 {
        ctx.lines_as(
            "Laboratory Guard",
            args![
                "Hold it! This is",
                "a restricted area to",
                "the public! Unless you",
                "have some special business,",
                "you'll have to leave right now."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Actually, I do have business here."), Val::from("Whoa, I'm leaving!")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Laboratory Guard",
                    args![
                        "Yes? State the nature",
                        "of your business here,",
                        "as well as any person that",
                        "you wish to contact inside",
                        "of this laboratory facility."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("I need to see Mr. Garins..."), Val::from("I'll... come back later.")],
                )? {
                    1 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I need to see Mr. Garins",
                                "and confirm that he received",
                                "a package that was sent to him."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Laboratory Guard",
                            args![
                                "Garins, eh?",
                                "Alright, let me check",
                                "the employee list. Hmm...",
                                "Garins... Garins... Eh?",
                                "He's not here. Maybe you",
                                "came to the wrong place?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Laboratory Guard",
                            args![
                                "Sorry, but it looks like",
                                "you've wasted your time.",
                                "We don't have a Garins",
                                "working here. Anyway,",
                                "I still can't allow you to",
                                "enter the laboratory."
                            ],
                        )?;
                        ctx.var("lhz_rekenber").set(Val::from(4))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I'll... come back later.",
                                "(^333333I better speak to Kazien",
                                "and let him know I'm having",
                                "trouble getting past this",
                                "guard. Otherwise, I might",
                                "never finish this job!^000000)"
                            ],
                        )?;
                        ctx.var("lhz_rekenber").set(Val::from(5))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(12008), Val::from(12010)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Whoa, I'm leaving!",
                        "(^333333I better speak to Kazien",
                        "and let him know I'm having",
                        "trouble getting past this",
                        "guard. Otherwise, I might",
                        "never finish this job!^000000)"
                    ],
                )?;
                ctx.var("lhz_rekenber").set(Val::from(5))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(12008), Val::from(12010)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines_as(
        "Laboratory Guard",
        args![
            "This area is restricted",
            "to the public. Unless you",
            "have some kind of special",
            "authorization, I'm going",
            "to have to ask you to leave."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn laboratory_guard_reken(ctx: &Ctx) -> Script {
    laboratory_guard_reken_body(ctx, Vec::new()).map(|_| ())
}

fn man_lyozien_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_rekenber").get()?.number()? > 21 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo11"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Hey, I hear from my",
                "brother that you can't work",
                "with us anymore because",
                "of some scheduling conflict.",
                "I'm sorry to hear that: it was",
                "really good working with you..."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        if ctx.var("lhz_rekenber").get()? == 21 {
            ctx.call(Function::Cutin, vec![Val::from("lhz_ryo03"), Val::from(2)])?;
            ctx.lines_as(
                "Lyozien",
                args![
                    "Oh good, you're back.",
                    "Mr. Ahman just left and",
                    "picked up his goods. We're",
                    "done here, so all you have to",
                    "do now is report to my",
                    "brother in Lighthalzen."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lyozien",
                args![
                    "You sure you're alright?",
                    "You seem kind of upset.",
                    "Do you need to take a",
                    "break or something?"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else {
            if ctx.var("lhz_rekenber").get()? == 20 {
                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo09"), Val::from(2)])?;
                ctx.lines_as(
                    "Lyozien",
                    args![
                        "Heya, keep up the",
                        "good work. Once you",
                        "talk to Mr. Ahman inside",
                        "Izlude Airport, we'll be",
                        "done with this delivery."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            } else {
                if ctx.var("lhz_rekenber").get()? == 19 {
                    ctx.call(Function::Cutin, vec![Val::from("lhz_ryo06"), Val::from(2)])?;
                    ctx.lines_as(
                        "Lyozien",
                        args![
                            "Whoa, you were great!",
                            "There were more of them",
                            "this time, but you easily",
                            "dispatched them. Great job!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_ryo01"), Val::from(2)])?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Thanks, but...",
                            "Now I'm really worried",
                            "about what could be in",
                            "those packages. Are you",
                            "sure you don't know, Lyozien?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_ryo02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Lyozien",
                        args![
                            "You don't know when to",
                            "stop, do you? Nah, I don't",
                            "know at all. Besides, so long",
                            "as my brother says it's a bad",
                            "idea, then I don't wanna find",
                            "out for myself. Oh, hey..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("lhz_rekenber").set(Val::from(20))?;
                    ctx.call(Function::GetItem, vec![Val::from(504), Val::from(1)])?;
                    ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
                    ctx.lines_as(
                        "Lyozien",
                        args![
                            "Here's a little",
                            "something to refresh",
                            "yourself after that fight.",
                            "Keep up the good work, okay?",
                            "Then, we'll be done once you",
                            "contact Mr. Ahman in Izlude."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("lhz_rekenber").get()? == 18 {
                        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo06"), Val::from(2)])?;
                        ctx.lines_as(
                            "Lyozien",
                            args![
                                "Okay, just like before, we",
                                "gotta get these packages to",
                                "Mr. Ahmam. When we arrive",
                                "in Izlude, find Mr. Ahman in",
                                "the Airport and tell him that",
                                "his packages have arrived."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo08"), Val::from(2)])?;
                        ctx.lines_as(
                            "Lyozien",
                            args![
                                "W-wait...",
                                "Did you hear that?",
                                "I heard--I think it's them.",
                                "Those thugs are back! Don't",
                                "let them damage the packages!"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.var("lhz_rekenber").set(Val::from(19))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#bully2::OnEnter")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnStop")])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("lhz_rekenber").get()? == 17 {
                            ctx.call(Function::Cutin, vec![Val::from("lhz_ryo14"), Val::from(2)])?;
                            ctx.lines_as(
                                "Lyozien",
                                args![
                                    "I heard that you upset",
                                    "Kyozien a little bit with",
                                    "your questions. I mean,",
                                    "I totally understand, but",
                                    "you gotta remember that",
                                    "we've got obligations."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
                            ctx.lines_as(
                                "Lyozien",
                                args![
                                    "I know that these packages",
                                    "might be putting us in danger,",
                                    "but I trust my brother. If he says those thugs are bad guys, then",
                                    "they're definitely bad guys."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("lhz_ryo13"), Val::from(2)])?;
                            ctx.lines_as(
                                "Lyozien",
                                args![
                                    "Yeah, ever since we were",
                                    "kids, Kyozien has always",
                                    "been right. Even though",
                                    "I'd like to know what's in the",
                                    "boxes, I don't ever wanna",
                                    "disappoint him, you know?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.var("lhz_rekenber").set(Val::from(18))?;
                            ctx.call(Function::Cutin, vec![Val::from("lhz_ryo10"), Val::from(2)])?;
                            ctx.lines_as(
                                "Lyozien",
                                args![
                                    "Anyway, that's",
                                    "enough chit-chat",
                                    "for now. Let's get",
                                    "back to work, shall we?"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("lhz_rekenber").get()? == 16 {
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "I'm lucky that you're",
                                        "around to keep those",
                                        "thugs off our backs, eh?",
                                        "Hey, when you're ready for",
                                        "another job, just talk to",
                                        "my brother Kazien, okay?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else if ctx.var("lhz_rekenber").get()? == 15 {
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo05"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "So you spoke to",
                                        "Mr. Ahman already?",
                                        "Good, good. Now we can",
                                        "go back to the Schwarzwald",
                                        "Republic for our next job."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "Just talk to my brother",
                                        "Kazien and he should give",
                                        "you any details you need",
                                        "to know. Man, it's good",
                                        "that you're working for us.",
                                        "Those thugs frighten me..."
                                    ],
                                )?;
                                ctx.var("lhz_rekenber").set(Val::from(16))?;
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo01"), Val::from(2)])?;
                                ctx.lines_as("Lyozien", args!["Now I feeel much more secure."])?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else if ctx.var("lhz_rekenber").get()? == 14 {
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo13"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "Now that those thugs are",
                                        "gone, let's concentrate on",
                                        "our task. Like before, just",
                                        "get off at Izlude and then tell",
                                        "Mr. Ahman that his packages",
                                        "have arrived. See you later~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else if ctx.var("lhz_rekenber").get()? == 13 {
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo13"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "Oh, thank you!",
                                        "You saved my life!",
                                        "As you can tell, I'm",
                                        "not much of a fighter...",
                                        "I just ran and hid when",
                                        "those thugs appeared."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo04"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "That's one reason why",
                                        "my brother has been hiring",
                                        "you adventurers-- we need",
                                        "packages from those hoodlums.",
                                        "They're always after us..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo07"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "Every time I see them,",
                                        "they're yelling things like",
                                        "we're the servants of evil,",
                                        "or that the packages must",
                                        "be destroyed. Boy, I sure",
                                        "am glad that you're here!"
                                    ],
                                )?;
                                ctx.var("lhz_rekenber").set(Val::from(14))?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            } else if ctx.var("lhz_rekenber").get()? == 12 {
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo11"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "Hey, we already have another",
                                        "package to deliver all the way",
                                        "to the Rune-Midgarts Kingdom",
                                        "again. Can you believe it? We",
                                        "seem to be doing a lot of",
                                        "business around there lately."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "It's so far away from",
                                        "home, but a job's a job.",
                                        "We're obliged to do what",
                                        "we're been assigned to do",
                                        "until we qui--whoa. You",
                                        "hear that? Wh-what was...?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("lhz_ryo03"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Lyozien",
                                    args![
                                        "Awwww, nuts!",
                                        "It's those thugs!",
                                        "I'll explain later, but",
                                        "for now, please protect",
                                        "the packages and make",
                                        "sure they don't get them!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                ctx.var("lhz_rekenber").set(Val::from(13))?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnStop")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("#bully1::OnEnter")])?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    }
    if ctx.var("lhz_rekenber").get()? == 11 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Hey, would you go see",
                "my brother Kazien to see",
                "if he's got another job for",
                "us to do? I'll just be over",
                "here waiting when you need",
                "to find me. See you later~"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("lhz_rekenber").get()? == 10 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo14"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Good work, Mr. Ahman just",
                "arrived and picked up his",
                "packages. It looks like we're",
                "done for today. When you're",
                "ready for another job, just",
                "ask to my brother Kazien, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Lyozien, do you know", "what kinds of things", "we're delivering?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo10"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "No clue. I've been",
                "a little curious myself,",
                "but my brother warned me",
                "not to ask. Besides, I don't",
                "think it makes a difference",
                "to what we gotta do, right?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo13"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Anyway, it oughta be",
                "fine. I mean, our clients",
                "are entitled to their privacy",
                "anyway. You've been in that",
                "sort of situation, right? You",
                "know, embarassing orders..."
            ],
        )?;
        ctx.next()?;
        ctx.var("lhz_rekenber").set(Val::from(11))?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo01"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "No? Eh, just meet up with",
                "my brother to see if he's",
                "got another job for us, okay?",
                "If you wanna find me again,",
                "I'll be waiting right here."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("lhz_rekenber").get()? == 9 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo05"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Alright, when this Airship",
                "arrives in Izlude, get off and",
                "enter the Airport to meet with",
                "a man named Mr. Ahman.",
                "Let him know his order has",
                "already arrived, okay?"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else if ctx.var("lhz_rekenber").get()? == 8 {
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo02"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Um, would you mind",
                "treading lightly around",
                "this area, and kind of go",
                "around the piles? Yeah,",
                "these are all pretty fragile.",
                "Thanks, I appreciate it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but", "are you Lyozien?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo07"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Hey, are you the one that",
                "my brother Kazien sent?",
                "Nice, I've been waiting",
                "for you. As you can see,",
                "I'm having trouble handling",
                "all of these packages here."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo14"), Val::from(2)])?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Wait, Kazien is",
                "your brother? That's",
                "weird, you figure he",
                "would mention that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Oh, yeah, he's been like",
                "that ever since we lived",
                "in Lighthalzen's slums.",
                "Luckily, he joined the",
                "corporation and helped",
                "us improve our lots in life..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo13"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "That's why I appreciate the",
                "fact that he lets me work for",
                "him. I can't let him down.",
                "Anyway, back to business: we",
                "gotta deliver these goods to",
                "the Rune-Midgarts Kingdom."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "I have to ensure that these",
                "packages aren't damaged or",
                "stolen by thieves. Your job is",
                "to go to Izlude, find Mr. Ahman",
                "at the Airport, and tell him that his orders have safely arrived."
            ],
        )?;
        ctx.next()?;
        ctx.var("lhz_rekenber").set(Val::from(9))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12011), Val::from(12012)])?;
        ctx.call(Function::Cutin, vec![Val::from("lhz_ryo11"), Val::from(2)])?;
        ctx.lines_as(
            "Lyozien",
            args![
                "Once you tell Mr. Ahman",
                "that message, he'll take",
                "care of picking up his own",
                "packages. But yeah, I need",
                "to stay behind to guard these",
                "products in the meantime."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("lhz_ryo12"), Val::from(2)])?;
    ctx.lines_as(
        "Lyozien",
        args![
            "Um, would you mind",
            "treading lightly around",
            "this area, and kind of go",
            "around the piles? Yeah,",
            "these are all pretty fragile.",
            "Thanks, I appreciate it."
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn man_lyozien(ctx: &Ctx) -> Script {
    man_lyozien_body(ctx, Vec::new()).map(|_| ())
}

fn man_lyozien_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Man#Lyozien")])?;
    return Err(Stop::End);
}

pub fn man_lyozien_onenter(ctx: &Ctx) -> Script {
    man_lyozien_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn man_lyozien_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Man#Lyozien")])?;
    return Err(Stop::End);
}

pub fn man_lyozien_onstop(ctx: &Ctx) -> Script {
    man_lyozien_onstop_body(ctx, Vec::new()).map(|_| ())
}
