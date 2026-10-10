use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn vincent_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? < 60 {
        ctx.lines_as(
            "Vincent",
            args![
                "You're inside Sir Zhed's",
                "looking for new employees,",
                "I don't think you're suited",
                "for this kind of domestic",
                "work, brave adventurer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lost_boy").get()?.number()? < 1 {
        ctx.lines_as(
            "Vincent",
            args![
                "I am Vincert, steward of",
                "this mansion and faithful",
                "servant to its master, Sir",
                "Zhed, the most powerful",
                "man in all of Arunafeltz."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Vincent",
            args![
                "My master is currently out",
                "to attend the High Priest",
                "assembly, and has been gone",
                "for a few days. I would like to",
                "ask you for your help with",
                "a problem on his behalf."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'm too busy.:Sure, why not?")])? {
            1 => {
                ctx.lines_as(
                    "Vincent",
                    args![
                        "I understand.",
                        "I'm sorry that you're",
                        "too busy at the moment...",
                        "If you should be available",
                        "later, then I'd like to ask for",
                        "your assistance once again."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Vincent",
                    args![
                        "Thank you. You see,",
                        "the pope awarded Sir Zhed",
                        "for his great contributions to",
                        "Arunafeltz with a precious gem.",
                        "However, this jewel is missing",
                        "and I need your help to find it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vincent",
                    args![
                        "I don't have any proof, but",
                        "I suspect it was stolen by",
                        "Phobe, a servant that",
                        "disappeared about the same",
                        "time the gem disappeared."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vincent",
                    args![
                        "Please retrieve this gem",
                        "and find who stole it before",
                        "Sir Zhed returns and finds",
                        "out what happened. If you",
                        "can keep this secret, I'd",
                        "very much appreciate it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vincent",
                    args![
                        "I believe that you'd want",
                        "to interrogate Phobe, but",
                        "he has run away somewhere.",
                        "You might want to question the",
                        "other servants of his whereabouts. Thanks again for your help."
                    ],
                )?;
                ctx.var("lost_boy").set(Val::from(1))?;
                ctx.call(Function::SetQuest, vec![Val::from(8089)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if (ctx.var("lost_boy").get()?.number()? >= 1 && ctx.var("lost_boy").get()?.number()? < 4) {
            ctx.lines_as(
                "Vincent",
                args![
                    "I suggest looking around",
                    "the mansion and asking the",
                    "servants for any clues about",
                    "Phobe's current location."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("lost_boy").get()?.number()? >= 4 && ctx.var("lost_boy").get()?.number()? < 7) {
                ctx.lines_as(
                    "Vincent",
                    args![
                        "We're running out",
                        "of time... Please find",
                        "the gem and Phobe",
                        "as soon as you can."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("lost_boy").get()? == 7 {
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Ah...!",
                            "H-hello! How are",
                            "you still, er... That",
                            "look on your face? Did",
                            "you happen to find Logan?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "No! I actually got",
                            "stabbed by Mogan, and",
                            "then we had a talk. What's",
                            "the big idea? I come to help",
                            "you, and you try to have me",
                            "killed! I want an explanation!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "I... Yes, you deserve",
                            "the truth after wh-what",
                            "I tried to do to you. First",
                            "of all, Phobe is my son, but",
                            "please don't tell anybody!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What?", "Why, what's", "the big deal?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "There's something of",
                            "a class system here in",
                            "Arunafeltz. No one talks",
                            "about it, but those that",
                            "immigrated here and built this",
                            "city are the dominant class."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "The native people are",
                            "second class citizens that",
                            "are looked down upon by the",
                            "descendents of the settlers",
                            "that developed this city. It",
                            "is a sad, undeniable truth."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "The native people typically",
                            "suffer from lower class status,",
                            "and usually do the hard, blue",
                            "collar work in the city. Jenny,",
                            "Phobe's mother, is one of them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "It's taboo for me to love her,",
                            "especially since Sir Zhed has",
                            "taken me under his wing and",
                            "been like a father to me. If",
                            "I married her, it'd greatly",
                            "damage his reputation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "I tried to control my",
                            "feelings but... Well,",
                            "Phobe was born. And there's",
                            "no going back now. We did",
                            "get secretly married though,",
                            "and I don't regret that."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Phobe must resent me...",
                            "I've had to treat him and",
                            "his mother like slaves",
                            "in front of other people.",
                            "I know it's horrible... To be",
                            "so cold to those you love."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "When he ran off with the",
                            "jewel, I was actually more",
                            "worried about Phobe than",
                            "my master's treasure. So...",
                            "I did what I could to try to",
                            "get him back: I hired you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "However, once you found",
                            "Phobe, I planned to have",
                            "you killed so that we could",
                            "blame you for the theft. I'm",
                            "sorry, I know it's wrong, but",
                            "I was so worried about my boy!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "I hope you forgive me...",
                            "I'll do what I should've",
                            "done from the beginning...",
                            "I'll take full responsibility",
                            "for the gem's theft, and Sir",
                            "Zhed can do to me what he will."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "The gem is very special...",
                            "Our pope commanded my",
                            "master to keep the jewel",
                            "safely, as it has the power",
                            "to save Arunafeltz and Rachel",
                            "when the time comes."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "The gem is really that",
                            "important, huh? Well, you",
                            "almost had me killed, but",
                            "since I'm still alive, I guess",
                            "I can overlook it, you know?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "You won't get in trouble",
                            "if I can find the jewel and",
                            "Phobe before Sir Zhed",
                            "returns, so I'll try to find",
                            "them for you as soon as I can."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "You're willing to",
                            "do that for me? ^333333*Sob*^000000",
                            "Even after what I've put",
                            "you through? ^333333*Sniff*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Eh, I went through all",
                            "this trouble already, so",
                            "I might as well finish the",
                            "job. Besides, Phobe is",
                            "just a kid, so he's probably",
                            "hiding somewhere in town."
                        ],
                    )?;
                    ctx.var("lost_boy").set(Val::from(8))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8094), Val::from(8095)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lost_boy").get()? == 8 {
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "The gem is very special...",
                            "Our pope commanded my",
                            "master to keep the jewel",
                            "safely, as it has the power",
                            "to save Arunafeltz and Rachel",
                            "when the time comes."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "The gem is really that",
                            "important, huh? Well, you",
                            "almost had me killed, but",
                            "since I'm still alive, I guess",
                            "I can overlook it, you know?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "You won't get in trouble",
                            "if I can find the jewel and",
                            "Phobe before Sir Zhed",
                            "returns, so I'll try to find",
                            "them for you as soon as I can."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "You're willing to",
                            "do that for me? ^333333*Sob*^000000",
                            "Even after what I've put",
                            "you through? ^333333*Sniff*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Eh, I went through all",
                            "this trouble already, so",
                            "I might as well finish the",
                            "job. Besides, Phobe is",
                            "just a kid, so he's probably",
                            "hiding somewhere in town..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lost_boy").get()? == 9 {
                    ctx.lines_as(
                        "Vincent",
                        args!["You came back!", "Were you able to", "find Phobe?! H-how", "is he? Is he alright?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Huh? Oh yeah, he's",
                            "just fine. I found Phobe",
                            "loitering around Freya's",
                            "Spring. Er, he's not willing",
                            "to come home yet, but he did",
                            "give me the gem he stole."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["So... I earned my", "reward now, right?", "And no one has to die?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Yes, yes, thank you so",
                            "much! I'll never forget",
                            "what you've done for me.",
                            "Oh, my boy is alright!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "H-hey! Um, take",
                            "this jewel! You need",
                            "to return it to wherever",
                            "it's supposed to go, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Oh, yes, I'd better do",
                            "that. While I return this",
                            "jewel, would you please",
                            "tell my wife Jenny that our",
                            "boy is okay? She's been",
                            "very worried about him, so..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You want me to tell", "Jenny about Phobe?", "Sure, sure, I'll do that."],
                    )?;
                    ctx.var("lost_boy").set(Val::from(10))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8096), Val::from(8097)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lost_boy").get()? == 10 {
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Oh! While I return this",
                            "jewel, would you please",
                            "tell my wife Jenny that our",
                            "boy is okay? She's been",
                            "very worried about him, so..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You want me to tell", "Jenny about Phobe?", "Sure, sure, I'll do that."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lost_boy").get()? == 11 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I told Jenny that",
                            "about Phobe, and that",
                            "the gem was returned...",
                            "She seemed pretty relieved..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Thank you so much,",
                            "adventurer. She's such",
                            "a kind, loving woman, and",
                            "I hate to see her go through",
                            "all that torment. I don't deserve",
                            "such a beautiful woman..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "It's very fortunate that",
                            "you've come to save us...",
                            "Please drop by the next",
                            "time that you're in Rachel,",
                            "and I'll try to help you if I can."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Ah, actually, I have",
                            "one last favor to ask",
                            "of you. Would you please",
                            "bring this note and package",
                            "to my master? He should still",
                            "be in the temple right now."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "If you ask for High",
                            "Priest Zhed and mention",
                            "that I sent you, then he",
                            "will meet with you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Now is the time for",
                            "me to give you your",
                            "reward. How about that?",
                            "I don't know what's inside,",
                            "but I know these items are",
                            "quite valuable nowadays..."
                        ],
                    )?;
                    ctx.var("lost_boy").set(Val::from(12))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(8098), Val::from(8099)])?;
                    ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Ah, and this note is a",
                            "letter of recommendation",
                            "that I have written for you.",
                            "Please deliver it to Sir Zhed",
                            "as soon as possible. Good bye,",
                            "and thank you for everything."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou received a letter",
                        "of recommendation",
                        "from Vincent.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Vincent",
                        args![
                            "Thank you for everything",
                            "that you've done for me",
                            "and my family. If you ever",
                            "need help, please don't",
                            "hesitate to ask me, alright?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn vincent_ra_in01(ctx: &Ctx) -> Script {
    vincent_ra_in01_body(ctx, Vec::new()).map(|_| ())
}

fn logan_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lost_boy").get()?.number()? < 1 || ctx.var("lost_boy").get()?.number()? >= 3) {
        ctx.lines_as(
            "Logan",
            args![
                "I'm just Logan, one",
                "of the many servants",
                "working here in Sir Zhed's",
                "glorious mansion. I've got",
                "a lot of work to do, so don't",
                "don't distract me, please."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 1 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Excuse me, but do",
                "you know man named",
                "Phobe by any chance?",
                "I heard that he used",
                "to work around here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Logan", args!["Er, may I ask", "you are? Why are", "you looking for him?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, I've just been",
                "hired to look for him,",
                "and I was told that asking",
                "the servants around here",
                "was a good starting point."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Logan",
            args![
                "Oh, okay. Yeah, Phobe's",
                "been missing ever since",
                "he left to buy stuff from",
                "the market a few days ago.",
                "I hope the kid is alright."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Er, kid? Just", "how old is Phobe?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Logan",
            args![
                "He just turned fifteen.",
                "That's all I know about",
                "him. Truth be told, I don't",
                "know much since I just",
                "started working here. Why don't",
                "you ask the senior employees?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Logan",
            args![
                "Let's see...",
                "You could ask Mr. Manson",
                "inside the mansion. He's",
                "been working here for a while."
            ],
        )?;
        ctx.var("lost_boy").set(Val::from(2))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8089), Val::from(8090)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 2 {
        ctx.lines_as(
            "Logan",
            args![
                "If you want to learn",
                "more about Phobe, you'd",
                "better ask one of the senior",
                "employees since I just started",
                "working here. Mr. Manson in",
                "the mansion is a good bet."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn logan_ra_in01(ctx: &Ctx) -> Script {
    logan_ra_in01_body(ctx, Vec::new()).map(|_| ())
}

fn manson_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lost_boy").get()?.number()? < 1 {
        ctx.lines_as(
            "Manson",
            args![
                "I am Manson, one of the",
                "many servants working here",
                "in Sir Zhed's mansion. Er,",
                "would you be more careful",
                "walking around here? I hate",
                "cleaning up after visitors."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 1 {
        ctx.lines_as(
            "Manson",
            args![
                "You know, considering",
                "that natives like me are",
                "looked down upon and",
                "are kinda of lower class,",
                "I'm really lucky to work for",
                "Sir Zhed here in the mansion."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 2 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Excuse me, but do you",
                "know a man named Phobe?",
                "He's gone missing, so I've",
                "been hired to look for him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "Ah, so you're the one",
                "that Vincent hired, huh?",
                "That's good, that's good.",
                "I'm pretty sure Phobe is",
                "hiding somewhere and",
                "just goofing around."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "I'm sure he can take care",
                "of himself, so we won't have",
                "to worry too much. Let's see,",
                "he asked me if I had any",
                "errands to for him to do,",
                "so I sent him to the market."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "That's the last time I saw",
                "him. I guess he's using the",
                "money I gave him to feed himself",
                "while he's run away. He's young,",
                "so I can't really blame him.",
                "We've all done crazy things~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "Ah, our gardener Jenny",
                "was the one that introduced",
                "me to him. She's worked for",
                "Sir Zhed for a long time, and",
                "she loves kids. I guess she's",
                "got a soft spot for Phobe."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "No one knows who his",
                "parents are, so I guess",
                "he's an orphan. That's",
                "probably why she has him.",
                "working with us in the mansion."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "Jenny has done a lot for",
                "that boy. We skipped our",
                "usual employment process",
                "just for her, you know? And",
                "now Jenny and even Vincent",
                "are worried about him missing."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manson",
            args![
                "You know, if you want to",
                "know more about that kid,",
                "you should talk to Jenny.",
                "She's working in the garden",
                "now, and she'd appreciate",
                "your help in finding Phobe."
            ],
        )?;
        ctx.var("lost_boy").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8090), Val::from(8091)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 3 {
        ctx.lines_as(
            "Manson",
            args![
                "You know, if you want to",
                "know more about that kid,",
                "you should talk to Jenny.",
                "She's working in the garden",
                "now, and she'd appreciate",
                "your help in finding Phobe."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Manson",
            args![
                "Nuts, there's so much",
                "work to do! I never seem",
                "to get it all done until",
                "the very end of the day.",
                "Ah well, it's a living."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn manson_ra_in01(ctx: &Ctx) -> Script {
    manson_ra_in01_body(ctx, Vec::new()).map(|_| ())
}

fn jenny_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lost_boy").get()?.number()? < 3 {
        ctx.lines_as(
            "Jenny",
            args![
                "These grounds are owned by",
                "Sir Zhed and are considered",
                "private property. Please",
                "leave immediately if you",
                "haven't been invited!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("lost_boy").get()? == 3 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Excuse me, but are",
                    "you Jenny? I've been",
                    "told by Manson to speak",
                    "to you if I wanted to learn",
                    "more about Phobe. You see,",
                    "Vincent hired me to find him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jenny",
                args!["Oh! That's wonderful", "news! Y-yes, I'm Jenny.", "What did you need to know?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Er, anything you could",
                    "tell me would be fine.",
                    "Things like his favorite",
                    "hangouts might also be",
                    "helpful for me to investigate."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jenny",
                args![
                    "Phobe is such a good",
                    "boy. I don't know what",
                    "kind of trouble he's gotten",
                    "into this time, but he's",
                    "really a great kid if you",
                    "just give him a chance."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jenny",
                args![
                    "Oh... Vincent must",
                    "also be--I'm-I'm very",
                    "glad that he's hired",
                    "you. But as for places",
                    "he might be found, I'm",
                    "not sure if I know."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jenny",
                args![
                    "Have you tried the",
                    "market? That's where",
                    "he was last seen, wasn't it?",
                    "Maybe they have some idea",
                    "of where he was going?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Oh... That's a good idea.",
                    "Alright, I guess I can go",
                    "to the market and ask around."
                ],
            )?;
            ctx.var("lost_boy").set(Val::from(4))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8091), Val::from(8092)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("lost_boy").get()?.number()? >= 3 && ctx.var("lost_boy").get()?.number()? < 6) {
                ctx.lines_as(
                    "Jenny",
                    args![
                        "Please find Phobe,",
                        "and bring him back",
                        "safely as soon as you",
                        "can! I can't help but worry",
                        "about that boy, you know?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I'll do my best",
                        "First, I guess I should ask",
                        "around the market where",
                        "Phobe was last seen."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("lost_boy").get()? == 6 {
                    ctx.lines_as(
                        "Jenny",
                        args![
                            "Please find Phobe,",
                            "and bring him back",
                            "safely as soon as you",
                            "can! I can't help but worry",
                            "about that boy, you know?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("lost_boy").get()?.number()? > 6 && ctx.var("lost_boy").get()?.number()? < 11) {
                    ctx.lines_as("Jenny", args!["Oh! You're back!", "Did you find Phobe?"])?;
                    ctx.next()?;
                    if ctx.var("lost_boy").get()? == 7 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Actually... Uh...",
                                "I have something very",
                                "important to discuss",
                                "with Vincent first."
                            ],
                        )?;
                    } else if ctx.var("lost_boy").get()? == 8 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Actually... Uh...",
                                "Not just yet. But I'm",
                                "following a really good",
                                "lead! Don't worry, I'll",
                                "find him soon, I promise."
                            ],
                        )?;
                    } else if ctx.var("lost_boy").get()? == 9 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Actually... Uh...",
                                "I should report to",
                                "Vincent first and return",
                                "this jewel, but I'll deliver",
                                "some good news soon,",
                                "I promise. Se eyou later~"
                            ],
                        )?;
                    } else if ctx.var("lost_boy").get()? == 10 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Yes, he's fine, just",
                                "hanging around the south",
                                "side of town. He's safe,",
                                "but he's not willing to",
                                "come home just right now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "The jewel's been returned,",
                                "so everything should be fine",
                                "now. Also, Vincent explained",
                                "everything to me. You know,",
                                "how you, him, and Phobe",
                                "are all related, so..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jenny",
                            args![
                                "Oh! Oh, I see. He told",
                                "you our secret? Well,",
                                "I guess we can trust you.",
                                "Mostly, I'm just relieved",
                                "that my son is alright, and",
                                "that he won't get in trouble."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jenny",
                            args![
                                "Anyway, even though Phobe",
                                "is being pretty stubborn,",
                                "I'm sure he'll come back",
                                "soon. As he grows up, I think",
                                "he'll understand his father's",
                                "position a little bit better."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jenny",
                            args![
                                "I hope so. Although",
                                "it's been very hard for",
                                "all three of us to live",
                                "this way, I couldn't be",
                                "happier. Thank you for",
                                "all of your help, adventurer~"
                            ],
                        )?;
                        ctx.var("lost_boy").set(Val::from(11))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(8097), Val::from(8098)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("lost_boy").get()?.number()? > 10 && ctx.var("lost_boy").get()?.number()? < 13) {
                    ctx.lines_as(
                        "Jenny",
                        args![
                            "Thank you so much for",
                            "finding my son. If there's",
                            "anything I can ever do for",
                            "you, please let me know.",
                            "You don't know how grateful",
                            "I am to you as a mother..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    return Err(Stop::End);
}

pub fn jenny_ra_in01(ctx: &Ctx) -> Script {
    jenny_ra_in01_body(ctx, Vec::new()).map(|_| ())
}

fn idle_merchant_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lost_boy").get()?.number()? < 4 || ctx.var("lost_boy").get()?.number()? >= 6) {
        ctx.lines_as(
            "Idle Merchant",
            args![
                "Man! Business is going",
                "sooo slooow right now!",
                "Well, no point standing",
                "around this dump much",
                "longer. Maybe I should",
                "pack it up and go home."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 4 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me?", "Hello? Sir?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "Finally! A customer!",
                "I knew someone would come",
                "eventually! So whaddya want",
                "to buy? I got all sorts of",
                "handy little knickknacks~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Sorry, not interested.",
                "I just wanted to ask if",
                "you've seen a boy around",
                "here that's about fifteen",
                "years old. He supposedly",
                "ran away from home, so."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "Fifteen? My own son is",
                "that age... I'm sorry, but",
                "I haven't seen any lads that",
                "age around here in the past",
                "few days. Though, you might",
                "want to ask the other merchants."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Cool. Thanks a lot."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "Alright, then!",
                "Good luck bringing that",
                "boy home! Oh, and are you",
                "sure that you don't want",
                "to take a look around?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Er, what exactly do",
                "you sell here? I can't",
                "recognize any of these",
                "goods that you're selling."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "You know how some restaurants",
                "offer dishes with mock meat",
                "made out of vegetables and",
                "stuff like wheat gluten, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "Get this: mock vegetables",
                "made out of pure meat! Now",
                "how's that for the best of both",
                "worlds? Yeah? Yeah? Here,",
                "why don't you try a sample?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "What the?!",
                "No way in hell am",
                "I putting that in my",
                "mouth! What the heck",
                "is that supposed to be?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "It's the world's",
                "first zucchini...",
                "Made completely out",
                "of Grade A Sirloin Steak!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "No no no! I've got to",
                "go and ask those other",
                "merchants about that",
                "missing boy. Er, but",
                "good luck selling that."
            ],
        )?;
        ctx.var("lost_boy").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()?.number()? >= 5 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "(^333333I better ask the other",
                "merchants around here if",
                "they've seen that boy. I'm",
                "wasting my time talking to",
                "this crazy merchant and his",
                "ridiculous mock vegetables.^000000)"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn idle_merchant_ra_in01(ctx: &Ctx) -> Script {
    idle_merchant_ra_in01_body(ctx, Vec::new()).map(|_| ())
}

fn idle_merchant_ra_in01_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lost_boy").get()?.number()? < 5 || ctx.var("lost_boy").get()?.number()? > 6) {
        ctx.lines_as(
            "Idle Merchant",
            args![
                "I'm thinking of quitting",
                "this business... No one",
                "to seems to want whatever",
                "I'm selling! I definitely can't",
                "make a living like this."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 5 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me,", "um... Hello?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Idle Merchant", args!["Oh! Welcome to my shop!", "How can I help you today?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Actually, I'm not",
                "here to buy anything...",
                "I'm looking for a boy",
                "named Phobe that was",
                "here a few days ago.",
                "Have you seen him?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "Oh! Yeah, last night,",
                "some guy asked me to tell",
                "anyone looking for this guy",
                "named Phobe to give you",
                "a message. Basically, he wants",
                "you to come to the ''ice cave.''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Who asked you to tell",
                "me that? How did he know",
                "that I'd come over here?",
                "Does he know where Phobe",
                "is? I-Is Phobe alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "No clue. I told you all",
                "I know. Let's see, his",
                "name was... Nogan?",
                "Rogan? Something",
                "like that. That's",
                "all I know, honest."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "So the kid's missing,",
                "huh? Good luck finding",
                "him. Oh, and be careful",
                "in that ice cave. That place",
                "can be plenty dangerous."
            ],
        )?;
        ctx.var("lost_boy").set(Val::from(6))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8092), Val::from(8093)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 6 {
        ctx.lines_as(
            "Idle Merchant",
            args![
                "So yeah, last night,",
                "some guy asked me to tell",
                "anyone looking for this guy",
                "named Phobe to give you",
                "a message. Basically, he wants",
                "you to come to the ''ice cave.''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Who asked you to tell",
                "me that? How did he know",
                "that I'd come over here?",
                "Does he know where Phobe",
                "is? I-Is Phobe alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "No clue. I told you all",
                "I know. Let's see, his",
                "name was... Nogan?",
                "Rogan? Something",
                "like that. That's",
                "all I know, honest."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Idle Merchant",
            args![
                "So the kid's missing,",
                "huh? Good luck finding",
                "him. Oh, and be careful",
                "in that ice cave. That place",
                "can be plenty dangerous."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn idle_merchant_ra_in01_2(ctx: &Ctx) -> Script {
    idle_merchant_ra_in01_2_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_man_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lost_boy").get()?.number()? < 6 || ctx.var("lost_boy").get()?.number()? > 7) {
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Th-there's not enough",
                "air here, it's so stuffy!",
                "Hey, get away! You're",
                "wasting all of my",
                "precious oxygen!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lost_boy").get()? == 6 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but do you", "know a man named", "Rogan? I'm here to--"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Hah! I had no idea",
                "it'd be this simple!",
                "Easiest money I've",
                "ever made! Bwahahaha!",
                "Time for you to die, kid!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Suspicious Man", args!["Hyah!"])?;
        ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThis suspicious man",
            "wounded you with a throwing",
            "dagger, and then threw another",
            "one that you managed to dodge.",
            "You quickly retaliate to keep",
            "him from further harming you.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Argh! I...",
                "I guess I underestimated",
                "you! You're a lot stronger",
                "than when you're taken by",
                "surprise, adventurer!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Hey, who the hell are",
                "you?! Are you Rogan?",
                "Are you the one that",
                "kidnapped Phobe?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "What th--? I didn't",
                "kidnap nobody! I don't",
                "know any Phobe! I was",
                "just hired to kill you and",
                "bring your body to my client.",
                "This wasn't what I expected."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Suspicious Man",
            args![
                "Vincent's instructions",
                "were to kill anyone that",
                "asked about Rogan, some",
                "name he made up. My name",
                "is Mogan. Gosh... I thought",
                "you were a really bad guy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mogan",
            args![
                "Hey, set me straight",
                "here. Did you steal",
                "something really",
                "valuable, like some",
                "red jewel or something?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Huh? No! Vincent actually",
                "sent me to look for the guy",
                "that stole that jewel, and",
                "to retrieve it for him!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mogan",
            args![
                "It looks like he set",
                "us up, dude. That's...",
                "That's really low. Sorry,",
                "if I had known, I wouldn't",
                "have done it. Why would",
                "he manipulate us like that?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I don't know...",
                "I'm going to talk",
                "to Vincent and make",
                "him explain everything."
            ],
        )?;
        ctx.var("lost_boy").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8093), Val::from(8094)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 7 {
        ctx.lines_as(
            "Mogan",
            args![
                "It looks like he set",
                "us up, dude. That's...",
                "That's really low. Sorry,",
                "if I had known, I wouldn't",
                "have done it. Why would",
                "he manipulate us like that?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I don't know...",
                "I'm going to talk",
                "to Vincent and make",
                "him explain everything."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn suspicious_man_ra_in01(ctx: &Ctx) -> Script {
    suspicious_man_ra_in01_body(ctx, Vec::new()).map(|_| ())
}

fn kid_ra_in01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lost_boy").get()?.number()? < 7 {
        ctx.lines_as("Kid", args!["Leave me alone!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 8 {
        ctx.lines(args![
            "^3355FFYou catch a shining",
            "glint from this boy's",
            "back pocket. Perhaps he",
            "has the jewel that Vincent",
            "wants you to retrieve.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hey, you.", "You must be", "Phobe, right?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Yeah?", "So what?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Your father sent", "me here to find you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Phobe", args!["No! Don't touch me!", "I have no father!", "H-he's dead to me!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, in any case, I need",
                "to return the jewel that",
                "you're hiding in your pocket.",
                "I know that it really belongs",
                "to Sir Zhed.  If I don't, then",
                "your father will be in trouble."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phobe",
            args![
                "Hah! You mean ''Mister",
                "Vincent!'' That's fine",
                "by me! That's exactly",
                "why I took this thing!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh, yeah? How do you",
                "think Ms. Jenny will",
                "feel? Do you think",
                "that she'll be happy",
                "hearing about this?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phobe",
            args![
                "..............................",
                "...So you know about",
                "our family secret, huh?",
                "Alright, I'll give this to",
                "you. But I'm doing this for",
                "my mom, and not for him!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFPhobe gingerly pulls",
            "the shining red jewel",
            "from his pocket and",
            "reluctantly hands it to you.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Phobe",
            args![
                "Just so you know,",
                "I don't plan on going",
                "back home! Well, just",
                "not this second anyway.",
                "But let my mom know that",
                "I'm fine and not to worry."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Alright, I'll tell your mom",
                "But you should be getting",
                "back to them soon. Your",
                "father took a really big",
                "risk covering up for you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Phobe", args!["Hmpf! I...", "I don't care!"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt doesn't look like",
            "you can force Phobe to",
            "return home for now, so",
            "you should bring this red",
            "jewel back to Vincent.^000000"
        ])?;
        ctx.var("lost_boy").set(Val::from(9))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8095), Val::from(8096)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lost_boy").get()? == 9 {
        ctx.lines_as(
            "Phobe",
            args![
                "I... I'm not ready",
                "to go back home just",
                "fine, and let Mister Vincent",
                "know that I'm not sorry",
                "for what I did, okay?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt doesn't look like",
            "you can force Phobe to",
            "return home for now, so",
            "you should bring this red",
            "jewel back to Vincent.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["You know, you should", "quit making trouble and", "listen to your parents."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phobe",
            args!["You're not the boss", "of me! Mind your own", "business! Jeez louise!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I can't mind my own",
                "business if your actions",
                "are causing this much trouble",
                "to so many other people! You",
                "need to act more responsibly!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Phobe", args!["Mmmrrrr...", "Gosh... Fine."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn kid_ra_in01(ctx: &Ctx) -> Script {
    kid_ra_in01_body(ctx, Vec::new()).map(|_| ())
}
