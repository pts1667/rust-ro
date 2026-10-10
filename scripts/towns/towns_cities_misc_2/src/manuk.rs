#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args};

pub fn soldier_ep13pa829(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Food Provider",
            args!["The Manuk family subsists mostly on refining Gray Hollows that were buried a long time ago deep down under the ground."],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Food Provider", args!["Gdiios duuie Dssoas pogggd fdrul fdddoweet"])?;
        ctx.close()
    }
}

pub fn soldier_ep13_2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Injured Manuk Soldier",
            args![
                "I can't absorb Bradium Essence anymore because of my fatal injury.",
                "Those wicked fairies attacked me and left me like this."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Injured Manuk Soldier", args!["Bhiio aaas dgwer fdds rrrrrpppp Ee"])?;
        ctx.close()
    }
}

pub fn soldier_ep13_3(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Anxious Soldier", args!["Hurry, I am in big trouble. I lost all the Manuk Coins. I think I dropped them somewhere on the snowfield. Gosh, I saw them right before I fell asleep!"])?;
        ctx.close()
    } else {
        ctx.lines_as("Anxious Soldier", args!["Qosi dhhui rffd poaner ouh."])?;
        ctx.close()
    }
}

pub fn piom(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Piom",
            args![
                "You are... tiny. But you don't seem like a Fairy.",
                "As long as you are not a damned Fairy,",
                "then you are not our foe!",
                "In this world, there are only friends or foe!"
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Piom",
            args![
                "As our wi nueo woud bus",
                "Gw pii rooop pishe",
                "Fw iusbn podim bn usow ",
                "Psbh io whe pasn jd"
            ],
        )?;
        ctx.close()
    }
}

pub fn benknee_ep13_2_1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Benknee",
            args![
                "What brings you here?",
                "Are you a human?",
                "If you are human, you shouldn't be here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Benknee",
            args![
                "Jotunheim is a blessed and sacred place.",
                "We, Saphas will be standing with our own feet.",
                "And rise against oppression!"
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Benknee",
            args!["Bdf sdio hs ioq", "Wfn is ao ps od jd", "No pip dd dow hso le"],
        )?;
        ctx.next()?;
        ctx.lines_as("Benknee", args!["Wsd oup nc xkh d", "Rww o jsd sp", "Yd aihd oa sd s dd"])?;
        ctx.close()
    }
}

pub fn piom_ep13_2_1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Piom",
            args![
                "We, Saphas are always together!",
                "Wherever we are. We are always connected to each other.",
                "I don't know where you are from but, you should learn our spirits."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Piom",
            args![
                "Ng go oois yus dd",
                "You ii iaao nfb ud",
                "Wqq ifn isp did",
                "Uy ydf sd fs wee",
                "Mgg gf fs d ff"
            ],
        )?;
        ctx.close()
    }
}

pub fn galtun_ep13_2_1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Galtun",
            args![
                "Recently, tiny things have been flying around.",
                "I am not sure if they are flies or not.",
                "But it is very annoying."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Galtun",
            args![
                "They can only use their small magic from a long distance.",
                "But I can kick them off quickly.",
                "They are so bothersome. But I better not waste my time with them."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Galtun",
            args!["Ya sda sdou sh dbi", "Av bu dgs ldo gp gf ", "Jg gfs dsd fw eerr "],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Galtun",
            args!["Mb ih ids oj fd", "Pg sdf dd sd fff", "Bq wer jfsd fsd ut yy", "Nx cxd fsd fs df "],
        )?;
        ctx.close()
    }
}

pub fn galtun_ep13_2_2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Galtun",
            args![
                "I can relax now that we have those piles of Bradium.",
                "But I am also worried that we can spend them in a short time."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Galtun",
            args![
                "Bu iu bus sfi a sd",
                "Zsd dwo uf sh osad ",
                "Qdf aih fas io d hoas",
                "Nas d iy as di"
            ],
        )?;
        ctx.close()
    }
}

pub fn benknee_ep13_2_2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Benknee",
            args![
                "Huh? Who?? Who are you??",
                "Oh, you are not a fairy.",
                "I thought you were a fairy thing.",
                "Anyway, who are you? Can you speak?"
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Benknee",
            args!["Bao j pj a sd", "Gi oh as d", "Ya sd Yrt sd ad", "Bq we ojj jd"],
        )?;
        ctx.close()
    }
}

