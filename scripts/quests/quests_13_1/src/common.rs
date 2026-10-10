use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum HeadOfTheAllianceMooStep {
    Start,
    OnInit,
    OnEnable,
    OnReset,
    OnDisable,
    OnMyMobDead,
    OnTimer900000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ResearchOfficialEp131Step {
    Start,
    OnTouch,
    OnInit,
    OnEnable,
    OnDisable,
    OnMeet,
    OnCall,
    OnMyMobDead,
    OnTimer300000,
}

pub(super) fn research_official_ep131_run(ctx: &Ctx, mut step: ResearchOfficialEp131Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ResearchOfficialEp131Step::Start => {
                step = ResearchOfficialEp131Step::OnTouch;
                continue 'machine;
            }
            ResearchOfficialEp131Step::OnTouch => {
                if (ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0
                    || (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500)
                {
                    ctx.lines_as(
                        "United Research Official",
                        args!["How come you've got so much to carry?", "Are you perhaps on training or something?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("ep13_ryu").get()?.number()? > 99 || ctx.var("ep13_start").get()?.number()? > 99) {
                    if ctx.var("ep13_1_rhea").get()?.number()? < 1 {
                        ctx.lines_as("United Research Official", args!["Hmmmm...mmm..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "United Research Official",
                            args!["Oh! Hello there!", "You must be the adventurer from Midgard."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("United Research Official", args!["We've come all the way here to keep track of Satan Morocc, but... Now it seems that's not our priority anymore... Tsk,tsk..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "United Research Official",
                            args![
                                "Well, since our research group has been gathered in such haste, many of our members collide too often.",
                                "I think it's just the process of trial and error, but this really slows things down."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "United Research Official",
                            args!["Alas, the head office keeps pushing me to submit a report... As if I could perform miracles!"],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.next()?;
                        ctx.lines_as("United Research Official", args!["I can't go around and meet those people myself, mediate among troubled people... This is impossible!! Unless...Unless someone helps me with that ! I need... someone..."])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "- He stared at you -",
                            "- with sparkling -",
                            "- eyes as if he were -",
                            "- demanding for help! -"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![".................", "................."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "United Research Official",
                            args!["Eh?! Oh, my... You really shouldn't. You must be extremely busy tracking down Satan Morocc!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "United Research Official",
                            args!["But... if you say so, could I really use your help? Hahaha."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("United Research Official", args!["Those researchers complain a lot these days. They're terribly stressed out and I think somebody should really listen to their problems."])?;
                        ctx.next()?;
                        ctx.lines_as("United Research Official", args!["Especially those dealing with document files that argue with each other so many times! You might wanna start by talking to them."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "United Research Official",
                            args!["Sounds good, huh? Please, you should go now."],
                        )?;
                        ctx.var("ep13_1_rhea").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(8196)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ep13_1_rhea").get()?.number()? > 0 && ctx.var("ep13_1_rhea").get()?.number()? < 10) {
                            ctx.lines_as("United Research Official", args!["Those researchers complain a lot these days. They're terribly stressed out and I think somebody should really listen to their problems."])?;
                            ctx.next()?;
                            ctx.lines_as("United Research Official", args!["Especially those dealing with document files that argue with each other so many times! You might wanna start by talking to them."])?;
                            ctx.next()?;
                            ctx.lines_as("United Research Official", args!["And, don't worry! I'm even willing to share my snacks with you if things go well. I'm not asking this for free, you got that~?"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ep13_1_rhea").get()? == 10 {
                                if ctx.call(Function::CountItem, vec![Val::from(6036)])?.number()? > 0 {
                                    ctx.lines_as("United Research Official", args!["Aha!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("United Research Official", args!["Isn't that an invitation to the meeting?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["And...and I really get to sign this thing and approve it...? Hahaha!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["This, this means I need to get prepared for the meeting! Finally!", "Hahahaha!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["Oh, by the way, did you get a rough idea of the atmosphere of this expedition?"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("United Research Official", args!["Normally, researchers just take their own instruments and find their own tracks of Satan Morocc.", "They don't really need to cooperate with other members."])?;
                                    ctx.next()?;
                                    ctx.lines_as("United Research Official", args!["Howerver, things get different when it comes to document files.", "Those researchers in charge of document files must contact eachother for information, and this just makes for endless work."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["People can't always be perfect, so they stress out and get to arguing..."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("United Research Official", args!["So... Could you do me a favor?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("United Research Official", args!["As you've heard, about the 3 researchers, they don't agree with eachother too often. It's just too obvious that their meeting will turn out a disaster, and the whole research group could break down!"])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["Oh, just thinking about it scares the crap out of me."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["What I'm asking is that you participate in the meeting with them and help them negotiate."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args![
                                            "So, first! Can you go meet those 3 researchers and check on them to prepare for the meeting?"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(6036), Val::from(1)])?;
                                    ctx.var("ep13_1_rhea").set(Val::from(11))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(8198), Val::from(8199)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("United Research Official", args!["How did it go?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["Did you talk to those researchers in charge of document files?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if (ctx.var("ep13_1_rhea").get()?.number()? > 10 && ctx.var("ep13_1_rhea").get()?.number()? < 19) {
                                    ctx.lines_as(
                                        "United Research Official",
                                        args!["Please, you should participate in the meeting and mediate them if anything goes wrong!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("United Research Official", args!["First, you should go meet those 3 researchers and check on them to prepare for the meeting. Hope they're not preparing any weapons though..."])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("ep13_1_rhea").get()? == 19 {
                                        ctx.call(Function::ChangeQuest, vec![Val::from(8205), Val::from(8206)])?;
                                        ctx.lines_as("United Research Official", args!["Ohhh, you came! How generous of you!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args![
                                                "The meeting's about to begin.",
                                                "Whew... I think I'm too tense... My chest hurts... Aaaah."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines(args!["- Somebody's knocking -", "- on the door... -"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Finally, the meeting's starting.", "You should come in."],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Research Official#ep131::OnMeet")])?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args![
                                                "So, everyone's here.",
                                                "Let us begin our meeting.",
                                                "First, I need to hear the report on the process of tracking Satan Morocc."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["Alright. I'm Ryosen from Rune-Midgarts. We're certain that Satan Morocc came to this place through the crack of dimension, however, we failed to find any traces within the area we inspected."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["The Tripartite Union is doing whatever it takes to track down Satan Morocc's whereabouts, but the research in Ash-Vacuum is very limited."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["Supplies, reinforcements as wel as demanded goods are all insufficient, so we can't really go on."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["I've heard the assassins took action in order to find the traces of Satan Morocc, but nothing's been reported so far."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["I'm Hue from the Schwarzwald Republic."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["We're still researching on the crack of dimension that Satan Morocc has come through, and the false data research on the noise created when moving through the crack is currently being corrected."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["The real cause of this problem has yet to be found, which makes it hard for us to provide any reinforcements or other supplies."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Ok. I see. How's it going with the research on Ash-Vacuum?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["Hi, I'm Hansenne from Arunafeltz. We have conclused that his world of Ash-Vacuum is nothing related to the continent of Midgard, as it is placed in a totally indipendent world dimension."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["The environment of the fields on the East and West where we can work on the research has nothing in common with the environment within the boundrary of the Tripartite Union's army post."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hansenne",
                                            args!["I also have been reported that the ecosystem in the researched are is different."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["We could not go further with the research on the ecosystem due to some violent living creatures which have never been found on the Midgard continent."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["So... All in all, nothing has been clearly found. Any problems among the members of the United Research Group?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["......................"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["......................"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["......................"])?;
                                        ctx.next()?;
                                        ctx.mes("- The room is in awkward silence for a short while... -")?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Ha..... Hahahaha...", "... Then, anything you need to request, or suggest, perhaps? Anything you want to complain about?"])?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "- Ryosen, who was fidgeting with -",
                                            "- his files, suddenly stops and -",
                                            "- studies the others. Then, he takes -",
                                            "- out something from the bottom of -",
                                            "- the chair and puts it on the table. -"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["I'm.... I... The document file..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Aaaaaahahhhhhhhhhhcckk!!!!!"])?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "- Hue, shocked by the sudden",
                                            "scream, stands up frantically and",
                                            "accidentally hits and drops the",
                                            "object which Ryosen had taken out. -"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Kkkkhaaaaaaahhhhhhhh!!!", "It's the Thief Bug!!!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "- As the Official screamed, all the",
                                            "researchers of the union started to",
                                            "scream as well, and rushed out of",
                                            "the room. -"
                                        ])?;
                                        ctx.var("ep13_1_rhea").set(Val::from(20))?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Research Official#ep131::OnCall")])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Warp, vec![Val::from("mid_camp"), Val::from(165), Val::from(236)])?;
                                        return Err(Stop::End);
                                    } else if ctx.var("ep13_1_rhea").get()? == 20 {
                                        ctx.lines_as("United Research Official", args!["Hhmm, hmmmm...", "I really don't know what to say.", "I mean... I'm totally OK with other monsters, mutants or whatever, but that black evil thing... Oh, I can't stand that one..."])?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                        ctx.next()?;
                                        ctx.lines(args!["- Knock, knock-", "- The researchers come back into the room. -"])?;
                                        ctx.next()?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Research Official#ep131::OnMeet")])?;
                                        ctx.lines_as(
                                            "Ryosen",
                                            args![
                                                "Uh-hmm! Sorry about the mess.",
                                                "I... Something urgent came up...",
                                                "............Aaaaaaaaahhhhhhhhh!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Whhhhhhaaattt!!!! What is it!!?", "The Thief Bug, Again!!!???"],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes(
                                            "- Except for Ryosen, the rest spring out of their seats and run hurriedly for the door. -",
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["My... my strawberry cake!!!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Ryosen picks up his strawberry cake, all squashed with some fingerprints. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ryosen",
                                            args!["Stop! Freeze, everyone!", "Move one more time and you'll be considered guilty!!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ryosen",
                                            args![
                                                "This cake was just fine before that evil thing appeared, and there were only 4 of us here!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ryosen",
                                            args!["One of you must be guilty of ruining my Rune-Midgarts' Strawberry Cake!!!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["I can't help checking your hands! This must be a fingerprint! It's loud and clear!! Hmm, I bet this is the mark of an index finger!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["Wait a minute! Wait! Stop treating us like criminals! What is wrong with you, it's only a cake!? Don't you think you're going a little too far?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["Whatttttt!? Only a cake????", "Aha, yeah, I just remembered. You people from Schwarzwald don't ever appreciate food at all, right?"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["Especially when the food is NOT yours, huh? That's why you people sneak a taste of other people's food like my strawberry cake!!!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["Hmm, are you talking about Dusty?", "He was expelled right after the incident and his job was taken away! I told you several times already!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["Ugh... You guys, I think you should calm down... This is ridiculous."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["Ughhhhh! It was I who did that!!!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hansenne", args!["Sorry, I was just too shocked by that thief bug and accidentally touched your cake with my finger!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["...Huh? No. Wait..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ryosen",
                                            args![
                                                "You again?!",
                                                "I can't belive how careless you are all the time, like with the requested data and all!!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hansenne",
                                            args![
                                                "Ah, I forgot to say thank you.",
                                                "Hue, thank you so much for restoring my documents!!!",
                                                "Thanks!!!",
                                                "Ahahahahaha. Hahahahaha."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hue", args!["No, I..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryosen", args!["Hey, stay on topic!!", "You are..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Ahhhhhh! I guess we should call it a day now! Thanks for comingm all of you. Ok, let's go. Dismissed!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Come on!!!"])?;
                                        ctx.next()?;
                                        ctx.mes("-The three researchers say nothing. They just stare at each other, standing still... Then they finally leave the room. -")?;
                                        ctx.var("ep13_1_rhea").set(Val::from(21))?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Ryosen#ep131_rhea05::OnDisable")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Hue#ep131_rhea06::OnDisable")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Hansenne#ep131_rhea07::OnDisable")])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("ep13_1_rhea").get()? == 21 {
                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ah..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Ugh, I've got nothing to say.", "All I wanted was to let those people be closter to each other, become more friendly. But, problems come up every single time."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Can you think of any idea how to get those people to get along??"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["... Actually... Hansenne is not the one who ruined that cake."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Eh? What did you say?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["But... I don't understand why Hansenne would take the blame..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["... Well, I should go now."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["What? Ah, adventurer?!"])?;
                                        ctx.var("ep13_1_rhea").set(Val::from(22))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(8206), Val::from(8207)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if (ctx.var("ep13_1_rhea").get()?.number()? > 21 && ctx.var("ep13_1_rhea").get()?.number()? < 26)
                                    {
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["... Actually... Hansenne is not the one who ruined that cake."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Eh? What did you say?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["But... I don't understand why Hansenne would take the blame..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["... Well, I should go now."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["What? Ah, adventurer?!"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("ep13_1_rhea").get()? == 26 {
                                        ctx.lines_as("United Research Official", args!["How are they?"])?;
                                        ctx.next()?;
                                        ctx.mes("- You tell the Official about the conversation that the 3 researchers had, in detail. -")?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Whew, I see."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Maybe I've been a little too worried."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["How often they collide, it must be a natural thing for people from different countries with different personalities and customs."])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Thank you for helping me."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args![
                                                "Here, I'll offer you a cup of tea.",
                                                "This is a really special drink I preserved. Please, drink it while it's nice and hot."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("- You feel so refreshed and light, as you drink the tea that the Official offered. -")?;
                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ABSORBSPIRITS")?])?;
                                        ctx.var("ep13_1_rhea").set(Val::from(100))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(300000), Val::from(100000)])?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(8210)])?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Now, I should get to work on the report I need to submit to the head office. Ah, and this is nothing big, but still... This might come in handy when you travel."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Please, stop by and say hellp to us from time to time."],
                                        )?;
                                        ctx.call(Function::GetItem, vec![Val::from(12110), Val::from(1)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("United Research Official", args!["Thank you for everything."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "United Research Official",
                                            args!["Please, stop by and say hellp to us from time to time."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("United Research Official", args!["Oh, I'm not asking you to keep your eye on those 3 researchers. Don't get me wrong, hahaha.~"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    ctx.lines_as("United Research Official", args!["...Mmmm? What brings you here?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "United Research Official",
                        args![
                            "I wouldn't be roaming around this place like that, if I were you.",
                            "It's very dangerous here, and people dwelling is this place are not that nice."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
                ctx.lines_as("United Research Official", args!["Hmm? What did you just say?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "United Research Official",
                    args!["I can't really hear you. Could you speak louder?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Research Official#ep131")])?;
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Research Official#ep131")])?;
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Research Official#ep131")])?;
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnMeet => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Ryosen#ep131_rhea05::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hue#ep131_rhea06::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hansenne#ep131_rhea07::OnEnable")])?;
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnCall => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("mid_campin"),
                        Val::from(376),
                        Val::from(134),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(1),
                        Val::from("Research Official#ep131::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Research Official#ep131::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Ryosen#ep131_rhea05::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hue#ep131_rhea06::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hansenne#ep131_rhea07::OnDisable")])?;
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("mid_campin"), Val::from("Research Official#ep131::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Research Official#ep131::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            ResearchOfficialEp131Step::OnTimer300000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mid_campin"), Val::from("Research Official#ep131::OnMyMobDead")],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Research Official#ep131::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum TimerAlba01Step {
    Start,
    OnInit,
    OnEnable,
    OnStop,
    OnTimer1000,
    OnTimer180000,
    OnTimer360000,
    OnTimer600000,
    OnTimer7800000,
}

pub(super) fn timer_alba01_run(ctx: &Ctx, mut step: TimerAlba01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerAlba01Step::Start => {
                shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
                ctx.mes("Please enter the password")?;
                ctx.next()?;
                if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])?.number()? < 1 {
                    ctx.mes("Wrong password.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("Current Status:")?;
                    if ctx.var("$@parttimeon").get()? == 1 {
                        ctx.mes("Recruiting.")?;
                    } else {
                        ctx.mes("Not Recruiting.")?;
                    }
                    ctx.lines(args![
                        ((Val::from("Recruited ") + ctx.var("$@parttimeslots").get()?) + Val::from(" part-timers.")),
                        "What do you want to do?"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Reset the recruiting.:Cancel.")])? {
                        1 => {
                            ctx.lines(args!["Recruiting has been reset.", "Timer has stopped!"])?;
                            ctx.call(Function::StopNpcTimer, vec![])?;
                            ctx.next()?;
                            ctx.mes("Global values have been reset.")?;
                            ctx.var("$@parttimeon").set(Val::from(0))?;
                            ctx.var("$@parttimeslots").set(Val::from(0))?;
                            ctx.next()?;
                            ctx.lines(args!["Recruiting has been reset.", "Timer has started!"])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("#timer_alba01::OnEnable")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("Canceled.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                step = TimerAlba01Step::OnInit;
                continue 'machine;
            }
            TimerAlba01Step::OnInit => {
                ctx.var("$@parttimeon").set(Val::from(0))?;
                ctx.var("$@parttimeslots").set(Val::from(0))?;
                step = TimerAlba01Step::OnEnable;
                continue 'machine;
            }
            TimerAlba01Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerAlba01Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerAlba01Step::OnTimer1000 => {
                ctx.var("$@parttimeslots").set(Val::from(0))?;
                ctx.var("$@parttimeon").set(Val::from(1))?;
                ctx.call(Function::MapAnnounce, vec![Val::from("mid_camp"), Val::from("Breeder Taab: Attention adventurers in the camp! I'm recruiting 5 part-timers for my breeding farm. Only the first to arrive here will be hired!"), ctx.constant("BC_MAP")?, Val::from("0x00ff00")])?;
                return Err(Stop::End);
            }
            TimerAlba01Step::OnTimer180000 => {
                step = TimerAlba01Step::OnTimer360000;
                continue 'machine;
            }
            TimerAlba01Step::OnTimer360000 => {
                if ctx.var("$@parttimeon").get()?.number()? < 5 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("mid_camp"), Val::from("Breeder Taab: I'm looking for a part-timer who can work for my breeding farm. If you're interested, please visit me at the farm."), ctx.constant("BC_MAP")?, Val::from("0x00ff00")])?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("mid_camp"),
                            Val::from("Breeder Taab: The recruitment for my breeding farm has ended. I'll see you next time. Thanks!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff00"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            TimerAlba01Step::OnTimer600000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("mid_camp"),
                        Val::from("Breeder Taab: The recruitment for my breeding farm has ended. I'll see you next time. Thanks!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.var("$@parttimeon").set(Val::from(0))?;
                return Err(Stop::End);
            }
            TimerAlba01Step::OnTimer7800000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#timer_alba01::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#monster_master::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Moc2EventOnStep {
    Start,
    OnEnable,
    OnDisable,
    OnStop,
    OnTouch,
    OnTimer300000,
    OnTimer303000,
    OnTimer306000,
    OnTimer307000,
    OnTimer308000,
}

pub(super) fn moc2_event_on_run(ctx: &Ctx, mut step: Moc2EventOnStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Moc2EventOnStep::Start => {
                step = Moc2EventOnStep::OnEnable;
                continue 'machine;
            }
            Moc2EventOnStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#moc2_event_on")])?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("#moc2_event_on")])?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnTouch => {
                if ctx.var("mao_morocc2").get()? == 10 {
                    ctx.call(Function::DisableNpc, vec![Val::from("#moc2_event_on")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#moc2_event01::OnEnable")])?;
                    ctx.call(Function::InitNpcTimer, vec![])?;
                } else {
                    ctx.mes("A mysterious force is repelling you, and you're unable to push forward into this area.")?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(100), Val::from(100)])?;
                    ctx.var("$@moc_mao_gate1").set(Val::from(0))?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from("...Hey-!!... Where are you?!!.... Hey-!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdda0dd"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnTimer303000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from("...Isn't this voice...? ...Kidd?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdda0dd"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnTimer306000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("que_dan01"), Val::from("mid_camp"), Val::from(205), Val::from(312)],
                )?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnTimer307000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Corpse#moc2_dead01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin02::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc2_bt_r01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#moc2_event01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Corpse#moc2_dead01::OnReset")])?;
                return Err(Stop::End);
            }
            Moc2EventOnStep::OnTimer308000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#moc2_event_on")])?;
                ctx.var("$@moc_mao_gate1").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Ok(Val::from(0));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum CorpseMoc2Dead01Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnCall,
    OnMyMobDead,
    OnReset,
    OnTimer4000,
    OnTimer7000,
    OnTimer10000,
    OnTimer11000,
}

pub(super) fn corpse_moc2_dead01_run(ctx: &Ctx, mut step: CorpseMoc2Dead01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CorpseMoc2Dead01Step::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(1)])? == 0 {
                    ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("que_dan01"), Val::from("Corpse#moc2_dead01::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    if ctx.var("mao_morocc2").get()? == 10 {
                        ctx.lines(args![
                            "You have found a laceration on the dead body;",
                            "he must have been killed by Rin."
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Check his pockets.:Shake the corpse.")])? {
                            1 => {
                                ctx.mes("It's creepy to touch a corpse, but you have decided to check his pockets for any clues.")?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines(args![
                                    "It's creepy to touch a corpse, but you have decided to shake and turn the body",
                                    "to find any possible clues",
                                    "And..."
                                ])?;
                                ctx.next()?;
                            }
                            _ => {}
                        }
                        ctx.lines(args![
                            "A scroll flew out of the body's jacket pocket.",
                            "You're not sure what this scroll is for, but obviously it seems pretty important."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^4d4dffYou have obtained a Sealed Magic Scroll.",
                            "Let's bring this to Kidd.^000000"
                        ])?;
                        ctx.var("mao_morocc2").set(Val::from(11))?;
                        ctx.call(Function::GetItem, vec![Val::from(6028), Val::from(1)])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#moc2_event_on::OnStop")])?;
                        ctx.call(Function::InitNpcTimer, vec![])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7020), Val::from(7021)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("mao_morocc2").get()? == 11 {
                        ctx.mes("You have found a Dandelion Member who was killed by Rin.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("It's just a corpse, but there seems to be something unnatural about it.")?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(104), Val::from(108)])?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines(args![
                        "Now's not the time to examine corpses.",
                        "You better take care of your living enemies first!"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnInit => {
                step = CorpseMoc2Dead01Step::OnDisable;
                continue 'machine;
            }
            CorpseMoc2Dead01Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Corpse#moc2_dead01")])?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Corpse#moc2_dead01")])?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnCall => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_dan01"),
                        Val::from(26),
                        Val::from(40),
                        Val::from("Dandelion Member"),
                        Val::from(1985),
                        Val::from(1),
                        Val::from("Corpse#moc2_dead01::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_dan01"),
                        Val::from(21),
                        Val::from(35),
                        Val::from("Dandelion Member"),
                        Val::from(1985),
                        Val::from(1),
                        Val::from("Corpse#moc2_dead01::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_dan01"),
                        Val::from(25),
                        Val::from(32),
                        Val::from("Dandelion Member"),
                        Val::from(1985),
                        Val::from(1),
                        Val::from("Corpse#moc2_dead01::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_dan01"),
                        Val::from(34),
                        Val::from(34),
                        Val::from("Dandelion Member"),
                        Val::from(1985),
                        Val::from(1),
                        Val::from("Corpse#moc2_dead01::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_dan01"),
                        Val::from(36),
                        Val::from(34),
                        Val::from("Dandelion Member"),
                        Val::from(1985),
                        Val::from(1),
                        Val::from("Corpse#moc2_dead01::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnMyMobDead => {
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_dan01"), Val::from("Corpse#moc2_dead01::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_dan01"),
                        Val::from("...Hey!... Where are you?!... Where did you go?!!!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdda0dd"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnTimer7000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("que_dan01"), Val::from("mid_camp"), Val::from(204), Val::from(312)],
                )?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnTimer10000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Corpse#moc2_dead01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc2_bt_rin02::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc2_bt_r01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#moc2_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#moc2_event01::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#moc2_event_on::OnEnable")])?;
                return Err(Stop::End);
            }
            CorpseMoc2Dead01Step::OnTimer11000 => {
                ctx.var("$@moc_mao_gate1").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}
