use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn elin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines_as("Elin", args!["^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Elin]")?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.lines(args![
            "Hello~!",
            "Heh heh, you're a boy, so you probably don't like dolls, right? Well, I like dolls very very much!"
        ])?;
    } else {
        ctx.mes("Hi hi~! Oh, oh, do you like dolls? I really really like dolls... Hee Hee~!")?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Elin",
        args!["You know, I really really want a new doll! I hope my daddy will give me one on my birthday...!"],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("Um, I hope your daddy gives you one too.:How about I give you one now?")],
    )? {
        1 => {
            ctx.lines_as(
                "Elin",
                args!["Yeah, I'm hoping", "he gives me a Yoyo", "doll. They're so cute!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 1000 {
                ctx.mes("[Elin]")?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.mes("Maybe you should put some of your stuff away first, you look like you're carrying too much, hee hee~")?;
                } else {
                    ctx.mes("Do you always carry so much with you? Put some of your things away first, okay?")?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("[Elin]")?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.lines(args!["Oh my goodness!", "Really? You're", "such a sweetie~!"])?;
            } else {
                ctx.lines(args!["Will you really?", "You're gonna give", "me a doll? Yaaaay~"])?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Elin",
                args!["What kind of doll are you going to give me? Are you really gonna give me one?"],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Poring Doll:Chonchon Doll:Puppet:Rocker Doll:Spore Doll:Osiris Doll:Baphomet Doll:Raccoon Doll:Yoyo Doll:I'm as adorable as a doll.",
                )],
            )? {
                1 => {
                    if ctx.call(Function::CountItem, vec![Val::from(741)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(741), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Aww...", "I have a lot", "of Poring dolls..."])?;
                        ctx.next()?;
                        ctx.mes("[Elin]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes(
                                "But since a cute boy is giving it to me, I'll happily take it! Hee hee, soft and fluffy Poring doll~",
                            )?;
                        } else {
                            ctx.mes("But since a nice girl is giving it to me, I'll take it! Hee he, soft and fluffy Poring doll~")?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Ooh, let me give you something as a little thank you. You see, I hid some candy so I can eat it without telling mommy."])?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(529), Val::from(1)])?;
                        ctx.mes("[Elin]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("I don't know if you're too old to like candy, but I guess it's okay.")?;
                        } else {
                            ctx.mes("So, try not to eat too much, okay? Otherwise, you might get in trouble!")?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Thank you", "so much for", "the Poring doll!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("[Elin]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.lines(args!["Aww...?", "Were you only teasing me?"])?;
                        } else {
                            ctx.lines(args!["Aww...", "You're not making fun of me are you?"])?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["I guess you forgot it somewhere..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    if ctx.call(Function::CountItem, vec![Val::from(742)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(742), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Agh--!", "A Ch-Ch, Chonchon doll?!"])?;
                        ctx.next()?;
                        ctx.mes("[Elin]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("But since... You're... Such a nice guy... I better take it...!")?;
                        } else {
                            ctx.mes("But since... Since it's a gift... I guess I'll take it...")?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Mmm~", "Let me give you", "something in return..."])?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(530), Val::from(1)])?;
                        ctx.lines_as(
                            "Elin",
                            args!["If you eat too much, your teeth will start to rot... at least, that's what Mommy says."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["And..", "Uh...", "Thank you", "for the doll?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Elin",
                            args!["Chonchon dolls are ugly anyways, but you still lied to me! How can you be so mean?!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Waaaaaaaaah~~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                3 => {
                    if ctx.call(Function::CountItem, vec![Val::from(740)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(740), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Wow...!", "It looks like a bunny!"])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["I really like this Puppet~ Heh hehe! Thank you so much~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["Ooh ooh!", "I have something", "for you too!", "Um, where", "did it... Ah!"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(530), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Here it is! It's some of the candy Santa gave me. Go ahead and try some! Oh, and thank you so much for the doll!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Elin",
                            args!["Awww, you were only kidding? W-W-Why are you teasing me like that?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["^666666*Sniff...*^000000"])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Waaaaaaaaah~~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                4 => {
                    if ctx.call(Function::CountItem, vec![Val::from(752)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(752), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Ooh! Rocker Doll!", "It's the Rocker that likes singing and dancing! I don't like grasshoppers, but I like this because it's cute~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["Hmmm, I should give you something too, huh? Let's see, I have what my grandpa gave me..."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(532), Val::from(7)])?;
                        ctx.lines_as("Elin", args!["Here you go! We have a lot of this at home, so I'll give it to you, okay? Oh, and thank you so much for the doll~ I love cute dolls!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Elin", args!["Awww..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["You don't", "really have a doll...?", "I was so excited about it, too..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                5 => {
                    if ctx.call(Function::CountItem, vec![Val::from(743)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(743), Val::from(1)])?;
                        ctx.lines_as(
                            "Elin",
                            args![
                                "It's a mushroom?",
                                "Ewwwwwwwww, yucky!",
                                "Mommy made me eat",
                                "mushrooms today, too..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["But this doll is cute, so I'll take it. But I'm still not gonna eat mushrooms! Heh heh~"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(538), Val::from(5)])?;
                        ctx.lines_as(
                            "Elin",
                            args!["Hehe~ My mommy made this! It's really yummy. I love cookies, too. So try it, you'll like it! Hee hee!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Thanks for", "the doll! I'll", "take good care of it!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Elin", args!["Eh...?", "You don't", "have a doll?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args![
                                "Awww...",
                                "It's not nice",
                                "to tease people",
                                "like that. ^666666*Sniff, sniff*^000000"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                6 => {
                    if ctx.call(Function::CountItem, vec![Val::from(751)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(751), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Ahhhhh!", "What is this", "thing?! It's so scary!"])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["I've never seen a doll like this before. Where it's from? Hmm? Since I've never seen this kind of doll before, I'm gonna show it to my daddy, too. Hehe~! He'll be surprised!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["Here, since you gave me such a nicely made doll, I better give you something good too."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(522), Val::from(2)])?;
                        ctx.lines_as(
                            "Elin",
                            args!["I found it when I secretly went to the forest. I took it because it has pretty colors!"],
                        )?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("Since there's two, you can share it with your girlfriend. Hee hee~!")?;
                        } else {
                            ctx.mes("Since there's two, you can share it with your boyfriend. Hee hee~!")?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Elin", args!["Awww...", "Why do you have", "to make fun of me?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                7 => {
                    if ctx.call(Function::CountItem, vec![Val::from(750)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(750), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Huh?", "It's a little goat..."])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["It's so weird. It's cute but it's also scary at the same time. Well, since you gave me a pretty doll, I wanna give you something, too."])?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(525), Val::from(5)])?;
                        ctx.lines_as("Elin", args!["My daddy gave me this when I was really sick. I'm not sick anymore so you can use it when you need to. Hehehe~"])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["Hee hee~", "Thank you for", "the doll. I won't lose it!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Elin", args!["You big liar! Why are you pretending to be nice?!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                8 => {
                    if ctx.call(Function::CountItem, vec![Val::from(754)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(754), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Hehe, it's a", "raccoon doll.", "It's very very cute~"])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["I don't like", "Smokies in real life,", "but this doll is nice!"])?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(539), Val::from(3)])?;
                        ctx.lines_as("Elin", args!["Here, let me give you some cake my grandma made. It's really yummy. I don't know if you like sweets or not, but it's really good! Please try some!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["Hee hee~", "Thanks for the doll.", "I'm gonna keep it", "in my room!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Elin",
                            args!["Hey! How come you have to say things like that? Are you making fun of me?!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                9 => {
                    if ctx.call(Function::CountItem, vec![Val::from(753)])?.number()? >= 1 {
                        ctx.call(Function::DelItem, vec![Val::from(753), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["Woooooooow~!"])?;
                        ctx.next()?;
                        ctx.lines_as("Elin", args!["A Yoyo doll! It's so pretty! You're really gonna give it to me?! Yay! Thank you so much! I really like it! Hee hee!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["Since you gave", "me such a pretty doll,", "I wanna give you", "something, too!"],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(608), Val::from(1)])?;
                        ctx.lines_as("Elin", args!["My daddy picked it up on his way to another town. It looks like some kind of seed. I tried planting it in front of our house but it won't grow. Do you want to try?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elin",
                            args!["Hehe~", "Thanks for", "the doll. I'll hug", "it before I go", "to sleep!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Elin", args!["Hey...", "How come you're making fun of me?!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                10 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I'm as adorable as a doll..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Elin", args!["Whaaaaaaaaaaat...?"])?;
                    ctx.next()?;
                    ctx.lines_as("Elin", args!["^3355FFWhat did^000000", "^3355FFyou just say?!^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn elin(ctx: &Ctx) -> Script {
    elin_body(ctx, Vec::new()).map(|_| ())
}

fn grampa_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Grampa",
        args![
            "*Gasp*...",
            "....*Gasp!*",
            "Oh...",
            "Some may say I've lived a full life on wine, women and song..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Grampa",
        args!["But my soul still burns with youthful vigor! It's just... This old body can't keep up anymore... *Hack! Wheeeeze~*"],
    )?;
    ctx.next()?;
    if ctx.call(Function::CountItem, vec![Val::from(1030)])?.number()? > 9 {
        match runtime::select_values(
            ctx,
            &[Val::from("Show him Tiger's Footskin.:Exchange it with Boys Cap.:Cancel")],
        )? {
            1 => {
                ctx.lines_as(
                    "Grampa",
                    args![
                        "Ohhh~ !!",
                        "Is that...?",
                        "It is! ",
                        "That's Tiger's Footskin!! Even from here I can feel its powers!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grampa",
                    args!["One glance at it can restore your strength, one sniff and your blood boils with vigorous pleasure..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Grampa", args!["Just one bite...", "Will revive my virility!!!! The Tiger's Footskin~!! Ohhhh! Ohh My God !! Please!!! Please give me that... Please..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(Function::DelItem, vec![Val::from(1030), Val::from(10)])?;
                ctx.call(Function::GetItem, vec![Val::from(5016), Val::from(1)])?;
                ctx.lines_as(
                    "Grampa",
                    args![
                        "T... Thank you !!",
                        "With this I can revive my youthful splendor!! I must eat this thing right away !"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grampa",
                    args![
                        "...",
                        "Nope. Still feel OLD.",
                        "If anything, I feel even worse. Th-there's this buzzing in my head..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Grampa", args!["He...Hey, kid !! W-Wait !"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        match runtime::select_values(ctx, &[Val::from("Talk:Cancel")])? {
            1 => {
                ctx.lines_as("Grampa", args!["Listen well...", "You've got to take care of yourself as well as your can. Without your health, one cannot enjoy the pleasures of this mortal realm."])?;
                ctx.next()?;
                ctx.lines_as("Grampa", args!["In order to restore my youth, I tried all sorts of things, mostly by hearsay, but nothing worked. In the end, I spent so much zeny on miracle cures, I ended up broke."])?;
                ctx.next()?;
                ctx.lines_as("Grampa", args!["I've mostly given up on restoring my youth. But there is one hope left... Eating ^3355FFTiger's Footskin^000000, The King of Invigorators!!"])?;
                ctx.next()?;
                ctx.lines_as("Grampa", args!["Haven't you heard?! One glance would restore the color in my white hair, one sniff, even just one touch, and an 80 year-old would have another chance at youth... It's The King of Invigorators !"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Grampa",
                    args![
                        "Before I die...",
                        "I wish I could eat 10 Tiger's Footskin. Then, I wouldn't have to die!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grampa",
                    args![
                        "If someone helped me get them, I would give that person my precious item, ^3355FFBoys Cap^000000 without regret..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Grampa",
                    args!["Cough Cough !!", "Tiger ....Tiger's ..... Foot ..... skin .....Cough Cough !!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn grampa(ctx: &Ctx) -> Script {
    grampa_body(ctx, Vec::new()).map(|_| ())
}

fn cherokee_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Cherokee", args!["Hey there~!"])?;
    ctx.next()?;
    ctx.lines_as("Cherokee", args!["What do you think about animal horns? Oh man, I think they're great! Don't you? I love animal horns so much I became a ^3355FFHorn Collector^000000."])?;
    ctx.next()?;
    ctx.lines_as("Cherokee", args!["I mean, there's so many things you can do with horns. You can wear them on your head, you can wear them on, um... your house? All sorts of things!"])?;
    ctx.next()?;
    ctx.lines_as("Cherokee", args!["I've collected almost every sort of horn except for one kind, and that's ^FF3355Evil Horn^000000. Some people say Evil Horn doesn't even come from an animal, but from a demon. But even so, I still want it."])?;
    ctx.next()?;
    ctx.lines_as(
        "Cherokee",
        args![
            "Do you...",
            "Do you have ^FF3355Evil Horn^000000?",
            "If you offer me 20 Evil Horns, I will give you an ^3355FFAntler^000000 from my precious collection. So, deal?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Okay, let's deal.:Shut up, Dumbo.")])? {
        1 => {
            if ctx.call(Function::CountItem, vec![Val::from(923)])?.number()? > 19 {
                ctx.call(Function::DelItem, vec![Val::from(923), Val::from(20)])?;
                ctx.lines_as(
                    "Cherokee",
                    args![
                        "Whoah~! This is the first time I've ever seen a real ^FF3355Evil Horn^000000!!",
                        "Thank you! Here, this is my Antler for you!"
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(2284), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as("Cherokee", args!["With your great help, I can make my wish come true this time. Finally, I'll be recognized as a serious horn collector. I appreciate this~!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Cherokee",
                    args![
                        "Hmm...",
                        "You don't seem to have enough. I need ^FF335520 Evil Horns^000000 for my collection."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines_as(
                "Cherokee",
                args!["Well that's fine. Although you're a rude prick, I will forgive you. We'll probably speak again..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn cherokee(ctx: &Ctx) -> Script {
    cherokee_body(ctx, Vec::new()).map(|_| ())
}

fn stylish_merchant_new30_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CountItem, vec![Val::from(10007)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(968)])?.number()? > 49)
    {
        ctx.lines_as("Zic", args!["I know I know, you just want to get a Bao Bao of your own. But I can't concentrate on my work if you keep rushing me like this."])?;
        ctx.next()?;
        ctx.lines(args!["^3355FF*Thud! Thud!*", "*Ah! Kek! Smash!*", "*Boom Boom!*^000000"])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(10007), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(968), Val::from(50)])?;
        ctx.lines_as("Zic", args!["Phew!", "There you go~!", "Now make good use of it!!"])?;
        ctx.call(Function::GetItem, vec![Val::from(5042), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CountItem, vec![Val::from(5041)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 9)
    {
        ctx.lines_as("Zic", args!["Alright alright! Gosh, your Crescent Hairpin will be ready in a bit, but I can't concentrate my work if you keep rushing me like this!"])?;
        ctx.next()?;
        ctx.lines(args!["^3355FF*Thud! Thud!*", "*Ah! Kek! Smash!*", "*Boom Boom!*^000000"])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(5041), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(999), Val::from(10)])?;
        ctx.lines_as(
            "Zic",
            args!["Phew!", "It's done~!", "Now wear it and", "look pretty or", "something, yeah?"],
        )?;
        ctx.call(Function::GetItem, vec![Val::from(5048), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CountItem, vec![Val::from(2271)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(975)])?.number()? > 0)
    {
        ctx.lines_as("Zic", args!["Yeah yeah, you came for your Fashionable Glasses. Just don't rush me, or I won't be able to concentrate on my work, alright?"])?;
        ctx.next()?;
        ctx.lines(args!["^3355FF*Thud! Thud!*", "*Ah! Kek! Smash!*", "*Boom Boom!*^000000"])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(2271), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(975), Val::from(1)])?;
        ctx.lines_as(
            "Zic",
            args![
                "Phew, it's done!",
                "Now go wear these and look, well, as fashionable as these glasses, I guess."
            ],
        )?;
        ctx.call(Function::GetItem, vec![Val::from(5047), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 1199 {
        ctx.lines_as("Zic", args!["Okay okay, you want your Heart hairpin, I'm working on it. Yeesh, I can't concentrate at all if you try to rush me, you know?"])?;
        ctx.next()?;
        ctx.lines(args!["^3355FF*Thud! Thud!*", "*Ah! Kek! Smash!*", "*Boom Boom!*^000000"])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(1200)])?;
        ctx.lines_as(
            "Zic",
            args!["Phew~!", "Finally, it's done!", "Make me happy and", "wear it with pride~"],
        )?;
        ctx.call(Function::GetItem, vec![Val::from(5041), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Zic", args!["Yay, this cool", "breeze is great!", "I love the sea!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Zic",
            args![
                "...Hm?",
                "Awww man. Can't you tell I'm on vacation? All I wanted was some peaceful rest."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zic",
            args!["Okay okay, you win. Once again, my reputation as a master craftsman precedes me."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zic",
            args!["Let me know what item you're interested in, and maybe I'll make it for you..."],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Bao Bao:Cresent Hairpin:Fashionable Glasses:Heart Hairpin")])? {
            1 => {
                ctx.lines_as(
                    "Zic",
                    args!["Sooo...", "You want me to make you a Bao Bao, huh? Alright, alright..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zic",
                    args![
                        "Let's see, I'll need...",
                        "1 ^0000FFSilk Ribbon^000000",
                        "50 ^0000FFHeroic Emblem^000000",
                        "...Did you know this already?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Zic",
                    args!["So you want a Crescent Hairpin, huh? Man, I guess these things are pretty high in demand."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zic",
                    args![
                        "Okay, I'll need...",
                        "1 ^0000FFHeart Hairpin^000000",
                        "10 ^0000FFSteel^000000",
                        "...Did you know this already?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Zic",
                    args!["Weird. How'd you know I make Fashionable Glasses? I guess I must be more famous that I thought."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zic",
                    args![
                        "I need to have...",
                        "1 ^0000FFJack be Dandy^000000",
                        "1 ^0000FFScarlet Dyestuffs^000000",
                        "...Did you know this already?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as(
                    "Zic",
                    args!["You want a heart hairpin, eh? Okay, I think I can work something out for you..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zic",
                    args!["Just gimmie...", "1200 ^0000FFCoral Reef^000000.", "...Did you know this already?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn stylish_merchant_new30(ctx: &Ctx) -> Script {
    stylish_merchant_new30_body(ctx, Vec::new()).map(|_| ())
}

fn hat_store_girl_new30_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Tempestra", args!["Ah, such a cool breeze. It's good to visit the seashore. I think it was the right choice to come here to take a break from my business."])?;
    ctx.next()?;
    ctx.lines_as("Tempestra", args!["Ooh, the sun is too strong today. I'm glad that I brought my hat. I'm going to get a sunburn if my skin is exposed to the sun like this everyday."])?;
    ctx.next()?;
    ctx.lines_as(
        "Tempestra",
        args!["Oh, I'm sooo thirsty! A nice, chilled Yellow Potion would be perfect right about now~"],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Let me treat you to a Yellow Potion.:What, you expect me to give you one?!",
            )],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.call(Function::CountItem, vec![Val::from(503)])?.number()? > 0 {
                ctx.lines_as(
                    "Tempestra",
                    args!["Ah, thank you so much. I'm so glad to have met a friendly person like yourself."],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF*Gulp, gulp*^000000")?;
                ctx.call(Function::DelItem, vec![Val::from(503), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as("Tempestra", args!["Hyaaaaaa~~!!!", "It's so cold!! Thank you~~~"])?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("It's really hot, isn't it~?:You should wear your hat...")],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as("Tempestra", args!["Yeah~ it's really hot...."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tempestra",
                            args![
                                "I have",
                                "^0000FFSunday Hat^000000,",
                                "^0000FFMage Hat^000000 and...",
                                "^0000FFMagician Hat^000000 and more in my room."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tempestra", args!["I brought these hats so I can sell them and use the money for my vacation here, but now it looks like I have to wear one~"])?;
                        ctx.next()?;
                        'b3: {
                            let subject3 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Sunday Hat!!?:Mage Hat?!?:Magician Hat??!??")],
                            )?);
                            let mut matched3 = false;
                            let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                && !subject3.loosely_equals(&Val::from(2))
                                && !subject3.loosely_equals(&Val::from(3));
                            if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Tempestra",
                                    args!["Oh? Didn't you know? I'm a hat merchant. Didn't I tell you that before?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tempestra",
                                    args!["Hmm, I guess I didn't. I apologize. Ooh! You know what, I could make you a Sunday Hat~!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tempestra",
                                    args![
                                        "I'd need...",
                                        "^0000FF250 Fabric^000000",
                                        "^0000FF1 Slotted Hat^000000",
                                        "^0000FF1 Slotted Cap^000000",
                                        "^0000FF600 Soft feather^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                if (((ctx.call(Function::CountItem, vec![Val::from(1059)])?.number()? > 249
                                    && ctx.call(Function::CountItem, vec![Val::from(2221)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(2227)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(7063)])?.number()? > 599)
                                {
                                    ctx.lines_as("Tempestra", args!["What?!? ", "You have all the items already? If there are forged items or slotted items with monster cards, then put them into your storage."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Tempestra", args!["I just want to make something for the Yellow Potion, so I won't charge you any zeny for my hat making service."])?;
                                    match runtime::select_values(ctx, &[Val::from("Oh, please do.:No thanks.")])? {
                                        1 => {
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Alrighty. Just give me a moment..."])?;
                                            ctx.next()?;
                                            ctx.lines(args!["^3355FF*Thud! Boot!*", "*Thic-Tac!*^000000"])?;
                                            ctx.next()?;
                                            ctx.mes("^3355FF*Beeeeeeeh~~~*^000000")?;
                                            ctx.next()?;
                                            ctx.call(Function::DelItem, vec![Val::from(1059), Val::from(250)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(2221), Val::from(1)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(2227), Val::from(1)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(7063), Val::from(600)])?;
                                            ctx.lines_as("Tempestra", args!["Here it is, tee hee~", "How about that? Do you like it?"])?;
                                            ctx.call(Function::GetItem, vec![Val::from(5032), Val::from(1)])?;
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Once again, thank you for your favour. I'll see you later~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Oh alright~"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Tempestra",
                                                args!["If by any chance you visit me later, I'd be more than happy to make a hat for you~"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Well then, see you later~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    ctx.lines_as(
                                        "Tempestra",
                                        args!["I will tell you a secret, because you gave me the Yellow Potion~"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Tempestra", args!["I'm looking forward seeing you again~~"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Tempestra",
                                    args!["Oh? Didn't you know? I'm a hat merchant. Didn't I tell you that before?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tempestra",
                                    args!["Hmm, I guess I didn't. I apologize. Ooh! You know what, I could make you a Mage Hat!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tempestra",
                                    args![
                                        "^0000FF1 Wizard Hat^000000",
                                        "^0000FF400 Dragon Scale^000000",
                                        "^0000FF50 Mould Powder^000000",
                                        "^0000FF1 Elder Willow Card^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                if (((ctx.call(Function::CountItem, vec![Val::from(2252)])?.number()? > 0
                                    && ctx.call(Function::CountItem, vec![Val::from(1036)])?.number()? > 399)
                                    && ctx.call(Function::CountItem, vec![Val::from(4052)])?.number()? > 0)
                                    && ctx.call(Function::CountItem, vec![Val::from(7001)])?.number()? > 49)
                                {
                                    ctx.lines_as("Tempestra", args!["What?!? ", "You have all the items already? If there are forged items or slotted items with monster cards, then put them into your storage."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Tempestra", args!["I just want to make something nice for you, so I won't charge you any zeny for my hat making service."])?;
                                    match runtime::select_values(ctx, &[Val::from("Oh, please do.:No thanks.")])? {
                                        1 => {
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Alrighty. Just give me a moment..."])?;
                                            ctx.next()?;
                                            ctx.lines(args!["^3355FF*Thud! Boot!*", "*Thic-Tac!*^000000"])?;
                                            ctx.next()?;
                                            ctx.mes("^3355FF*Beeeeeeeh~~~*^000000")?;
                                            ctx.next()?;
                                            ctx.call(Function::DelItem, vec![Val::from(2252), Val::from(1)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(1036), Val::from(400)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(4052), Val::from(1)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(7001), Val::from(50)])?;
                                            ctx.lines_as("Tempestra", args!["Here it is, tee hee~", "How about that?", "Do you like it?"])?;
                                            ctx.call(Function::GetItem, vec![Val::from(5027), Val::from(1)])?;
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Once again, thank you for your favour. I'll see you later~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Oh alright~"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Tempestra",
                                                args!["If by any chance you visit me later, I'd be more than happy to make a hat for you~"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Well then, see you later~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    ctx.lines_as(
                                        "Tempestra",
                                        args!["I will tell you a secret, because you gave me the Yellow Potion~"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Tempestra", args!["I'm looking forward seeing you again~~"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Tempestra",
                                    args!["Oh? Didn't you know? I'm a hat merchant. Didn't I tell you that before?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tempestra",
                                    args!["Hmm, I guess I didn't. I apologize. Ooh! You know what, I could make you a Magician Hat!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tempestra",
                                    args![
                                        "^0000FF1 Wizard Hat^000000",
                                        "^0000FF450 Ancient Lips^000000",
                                        "^0000FF1200 Solid Shell^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                if ((ctx.call(Function::CountItem, vec![Val::from(2252)])?.number()? > 0
                                    && ctx.call(Function::CountItem, vec![Val::from(1054)])?.number()? > 449)
                                    && ctx.call(Function::CountItem, vec![Val::from(943)])?.number()? > 1199)
                                {
                                    ctx.lines_as("Tempestra", args!["What?!? ", "You have all the items already?", "If there are forged items or slotted items with monster cards, then put them into your storage."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Tempestra", args!["I just want to make something nice for you, so I won't charge you any zeny for my hat making service."])?;
                                    match runtime::select_values(ctx, &[Val::from("Please do:No thanks")])? {
                                        1 => {
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Alrighty. Just give me a moment..."])?;
                                            ctx.next()?;
                                            ctx.lines(args!["^3355FF*Thud! Boot!*", "*Thic-Tac!*^000000"])?;
                                            ctx.next()?;
                                            ctx.mes("^3355FF*Beeeeeeeh~~~*^000000")?;
                                            ctx.next()?;
                                            ctx.call(Function::DelItem, vec![Val::from(2252), Val::from(1)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(1054), Val::from(450)])?;
                                            ctx.call(Function::DelItem, vec![Val::from(943), Val::from(1200)])?;
                                            ctx.lines_as("Tempestra", args!["Here it is, tee hee~", "How about that?", "Do you like it?"])?;
                                            ctx.call(Function::GetItem, vec![Val::from(5045), Val::from(1)])?;
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Once again, thank you for your favour. I'll see you later~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Oh alright~"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Tempestra",
                                                args!["If by any chance you visit me later, I'd be more than happy to make a hat for you~"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Tempestra", args!["Well then, see you later~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    ctx.lines_as(
                                        "Tempestra",
                                        args!["I will tell you a secret, because you gave me the Yellow Potion~"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Tempestra", args!["I'm looking forward seeing you again~~"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Tempestra",
                            args![
                                "Well, the hats I have with me are for my customers. If I use them for myself, I'll have nothing to sell..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tempestra", args!["I want to make something special for you because you showed me such kindness, but I don't have anything like that right now."])?;
                        ctx.next()?;
                        if ((((ctx.call(Function::CountItem, vec![Val::from(7086)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 9)
                            && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 39)
                            && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 49)
                            && ctx.call(Function::CountItem, vec![Val::from(984)])?.number()? > 1)
                        {
                            ctx.lines_as(
                                "Tempestra",
                                args!["Hmm...", "I think I can make a special item from the items you already have."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Tempestra", args!["If there are any forged items or slotted items with monster cards, then put them into your Kafra storage."])?;
                            ctx.next()?;
                            ctx.lines_as("Tempestra", args!["Heh heh...", "I think I'll make you a Hat of the Sun God~"])?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(7086), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(969), Val::from(10)])?;
                            ctx.call(Function::DelItem, vec![Val::from(999), Val::from(40)])?;
                            ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(50)])?;
                            ctx.call(Function::DelItem, vec![Val::from(984), Val::from(2)])?;
                            ctx.lines_as(
                                "Tempestra",
                                args!["See! Here it is!!", "Haha, I made this quicker than the speed of light!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tempestra",
                                args!["...Or maybe I just gave you the one I already had, and took your items. Hee hee!"],
                            )?;
                            ctx.call(Function::GetItem, vec![Val::from(5022), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tempestra",
                                args!["Anyway, I justed want to give something really nice to you..."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Tempestra",
                                args![
                                    "Hmm...",
                                    "^0000FF1 Emblem of the Sun God^000000",
                                    "^0000FF10 Gold^000000",
                                    "^0000FF40 Steel^000000",
                                    "^0000FF50 Coal^000000",
                                    "^0000FF2 Oridecon^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tempestra",
                                args!["If you have these, I can make you my fantastic 'Hat of the Sun God!'"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tempestra",
                                args![
                                    "I just want to make something really special for you, so I won't charge you any zeny to make this hat."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            } else {
                ctx.lines_as(
                    "Tempestra",
                    args!["Umm, I appreciate it but, I guess you just ran out of Yellow Potions?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Tempestra", args!["....How rude!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn hat_store_girl_new30(ctx: &Ctx) -> Script {
    hat_store_girl_new30_body(ctx, Vec::new()).map(|_| ())
}

fn kinsey_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kinsey",
        args!["What does a man have to do to get a stiff drink around here? I mean, there are absolutely no good bars in Alberta!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kinsey",
        args!["Well, the alcohol in the pub here is second rate, but I gotta admit, it's got a great atmosphere."],
    )?;
    ctx.next()?;
    ctx.lines_as("Kinsey", args!["In fact, this old man is always there, telling stories about Turtle Island! But we're not sure if he's been there, or if he's just that drunk."])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("^3333FF*ting!*^000000:Sounds fun~")])?) == 1 {
        ctx.lines_as(
            "Kinsey",
            args![
                "Whoa...",
                "Hey, your eyes just lit up right when I said 'Turtle Island!' Hmm, let me try that again."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kinsey", args!["*Ahem*", "'Tur-tle", "Is-land.'"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["^3333FF*ting!*^000000"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kinsey",
            args![
                "Ah...",
                "You must be an adventurer. It seems like you really wanna learn more about that place, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kinsey", args!["Why don't you look around near the pub, and look for that old man? He's usually pretty drunk, and speaks nonsense, but he might have something to say to you."])?;
        ctx.next()?;
        ctx.lines_as("Kinsey", args!["Oh, and um, I've been in his room before. He leaves some sort of letter lying around the table that has something to do with his Turtle Island stories."])?;
        ctx.next()?;
        ctx.lines_as("Kinsey", args!["But if he asks for you to buy him a drink, I wouldn't do it. He's got like, three extra livers or something, so he'll drink you to bankruptcy."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Kinsey", args!["Yeah, I guess it's pretty cool listening to his stories. But it would be even cooler if I could hear them in a better drinking establishment."])?;
    ctx.next()?;
    ctx.lines_as("Kinsey", args!["I mean, they don't even serve a 'Boogieman,' much less a 'Cobo.' It's getting harder and harder to find those fine, fine specialty drinks."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kinsey_tur(ctx: &Ctx) -> Script {
    kinsey_tur_body(ctx, Vec::new()).map(|_| ())
}

fn grandpa_turtle_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Grandpa Turtle", args!["Ooog...", "So dizzy..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Grandpa Turtle",
        args!["There isn't one decent drinking establishment in all of Alberta! But then again, why did I drink so much? Hmm..."],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("Tell me about Turtle Island:How can I get there?:Stop talking")],
    )? {
        1 => {
            ctx.lines_as("Grandpa Turtle", args!["Eh...?", "Turtle Island?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Grandpa Turtle",
                args!["Well, there's a lot of folklore about that place. Some folk say that there's a mountain of treasure there."],
            )?;
            ctx.next()?;
            ctx.lines_as("Grandpa Turtle", args!["Others folk say a potion that can prolong your lifespan can be found somewhere in Turtle Island. But nobody really knows for sure."])?;
            ctx.next()?;
            ctx.lines_as(
                "Grandpa Turtle",
                args!["But I'm sure that Turtle Island does exist, and that something wonderful can be found there."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grandpa Turtle",
                args!["But only someone with a good heart that's eager for adventure should be able to go to that kind of place."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("^3333FF*ting!*^000000:Tell me more, old man!")])? {
                1 => {
                    ctx.lines_as("Grandpa Turtle", args!["Ha ha ha~!"])?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.mes("I like the shine in your eye, my boy! I can see a little bit of myself in those eyes. Yes...")?;
                    } else {
                        ctx.mes("No use hiding the glint in your eyes, my dear! It's adventure you wish for, that much I can tell...")?;
                    }
                    ctx.next()?;
                    ctx.lines_as("Grandpa Turtle", args!["Tell me...", "Have you heard of a man named ^3355FFJornadan Niliria^000000? He, and a group of ten other men actually found Turtle Island!"])?;
                    ctx.next()?;
                    ctx.lines_as("Grandpa Turtle", args!["^3355FFJornadan Niliria^000000 was a man of strength and conviction. He and his crew gathered every clue they could about Turtle Island."])?;
                    ctx.next()?;
                    ctx.lines_as("Grandpa Turtle", args!["Even after learning all that they could, much of their voyage relied on their hope and faith that the Turtle Island legend was true..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grandpa Turtle",
                        args!["After a long, arduous journey, their efforts were rewarded, and they landed on Turtle Island!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Grandpa Turtle", args!["However, it gets more complicated after that. I hear Niliria's crew had great difficulty getting back home for some reason."])?;
                    ctx.next()?;
                    ctx.lines_as("Grandpa Turtle", args!["Heh heh...", "If you want to learn more, why don't you go to the eastern end of the Alberta port? There's a scholar there that knows more of this story."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grandpa Turtle",
                        args!["He could probably shed more light on the subject of Turtle Island's legend."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Grandpa Turtle", args!["Ah, one more thing! Look around that scholar and you should find his ^3355FFJournal^000000. I promise that it will be an interesting read."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grandpa Turtle",
                        args![
                            "Ah...",
                            "The fire of youth!",
                            "When the winds die, may your dreams fill your sails~ Good luck to you!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.lines_as("Grandpa Turtle", args!["Grrrr...!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Grandpa Turtle",
                args!["I may not know much, but I won't talk to anyone motivated solely by greed. Go home!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            if (runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(65536))?.is_true() || ctx.var("turtle").get()?.is_true()) {
                if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(65536))?.is_true() {
                    ctx.var("turtle").set(Val::from(0))?;
                }
                ctx.lines_as(
                    "Grandpa Turtle",
                    args![
                        "Well...",
                        "If you go to eastern side of the Alberta port, you should find an old ferryboatman."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grandpa Turtle",
                    args!["His name is ^3355FFGotanblue^000000. I believe he may know a way to Turtle Island. Heh heh, good luck~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Grandpa Turtle", args!["Turtle Island...?"])?;
            ctx.next()?;
            ctx.lines_as("Grandpa Turtle", args!["My, that place is difficult to find and even more difficult to travel to. You're sure you want to go? I would love to give you some advice, but I feel so dizzy after drinking so much..."])?;
            ctx.next()?;
            ctx.lines_as("Grandpa Turtle", args!["Ho ho ho...", "Why don't you talk to a chubby little sailor I know named ^3355FFGotanblue^000000. You can find him loitering on one of the Alberta ports."])?;
            ctx.next()?;
            ctx.lines_as(
                "Grandpa Turtle",
                args!["Tell him that I sent you, and he may tell you more about Turtle Island. He may even know how to get there~"],
            )?;
            ctx.next()?;
            ctx.lines_as("Grandpa Turtle", args!["Well, then...", "Good luck~"])?;
            ctx.var("turtle").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(11029)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Grandpa Turtle",
                args!["Oooh...", "Even at this age...", "I don't understand", "how I can drink so much..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn grandpa_turtle_tur(ctx: &Ctx) -> Script {
    grandpa_turtle_tur_body(ctx, Vec::new()).map(|_| ())
}

fn sailor_alberta_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("turtle").get()?.is_true() || runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(65536))?.is_true()) {
        ctx.lines_as(
            "Gotanblue",
            args![
                "Heh...",
                "Your eyes...",
                "I can tell you're curious about Turtle Island. I guess you must have been talking to that drunken old man!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Do you know about Turtle Island?:How can I get there?:Stop talking.")],
        )? {
            1 => {
                ctx.lines_as("Gotanblue", args!["Turtle Island...?", "Well, first I think it's fair to warn you that Turtle Island took the lives of my buddies. That's right, I was part of ^3355FFJornadan Niliria^000000's crew."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args![
                        "We wanted to find out if the legends about that island were true, so we left everything behind to learn the truth."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Jornadan Niliria was one of the greatest treasure hunters ever, and he gathered men for his crew from around the globe. I'm proud to say that each member of our crew was the best in his field."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["I happened to be the best at navigating. I was only twenty, but I was still invited to his team. It was such an honor to be accepted as an equal amongst these great men."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Anyway, our biggest clue about Turtle Island's location hinted that it was near Alberta, so our voyage began from here."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["We sailed day and night, drifting for weeks, until one day, we were surrounded by an incredibly thick fog. We had no idea of which direction we were going and the mist wouldn't clear."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args!["But none of us had any regrets and we just kept moving onward. There would be no turning back!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Finally, a coral reef sudden appeared and we couldn't steer away from it. Our ship was critically damaged. But when the fog cleared, we saw that we had finally arrived at Turtle Island! It was real!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args!["After we made camp on shore and explored the island, we learned something quite remarkable."],
                )?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["There was a man who had actually made it to Turtle Island before us. His records, however, never reached Rune-Midgarts. He was a great swordmaster known simply as ^3355FFOne^000000."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["According to his records, he had traveled alone to find Turtle Island and made it by himself. But we couldn't find any sign of him anywhere..."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["As we explored Turtle Island further, we learned that not only was it abundant with treasure, but that it also held an item of great interest."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args!["From the first explorer's record, some kind of amazing ^3355FFjewel fragment^000000 was mentioned."],
                )?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["He had traveled to Turtle Island with the purpose of discovering secrets to the sword arts, and stumbled upon a jewel he claimed was the most beautiful in the world."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args![
                        "By the ^3355FFOne^000000's records, we were able to thoroughly explore all to be seen on Turtle Island. However..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Even after months of searching, we could never find a trace of the jewel fragment. Eventually, we had to give up on our search."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Finally, we packed our things to leave the island. But then, as we were traveling at sea, we encountered a thick, blinding fog once again."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["We spent a month trying to steer through that mist. When we finally sailed out of that mist, we saw that we had arrived at the other side of Turtle Island!"])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Our spirits were crushed!! We tried again and again to leave that place, and spent nearly a year trying to leave but we kept winding up at Turtle Island's shores."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["As we struggled with the fog, we lost our comrades one by one. In the end, Jornadan, the drunken old man you met, and myself were the only ones who were able to return home to Alberta."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["We were the only ones who did not give in to despair, and doggedly held onto our hopes. Still, it was by pure luck that we found a way back to Rune-Midgarts."])?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["Well, that's my story. You know, if you want to learn more about Turtle Island, why don't you speak to the scholar on the eastern port of Alberta?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args![
                        "Heh heh...",
                        "I'm sure he can tell give you details that even I wouldn't be able to provide..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Gotanblue",
                    args!["After my story of Turtle Island, you're still not afraid of going? I'm impressed! Alright then..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Gotanblue", args!["If you wish for me to guide you there, I will charge you 10,000 zeny. I'm the only navigator that can guide you there and back safely."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Turtle island -> 10000 zeny:Cancel")])?) == 1 {
                    if ctx.var("Zeny").get()?.number()? > 9999 {
                        ctx.lines_as(
                            "Gotanblue",
                            args![
                                "Alright!!",
                                "You've made your choice! With my experience, we will arrive without fail! I appreciate your spirit!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FF*Choo Choo*^000000")?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                        ctx.call(Function::Warp, vec![Val::from("tur_dun01"), Val::from(157), Val::from(39)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Gotanblue", args!["Hmmm...", "Sorry, but you don't have enough zeny. I hope you understand that I can let my expertise and experience be undervalued..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Gotanblue",
                    args![
                        "Alright then...",
                        "Well, if the spirit of adventure should happen to grab you, I will be here waiting."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Gotanblue", args!["Heh..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Gotanblue",
                    args!["Come back whenever you feel you're ready to hear my story, will you?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as("Gotanblue", args!["Ahhhh...!", "Just look at that ocean! It covers the earth as far as the eye can see. Tell me that's not one of the most beautiful things you've ever seen..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sailor_alberta(ctx: &Ctx) -> Script {
    sailor_alberta_body(ctx, Vec::new()).map(|_| ())
}

fn sailor_tur2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Gotanblue", args!["Do you want", "to return", "to Alberta?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Go to Alberta:Stop talking")])?) == 1 {
        ctx.lines_as("Gotanblue", args!["Heh heh...", "I certainly hope you've found what you were looking for. Alright, I guess there's always a time for an adventurer to return home..."])?;
        ctx.next()?;
        ctx.mes("^3355FF* Choo Choo *^000000")?;
        ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(241), Val::from(115)])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sailor_tur2(ctx: &Ctx) -> Script {
    sailor_tur2_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_scholar_alberta_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jornadan Niliria",
        args!["Every single place", "has its own unique", "smells, sights and sounds."],
    )?;
    ctx.next()?;
    ctx.lines_as("Jornadan Niliria", args!["Even the great, ever expanding sky that's shared by all the peoples of the earth looks strange when you're in a new and foreign land."])?;
    ctx.next()?;
    ctx.lines_as(
        "Jornandan Niliria",
        args!["Heh heh...", "Just like my", "time on Turtle Island.", "Hah Hah Hah~"],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("About Turtle island:You're Jornadan Niliria?!:Stop talking")])? {
        1 => {
            ctx.lines_as("Jornadan Niliria", args!["Turtle...", "Island..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Jornadan Niliria",
                args!["It's almost silly, but Turtle Island was named simply because it's shaped just like a turtle."],
            )?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["Now, Turtle Island is surrounded by a dense fog. When we were first stuck in it, a lot of us believed it was the result of a curse, or some kind of magic."])?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["But in actuality, the fog is the result of a natural phenomenon. It's created by a waterfall inside a cave located on Turtle Island's coast."])?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["Heh heh...", "Once you attain understanding, the truth becomes so simple. I guess confusion and fear is the fog that clouds your judgment, your path on life."])?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["Believe it or not, I too used to fear the dangers of Turtle Island. But now that I understand most of its secrets, I look at my experiences at Turtle Island with fondness."])?;
            ctx.next()?;
            ctx.lines_as(
                "Jornadan Niliria",
                args!["Still, there is one thing I haven't yet uncovered. The ^3355FFunknown jewel fragment^000000."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jornandan Niliria",
                args!["So, until I find it, I'll keep researching and learn as much as I can until I do. Ha ha ha ha~!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Jornadan Niliria", args!["Hmm...?", "You've heard of me?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Jornadan Niliria",
                args![
                    "Ah...",
                    "I suppose that you know about my treasure hunting days, and probably about Turtle Island."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jornadan Niliria",
                args!["Well, as you can see, I'm a little older now. These old bones aren't what they used to be."],
            )?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["But perhaps, someday, if my research bears fruit, I'll venture out once more and seek the one treasure I've never found..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as("Jornadan Niliria", args!["When you want to discover the truth, never give in to despair. If you never give up your search, the answer will come to you."])?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as("Jornadan Niliria", args!["By the way, I'm pretty hungry. Why doesn't Alberta have any good restaurants?! I hear they have good dimsum in Kunlun..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn turtle_scholar_alberta(ctx: &Ctx) -> Script {
    turtle_scholar_alberta_body(ctx, Vec::new()).map(|_| ())
}

fn letter_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Explorer's Letter",
        args![
            "- O / X / XOVX -",
            "If you find this letter, please don't disregard what you have read."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Explorer's Letter",
        args!["Although we have found Turtle Island, it seems our expedition will fail."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Explorer's Letter",
        args!["Only half of our crew is left, and we only have enough food for ten more days. Our condition is truly grave."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Explorer's Letter",
        args!["This damn island must be cursed. There's nothing to eat and we're so close to starving!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Explorer's Letter",
        args!["If we don't get help or find Alberta soon...", "We'll..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn letter_tur(ctx: &Ctx) -> Script {
    letter_tur_body(ctx, Vec::new()).map(|_| ())
}

fn voyage_log_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
        ctx.lines_as("Voyage log", args!["03:20 am", "There's no light from the stars tonight, and we can't even see a hundred meters ahead of us. The men seem to feel something bad in the air."])?;
        ctx.next()?;
        ctx.lines_as("Voyage Log", args!["The faces of my comrades betrayed their fears. They couldn't manage to sleep last night. I hope we will see sunrise soon..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Voyage Log",
            args![
                "04:10 ",
                "5 minutes ago, our comrade, Cooker, was killed when the mast suddenly cracked and fell on his head."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Voyage Log", args!["The estimated time of death is 04:07. The mast broke due to the shock of the ship hitting a reef. The left side of the deck also suffered from serious damage."])?;
        ctx.next()?;
        ctx.lines_as("Voyage Log", args!["04:45", "While two of our workers were fixing the bottom of the deck, they were attacked by monsters that had snuck through cracks in the deck."])?;
        ctx.next()?;
        ctx.lines_as("Voyage Log", args!["These two were lost in the attack. The estimated time of death was 04:32. Fortunately, we are able to stay afloat, but we must hurry and find land."])?;
        ctx.next()?;
        ctx.lines_as(
            "Voyage Log",
            args!["From the damage to the deck, we've lost 30% of our supplies. At this rate, we'll run out of food soon."],
        )?;
        ctx.next()?;
        ctx.lines_as("Voyage Log", args!["05:23", "There seem to be more and more reefs to steer around, and they are growing larger as we travel. I wonder if we'll find land soon..."])?;
        ctx.next()?;
        ctx.lines_as("Voyage Log", args!["Written by", "Captain Jornadan Niliria"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("turtle").get()? == 1 {
            ctx.mes("^3355FFThe paper is torn and seaweed and mold are stuck to the paper. It seems this log is in very poor condition...^000000")?;
            ctx.next()?;
            let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
            if subject1 == 1 {
                ctx.mes("^3355FFThere is a banana leaf between some of the pages. Here is what is written.^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "O / X date",
                        "Just after we arrived on Turtle Island, we frantically searched all over for food."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["It was so bad, that you could see the bones through our skin."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "X / X date",
                        "We found some kind of fruit to eat! It's covered in some sort of yellow skin and looks like a banana!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "XO / X date",
                        "Well, it wasn't exactly like the bananas we have in Rune-Midgarts, but it was very similar."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "O / O date",
                        "In the middle of the night, one of the men reported that he felt sick after eating the food."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "OO / O date",
                        "Another crew member, Berot Berot, was also found to have severe indigestion."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["It's now becoming very clear that the food we have been eating contains some kind of poison."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "XO / O date",
                        "The third person to experience indigestion passed away today. We are all very worried."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "Our suspicions were confirmed when we found that the animals on Turtle Island wouldn't eat the bananas we found."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "OX / O date",
                        "We decided to seal away this poisonous fruit, but learned that it didn't rot, not even after we removed the skin."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["We have no idea why it's harmful to eat, or why it never perishes, but it be of some medical or scientific interest."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["In the meantime, we've decided to bury this fruit until we can get back to Alberta."],
                )?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["^FF3355tur_dun01^000000", "^FF3355X : 160 , Y : 81^000000"])?;
                ctx.next()?;
                ctx.mes("^3355FFIn the Voyage log is a thin key marked with a skull. You've taken this Skull key, as it may be of some use later.^000000")?;
                ctx.var("turtle").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11029), Val::from(11030)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject1 == 2 {
                ctx.mes(
                    "^3355FFYou notice a page with a stamp shaped like a bird's foot. The black ink smells sort of like fruit.^000000",
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "X / OO date",
                        "We've found evidence that other people were here, and that we are not the first to find Turtle Island."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["There was at least one person who made it here before us. Hopefully, his records are on the island somewhere."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["Starting tomorrow, we will begin searching for these hidden records."],
                )?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["O / OO date", "We are having difficulty trying to find any records. Is it possible that the first people here wrote nothing of their journey?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["XO / OO date", "We finally found the records we have been searching for."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "His notes were so hard to find because only one man, rather than a whole team, came to Turtle Island before us!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "This man was a swordsman simply known as 'One.' His records tell much about what can be found on Turtle Island."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["His records were written on animals skin, but they were durable and still easy to read after all this time."],
                )?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["According to his notes, Turtle Island consists of 4 levels. Although there are no people here, but there are many traps and devices which operate through a mysterious force."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["Still, it is certain that something has some control over Turtle Island."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "To keep this record safe, our crew has decided to hide this valuable record on the second level of Turtle Island."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["Search here...", "^FF3355tur_dun02^000000", "^FF3355X : 132 , Y : 251^000000"],
                )?;
                ctx.next()?;
                ctx.mes("^3355FFThere is a picture of a tree in which there is a small keyhole clearly shown under the roots.^000000")?;
                ctx.next()?;
                ctx.lines(args!["^3355FFYou've gained^000000", "^3355FFthe Roots key.^000000"])?;
                ctx.var("turtle").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11029), Val::from(11031)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject1 == 3 {
                ctx.mes("^3355FFThese pages of this log are soiled with mud, and some of them are missing.^000000")?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["O / XX date", "The greatest treasure of Turtle Island, the likes of which have never been seen, is hidden, sealed in some secret place..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["We have spent many days searching for it, but found not one trace."],
                )?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["In the meantime, we've collected many precious treasures. We've decided to take only some of it back home, and to leave the rest here."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args![
                        "XO / XX date",
                        "We have hidden the treasure we have left behind to keep it from getting stolen."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Voyage Log", args!["It is somewhere on the fourth level, the bottom of the island. The treasure is sealed in a box that is a relic from an ancient culture."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["However, the technology of this treasure box is very sophisticated, and it won't be easy to open by force."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Voyage Log",
                    args!["Here...", "^FF3355tur_dun01^000000", "^FF3355X : 203 , Y : 155^000000"],
                )?;
                ctx.next()?;
                ctx.mes("^3355FFYou find three small holes under the turtle stone. Within one of these holes is a thin key.^000000")?;
                ctx.next()?;
                ctx.lines(args!["^3355FFYou've gained ^000000", "^3355FFthe Security key^000000"])?;
                ctx.var("turtle").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11029), Val::from(11032)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.mes("^3355FFYou've closed the voyage log.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn voyage_log_tur(ctx: &Ctx) -> Script {
    voyage_log_tur_body(ctx, Vec::new()).map(|_| ())
}

fn skull_stone_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 2 {
        ctx.lines(args![
            "^3355FFUnder the stone^000000",
            "^3355FFis a tiny key hole^000000",
            "^3355FFwith a skull mark.^000000",
            "^3355FFYou used the Skull key^000000",
            "^3355FFin that key hole.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FF*Click! Click!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFSuddenly, the top of^000000",
            "^3355FFthe stone opened and^000000",
            "^3355FFsome items popped out!!^000000"
        ])?;
        ctx.next()?;
        ctx.var("misc_quest")
            .set(runtime::op(&ctx.var("misc_quest").get()?, "|", &Val::from(65536))?)?;
        ctx.var("turtle").set(Val::from(0))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(11030)])?;
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject1 == 1 {
            ctx.call(Function::GetItem, vec![Val::from(532), Val::from(5)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FF5 Banana Juice^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 2 {
            ctx.call(Function::GetItem, vec![Val::from(513), Val::from(5)])?;
            ctx.lines(args!["^3355FFYou've gained^000000", "^3355FF5 Banana^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 3 {
            ctx.call(Function::GetItem, vec![Val::from(513), Val::from(5)])?;
            ctx.call(Function::GetItem, vec![Val::from(532), Val::from(5)])?;
            ctx.lines(args![
                "^3355FFYou've gained^000000",
                "^3355FF5 Banana and^000000",
                "^3355FF5 Banana Juice^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines(args![
        "^3355FFIt is a frightening^000000",
        "^3355FFstone tomb with a^000000",
        "^3355FFhorrible skull on it.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn skull_stone_tur(ctx: &Ctx) -> Script {
    skull_stone_tur_body(ctx, Vec::new()).map(|_| ())
}

fn turtle_tree_roots_tur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("turtle").get()? == 3 {
        ctx.lines(args![
            "^3355FFUnder the tree roots^000000",
            "^3355FFis a tiny key hole^000000",
            "^3355FFmarked with a root insignia.^000000",
            "^3355FFYou used the Roots key^000000",
            "^3355FFin that key hole.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FF*Swishy swashy!*^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFBetween the roots^000000",
            "^3355FFa little door opens,^000000",
            "^3355FFrevealing an^000000",
            "^3355FFold scroll.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Old scroll",
            args![
                "^FF3355Following the^000000",
                "^FF3355legend from^000000",
                "^FF3355my village,^000000",
                "^FF3355I've come^000000",
                "^FF3355to improve my^000000",
                "^FF3355swordmanship...^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Old scroll",
            args![
                "^FF3355Only one who^000000",
                "^FF3355cuts the shell^000000",
                "^FF3355of a turtle^000000",
                "^FF3355can become a^000000",
                "^FF3355grand master^000000",
                "^FF3355of the sword...^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Old scroll",
            args![
                "^FF3355To fufill this^000000",
                "^FF3355legend, I have^000000",
                "^FF3355come. I, 'One,'^000000",
                "^FF3355will cut the^000000",
                "^FF3355turtle's shell!!^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Old scroll",
            args!["^FF3355Go...^000000", "^FF3355tur_dun02^000000", "^FF3355X : 46 , Y : 125^000000"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFInside the pile of turtle^000000",
            "^3355FFcrystals, a scroll is hidden.^000000"
        ])?;
        ctx.var("turtle").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(11031), Val::from(11033)])?;
        ctx.lines(args!["^3355FFYou've gained a^000000", "^3355FFTurtle Crystal key.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThere are old,^000000",
        "^3355FFthick tree roots^000000",
        "^3355FFanchored into^000000",
        "^3355FFthe ground here.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turtle_tree_roots_tur(ctx: &Ctx) -> Script {
    turtle_tree_roots_tur_body(ctx, Vec::new()).map(|_| ())
}