pub fn piom_ep13_2_2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Piom",
            args![
                "I'll never forget the deep-rooted rancor against those traitors.",
                "I remember how our ancestors died.",
                "I swear that I would avenge them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Piom",
            args!["First, I'll kick those bastards.", "Those flying little things bother me so much."],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Piom",
            args!["Vio hs pf I aps", "Vs ou oas de ee", "Bzi sh da opd", "Mc oju asop dj a ps"],
        )?;
        ctx.next()?;
        ctx.lines_as("Piom", args!["Be juas da sd", "Eoj ssr owq w e ", "Wps dj i ao sj daasd asd"])?;
        ctx.close()
    }
}

pub fn piom_ep13_2_3(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Piom",
            args![
                "Our lives exist for Saphas.",
                "On the other hand,",
                "Saphas lives exist for me.",
                "Hum hahaha!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Piom",
            args!["We, Saphas are always together!", "Wherever we are!", "Cheer for Saphas!"],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Piom",
            args!["Esd fas hdi as sp ad osd", "Ns id pie sj idf", "Rto osd ps ad ", "Mi sho oo pesd"],
        )?;
        ctx.next()?;
        ctx.lines_as("Piom", args!["N sd sou as d ", "Ma asd psh ds ii ", "Qso uf lj dhis id"])?;
        ctx.close()
    }
}

pub fn galtun_ep13_2_3(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Galtun",
            args!["I will devote myself to", "protect my family and Saphas.", "That is all I want..."],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Galtun", args!["Mr ishh qw e ee", "Baa eou sh ua sd", "Up idhs ish dk I jsd"])?;
        ctx.close()
    }
}

pub fn piom_ep13_2_4(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Piom",
            args![
                "Human, you think our battle is stupid, don't you?",
                "And a waste of time?",
                "But it is really depends on this war whether we can survive or not."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Piom",
            args![
                "Nsa dhi pao sdi a jp das",
                "Uaa as iijds kn sdg f",
                "Bzi hd sia pasd ",
                "Es do ja pda sj d",
                "Bs oju lujdi ni sdgf g "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Piom", args!["Us id jd nai dh"])?;
        ctx.close()
    }
}

pub fn worker_ep13bsg1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Worker",
            args![
                "It is dangerous if the valve is not checked properly every day.",
                "In fact, there was an incident.",
                "It gives me the creeps just thinking about it."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Worker", args!["Gs df o aj ud pa", "N sd asw ewt jj ", "Ud aso pda s "])?;
        ctx.close()
    }
}

pub fn worker_ep13bsg2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Worker",
            args![
                "What!! Wh.. Oh... I... I didn't fall asleep!!",
                "Let's get back to work... that's right work..."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Worker", args!["Ns ad jai osd", "Rt odj as jo dp as"])?;
        ctx.close()
    }
}

pub fn worker_ep13bsg3(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Worker", args!["Hmm... It's working just fine... No problems at all..."])?;
        ctx.close()
    } else {
        ctx.lines_as("Worker", args!["Mou ii ros oa d d "])?;
        ctx.close()
    }
}

pub fn worker_ep13bsg4(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Worker", args!["My eyesight is getting weaker these days."])?;
        ctx.close()
    } else {
        ctx.lines_as("Worker", args!["Yw I eus ia d ap s"])?;
        ctx.close()
    }
}

pub fn worker_ep13bsg5(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Worker", args!["Isn't this fabulous?"])?;
        ctx.close()
    } else {
        ctx.lines_as("Worker", args!["R tt osj dj d"])?;
        ctx.close()
    }
}

pub fn worker_ep13bsg6(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Worker", args!["It is fortunate to have lots of fine quality Bradium today."])?;
        ctx.next()?;
        ctx.lines_as("Worker", args!["This is all that is left for us."])?;
        ctx.close()
    } else {
        ctx.lines_as("Worker", args!["Qw eI hs pado as d p "])?;
        ctx.next()?;
        ctx.lines_as("Worker", args!["Too fn ish d fd"])?;
        ctx.close()
    }
}

