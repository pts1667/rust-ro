use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn promotional_staff_prt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_1::promotional_staff(ctx, vec![])?;
    return Err(Stop::End);
}

pub fn promotional_staff_prt(ctx: &Ctx) -> Script {
    promotional_staff_prt_body(ctx, Vec::new()).map(|_| ())
}

fn alliance_manager_prt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 13 {
        ctx.lines_as(
            "Alliance Manager",
            args!["If we'd had the initiative over Schwarzwald, we could take advantage over them! But we lost the opportunity."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "Though we are allied with them, that's nothing more than a formality.",
                "We will take advantage over them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Alliance Manager", args!["Do you know what we are here for? These are the things that help to have control over multiple countries and get to know their magic."])?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args!["Whatever. Though the things are not useful to us, we'd better destroy them, to prevent them from being used by others."],
        )?;
        ctx.next()?;
        ctx.lines_as("Alliance Manager", args!["Anything except things which are not found on Rune-Midgarts... keep them hidden. Just let us know of whatever you are holding."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 12 {
        ctx.lines_as("Alliance Manager", args!["...Do you have any business with me?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args!["Hm... The boss had you come to me,", "to talk about the issues? Ok!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "So, everything is going well?",
                "You're allied with Schwarzwald?",
                "They will join with you.",
                "I know this."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Alliance Manager", args!["Hm....ok."])?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args!["You are all going, forward through", "the dimensional rift..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "Good. It's started.",
                "We also need to be preparted to not",
                "let ourselves be convinced to turn over."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "We are so-called 'allies',",
                "so we can't take advantages over",
                "each other in public, but we can do",
                "it secretly."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "The Schwarzwald... they are all",
                "snobs. I know what they did.",
                "They'll try to get what they can",
                "with little or no work. I know",
                "their dirty tricks."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "We shouldn't loose anything to them,",
                "no way! We are not as powerful as",
                "most of them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "We only have a strange dimensional",
                "rift in our reserves, but nothing",
                "else... Only we are about to trick",
                "them, to win over them!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alliance Manager",
            args!["You are a member of our alliance.", "Always be careful not to get lost."],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10068), Val::from(10069)])?;
        ctx.var("ep13_ryu").set(Val::from(13))?;
        ctx.lines_as(
            "Alliance Manager",
            args![
                "Thanks for reporting to us.",
                "We are better prepared.",
                "Good job. You can go now."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Alliance Manager", args!["Do you have any business here?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn alliance_manager_prt(ctx: &Ctx) -> Script {
    alliance_manager_prt_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_prt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ep13_ryu").get()? == 12 || ctx.var("ep13_ryu").get()? == 13) {
        ctx.lines_as(
            "Member of Alliance",
            args!["If we have absolute power, we wouldn't have to get along with Schwarzwald and Arunafeltz."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Member of Alliance",
            args!["I don't know when Satan Morocc will attack us, and whether we're powerful enough to detect him."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Member of Alliance",
            args!["We couldn't avoid cooperating with them. We don't have many choices left."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Member of Alliance",
            args!["So we will take advantage over them in secret, in this dimensional rift."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Member of Alliance",
        args![
            "If we'd have had more power,",
            "we wouldn't be like this now.",
            "Strength is most important."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_prt(ctx: &Ctx) -> Script {
    member_of_alliance_prt_body(ctx, Vec::new()).map(|_| ())
}

fn recruiter_for_the_brave_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 12 {
        ctx.lines_as(
            "Recruiter",
            args!["Why have you come again?", "You'd better go to Schwarzwald to register."],
        )?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["What? You came to here to convey a story? Go ahead."])?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["Move to the side. There's a Manager who is dealing with all of us, and is involved in international relations here on Midgard."])?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["Just talk to him."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()?.number()? > 8 {
        ctx.lines_as(
            "Recruiter",
            args![
                "Just go there.",
                "You can get nothing from me.",
                "Let's rush to",
                "^FF0000Lighthalzen^000000, which is the original place for the three kingdoms..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 8 {
        ctx.lines_as(
            "Recruiter",
            args!["I've heard of you.", "You've finished the tests from the Promotional Staff?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args![
                "You are definitely brave enough to venture to this new land.",
                "I will now appoint you to detect and have adventures in Ash-Vacuum."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Ash-Vacuum?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Recruiter", args!["Hm. That's the code name of the place where you will have your adventure. Frankly, we don't know exactly what it is, but we call it 'Ash-Vacuum'."])?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args!["Believe it or not,", "it's definitely true.", "So listen carefully.", "Please."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args!["Ash-Vacuum... is a living place... that's totally different from where we are living now."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args![
                "It's an unknown place.",
                "We don't know what's there,",
                "or who is living there.",
                "It's beyond our guess.",
                "Nobody knows."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["Only the brave, like you, deserve to go there. There will be many dangerous things around you, but you must explore it on behalf of Midgard."])?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["Hmm......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args!["Anyway, I am sure that there are many opportunities for fame and fortune there..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["First, you should go to ^FF0000Lighthalzen^000000 to register for the United Midgard Alliance. I believe they are meeting at the Rekenber Corporation Headquarters..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args!["You can get a detailed guide, better than me, there. Go there and find him..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Recruiter", args!["I've done what I can. Go there, and get to it!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Recruiter",
            args!["Remember, go to ^FF0000Lighthalzen^000000, then follow the directions of the guide there."],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10064), Val::from(10065)])?;
        ctx.var("ep13_ryu").set(Val::from(9))?;
        ctx.call(Function::GetExperience, vec![Val::from(660000), Val::from(210000)])?;
        ctx.lines_as(
            "Recruiter",
            args!["Just go there!", "And listen carefully", "about your mission!", "I'm done here..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()?.number()? > 0 {
        ctx.lines_as("Recruiter", args!["What're you doing here?", "Just go to ^FF0000Aldebaran^000000, and find the ^FF0000Promotional Staff^000000. As I advised you, you'd better save time."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Recruiter",
        args![
            "Brave ones! Come to me!",
            "Real brave people!",
            "Would you mind trying a challenging mission?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Recruiter",
        args!["I am a recruiter looking for brave people in Prontera! My goal is to recruit the bravest adventurers for this mission."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Recruiter",
        args![
            "Would you like to join?",
            "This is always open to you.",
            "I know, you're super curious.",
            "Just try whatever you want."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes, I am interested.:No, thanks.")])? {
        1 => {
            ctx.lines_as(
                "Recruiter",
                args![
                    "Oh! Yes! You're quite curious!",
                    "You're a real adventurer.",
                    "A genuine desire for adventure.",
                    "Ok! Let me start by telling you the story of the mission!!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Recruiter", args!["All of this of course, the Promotional Staff can tell you, but they're not that reliable. So, let me explain to you. Just listen carefully."])?;
            ctx.next()?;
            ctx.lines_as("Recruiter", args!["Um... There's not that much to tell you...", "..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Recruiter",
                args!["The point is that we want someone who is curious and brave, to go to new places. And I choose you!!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Recruiter", args!["This is a special place among unknown places. Quite special! I can't tell you much just now. Just believe me that it is truly uncommon!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Recruiter",
                args![
                    "So, now, let me test you.",
                    "Once I get you qualified,",
                    "I will tell you everything."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Recruiter", args!["Just go to the ^FF0000Promotional Staff^000000 in Aldebaran and get permission. They can judge you more carefully than me. Go ahead."])?;
            ctx.next()?;
            ctx.lines_as(
                "Recruiter",
                args!["They will take care of everything, if you say that you've been sent by the Recruiter in Prontera."],
            )?;
            ctx.next()?;
            ctx.call(Function::SetQuest, vec![Val::from(10057)])?;
            ctx.var("ep13_ryu").set(Val::from(1))?;
            ctx.lines_as(
                "Recruiter",
                args![
                    "Time is zeny, just go.",
                    "As soon as you visit the Promotional Staff, you will be allowed to travel to the unknown land."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Recruiter",
                args!["Are you seriously enjoying your adventures? You're not that curious."],
            )?;
            ctx.next()?;
            ctx.lines_as("Recruiter", args!["Just go about your business!"])?;
            ctx.next()?;
            ctx.lines_as("Recruiter", args!["-Muttering and complaining...-"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn recruiter_for_the_brave(ctx: &Ctx) -> Script {
    recruiter_for_the_brave_body(ctx, Vec::new()).map(|_| ())
}

fn promotional_staff_alde_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 3 {
        ctx.lines_as(
            "Promotional Staff",
            args!["Did you try out for the test, or are you already enjoying your adventures?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 3 {
        ctx.lines_as("Promotional Staff", args!["Just visit the ^FF0000Promotional Staff in Geffen^000000. You have no business with me anymore. Bless you. I wish you a safe adventure."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 2 {
        if ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? < 300 {
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "Don't forget to bring... ^FF0000300 Jellopy^000000.",
                    "Got it? ^FF0000300 Jellopy^000000!",
                    "Let me know once you got them."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Hey! Let me know once you collect all of them.",
                "Don't disturb me anymore. I'm a busy man... What a waste."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Hm? Did you bring all of them?",
                "No! It's quite an annoying job",
                "I'm impressed that you managed to do such a menial thing to bring 300 Jellopy!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["You have guts kid!", "You're a good-willed person!", "Ok, you win!", "You pass!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Adventurers like you",
                "are not common",
                "A hard-boiled soldier!",
                "Stronger than me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Now can you go to the", "Promotional Staff in Geffen?", "He will test you again."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "You know this isn't an easy quest.",
                "You resolved to carry it out, didn't you?",
                "Brace yourself for it."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(909), Val::from(300)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10058), Val::from(10059)])?;
        ctx.var("ep13_ryu").set(Val::from(3))?;
        ctx.lines_as("Promotional Staff", args!["Anyway you did the first step.", "Bless you!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 1 {
        ctx.lines_as(
            "Promotional Staff",
            args!["We want the adventurers who are super curious and extremely brave. Join us for a wonderful adventure!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Hey, you're a well-versed person, right? Are you interested in my story?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I have something for you.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Promotional Staff", args!["For me??", "I hate annoying stuff."])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Ahh... I sensed that I would get your request.",
                "The recruiter sent you here to get tested?",
                "Hmm."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Ah! They always push me,", "to suffer from so much work", "I live a hard life!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "My job is testing if you are strong and brave enough for this adventure.",
                "What a bother!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "I will accept you if you bring me ^FF0000300 Jellopy^000000.",
                "I believe that you can bring them to me.",
                "Are you ready?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10057), Val::from(10058)])?;
        ctx.var("ep13_ryu").set(Val::from(2))?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Just go ahead and hunt monsters, or whatever and collect 300 jellopies.",
                "Then we can go forward."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    shared::quests_quests_13_1::promotional_staff(ctx, vec![])?;
    return Err(Stop::End);
}

pub fn promotional_staff_alde(ctx: &Ctx) -> Script {
    promotional_staff_alde_body(ctx, Vec::new()).map(|_| ())
}

fn promotional_staff_gef_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 6 {
        ctx.lines_as(
            "Promotional Staff",
            args!["Did you take the test?", "Or are you already enjoying your adventure?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 6 {
        ctx.lines_as(
            "Promotional Staff",
            args![
                "You didn't visit ^FF0000Izlude^000000, did you?",
                "Or do you have any business in Geffen?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 5 {
        if ctx.call(Function::CountItem, vec![Val::from(723)])? == 0 {
            ctx.lines_as(
                "Promotional Staff",
                args!["^FF00001 Ruby!!^000000.", "Don't you forget it!", "Hurry up! Time is zeny!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Wow, you brought a genuine Jewel.",
                "You are absolutely qualified.",
                "I can tell these things."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Am I exaggerating?", "It's true that you're", "considerably strong."],
        )?;
        ctx.next()?;
        ctx.lines_as("Promotional Staff", args!["Ok! My job is done.", "Good job!!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Next step is Izlude!",
                "You can get tested by the Promotional Staff there.",
                "Hurry up!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "The unknown place...",
                "It is a very dangerous place so you must go through these tests before we can send you....",
                "We're pretty picky."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(723), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10061), Val::from(10062)])?;
        ctx.var("ep13_ryu").set(Val::from(6))?;
        ctx.lines_as("Promotional Staff", args!["I hope you can do it!", "Good luck!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 4 {
        if ctx.call(Function::CountItem, vec![Val::from(721)])? == 0 {
            ctx.lines_as(
                "Promotional Staff",
                args!["The one you should bring me is", "^FF0000Emerald^000000. Can you get it?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Promotional Staff",
            args![
                "You brought it so soon!",
                "You're a real adventurer!",
                "I will accept this Emerald as evidence."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["This isn't something I wanted to keep for myself.", "You know that, right?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Next bring me ^FF00001 Ruby^000000.",
                "I can't judge you with only one jewel.",
                "So I need one more stone."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(721), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10060), Val::from(10061)])?;
        ctx.var("ep13_ryu").set(Val::from(5))?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Please bring me a Ruby.",
                "As I mentioned, this is the second request.",
                "I will wait until you bring me a stone again."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 3 {
        ctx.lines_as(
            "Promotional Staff",
            args!["We are recruiting adventurers who are strong and curious. Try your life at something more challenging."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Hey, you're a knowledgeable person, right? Are you interested in my story?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("The Staff in Aldebaran sent me.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Promotional Staff", args!["Oh he did?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Ok, I guess that it's time for the second part of your test."],
        )?;
        ctx.next()?;
        ctx.lines_as("Promotional Staff", args!["I need you to gather some stones for me."])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["I will accept you if you bring ^FF00001 Emerald^000000.", "Are you ready?"],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10059), Val::from(10060)])?;
        ctx.var("ep13_ryu").set(Val::from(4))?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Just bring me ^ff00001 Emerald^000000.",
                "That isn't hard for you right?",
                "Then we can go forward."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    shared::quests_quests_13_1::promotional_staff(ctx, vec![])?;
    return Err(Stop::End);
}

pub fn promotional_staff_gef(ctx: &Ctx) -> Script {
    promotional_staff_gef_body(ctx, Vec::new()).map(|_| ())
}

fn promotionalstaff_izlude_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 8 {
        ctx.lines_as(
            "Promotional Staff",
            args!["I bless you for your future! May it be full of happiness!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 8 {
        ctx.lines_as(
            "Promotional Staff",
            args!["The test is done!", "If you have any other business, go to the palace."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 7 {
        if ctx.var("BaseLevel").get()?.number()? < 70 {
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "You don't look that strong.",
                    "Nothing but skin and bones!",
                    "Not reliable.",
                    "You should level up more before",
                    "considering this adventure."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "I don't need adventurers who are",
                    "body-builders... but at least",
                    "someone not so little!!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Wow! You are considerably",
                "stronger than before!",
                "Let me see...",
                "You look different!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Promotional Staff", args!["Um..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Ok, it's done!",
                "You seemed a sore weak of a soul,",
                "but now, you're quite stronger than",
                "before. Great! You will do!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "We have quite high expectations to meet.",
                "Though the new place is dangerous,",
                "no need to check thoroughly",
                "anymore."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Promotional Staff", args!["It's not a hell-hole!", "Ukk!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Anyway, just go back to the",
                "Recruiter. I will forward to him",
                "that you passed all the steps. Good",
                "job, friend!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10063), Val::from(10064)])?;
        ctx.var("ep13_ryu").set(Val::from(8))?;
        ctx.lines_as("Promotional Staff", args!["Bless all of your heart,", "for your great future!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 6 {
        ctx.lines_as(
            "Promotional Staff",
            args!["We are recruiting adventurers", "who are quite strong and curious."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Hey, you're a knowledgeable adventurer. Are you interested in my story?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I have something for you.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Um, you're the one sent to be tested, correct? You are in the right place. I'm the Promotional Staff in Izlude."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["I guess you're qualified enough to go... I think no more testing is needed."],
        )?;
        ctx.next()?;
        ctx.lines_as("Promotional Staff", args!["Let me see..."])?;
        ctx.next()?;
        if ctx.var("BaseLevel").get()?.number()? > 69 {
            ctx.lines_as(
                "Promotional Staff",
                args!["I think you're good enough!", "No more testing!!", "Ok! You pass!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "We have quite high expectations to meet.",
                    "Though the new place is dangerous,",
                    "no need to check thoroughly right now."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Promotional Staff", args!["It's not a hell-hole!", "Ukk..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "Anyway, just go back to the recruiter.",
                    "I will forward to him that you passed all the steps.",
                    "Good job, friend!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10062), Val::from(10064)])?;
            ctx.var("ep13_ryu").set(Val::from(8))?;
            ctx.lines_as("Promotional Staff", args!["May Freya bless you,", "for your great future!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Promotional Staff", args!["Hmm..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "You don't look that strong.",
                    "Nothing but skin and bones!",
                    "Not reliable.",
                    "I can't let you pass."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10062), Val::from(10063)])?;
            ctx.var("ep13_ryu").set(Val::from(7))?;
            ctx.lines_as(
                "Promotional Staff",
                args![
                    "Please level up a little more. I",
                    "can't accept that you're strong",
                    "enough. Sorry."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    shared::quests_quests_13_1::promotional_staff(ctx, vec![])?;
    return Err(Stop::End);
}

pub fn promotionalstaff_izlude(ctx: &Ctx) -> Script {
    promotionalstaff_izlude_body(ctx, Vec::new()).map(|_| ())
}

fn guide_ep13_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 8 {
        ctx.lines_as(
            "Guide",
            args!["Welcome! You've arrived at Lighthalzen, which is the base of three kingdoms."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guide",
            args!["You have to register your name. Go to the 2nd floor in Rekenbereu on the northwest side."],
        )?;
        ctx.next()?;
        ctx.lines_as("Guide", args!["Be careful not to lose your way!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Guide",
        args!["Welcome! You've arrived at Lighthalzen, which is the base of three kingdoms."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guide_ep13_1(ctx: &Ctx) -> Script {
    guide_ep13_1_body(ctx, Vec::new()).map(|_| ())
}

fn munkenro_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 20 {
        ctx.mes("-At this spot, it's possible to overhear something. Listen carefully through the wall...-")?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["You're a good buddy!", "But..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args![
                "We can't avoid this...",
                "You know better than I!",
                "We can't go through this,",
                "in this current status."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["I did all I could do...", "Please..."])?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["............"])?;
        ctx.next()?;
        ctx.mes("..................")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["There's no avoiding it.", "Ok, I need to try."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ep13_ryu").get()? == 18 || ctx.var("ep13_ryu").get()? == 19) {
        ctx.lines_as("Munkenro", args!["You are ready to try this..."])?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["Oh! My friend..."])?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["Nothing can be controlled anymore... Am I trying this...?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("ep13_ryu").get()? == 15 || ctx.var("ep13_ryu").get()? == 16) || ctx.var("ep13_ryu").get()? == 17) {
        ctx.lines_as("Munkenro", args!["Peace for the three kingdoms...?", "Isn't it impossible...?"])?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["Are you sure that", "you can do that?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 14 {
        ctx.lines_as("Munkenro", args!["Nothing shall prevent me from doing my duty. Nothing!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["Sikaiz is a kind and upright person. But he can't be cautious about everything. It could cause serious problems."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["So that's why I am here for him. I always worry about him..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["I don't know what I would do if something were to happen..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Munkenro", args!["......"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn munkenro_1(ctx: &Ctx) -> Script {
    munkenro_1_body(ctx, Vec::new()).map(|_| ())
}

fn sikaiz_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("ep13_ryu").get()? == 18 || ctx.var("ep13_ryu").get()? == 19) {
        ctx.lines_as("Sikaiz", args!["Please come to me ASAP.", "I hope you play an active part."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["We all expect you to make the biggest impact."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 17 {
        ctx.lines_as(
            "Sikaiz",
            args!["Only thing you should do is write down your name and show me your great face"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["I already accept you. Just write down your name here. Go ahead."],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if !l_input_s
            .clone()
            .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
        {
            ctx.lines_as("Sikaiz", args!["Don't you know your own name?", "Write it again."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Sikaiz",
                args![
                    ((Val::from("Ok. I got your name, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(". Great name! Registration is done!")),
                    "You've become a member of the three kingdoms!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sikaiz",
                args![
                    "All done.",
                    "Go to the field Officer of Schwarzwald. He should be in the banquet hall of this building downstairs."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sikaiz",
                args!["Only thing left for you is to go to the Officer to say that you are leaving now."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sikaiz",
                args![
                    "As soon as you tell him,",
                    "come back to me again.",
                    "You are not the only one expecting this great challenge. I am too."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Sikaiz", args!["I believe you can do it!", "It would bear watching you."])?;
            ctx.next()?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10073), Val::from(10074)])?;
            ctx.var("ep13_ryu").set(Val::from(18))?;
            ctx.lines_as("Sikaiz", args!["See you then."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ep13_ryu").get()? == 16 {
        ctx.lines_as(
            "Sikaiz",
            args!["You came back!", "Fast learner!", "I love that you're a member of my alliance."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["Our preparations will soon be complete."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["We will surely share all our information with the three kingdoms, and study them."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["Definitely, you are the one to face all the trials and tribulations."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["The important thing is that we need to share all the current information relevant to this place."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "I advise you,",
                "if you get friendly with",
                "the creatures in there,",
                "that's a kind source to know.",
                "I'll believe you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "Let's conclude.",
                "You need to investigate and report to us as much as possible. That's not that tough."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10072), Val::from(10073)])?;
        ctx.var("ep13_ryu").set(Val::from(17))?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "Don't worry so much.",
                "We will support you.",
                "As we can.",
                "So I will tell you the last step to becoming a member of the alliance."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 15 {
        ctx.lines_as(
            "Sikaiz",
            args!["In ^FF0000Cheshrumnir in Rachel^000000,", "there is a Manager."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["You already met someone like that from Rune-Midgarts. So no more explanation is needed."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 14 {
        ctx.lines_as(
            "Sikaiz",
            args![
                "Ok, so we should talk about",
                "the tasks we should go through.",
                "These are about it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Munkenro",
            args!["Hey, sorry for interrupting you. I also have information to send to Arunafeltz."],
        )?;
        ctx.next()?;
        ctx.lines_as("Munkenro", args!["We're in a hurry."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["Hey! You hear that? Sorry I'm sure that you've got many things to do. Can I request one more thing? I can't deal with it myself."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["I feel like I am forcing you, but I hope to get your help. Go to the ^FF0000Cheshrumnir Guard^000000 in Rachel and deliver Munkenro's message for me."])?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10070), Val::from(10071)])?;
        ctx.var("ep13_ryu").set(Val::from(15))?;
        ctx.lines_as(
            "Sikaiz",
            args!["It's the same mission you did for Rune-Midgarts, so it shouldn't be that hard. I hope you can do it."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 13 {
        ctx.lines_as("Sikaiz", args!["You did it already?", "You are a quick worker."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["Anyway, I promised to tell you about the three kingdoms... This is kind of a long story, so please listen carefully."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "The adventurers for Ash-vacuum! The alliance for the three kingdoms! Rune-Midgarts, Schwarzwald, Arunafeltz!",
                "Three kingdoms united for one thing!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["Frankly, there's a Dimensional Rift in the Sograt desert, so they can inspect it themselves, but it's not that good of a place to inspect the rift's magic well."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["We are a little bit better in magic, but not that much developed in science as compared with Schwarzwald, and not as wise compared with Arunafeltz."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["So we need to get help from other countries, because the Ash-vacuum is quite a profitable place. That is why we made the Alliance to get into the Ash-Vacuum."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["The three kingdoms were enemies before. Fortunately, it's going well now. It's fine."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["This alliance is a first step for peace among the three kingdoms. You know they are all for themselves, so this needs harmony. That's the one thing to pursue, as head of an alliance."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["Honestly, I am also forced by my kingdom, to further our own interests and not others."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["But the most valuable thing is the unity. Not each kingdom's interest. If we spoil the greatest plans, it's too regrettable."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["Though I belong to Schwarzwald, I need to balance the three kingdoms, as a leader... so I can't support my kingdom too much over the others."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "Anyway, I believe that my friend can assist me well.",
                "Though he is from another kingdom, he's supported me thus far."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["He's the man to my left.", "We call him Munk.", "His real name is... Munkenro."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["He is a kind of manager for Schwaltzvald and he is the main contact point of the kingdom's people, since he's quite closely in communication with them."])?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10069), Val::from(10070)])?;
        ctx.var("ep13_ryu").set(Val::from(14))?;
        ctx.lines_as(
            "Sikaiz",
            args!["That's enough talk about the three kingdoms. Now, let's move on to the next stories."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 12 {
        ctx.lines_as("Sikaiz", args!["Go to Prontera in Rune-Midgarts and find the Manager there. Tell him that the Alliance is doing well and that we are entering the rift."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 11 {
        ctx.lines_as("Sikaiz", args!["So, what do you want to know more of? You already know about the Ash-Vacuum. Is there anything else you'd like me to tell you?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("The three kingdoms...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Sikaiz",
            args!["Ah, I need to tell you about the alliance, but first, may I ask something?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["I need to report its status to the managers in Rune-Midgarts. I can't do it right now. May I ask you a favor?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["It's not that difficult. Just tell them to prepare for the trip to the Dimensional Rift. It's quite easy, isn't it?"],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10067), Val::from(10068)])?;
        ctx.var("ep13_ryu").set(Val::from(12))?;
        ctx.lines_as(
            "Sikaiz",
            args!["We're running out of time, so I will answer you when you come back. Ok?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 10 {
        ctx.lines_as("Sikaiz", args!["Whew......"])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["You come to be registered as an adventurer? Then I'm the one to talk to. I am Sikaiz, the head of the United Midgard Alliance."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["You come here from Prontera? Then you have been sent by the Recruiter. Didn't he tell you about the stories? Maybe he thought that I would tell you everything."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["I can tell you, but, he's so obstinate. So many kinds of people are living in this world, but quite few are as obstinate as him."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["The registration is also important now, but I will tell you about our status here. May I begin?"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Sure.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Sikaiz",
            args!["Ok. Let me tell you about the Ash-Vacuum. It's quite a long story. So please bear with me."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "^FF0000Ash-Vacuum^000000....",
                "This isn't the world of mortals.",
                "Then what is it...",
                "Nobody knows."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["Now, we only know that there are living beings there, and that they are not approachable by everyone."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["The name, 'Ash-Vacuum', is so we can identify it easily."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "You're probably wondering how we got there...",
                "The answer lies with Satan Morocc."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "Can you imagine?",
                "Why did Satan Morocc",
                "run away from there???",
                "That's the important thing."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["He made a gate to run away, by himself. It's very unlike him."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["But he did make some mistakes. He left an opened gate. Nobody knows if it was intentional... or maybe..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "We call the dimensional rift:",
                "Time-Gap of Dimension.",
                "Who knows what secrets lie in that world beyond."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["So we've inspected it.", "And there's not much info to get from it. We can't guess if it's from the past... or the future... Even WE don't know where it is."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["That's the place!", "Ash-Vacuum!", "We are all in this together!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "Is that enough?",
                "Though I am the head of the Alliance, I don't know much else about it. That's why we are all here. To do some research."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["Anyway, I told you about Ash-Vacuum. What I know about it all. Though you still don't know much about it, that's all I can tell you."])?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10066), Val::from(10067)])?;
        ctx.var("ep13_ryu").set(Val::from(11))?;
        ctx.lines_as("Sikaiz", args!["If you have any other questions, you can still ask me..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 9 {
        ctx.lines(args![
            "- The man standing in front -",
            "- is delivering a speech. -",
            "- Let's listen. -"
        ])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["I, Sikaiz, make it a point that we should pursue the great development of the Midgard continent with the research of the newly discovered Ash-Vacuum."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["So we must not fight for own interests, we must not forget the mission. We are all together on this, to go through all the same trials."])?;
        ctx.next()?;
        ctx.lines_as("Sikaiz", args!["This mission shouldn't be for each kingdom's interest alone. If we are not strongly allied for our mission, we will not succeed in discovering the unknown places of Ash-Vacuum."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args!["Now we should think we are all belonging to the Midgard continent. That's home for all of us."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sikaiz",
            args![
                "We will leave for Ash-Vacuum, through the dimensional rift soon.",
                "You all should be relaxed about completing this mission. If there are more adventurers to register, let them come to me."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10065), Val::from(10066)])?;
        ctx.var("ep13_ryu").set(Val::from(10))?;
        ctx.lines(args!["- The speech has ended. -", "- Let's move on. -"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Sikaiz", args!["......"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sikaiz_1(ctx: &Ctx) -> Script {
    sikaiz_1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardEp1311Step {
    Start,
    OnTouch,
}

fn guard_ep13_1_1_run(ctx: &Ctx, mut step: GuardEp1311Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardEp1311Step::Start => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(0), Val::from(108), Val::from(252)])?;
                step = GuardEp1311Step::OnTouch;
                continue 'machine;
            }
            GuardEp1311Step::OnTouch => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(1), Val::from(108), Val::from(252)])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guard_ep13_1_1(ctx: &Ctx) -> Script {
    guard_ep13_1_1_run(ctx, GuardEp1311Step::Start, Vec::new()).map(|_| ())
}

pub fn guard_ep13_1_1_ontouch(ctx: &Ctx) -> Script {
    guard_ep13_1_1_run(ctx, GuardEp1311Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardEp1313Step {
    Start,
    OnTouch,
}

fn guard_ep13_1_3_run(ctx: &Ctx, mut step: GuardEp1313Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardEp1313Step::Start => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(0), Val::from(152), Val::from(252)])?;
                step = GuardEp1313Step::OnTouch;
                continue 'machine;
            }
            GuardEp1313Step::OnTouch => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(1), Val::from(152), Val::from(252)])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guard_ep13_1_3(ctx: &Ctx) -> Script {
    guard_ep13_1_3_run(ctx, GuardEp1313Step::Start, Vec::new()).map(|_| ())
}

pub fn guard_ep13_1_3_ontouch(ctx: &Ctx) -> Script {
    guard_ep13_1_3_run(ctx, GuardEp1313Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardEp1315Step {
    Start,
    OnTouch,
}

fn guard_ep13_1_5_run(ctx: &Ctx, mut step: GuardEp1315Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardEp1315Step::Start => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(0), Val::from(123), Val::from(229)])?;
                step = GuardEp1315Step::OnTouch;
                continue 'machine;
            }
            GuardEp1315Step::OnTouch => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(1), Val::from(123), Val::from(229)])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guard_ep13_1_5(ctx: &Ctx) -> Script {
    guard_ep13_1_5_run(ctx, GuardEp1315Step::Start, Vec::new()).map(|_| ())
}

pub fn guard_ep13_1_5_ontouch(ctx: &Ctx) -> Script {
    guard_ep13_1_5_run(ctx, GuardEp1315Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardEp1317Step {
    Start,
    OnTouch,
}

fn guard_ep13_1_7_run(ctx: &Ctx, mut step: GuardEp1317Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardEp1317Step::Start => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(0), Val::from(139), Val::from(228)])?;
                step = GuardEp1317Step::OnTouch;
                continue 'machine;
            }
            GuardEp1317Step::OnTouch => {
                shared::quests_quests_13_1::guard_13_1(ctx, vec![Val::from(1), Val::from(139), Val::from(228)])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guard_ep13_1_7(ctx: &Ctx) -> Script {
    guard_ep13_1_7_run(ctx, GuardEp1317Step::Start, Vec::new()).map(|_| ())
}

pub fn guard_ep13_1_7_ontouch(ctx: &Ctx) -> Script {
    guard_ep13_1_7_run(ctx, GuardEp1317Step::OnTouch, Vec::new()).map(|_| ())
}

fn guard_ep13_1_9_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.lines_as(
            "Guard",
            args![
                "You're not allowed to enter.",
                "All of you are instructed to go to the Time-Gap of Dimension on Rune-Midgarts!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Just go there.", "This is a restricted area."])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Will you go to the dimensional rift right now?"])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["If you want,", "I can send you to the Time-Gap of Dimension."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Send me there.:I will go there myself.")])? {
            1 => {
                ctx.lines_as(
                    "Guard",
                    args!["Ok, I will send you to the dimensional rift. Once you get there, just find the head of the alliance."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("moc_fild20"), Val::from(342), Val::from(179)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Guard", args!["You are something of an adventurer... Ok. I got it..."])?;
                ctx.next()?;
                ctx.lines_as("Guard", args!["Once you get there, just find the head of the alliance."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ep13_ryu").get()?.number()? > 8 {
        ctx.lines_as(
            "Guard",
            args!["You've come here", "to register as a member of the three kingdoms?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Enter."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Guard", args!["You are not allowed", "to enter."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn guard_ep13_1_9(ctx: &Ctx) -> Script {
    guard_ep13_1_9_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-Munkenro and Sakaiz don't seem to be getting along very well anymore.-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Member of Alliance", args!["Hm, ok!", "That's right."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_1(ctx: &Ctx) -> Script {
    member_of_alliance_1_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-You should probably head over to the dimensional rift next to Morocc and find the Guard there-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Member of Alliance", args!["The heads of the alliance, Sikaiz and Munkenro..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Member of Alliance",
        args![
            "They have different values,",
            "so that sometimes causes conflict, but nothing serious has happened so far."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["Friendship is the most valuable in this world."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_2(ctx: &Ctx) -> Script {
    member_of_alliance_2_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-You're still here? You're a member of the alliance now so you should head to the dimensional rift next to Morocc and find the Guard there-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Member of Alliance",
        args![
            "I know we are all to go to the dimensional rift. That's why we are allied strongly. But we gather here... so far from there..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Member of Alliance",
        args!["I think we'd better gather in Prontera on Rune-Midgarts, where I am living."],
    )?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["Or just warp for commuting.", "I would hope so."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_3(ctx: &Ctx) -> Script {
    member_of_alliance_3_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-I'm sure they'll settle their little dispute somehow. You should be heading to the dimensional rift next to Morocc and find the Guard there-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Member of Alliance", args!["Hm..."])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["Hm..."])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["Whoosh..."])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["I am concentrating now!!", "Don't worry!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_4(ctx: &Ctx) -> Script {
    member_of_alliance_4_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-Maybe Munk is a better leader instead of Sakaiz. Anyways you should be heading to the dimensional rift next to Morocc and find the Guard there-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Member of Alliance",
        args!["The head of the alliance, Sikaiz... doesn't look healthy."],
    )?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["But his upright eyes", "are quite reliable."])?;
    ctx.next()?;
    ctx.lines_as(
        "Member of Alliance",
        args!["Munkenro looks so willing to do anything for success. As well as he also seems flexible..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_5(ctx: &Ctx) -> Script {
    member_of_alliance_5_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-I'm so bored!!! You should be heading to the dimensional rift next to Morocc and find the Guard there. It's probably less boring there than it is here-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Member of Alliance", args!["I am almost dying of boredom!"])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["So boring!"])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["I am not in a good mood."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_6(ctx: &Ctx) -> Script {
    member_of_alliance_6_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-We haven't even started and we're already bickering. Just ignore the politics and head to the dimensional rift next to Morocc and find the Guard there-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Member of Alliance",
        args!["Three kingdoms for world peace...", "Will it be successful?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_7(ctx: &Ctx) -> Script {
    member_of_alliance_7_body(ctx, Vec::new()).map(|_| ())
}

fn member_of_alliance_8_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.mes("-You shouldn't stay around here and listen to all the complaining. You should be heading to the dimensional rift next to Morocc and find the Guard there-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Member of Alliance", args!["Why is there so much complaining?"])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["I'm going along for the benefit of my country."])?;
    ctx.next()?;
    ctx.lines_as("Member of Alliance", args!["I wonder if the managers are all talk."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_of_alliance_8(ctx: &Ctx) -> Script {
    member_of_alliance_8_body(ctx, Vec::new()).map(|_| ())
}

fn alliance_manager_ra_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 16 {
        ctx.lines_as(
            "Manager",
            args!["With blessings from the Goddess Freya! We three kingdoms will know success! It's all we want. ...Khkhkhkh..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 15 {
        ctx.lines_as(
            "Manager",
            args!["What brings you here, if you won't worship Freya? Just go away."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args!["Hm, you've come here to report the status of the alliance? Ok, let me listen."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args!["Hm, everything is going well", "and almost prepared for a good start... Hm-hmm..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args!["I hate to meet people from Rune-Midgarts, but I can't avoid it."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manager",
            args!["We're all together for this mission now... They would sacrifice for Freya."],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10071), Val::from(10072)])?;
        ctx.var("ep13_ryu").set(Val::from(16))?;
        ctx.lines_as(
            "Manager",
            args!["Now you can go back.", "Just say I received the report of the status well."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Manager", args!["Goddess Freya, keep us forever!!"])?;
    ctx.next()?;
    ctx.lines_as("Manager", args!["Ash-Vacuum...", "It will be mine soon...", "Khkhkh!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn alliance_manager_ra(ctx: &Ctx) -> Script {
    alliance_manager_ra_body(ctx, Vec::new()).map(|_| ())
}

fn officer_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_ryu").get()? == 20 {
        ctx.lines_as(
            "Officer",
            args!["Why don't you go back there?", "Are you afraid to go back?", "Khkhkh!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer",
            args![
                "Just go back to the meeting room.",
                "Your allies are there together.",
                "Khkhkh!",
                "Nevermind.",
                "Bye."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 19 {
        ctx.lines_as("Officer A", args!["So why do you come to me?", "What's wrong?"])?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["Got any business?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args![
                "I think not.",
                "You come to tell me that you're leaving for the mission.",
                "Am I right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["You were sent by Sikaiz...", "Right?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args!["Ok, I got your report.", "Just go back there.", "Good luck."],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["Just do your best", "...or whatever you do..."])?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10075), Val::from(10076)])?;
        ctx.var("ep13_ryu").set(Val::from(20))?;
        ctx.lines_as("Officer A", args!["Just return there. Good job."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()? == 18 {
        ctx.lines_as("Officer A", args!["Sikaiz isn't understandable...", "I can't help it..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args![
                "His voice for peace is",
                "only his own way.",
                "Three kingdoms for one mission...",
                "No peace...",
                "We even get no advantage."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args!["We can't leave them as they are now. Though I am the leader among them, I'm just a figurehead."],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer A", args!["Is there any way to change their ways?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args!["If they changed,", "we wouldn't get in trouble.", "But he is not compromising."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args!["So what??", "We wouldn't get our interests,", "and just do nothing for ourselves?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Officer B", args!["There's always Munkenro."])?;
        ctx.next()?;
        ctx.lines_as("Officer A", args!["So what...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args!["As honest as Sikaiz...", "As flexible as Sikaiz...", "But easy to go over him..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer B",
            args!["We can't cut him out,", "so just take the lead from Munkenro. It's easier!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Officer A",
            args![
                "Oh!!",
                "That's a good idea.",
                "Anyway...",
                "Some visitors are coming.",
                "Let's talk about it later."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(10074), Val::from(10075)])?;
        ctx.var("ep13_ryu").set(Val::from(19))?;
        ctx.lines_as("Officer B", args!["Um, what's your business?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Officer A",
        args!["For our own interest,", "we need to abandon other kingdoms."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn officer_a(ctx: &Ctx) -> Script {
    officer_a_body(ctx, Vec::new()).map(|_| ())
}
