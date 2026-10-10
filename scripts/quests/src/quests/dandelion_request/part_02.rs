use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MiyaStep {
    Start,
    OnTouch,
}

fn miya_run(ctx: &Ctx, mut step: MiyaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MiyaStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
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
                if !(ctx.var("BaseLevel").get()?.number()? > 59) {
                    ctx.lines_as(
                        "Girl",
                        args![
                            "Hey little kid...",
                            "What are you doing",
                            "hanging around here?",
                            "You know children like",
                            "you have been missing",
                            "from this city, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Girl",
                        args![
                            "You had better leave",
                            "and play someplace safer,",
                            "like Prontera or Alberta.",
                            "Run along now and stay",
                            "away from strangers, okay?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?)
                    || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?))
                {
                    ctx.lines_as(
                        "Girl",
                        args![
                            "...............................",
                            "Huh? What the hell",
                            "do you want? Buzz off",
                            "and don't talk to me,",
                            "unless you want a fat lip!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("mao_request").get()?.is_true()) {
                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?) {
                        ctx.lines_as(
                            "Miya",
                            args![
                                "Wait! Wait up! You're",
                                "a Taekwon Master, right?",
                                "Remember Moohyun? He",
                                "helped you job change...?",
                                "He wrote this letter for you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                ((Val::from("Listen, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                                "He's asking you to help",
                                "the Assassin Guild on a",
                                "special mission. I think",
                                "it's pretty serious business."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "If you're interested in",
                                "assisting the Assassins,",
                                "then meet with your contact to",
                                "the west, somewhere around",
                                "here in this city. Well, that's",
                                "all he told me, so I'll seeya~"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF_HIGH")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STALKER")?))
                    {
                        ctx.lines_as(
                            "Miya",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                "I've got some important",
                                "work for you. The Assassin",
                                "Guild actually asked the Thief",
                                "and Rogue Guilds for help!",
                                "Can you believe that?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "Anyway, all of us ruffians",
                                "are supposed to help out if",
                                "we can. Here, take this letter",
                                "of recommendation and talk to",
                                "Jack, our contact west in this city. You better go right away..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                    {
                        ctx.lines_as(
                            "Miya",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "The Assassin Guild Master",
                                "wants you right away. Do you",
                                "know where the secret pub is?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ah, h-hi, Miya,", "it's been a while.", "What secret pub?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "I figured you might not",
                                "have heard of it. Just talk",
                                "to Jack, west of the Oasis",
                                "in this city, and he'll let you",
                                "in. From now on, you'll be",
                                "spending plenty of time there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "That pub is probably the",
                                "only place where Assassins",
                                "like us can relax. But yeah,",
                                "this mission is pretty major.",
                                "We'll need all the help that",
                                "we can possibly get..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_request").get()? == 1 {
                        if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?) {
                            ctx.lines_as(
                                "Miya",
                                args![
                                    "If you're interested in",
                                    "assisting the Assassins,",
                                    "then meet with your contact to",
                                    "the west, somewhere around",
                                    "here in this city. Well, that's",
                                    "all he told me, so I'll seeya~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
                            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF_HIGH")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STALKER")?))
                        {
                            ctx.lines_as(
                                "Miya",
                                args![
                                    "There's supposed to be",
                                    "some private pub to the",
                                    "west of the Oasis in this",
                                    "city. Ah, you're supposed",
                                    "to talk to our contact, Jack,",
                                    "who's right outside of the pub."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                        {
                            ctx.lines_as(
                                "Miya",
                                args![
                                    "Look west of the Oasis",
                                    "here in Morocc to find Jack,",
                                    "who will let you go inside the",
                                    "pub. Our guildmaster will be",
                                    "waiting for you inside..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ((ctx.var("mao_request").get()?.number()? > 2 && ctx.var("mao_request").get()?.number()? < 27)
                            || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 125))
                        {
                            if ((((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?)
                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?))
                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF_HIGH")?))
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STALKER")?))
                            {
                                ctx.lines_as(
                                    "Miya",
                                    args![
                                        "So how's it been going?",
                                        "I think the assignment you're",
                                        "working on must be pretty",
                                        "important. I mean, if it's",
                                        "giving Kidd and Lin trouble..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                            {
                                ctx.lines_as(
                                    "Miya",
                                    args![
                                        "How's your current",
                                        "assignment coming along?",
                                        "The guildmaster must have",
                                        "given you something really",
                                        "weird or almost impossible",
                                        "to do again, didn't he?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            ctx.lines_as(
                                "Miya",
                                args![
                                    "It's been way too",
                                    "quiet lately. I really",
                                    "hope that something crazy",
                                    "happens soon. Otherwise,",
                                    "I think I'll die of boredom!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                step = MiyaStep::OnTouch;
                continue 'machine;
            }
            MiyaStep::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
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
                if ctx.var("BaseLevel").get()?.number()? > 59 && !(ctx.var("mao_request").get()?.is_true()) {
                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?) {
                        ctx.lines_as(
                            "Miya",
                            args![
                                "Wait! Wait up! You're",
                                "a Taekwon Master, right?",
                                "Remember Moohyun? He",
                                "helped you job change...?",
                                "He wrote this letter for you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                ((Val::from("Listen, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                                "He's asking you to help",
                                "the Assassin Guild on a",
                                "special mission. I think",
                                "it's pretty serious business."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "If you're interested in",
                                "assisting the Assassins,",
                                "then meet with your contact to",
                                "the west, somewhere around",
                                "here in this city. Well, that's",
                                "all he told me, so I'll seeya~"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF_HIGH")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STALKER")?))
                    {
                        ctx.lines_as(
                            "Miya",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                "I've got some important",
                                "work for you. The Assassin",
                                "Guild actually asked the Thief",
                                "and Rogue Guilds for help!",
                                "Can you believe that?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "Anyway, all of us ruffians",
                                "are supposed to help out if",
                                "we can. Here, take this letter",
                                "of recommendation and talk to",
                                "Jack, our contact west in this city. You better go right away..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                    {
                        ctx.lines_as(
                            "Miya",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "The Assassin Guild Master",
                                "wants you right away. Do you",
                                "know where the secret pub is?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Ah, h-hi, Miya,", "it's been a while.", "What secret pub?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "I figured you might not",
                                "have heard of it. Just talk",
                                "to Jack, west of the Oasis",
                                "in this city, and he'll let you",
                                "in. From now on, you'll be",
                                "spending plenty of time there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Miya",
                            args![
                                "That pub is probably the",
                                "only place where Assassins",
                                "like us can relax. But yeah,",
                                "this mission is pretty major.",
                                "We'll need all the help that",
                                "we can possibly get..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn miya(ctx: &Ctx) -> Script {
    miya_run(ctx, MiyaStep::Start, Vec::new()).map(|_| ())
}

pub fn miya_ontouch(ctx: &Ctx) -> Script {
    miya_run(ctx, MiyaStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Guildsman3Step {
    Start,
    OnTouch,
}

fn guildsman_3_run(ctx: &Ctx, mut step: Guildsman3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guildsman3Step::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
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
                if !(ctx.var("BaseLevel").get()?.number()? > 59) {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Please be careful if you're",
                            "planning to travel through",
                            "Morocc. Recently, there have",
                            "been reports of missing",
                            "children there..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (!(ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?))
                    || !(ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)))
                {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Excuse me, but if you happen",
                            "to encounter any bowmen, or",
                            "people with any archery skill,",
                            "would you ask them to come to",
                            "me? I have an urgent message",
                            "that I should tell them."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("mao_request").get()?.is_true()) {
                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?) {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Ah... Finally!",
                                "Another practitioner",
                                "of Taekwon Do! You're...",
                                "Let's see... You must",
                                ((Val::from("be ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", yes?"))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Mr. Phoenix has charged",
                                "me with the task of notifying",
                                "all Taekwon Do practitioners",
                                "of the request sent to us by",
                                "the Assassin Guild. Apparently,",
                                "they need aid for a mission..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "If you are interested in",
                                "assisting them, please take",
                                "this Letter of Recommendation",
                                "and bring it to the contact from the Assassin Guild in Morocc."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "You can meet the",
                                "Assassin Guildsman near",
                                "some hut on the west side",
                                "of the Oasis inside Morocc.",
                                "Good luck, my friend, and please represent Taekwon Do with pride."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER_HIGH")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SNIPER")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                                "Oh, wow, I'm so lucky to",
                                "have finally found you!",
                                "Listen, I've got a message",
                                "for you from the Icarus Guild."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "It looks like the Assassin",
                                "Guild has formally asked them",
                                "for help in some mission, so",
                                "Icarus has decided to send you.",
                                "I hope you choose to represent",
                                "us and help those Assassins."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Here, please take this",
                                "letter of recommendation",
                                "and meet with our Assassin",
                                "Guild contact to the west",
                                "of the Oasis in Morocc.",
                                "Well, good luck, pal~"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CLOWN")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "I've been looking all over",
                                "for you! I've got a message",
                                "for you from Lalo. You...",
                                "You remember him, right?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Sure, I do! I owe",
                                "Lalo so much... If it",
                                "weren't for him, I'd never",
                                "have job changed to a Bard",
                                "in the first place. So what",
                                "exactly does he need?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Well, I'm not too sure. All",
                                "I heard was something about",
                                "a request from the Assassin",
                                "Guild and a recommendation",
                                "that you help them. Yeah.",
                                "Here's the letter he wrote."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Just... Just meet with the",
                                "contact from the Assassin",
                                "Guild just west of the Oasis",
                                "inside Morocc. I'm sure he",
                                "can explain everything better."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_GYPSY")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Hey... Th-That face!",
                                "Just like Aile described!",
                                ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", right?")),
                                "This is great, I've been",
                                "looking all over for you!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Aile? Oh, that's right,",
                                "she was there during my",
                                "job change test and helped",
                                "me become a Dancer. Sure,",
                                "I remember her now...",
                                "So what did you need?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Ah, right. From what",
                                "I know, the Assassin Guild",
                                "asked Aile to recommend",
                                "a Dancer that might be able",
                                "to help them in a mission",
                                "of pretty major importance."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Anyway, Aile then sent me",
                                "out to look for you and give",
                                "this letter of recommendation.",
                                "I guess she thinks you'll do",
                                "the best job. So, um, congrats~"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "For now, your cooperation",
                                "with the Assassin Guild is",
                                "probably your biggest priority.",
                                "Head over to Morocc and look",
                                "for your contact to the west",
                                "of the Oasis inside town."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Alright, then.",
                                "Good luck! Ah, I almost",
                                "forgot. Aile wants you to",
                                "remember that you'll be a",
                                "representative of Dancers",
                                "everywhere, so do a good job!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_request").get()? == 1 {
                        if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?) {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "You can meet the",
                                    "Assassin Guildsman near",
                                    "some hut on the west side",
                                    "of the Oasis inside Morocc.",
                                    "Good luck, my friend, and please represent Taekwon Do with pride."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)
                            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER_HIGH")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SNIPER")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Please take your",
                                    "letter of recommendation",
                                    "and meet with our Assassin",
                                    "Guild contact to the west",
                                    "of the Oasis in Morocc.",
                                    "Well, good luck, pal~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CLOWN")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Just... Just meet with the",
                                    "contact from the Assassin",
                                    "Guild just west of the Oasis",
                                    "inside Morocc. I'm sure he",
                                    "can explain everything better."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_GYPSY")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "For now, your cooperation",
                                    "with the Assassin Guild is",
                                    "probably your biggest priority.",
                                    "Head over to Morocc and look",
                                    "for your contact to the west",
                                    "of the Oasis inside town."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Alright, then.",
                                    "Good luck! Ah, I almost",
                                    "forgot. Aile wants you to",
                                    "remember that you'll be a",
                                    "representative of Dancers",
                                    "everywhere, so do a good job!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ((ctx.var("mao_request").get()?.number()? > 2 && ctx.var("mao_request").get()?.number()? < 27)
                            || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 125))
                        {
                            if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?) {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "I'm glad to see that you",
                                        "are working well together",
                                        "with the Assassins. But don't",
                                        "drop your guard, not until the",
                                        "mission is accomplished."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF_HIGH")?))
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STALKER")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "It's good to see that",
                                        "you showing pretty good",
                                        "teamwork with the Assassins.",
                                        "I hope that you're giving them",
                                        "a good impression of bowmen..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CLOWN")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "Lalo will be pleased to",
                                        "know that you've been an",
                                        "asset to the Assassins.",
                                        "But until the mission is",
                                        "accomplished, remember that",
                                        "you're representing all Bards!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_GYPSY")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "I think Aile will be",
                                        "very happy to know that",
                                        "the Assassins are pleased",
                                        "with your assistance. But",
                                        "until the mission is over,",
                                        "be careful out there, okay?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "It's a very beautiful day",
                                    "today--let's drink in as much",
                                    "of the beauty of the day as we",
                                    "can. Good days and bad days",
                                    "pass, but each one is unique",
                                    "in itself... just because."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                step = Guildsman3Step::OnTouch;
                continue 'machine;
            }
            Guildsman3Step::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
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
                if ctx.var("BaseLevel").get()?.number()? > 59 && !(ctx.var("mao_request").get()?.is_true()) {
                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?) {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Ah... Finally!",
                                "Another practitioner",
                                "of Taekwon Do! You're...",
                                "Let's see... You must",
                                ((Val::from("be ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", yes?"))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Mr. Phoenix has charged",
                                "me with the task of notifying",
                                "all Taekwon Do practitioners",
                                "of the request sent to us by",
                                "the Assassin Guild. Apparently,",
                                "they need aid for a mission..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "If you are interested in",
                                "assisting them, please take",
                                "this Letter of Recommendation",
                                "and bring it to the contact from the Assassin Guild in Morocc."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "You can meet the",
                                "Assassin Guildsman near",
                                "some hut on the west side",
                                "of the Oasis inside Morocc.",
                                "Good luck, my friend, and please represent Taekwon Do with pride."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER_HIGH")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SNIPER")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                                "Oh, wow, I'm so lucky to",
                                "have finally found you!",
                                "Listen, I've got a message",
                                "for you from the Icarus Guild."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "It looks like the Assassin",
                                "Guild has formally asked them",
                                "for help in some mission, so",
                                "Icarus has decided to send you.",
                                "I hope you choose to represent",
                                "us and help those Assassins."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Here, please take this",
                                "letter of recommendation",
                                "and meet with our Assassin",
                                "Guild contact to the west",
                                "of the Oasis in Morocc.",
                                "Well, good luck, pal~"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CLOWN")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "I've been looking all over",
                                "for you! I've got a message",
                                "for you from Lalo. You...",
                                "You remember him, right?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Sure, I do! I owe",
                                "Lalo so much... If it",
                                "weren't for him, I'd never",
                                "have job changed to a Bard",
                                "in the first place. So what",
                                "exactly does he need?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Well, I'm not too sure. All",
                                "I heard was something about",
                                "a request from the Assassin",
                                "Guild and a recommendation",
                                "that you help them. Yeah.",
                                "Here's the letter he wrote."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Just... Just meet with the",
                                "contact from the Assassin",
                                "Guild just west of the Oasis",
                                "inside Morocc. I'm sure he",
                                "can explain everything better."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_GYPSY")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Hey... Th-That face!",
                                "Just like Aile described!",
                                ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", right?")),
                                "This is great, I've been",
                                "looking all over for you!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Aile? Oh, that's right,",
                                "she was there during my",
                                "job change test and helped",
                                "me become a Dancer. Sure,",
                                "I remember her now...",
                                "So what did you need?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Ah, right. From what",
                                "I know, the Assassin Guild",
                                "asked Aile to recommend",
                                "a Dancer that might be able",
                                "to help them in a mission",
                                "of pretty major importance."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Anyway, Aile then sent me",
                                "out to look for you and give",
                                "this letter of recommendation.",
                                "I guess she thinks you'll do",
                                "the best job. So, um, congrats~"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "For now, your cooperation",
                                "with the Assassin Guild is",
                                "probably your biggest priority.",
                                "Head over to Morocc and look",
                                "for your contact to the west",
                                "of the Oasis inside town."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Alright, then.",
                                "Good luck! Ah, I almost",
                                "forgot. Aile wants you to",
                                "remember that you'll be a",
                                "representative of Dancers",
                                "everywhere, so do a good job!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guildsman_3(ctx: &Ctx) -> Script {
    guildsman_3_run(ctx, Guildsman3Step::Start, Vec::new()).map(|_| ())
}

pub fn guildsman_3_ontouch(ctx: &Ctx) -> Script {
    guildsman_3_run(ctx, Guildsman3Step::OnTouch, Vec::new()).map(|_| ())
}

fn sharp_looking_kid_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
    {
        if ctx.var("mao_request").get()? == 1 {
            ctx.lines_as(
                "Jack",
                args![
                    "I've been waiting for",
                    "you. Erm, why don't you",
                    "go inside? Ah, this must",
                    "be your first time here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jack",
                args![
                    "Hey, you shouldn't worry,",
                    "this bar is just for people",
                    "like us. Besides, you'll be",
                    "coming more often from now",
                    "on. Anyway, the Bar Master",
                    "is waiting for you inside."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("mao_request").get()?.number()? > 1 {
            ctx.lines_as(
                "Jack",
                args![
                    "Hey, it looks like",
                    "you've been keeping",
                    "busy. Good for you~",
                    "Hey, I'll see you around."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Jack",
                args![
                    "Whoa, whoa...",
                    "Wait. I dunno if I can",
                    "let you in just yet. Ah,",
                    "whatever, you're one of",
                    "us. If it's just for a drink,",
                    "I shouldn't get in trouble..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("mao_request").get()? == 1 && ctx.call(Function::CountItem, vec![Val::from(7416)])?.is_true()) {
        ctx.lines_as(
            "Jack",
            args![
                "Oh, hey, you must be the",
                "one I've been waiting for.",
                "Yeah, alright, come on in,",
                "and don't forget to show",
                "the letter of recommendation",
                "to the Bar Master, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()?.number()? > 1 {
        ctx.lines_as(
            "Jack",
            args![
                "It's a little weird since",
                "you're not really part of",
                "our guild, but you're allowed",
                "to enter our secret hideaway",
                "from now on. But don't you",
                "dare tell anybody about this!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Jack",
            args![
                "What? I don't have",
                "anything to talk to",
                "you about. Move along,",
                "adventurer, and don't",
                "bother me anymore."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sharp_looking_kid(ctx: &Ctx) -> Script {
    sharp_looking_kid_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobar1Step {
    Start,
    OnTouch,
}

fn maobar1_run(ctx: &Ctx, mut step: Maobar1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_mao_pass = Val::from(0);
    'machine: loop {
        match step {
            Maobar1Step::Start => {
                step = Maobar1Step::OnTouch;
                continue 'machine;
            }
            Maobar1Step::OnTouch => {
                if ((ctx.var("mao_request").get()?.is_true() || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?))
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(9), Val::from(94)])?;
                } else {
                    l_mao_pass = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if l_mao_pass.clone() == 2 {
                        ctx.lines_as(
                            "Jack",
                            args![
                                "Oh, man...",
                                "How did you find this",
                                "place? This bar is one of",
                                "the Assassin Guild's most",
                                "closely guarded secrets! Well, the cat's out of the bag..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jack",
                            args![
                                "But I still can't",
                                "let you in. No matter",
                                "what. Yeah, my mind's",
                                "made up. But if you really",
                                "want to go in, maybe you",
                                "could, I dunno... distract me."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Pay 1,000 Zeny"), Val::from("Cancel")])? {
                            1 => {
                                if ctx.var("Zeny").get()?.number()? > 999 {
                                    ctx.lines_as(
                                        "Jack",
                                        args![
                                            "Oh man... I'm sooo",
                                            "tired. Let me take a",
                                            "freakin' 20 second yawn.",
                                            "*Yaaaaaaaaaaaaaaaaaaaaaaaaaawn*"
                                        ],
                                    )?;
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(9), Val::from(94)])?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Jack",
                                    args![
                                        "What the...?",
                                        "That's not a lot of",
                                        "money at all. I doubt",
                                        "you can even afford just",
                                        "one drink at this joint.",
                                        "Just drink at home, willya?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Jack",
                                    args![
                                        "Hmm...",
                                        "You... You didn't know",
                                        "what I meant, did you?",
                                        "Oh well. You better scram."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(45), Val::from(103)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    ctx.lines_as(
                        "Jack",
                        args![
                            "Hey...",
                            "You're crowding my",
                            "personal space. Step",
                            "back before I get violent.",
                            "You wouldn't like me when",
                            "I'm violent, ya creep."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(45), Val::from(103)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maobar1(ctx: &Ctx) -> Script {
    maobar1_run(ctx, Maobar1Step::Start, Vec::new()).map(|_| ())
}

pub fn maobar1_ontouch(ctx: &Ctx) -> Script {
    maobar1_run(ctx, Maobar1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobar5Step {
    Start,
    OnTouch,
}

fn maobar5_run(ctx: &Ctx, mut step: Maobar5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maobar5Step::Start => {
                step = Maobar5Step::OnTouch;
                continue 'machine;
            }
            Maobar5Step::OnTouch => {
                if ctx.var("mao_request").get()?.number()? > 1 {
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(61), Val::from(50)])?;
                } else {
                    ctx.lines_as(
                        "Litheron",
                        args![
                            "Whoa, you're not",
                            "allowed to be in here.",
                            "Hey, master! Do you",
                            "know this guy?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("prt_curse").get()? == 24 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I, um, I'm looking",
                                "for somebody named",
                                "Marjana? I learned",
                                "that she's around",
                                "here somewhere?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Litheron",
                            args!["Marjana? How did you", "know that? Hey master,", "what do I do with this guy?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Master",
                            args![
                                "Hmm. I sense no ill intent",
                                "from this adventurer. I've",
                                "also heard a rumor that the",
                                "Prontera Church needs to",
                                "investigate poison for",
                                "some reason."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Master",
                            args![
                                "However, there is no",
                                "way to tell if this person",
                                "has been sent by Prontera",
                                "Church. I suppose whether",
                                "this person can enter is",
                                "really up to you, Litheron."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Litheron",
                            args![
                                "Hah! Did you hear that?",
                                "Alright, how about this?",
                                "I'll let you in if you buy me",
                                "a drink. Besides, you can't risk making trouble here: this place",
                                "is full of deadly Assassins."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure"), Val::from("Why should I?!")])? {
                            1 => {
                                if ctx.var("Zeny").get()?.number()? > 999 {
                                    ctx.lines_as(
                                        "Litheron",
                                        args![
                                            "Heh, that's what",
                                            "I'm talking about!",
                                            "Hey, bartender! Gimme",
                                            "the usual! I like your",
                                            "style, adventurer..."
                                        ],
                                    )?;
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Litheron",
                                        args![
                                            "Alright, you can come",
                                            "on in. But don't you dare",
                                            "breathe a word about this",
                                            "bar to another living soul."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(61), Val::from(50)])?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Litheron",
                                    args![
                                        "Huh...",
                                        "Oh, you don't even",
                                        "have enough zeny to",
                                        "buy water here. Oh boy...",
                                        "If you really want to enter,",
                                        "make sure you have the cash!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(85), Val::from(77)])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Litheron",
                                    args![
                                        "Not the savvy type,",
                                        "are you...? Fine, fine.",
                                        "If you're not gonna do",
                                        "me any favors, then why",
                                        "should I help you? Go away!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(85), Val::from(77)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    ctx.lines_as(
                        "Master",
                        args!["Not at all.", "Make sure that this", "one isn't allowed", "to enter."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Litheron",
                        args![
                            "You heard the man.",
                            "You better retreat",
                            "from this place while",
                            "you still have legs..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(85), Val::from(77)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maobar5(ctx: &Ctx) -> Script {
    maobar5_run(ctx, Maobar5Step::Start, Vec::new()).map(|_| ())
}

pub fn maobar5_ontouch(ctx: &Ctx) -> Script {
    maobar5_run(ctx, Maobar5Step::OnTouch, Vec::new()).map(|_| ())
}

fn idle_knight_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Litheron",
        args![
            "What's the matter?",
            "Oh, I see... You're",
            "probably wondering why",
            "a Knight like me is in a",
            "secret Assassin's pub."
        ],
    )?;
    if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_LORD_KNIGHT")?))
    {
        ctx.mes("What about you, huh?")?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Litheron",
        args![
            "It just so happens that",
            "I'm really old friends with",
            "the owner of the pub. I do",
            "favors for him, and he does",
            "favors for me. That's how",
            "we do it around here."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn idle_knight(ctx: &Ctx) -> Script {
    idle_knight_body(ctx, Vec::new()).map(|_| ())
}

fn bar_master_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_maodrink = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(12112), Val::from(1)])? != 1 {
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
    if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
    {
        ctx.lines_as("Master", args!["Welcome to my", "little pub. What", "will you be having?"])?;
    } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF_HIGH")?))
        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?))
        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STALKER")?))
    {
        ctx.lines_as(
            "Master",
            args![
                "Huh. How did",
                "a ruffian like you",
                "get in here? Well, if",
                "Jack let you in, I guess",
                "you must be alright. So",
                "what do you wanna drink?"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Master",
            args![
                "This place is supposed",
                "to be Assassins only, but",
                "I guess I'll make an exception",
                "for you. Hell, you must have",
                "had a rough time just getting",
                "in. So what are you drinking?"
            ],
        )?;
    }
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Order a Drink"), Val::from("Ask about Mission"), Val::from("Cancel")],
        )?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Master",
                args![
                    "In this place, I only",
                    "serve two drinks. You",
                    "want a Tropical Sograt or",
                    "a Vermilion on the Beach?"
                ],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(
                    ctx,
                    &[
                        Val::from("Tropical Sograt"),
                        Val::from("Vermilion on the Beach"),
                        Val::from("Do you have anything cheaper?"),
                    ],
                )?);
                let mut matched2 = false;
                let no_case2 = !subject2.loosely_equals(&Val::from(1))
                    && !subject2.loosely_equals(&Val::from(2))
                    && !subject2.loosely_equals(&Val::from(3));
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    l_maodrink = Val::from(1);
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.call(Function::CheckWeight, vec![Val::from(12112), Val::from(1)])? != 1 {
                        ctx.lines_as(
                            "Master",
                            args![
                                "Hey, why did you bring",
                                "so much stuff with you?",
                                "I can't give you anything",
                                "to drink if you don't have",
                                "the room to carry it around.",
                                "Clear out your inventory!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                    {
                        if ctx.var("Zeny").get()?.number()? < 800 {
                            ctx.lines_as(
                                "Master",
                                args![
                                    "You know...",
                                    "I was going to charge you",
                                    "800 zeny for this drink, but",
                                    "it looks like you can't afford",
                                    "it right now. Come back when",
                                    "you have the zeny, alright?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Master",
                            args![
                                "Most bartenders would",
                                "usually charge 1,000 zeny",
                                "for this stuff, but I'll only",
                                "ask you for 800. Enjoy it,",
                                "my friend, and drink it as",
                                "deeply as you would life."
                            ],
                        )?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(800))?))?;
                        if l_maodrink.clone().is_true() {
                            ctx.call(Function::GetItem, vec![Val::from(12112), Val::from(1)])?;
                        } else {
                            ctx.call(Function::GetItem, vec![Val::from(12113), Val::from(1)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ctx.var("Zeny").get()?.number()? < 1000 {
                        ctx.lines_as(
                            "Master",
                            args![
                                "Hey, you don't have",
                                "the zeny for this drink.",
                                "Make sure you come back",
                                "with 1,000 zeny if you want",
                                "me to fix you something, okay?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Master",
                        args![
                            "For Assassins, I charge",
                            "800 zeny, but for you, I'm",
                            "gonna charge 1,000. No",
                            "hard feelings, but you're",
                            "already lucky to be here.",
                            "Hey, enjoy your drink~"
                        ],
                    )?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                    if l_maodrink.clone().is_true() {
                        ctx.call(Function::GetItem, vec![Val::from(12112), Val::from(1)])?;
                    } else {
                        ctx.call(Function::GetItem, vec![Val::from(12113), Val::from(1)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Master",
                        args![
                            "What are you...?",
                            "I just told you",
                            "I only serve two",
                            "drinks. If you want",
                            "water, you oughtta",
                            "go someplace else."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("mao_request").get()? == 1 {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Master",
                        args![
                            "Ah, you must be here on",
                            "the orders of the guildmaster.",
                            "Alright, just go inside. Hey,",
                            "but be careful. This won't be",
                            "easy like most assignments..."
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(2))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Master",
                    args![
                        "Mission...?",
                        "Come on, what are",
                        "you talking about?",
                        "Stop talking crazy and",
                        "order something to drink."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Show Letter of Recommendation")])? {
                    1 => {}
                    _ => {}
                }
                if ctx.call(Function::CountItem, vec![Val::from(7416)])?.is_true() {
                    ctx.lines_as(
                        "Master",
                        args![
                            "Ah, I see you that you",
                            "came prepared. Sorry about",
                            "that, but we've got protocols",
                            "about secrecy that I'm sworn",
                            "to upkeep. Alright, do see that",
                            "Knight over there? Litheron!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Master",
                        args![
                            "Hey Litheron! Let this",
                            "guy go through, willya?",
                            "Yeah, just talk to Litheron,",
                            "and he'll show you where to go."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Litheron",
                        args!["Hey, since when did", "I become your doorman?!", "Eh, ah well. You got it."],
                    )?;
                    ctx.var("mao_request").set(Val::from(2))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args!["^3355FFYou forgot to bring your", "Letter of Recommendation.^000000"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Master",
                    args![
                        "Alright...",
                        "You're looking way",
                        "too suspicious now.",
                        "Maybe I'm wrong, but",
                        "I don't like taking chances.",
                        "Boys, show this guy the door!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(45), Val::from(106)])?;
                return Err(Stop::End);
            } else if ctx.var("mao_request").get()? == 2 {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Master",
                        args![
                            "Just go through the",
                            "door behind Litheron.",
                            "You know the drill, so",
                            "hurry it up, willya?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Master",
                    args![
                        "Go through the door",
                        "behind Litheron if you",
                        "wanna learn more about",
                        "your secret mission."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("mao_request").get()?.number()? > 2 {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Master",
                        args![
                            "How's your assignment",
                            "coming along? Sometimes,",
                            "you need to relax and take a",
                            "break. When that time comes,",
                            "I'll have a drink ready for you."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Master",
                    args![
                        "How are you doing",
                        "with the mission? It's",
                        "good to see you getting",
                        "along with the Assasssins.",
                        "Anyway, best of luck, pal."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("mao_request").get()? == 30 || ctx.var("mao_request").get()? == 128) {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Master",
                        args!["Hey, thanks for all", "of your hard work, pal.", "I'll see you around."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Master",
                    args![
                        "Thanks for all of your",
                        "help, even if you did go",
                        "through a little more trouble",
                        "than you should have. Anyway,",
                        "you seem alright, so go ahead and come back whenever you want."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Master",
                        args![
                            "So you haven't been",
                            "assigned any missions",
                            "from this joint, eh? By the",
                            "way, you like this place?",
                            "I happen to really love",
                            "this little pub of mine."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Master",
                        args![
                            "If you want, I'll recommend",
                            "you to the guildmaster for",
                            "a mission. Until then, you've",
                            "got to focus on your training.",
                            "And until you train... Why",
                            "don't you enjoy a drink?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Master",
                    args![
                        "Mission...?",
                        "Come on, what are",
                        "you talking about?",
                        "Stop talking crazy and",
                        "order something to drink."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Master", args!["..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn bar_master(ctx: &Ctx) -> Script {
    bar_master_body(ctx, Vec::new()).map(|_| ())
}