pub fn manuk_galtun_door1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Manuk Galtun",
            args!["Here is Manuk where the Sapha who is descendant of Hwergelmir lives."],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Manuk Galtun", args!["Zd sng pps fsr"])?;
        ctx.close()
    }
}

pub fn manuk_galtun_door2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Manuk Galtun",
            args!["Here is Manuk where the Sapha who is descendant of Hwergelmir lives."],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Manuk Galtun", args!["To osn dia fg gh gh"])?;
        ctx.close()
    }
}

pub fn manuk_piom_tre1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Manuk Piom",
            args!["Galtuns are brave Sapha warriors.", "I am a Piom class which is general labor."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manuk Piom",
            args![
                "By virtue of the braveness of the Galtun, we can stand for a long time from the diversions of the Laphine.",
                "We always appreciate their efforts."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Manuk Piom",
            args!["H dn i sid p sd ", "Nd isjd sapd j s id", "Bsi o ps dkm jgf", "Eo oo ptr n sid"],
        )?;
        ctx.close()
    }
}

pub fn manuk_piom_tre2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as("Manuk Piom", args!["My leg...", "It's time to already."])?;
        ctx.close()
    } else {
        ctx.lines_as("Manuk Piom", args!["Fn is d id ", "Yon sdi dh so dps"])?;
        ctx.close()
    }
}

pub fn manuk_galtun_tre3(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as("Manuk Galtun", args!["Welcome to Manuk.", "How can I help you?"])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn manuk_piom_tre4(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Manuk Piom",
            args![
                "Hey, Be careful!",
                "This mineral is Bradium which is the life of our tribe.",
                "If you don't handle the stone carefully, you'll be in trouble!"
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Manuk Piom",
            args!["Bmm ish di sd", "Fii sd ani s a d s k ds ", "Ti h is so so pd"],
        )?;
        ctx.close()
    }
}

pub fn manuk_benknee_tre5(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 && ctx.var("ep13_2_rhea").get()? == 100 {
        ctx.lines_as(
            "Manuk Benknee",
            args![
                "Can you see that statue?",
                "He's the Hwergelmir, who is like a legend for us Sapha.",
                "He was a real majestic and brave man."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Manuk Piom",
            args!["Ys oadj oa s d", "Bni ii osd jo as das", "Qa oj df isd oo o"],
        )?;
        ctx.close()
    }
}

pub fn young_villager_ep13bs(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Young Villager",
            args!["It's past the time of our date, why isn't she here yet!!?"],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Asd", args!["Ywo di pi butfs oui Afbsu "])?;
        ctx.close()
    }
}

pub fn mechanic_ep13bs(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Mechanic",
            args![
                "Alien races are not allowed to enter.",
                "It's very dangerous here, please don't come any closer."
            ],
        )?;
        ctx.close()
    } else {
        ctx.lines_as("Asoui", args!["Fs iua sdjosow ww ", "Adds wwpq iusnd "])?;
        ctx.close()
    }
}

pub fn worker_ep13bs1(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as(
            "Worker",
            args!["Hmm, it smells delicious.", "It should be time to turn it around now."],
        )?;
        ctx.next()?;
        ctx.lines_as("Worker", args!["Hardrock Mammoth steak should be eaten slightly raw!"])?;
        ctx.close()
    } else {
        ctx.lines_as("Tee", args!["As woue dpi sha we", "Two psie bu le"])?;
        ctx.next()?;
        ctx.lines_as("Tee", args!["Tr sdou powee wwee "])?;
        ctx.close()
    }
}

pub fn worker_ep13bs2(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Worker", args!["Chef Cook, how many plates should I put down?"])?;
        ctx.close()
    } else {
        ctx.lines_as("Tee", args!["We pishd bugs ouwwe iro "])?;
        ctx.close()
    }
}

pub fn scientist_ep13bs(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![2782])? == 1 {
        ctx.lines_as("Scientist", args!["Is there only one way we can survive..?"])?;
        ctx.close()
    } else {
        ctx.lines_as("Apti", args!["Dso piey pioit ioep "])?;
        ctx.close()
    }
}
