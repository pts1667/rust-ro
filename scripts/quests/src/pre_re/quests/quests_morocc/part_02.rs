use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn prince_poe_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_card = Val::from(0);
    let mut l_p_a = Val::from(0);
    let mut l_p_b = Val::from(0);
    let mut l_p_c = Val::from(0);
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    let mut l_wincount = Val::from(0);
    if ctx.var("nk_prince").get()?.number()? > 6 {
        ctx.lines_as(
            "Poe",
            args!["Whatever the condition is, he is just a loser in the match of life."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(10018)])? == 2 {
        ctx.lines_as("Poe", args!["He gave up without any challenge. That's not what the man has to do. He is not as good as me. But okay. I am disappointed in him. Eigen Ahrum."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10018)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10018)])? == 1)
    {
        ctx.lines_as("Poe", args!["......"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nk_prince").get()?.number()? < 5 {
        ctx.lines_as(
            "Prince",
            args![
                "Hi, how are you?",
                "I don't think you deserve",
                "to be here.",
                "What about setting straight your genealogy first and then come?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as(
            "Prince",
            args![
                "Come here.",
                "You are the adventurer!",
                "I love the challenge of hard trips too. Talkative old men and their adventures... You can't make me stop adventuring..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Prince",
            args![
                "My name is Poe.",
                "I am the prince of the Richard family. Remember me, and be my supporter..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Poe", args!["Above all, I want to", "test your ability as an adventurer. The ability test is not so serious. I just want to know how accurate your intuition is... That's all."])?;
        ctx.next()?;
        ctx.lines_as(
            "Poe",
            args![
                "I don't want to talk with",
                "a person of low intuition.",
                "Let me explain briefly,",
                "and test your own intuition."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Poe", args!["My card has numbers from 1 to 13. You give me an answer about the card number I pick, whether it's a lesser or higher number."])?;
        ctx.next()?;
        ctx.lines_as("Poe", args!["Ah, of course, 7 is the middle number. 7 means no success or no failure. The goal of this game is to give the right answer 2 times in a row. Let's begin!"])?;
        ctx.next()?;
        'l1: loop {
            if !(l_wincount.clone().number()? < 2) {
                break 'l1;
            }
            'b1: {
                l_card = ctx.call(Function::Rand, vec![Val::from(1), Val::from(13)])?;
                ctx.lines_as("Poe", args!["Yes, now choose one", "from higher and lower.", "Just one."])?;
                ctx.next()?;
                'b2: {
                    match runtime::select_values(ctx, &[Val::from("Higher:Lower")])? {
                        1 => {
                            ctx.lines_as("Poe", args!["Hmm... higher?...", "I now pick a card!"])?;
                            ctx.next()?;
                            ctx.lines_as("Poe", args![((Val::from("It is...") + l_card.clone()) + Val::from("!!"))])?;
                            ctx.next()?;
                            if l_card.clone().number()? > 7 {
                                l_wincount = (l_wincount.clone() + Val::from(1));
                                ctx.lines_as("Poe", args!["Whooah, you gave the right answer!"])?;
                                if l_wincount.clone() == 2 {
                                    ctx.mes("You won 2 times in a row right?...")?;
                                    ctx.next()?;
                                    break 'b2;
                                } else {
                                    ctx.mes("But you just gave the correct answer 1 time, as of yet.")?;
                                    ctx.next()?;
                                }
                            } else if l_card.clone() == 7 {
                                ctx.lines_as("Poe", args!["This game is a draw.", "Do better next time."])?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as("Poe", args!["Wrong...", "Visit Hollgrehenn and ask him to refine your luck."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as("Poe", args!["Eh, lower?...", "Look at my card!"])?;
                            ctx.next()?;
                            ctx.lines_as("Poe", args![((Val::from("It is...") + l_card.clone()) + Val::from("!!"))])?;
                            ctx.next()?;
                            if l_card.clone().number()? < 7 {
                                l_wincount = (l_wincount.clone() + Val::from(1));
                                ctx.lines_as("Poe", args!["Whooah, you gave the right answer!"])?;
                                if l_wincount.clone() == 2 {
                                    ctx.mes("You won 2 times in a row right?...")?;
                                    ctx.next()?;
                                    break 'b2;
                                } else {
                                    ctx.mes("But you just gave the correct answer 1 time, as of yet.")?;
                                    ctx.next()?;
                                }
                            } else if l_card.clone() == 7 {
                                ctx.lines_as("Poe", args!["This game is a draw.", "Do better next time."])?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as("Poe", args!["Wrong...", "Visit Hollgrehenn and ask him to refine your luck."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        ctx.lines_as(
            "Poe",
            args!["Good, it's natural for me to disclose my words to such a high-intuitive person that likes pulling out all the cards."],
        )?;
        ctx.next()?;
        ctx.lines_as("Poe", args!["I could know about you with shown cards, very well, but let me share my hidden card this time! Ask me whatever you want to know."])?;
        ctx.next()?;
        ctx.lines_as("Poe", args!["So... What do you want to know?!"])?;
        ctx.next()?;
        'l3: loop {
            if !(true) {
                break 'l3;
            }
            'b3: {
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Your background story...:Your view of the nation...:Hobbies and interests...:I will come by later.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Poe",
                            args![
                                "My background...",
                                "Past stories.",
                                "I don't give it importance at all, so I can't remember it clearly..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poe",
                            args!["Can I just say I am a survivor from many matches and battles. That is quite fit for me. Hahahahaha!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["I've had to cope with many accidents and challenges... from which I have survived, and have been pursuing new things. Life means..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poe",
                            args!["Never let yourself down with given cards, and aim at deadly strokes!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["......"])?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["Hey it's cool, huh?", "We have same tastes, haven't we?"])?;
                        l_p_a = Val::from(1);
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Poe",
                            args!["My opinion is a nation should provide the minimum guard for people, to guarantee respective freedom.."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["Men should challenge the stuff that they can be passionate about, so the free action of people should not be obstructed by their nation."])?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["If I were King, all the restrictions would be removed and liberal life would be allowed to everyone in an invulnerable range."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poe",
                            args!["Coming to think of my family, my people are so conservative and not flexible."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["They keep saying to not do this and that, that is dangerous and this is natural... The nagging has been endless all throughout my life..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poe",
                            args![
                                "Ah, those last words",
                                "are off-the-record.",
                                "I don't want to be bothered by my family."
                            ],
                        )?;
                        l_p_b = Val::from(1);
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as("Poe", args!["Experiencing anything new!", "I don't care what it is!"])?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["If our time is the exploring age, I possibly am the captain of an exploring group! Unexplored, pathfinding, investigating... Those are my middle names. How about you??"])?;
                        ctx.next()?;
                        ctx.lines_as("Poe", args!["But many say, about my character, that I am addicted to gambling. I am just full of a challenging spirit and I like raking in money! I am a free-spirited normal man."])?;
                        l_p_c = Val::from(1);
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as(
                            "Poe",
                            args!["Alright,", "let's meet next at some more exciting and dangerous spot."],
                        )?;
                        if ((l_p_a.clone() + l_p_b.clone()) + l_p_c.clone()) == 3 {
                            ctx.call(Function::CompleteQuest, vec![Val::from(10007)])?;
                        }
                        l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                        l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                        l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                        l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                        l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                        l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                        l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                        if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                            + l_prin6.clone())
                            + l_prin7.clone())
                            == 14
                        {
                            ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn prince_poe(ctx: &Ctx) -> Script {
    prince_poe_body(ctx, Vec::new()).map(|_| ())
}

fn prince_peter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    let mut l_quest = Val::from(0);
    if ctx.var("nk_prince").get()?.number()? > 6 {
        ctx.lines_as("Peter", args!["Appraiser.", "Have you met the little girl in Aldebaran?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, I saw her for you.:No, I don't want to.")])? {
            1 => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10014)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10014)])? == 1)
                {
                    ctx.lines_as(
                        "Peter",
                        args!["I am glad that the girl liked it. Once I got cured, I wanted to go meet with her."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Peter", args!["Anyhow, thanks for your good work. I really feel sorry about bothering you with trifle things. This is my sense of gratitude. Don't feel so much burden and take this."])?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(10014)])?;
                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Peter",
                    args![
                        "Thinking about the little girl calms me. I still can't believe my eyes. My prince Ahrum who just passed away...~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if (ctx.call(Function::CheckQuest, vec![Val::from(10014)])? == 0
                    || ctx.call(Function::CheckQuest, vec![Val::from(10014)])? == 1)
                {
                    ctx.lines_as(
                        "Peter",
                        args![
                            "As you said.",
                            "I've heard that her life is like a flower when I lost it.",
                            "I will go see her on my own."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Peter",
                        args!["I really feel sorry about bothering you with trifle things. Please take this."],
                    )?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(10014)])?;
                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Peter",
                    args![
                        "Never mind.",
                        "I still can't believe my eyes. My prince Ahrum who just passed away...~~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(10019)])? == 2 {
        ctx.lines_as("Peter", args!["Child... What the heck happened to you?..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10019)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10019)])? == 1)
    {
        ctx.lines_as("Peter", args!["......"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, vec![Val::from(10008)])? == 2 {
        l_quest = ctx.call(Function::CheckQuest, vec![Val::from(10014)])?;
        if l_quest.clone() == 2 {
            ctx.lines_as("Peter", args!["I really appreciate it.", "You are so kind."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (l_quest.clone() == 0 || l_quest.clone() == 1) {
            ctx.lines_as(
                "Peter",
                args!["I am glad that the girl liked it. Once I got cured, I wanted to go meet with her."],
            )?;
            ctx.next()?;
            ctx.lines_as("Peter", args!["Anyhow, thanks for your good work. I really feel sorry about bothering you with trifle things. This is my sense of gratitude. Don't feel so much burden and take this."])?;
            ctx.call(Function::CompleteQuest, vec![Val::from(10014)])?;
            ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Peter", args!["Do you have any more", "business, appraiser?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("nk_prince").get()?.number()? < 4 {
        ctx.lines_as("Prince", args!["I think you are not allowed", "to be here."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as(
            "Prince",
            args!["Hello,", "I am Peter, from the family of Heine. I am glad to meet you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Peter", args!["Now, where shall we start?"])?;
        ctx.next()?;
        'l2: loop {
            if !(true) {
                break 'l2;
            }
            'b2: {
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Your background story...:Your view of the nation...:Hobbies and interests...:I will come by later.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Peter", args!["My family was not that influential. My childhood was not that abundant; I had to seek jobs and make money from part-time jobs."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Peter",
                            args!["But it helped me to check people's lives; the full particulars. That's my good side."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Peter", args!["Was it enough for your question?"])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Peter", args!["I guess the important thing is if people fulfill themselves in their field, then only supplements of support is needed from the nation's side.", "What I want to say, in short, is the nation does not reign. People need to keep peace with people."])?;
                        ctx.next()?;
                        ctx.lines_as("Peter", args!["Accordingly, I will take benefits away from the difference of classes. It is not an equality issue, but for the ascension of national power."])?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as("Peter", args!["I only like reading books as a hobby. I think men should go forward; they need to have a striving attitude."])?;
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as("Peter", args!["Ah, is that it?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Peter",
                            args!["Then I have a favor to ask of you... Can you listen to my story?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Peter", args!["Originally, I didn't like growing a beard... but I had one opportunity to change... My thoughts recently are that someone has offered me the chance."])?;
                        ctx.next()?;
                        ctx.lines_as("Peter", args!["It is because of an unknown girl's letter..."])?;
                        ctx.next()?;
                        ctx.lines_as("Peter", args!["What she told me in the letter was that my image was too sharp, and if I grew my beard, the contour of my face would be quite hidden; and it would look far better."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Peter",
                            args![
                                "I want to repay her.",
                                "Because her words helped me much. Nothing special, but maybe a small bunch of flowers."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Peter",
                            args!["Would you take these flowers and give thanks to that girl somewhere in Al de Baran?"],
                        )?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(10008)])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Okay, no problem.:Please ask it of another.")])? {
                            1 => {
                                ctx.lines_as("Peter", args!["Yeah, thank you very much.", "Please take care."])?;
                                ctx.call(Function::GetItem, vec![Val::from(744), Val::from(1)])?;
                                ctx.call(Function::SetQuest, vec![Val::from(10013)])?;
                                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                                    + l_prin6.clone())
                                    + l_prin7.clone())
                                    == 14
                                {
                                    ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Peter", args!["I understand you...", "I know you have your", "own business."])?;
                                ctx.call(Function::SetQuest, vec![Val::from(10013)])?;
                                l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                                l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                                l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                                l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                                l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                                l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                                l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                                if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                                    + l_prin6.clone())
                                    + l_prin7.clone())
                                    == 14
                                {
                                    ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn prince_peter(ctx: &Ctx) -> Script {
    prince_peter_body(ctx, Vec::new()).map(|_| ())
}

fn girl_prince_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::CheckQuest, vec![Val::from(10013)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10013)])? == 1) {
        if ctx.call(Function::CountItem, vec![Val::from(744)])?.number()? > 0 {
            ctx.lines_as(
                "Girl",
                args!["Wooah, Uncle Peter sent these flowers", "for me?", "Hmmm~ They smell really good."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Girl",
                args![
                    "Hehe~ You must be a friend of Peter's.",
                    "When you have a chance to meet him",
                    "later, please give my regards to uncle Peter.",
                    "Tell him that I loved the flowers so so much...",
                    "Okay, see you later."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Girl",
                args!["I will go home early and", "Put these flowers in a vase.", "Lalala~"],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(744), Val::from(1)])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(10013), Val::from(10014)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Girl", args!["Hey you, have you heard the story of Daddy-Long-Legs? ... ..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Girl", args!["Hey you, have you heard the story of Daddy-Long-Legs? ... ..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn girl_prince(ctx: &Ctx) -> Script {
    girl_prince_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    if ctx.var("nkprince_eisen").get()?.number()? > 2 {
        if ctx.var("nkprince_eisen").get()? == 3 {
            ctx.lines_as(
                "Ahrum",
                args![
                    "How is Ernst ...?",
                    "He is so serious about everything. He doesn't make trouble or have outbursts... He can be kind of weird."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Ahrum", args!["We were close friends. Unlike me, he is kind and compassionate; a distinct official of public affairs. When he has to be strict, he turns strict with everyone."])?;
            ctx.var("nkprince_eisen").set(Val::from(4))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("nkprince_eisen").get()? == 4 {
                ctx.lines_as("Ahrum", args!["I am Eigen Ahrum,", "I will be the light of Rune-Midgarts."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("nkprince_eisen").get()? == 5 {
                    ctx.lines_as(
                        "Ahrum",
                        args!["Whoever be the king, let's make Rune-Midgarts... the best country in the world, by helping each other."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args![
                            "We can make it.",
                            "No, we shall make it. For Brother and I together, we shall make it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["But... If I am to reign, or help you; and if I am corrupted... you kill me... by your hand."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["In the opposite case, I will kill you too, you understand? So don't ever stray, and do not ever forget your determination."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Brother, you think about hell. It will never happen, so... Don't talk that way."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["It is an assumption.", "Don't take it too seriously."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args![
                            "Alright, alright. Don't",
                            "take it the wrong way.",
                            "I have been acting for Brother and Kingdom, and I will be the same ever after."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.lines_as(
                        "Ahrum",
                        args![
                            "Ah, the appraiser has come.",
                            "How could you appear so suddenly? Ern and I were in the middle of conversation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["What's wrong, brother? We were not talking about anything bad. We were just making our resolution firm."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ahrum",
                        args!["You came here in good timing. We've just decided to kill the other... if one of us is corrupt."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Brother?!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["By all means, be the witness of this agreement. You cannot say no. Because it is a very good thing, in some respects, for your work, isn't it?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Br...Brother. You are way too suppressive. Our appraiser must have his own position and opinion."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ahrum", args!["You still have a weak personality. I am not forcing something so difficult on him! Who cares? I take silence as a positive answer! My bid is successful! And our promise is formed too!"])?;
                    ctx.next()?;
                    ctx.lines_as("Ernst", args!["Haah...haha... Appraiser, I deeply apologize for offending you. Sorry, but there is no possibility of fulfilling this promise. So, just don't take this situation seriously... You know... When pigs fly... ~"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ernst",
                        args!["Now, let me return to my room. Keep up the good work, appraiser."],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern::OnDisable")])?;
                    ctx.var("nkprince_eisen").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("nkprince_eisen").get()? == 6 {
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "I heard that one person",
                                "from my family is coming today. But I haven't found him yet."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "I worry about your words...",
                                "What do you want to say this time?...",
                                "Have you seen him, appraiser?"
                            ],
                        )?;
                        ctx.var("nkprince_eisen").set(Val::from(7))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(10012), Val::from(10016)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("nkprince_eisen").get()? == 7 {
                            ctx.lines_as("Ahrum", args!["Aahh...boring, boring.", "I am in real penance right now!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("nkprince_eisen").get()? == 8 {
                            ctx.lines_as("Ahrum", args!["You come here so often.", "I know you are a faithful person, engaged in a very important issue for the kingdom, but you come here more than is necessary..."])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("I have something...:Is that so? Then, see you later.")])? {
                                1 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["I saw a person from the Walter family."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Ahrum", args!["Oh! really?", "why didn't he come to me?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["-I tell Ahrum the story.-"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Ahrum", args!["......"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ahrum", args!["What! Are you 100% sure about your story? Ahhhh..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ahrum", args!["......"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ahrum",
                                        args!["I am very sorry, but I want to be left alone. Leave me alone, right now!"],
                                    )?;
                                    ctx.var("nkprince_eisen").set(Val::from(9))?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(273)])?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("Ahrum", args!["Okay, good riddance."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else if (((ctx.var("nkprince_eisen").get()? == 9 || ctx.var("nkprince_eisen").get()? == 10)
                            || ctx.var("nkprince_eisen").get()? == 11)
                            || ctx.var("nkprince_eisen").get()? == 12)
                        {
                            ctx.lines_as(
                                "Ahrum",
                                args!["I am very sorry, but I want to be left alone. Leave me alone, right now!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(273)])?;
                            return Err(Stop::End);
                        } else if ctx.var("nkprince_eisen").get()? == 13 {
                            ctx.lines_as(
                                "Ahrum",
                                args!["I told you that I don't want to be king, and I don't have any intention to change my mind!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ernst",
                                args!["Brother! I don't know why you are suffering so much! Are you this weak a creature?!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ahrum",
                                args!["Suffering? Me?..Aaahhh... It looks so...? Yeahh... Yes, it does. Stressful...Huhuhu...Hahaha!!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Brother, Ahrum...?"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["...Is it only me that gets away... Anyhow, I am a disqualified person ... huhuhu. But, I've made a decision."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Ernst."])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Yes?... Yes?"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Take this."])?;
                            ctx.next()?;
                            ctx.mes("- swish. - ")?;
                            ctx.next()?;
                            ctx.mes("- Ahrum casually throws a Bazerald to Ernst.-")?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ernst",
                                args!["Eh?... What's this about?... A Bazerald of Walter family?...Why?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ahrum",
                                args![
                                    "Now this is perfect timing. Even our witness is here! Ern, you remember our promise clearly, right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["What? Promise? Witness?...Ahh... Re-really..."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Yes, really. Now is the time."])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Nonsense! Do you think I can do that to you?!"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Idiot! Being King should be followed by decisive action. I have no chances to be King. Corruption, and living like that, is worse than being killed by you..."])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["B-but, you can live as you are! As if nothing happened. You are just needed to return to the way you were before. Brother, let me help you. Tell me everything you hide..."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["This is so moronic! Can you say that you are kingly?! Are you showing me sympathy now? You should be a man of sense! If not, you are not eligible as a king candidate!"])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Bu...but!"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Talking back and forth is of no use! I cannot help it. If you keep insisting instead of trying to be a man of sense, then such a person should not be a king."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Ghh?! Brother?!"])?;
                            ctx.next()?;
                            ctx.mes("(dagger thrusting sound)")?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                            ctx.var("nkprince_eisen").set(Val::from(14))?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern::OnDisable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern1::OnEnable")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("nkprince_eisen").get()? == 14 {
                            ctx.lines_as(
                                "Ernst",
                                args![
                                    "Bbb... Brother?... You... told me ",
                                    "you would kill me... You just wanted to be killed",
                                    "by me?... Bbb...brother?..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Huhu... Not at all. I just wanted to kill you... That's it... Good job, Ern... This is legal self-defense, killing a villain... right?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ernst",
                                args!["B..Brother... even now it is not too late! If you go to a medic, you can be restored!"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["It's too late... Once your vital organs are stabbed... I wonder how could I stay alive... or even how I could come back to life... I don't need to live."])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["B..Brother....How...how could you?!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ahrum",
                                args!["Goodbye, my brother... Be King, and change this nation. You... You can make it...guk..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ernst",
                                args!["...Brother...Brother... Why... Why... How can your face look so satisfied? How...? Brother..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Huhu...You don't need to know that. By the way, you kept your promise... You should be king... But if you are lost, and stray, I will come kill you at any time... from hell!!"])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["I...I don't know! I don't know what's going on!"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["You don't need to... What you have to know about is that there was a sacrifice from me. To make my death meaningful, you should be a good king. That's my conviction... That's it..guk..! Don't lose my words... gukkuk."])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Idiot... Moron..."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Ahh... Appraiser, do you know what you have to do? Be mindful of your words. About what you saw, what you heard... You must not tell all of your experience to the inspector."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Ern killed a villain here who bullied around.. I had no chance... He showed decisive action, and that will deserve him the right to be king..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ahrum",
                                args!["If you react wrongfully to this incident, my death will be worthless...kuk... Do you understand?"],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("... Okay I will follow your will...:......")])? {
                                1 => {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["... Okay, I will follow your will. Don't worry..."],
                                    )?;
                                    ctx.next()?;
                                }
                                2 => {
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ahrum",
                                        args!["Ignorance... is positive... It means... My bid... is successful..."],
                                    )?;
                                    ctx.next()?;
                                }
                                _ => {}
                            }
                            ctx.lines_as(
                                "Ahrum",
                                args!["Now... I can die... with peace... Thank... you very... much, both... of you."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Brother..."])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["Then, perpetually... Bye... Sorry ... now I can't go fishing."])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["B...bro?"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["......"])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["B... bro, brother!"])?;
                            ctx.next()?;
                            ctx.lines_as("Ahrum", args!["......"])?;
                            ctx.next()?;
                            ctx.lines_as("Ernst", args!["Brotherrrrrrrr!!!"])?;
                            ctx.next()?;
                            ctx.var("nkprince_eisen").set(Val::from(15))?;
                            ctx.var("nk_prince").set(Val::from(7))?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#another_ern1::OnDisable")])?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(10024), Val::from(10025)])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("prt_castle"), Val::from(318), Val::from(273)])?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    if ctx.var("nk_prince").get()?.number()? < 5 {
        ctx.lines_as("Prince", args!["You are not allowed to be here.", "Hmmm..."])?;
        ctx.next()?;
        ctx.lines_as("Prince", args!["Get out of my way.", "I don't want to confront you."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as("Prince", args!["Are you the new adventurer appraiser? I am Eigen Ahrum from the Walter family. You can call me Prince Ahrum, for short, at your convenience."])?;
        ctx.next()?;
        ctx.lines_as(
            "Ahrum",
            args!["I hope you ask me short and simple questions. Long questions don't always mean special and good answers."],
        )?;
        ctx.next()?;
        'l3: loop {
            if !(true) {
                break 'l3;
            }
            'b3: {
                ctx.mes("-What shall I ask...-")?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I'd like to ask about..."],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Your background story...:Your view of the nation...:Your hobbies or interests...:Never mind, I'll come by later.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Ahrum",
                            args!["You aren't just trying to dissolve your curiosity about the king's family, are you?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["From my childhood, I liked to experience many things and learn them. As a result, I should bear interference from my family. To them, I was not a decent noble family member."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["But I am not sensitive to that, and I think a good king should neglect those trifles."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["Others can say that my behavior makes them embarrased, but if something ends well, they are all satisfied in the end. Am I wrong?"])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["Although I've skipped some lessons in life, I can back up missing lessons easily. Who cares who can swear to me as King! This is how I am."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "Ern nags me a lot",
                                "about this, but I don't make an issue in any case. Such a fastidious jerk."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["Like I said, I've been growing up with his scolds. I feel his words were harsher than others who shut their mouth after I accomplish something."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["He always says that results cannot be everything that matter."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["Before everything else, it is luck and glory for me, that I could be born in Walter, a distinguished noble family."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "Of course, my abilities",
                                "are underestimated because of my background. In other respects, I feel sad about it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["In the coming age, people will talk about me as Eigen Ahrum from the Walter family, not Eigen Ahrum of Walter."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["The Walter family itself has high distinction and fame, but I will make the name shine further, and others will be proud of me."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "The nation I organize",
                                "will not be harmonious",
                                "with pastel color of fairy",
                                "tales."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["For sure, losers and depressed people have their own faults, like lesser challenges for their lives."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["I have some belief in the benefit of unlimited competition. Frankly, I don't like this king election system from noble families."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "Although all these",
                                "status system fade away",
                                "I will be the king, with my sole capability and aptitude! Hahaha."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "I know quite well about",
                                "other king cadidates, personally.",
                                "But I can't find one distinguished person who can lead this kingdom well."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["I'm not saying they aren't outstanding, but considering the current situation of Rune-Midgarts, they don't seem to be astonishing leaders."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["For this reason, I assure you that no candidate is fit for the position among them. If I have to pick someone who can be a match for me, I can say that one is Ernst."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["He has a soft personality and thinks too long about things, but I admit many merits to him. I can say he is a fairly high-standard candidate."])?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["........"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["Hmm, I was too talkative.", "I apologize for my redundant words, hahaha."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Ahrum",
                            args![
                                "I enjoy activities.",
                                "I can't stand passive games,",
                                "like card and board games",
                                "I think those games are",
                                "quite fit for the scholar type people."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["My brother Ern can stick to something for a long time, like books or study."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ahrum",
                            args!["Once I become king,", "I won't handle documents", "while sitting all day."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ahrum", args!["It's very hard for me to do that."])?;
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as("Ahrum", args!["Okay...", "as you please."])?;
                        if ctx.var("nkprince_eisen").get()? == 2 {
                            ctx.var("nkprince_eisen").set(Val::from(3))?;
                        } else {
                            ctx.var("nkprince_eisen").set(Val::from(1))?;
                        }
                        ctx.call(Function::CompleteQuest, vec![Val::from(10005)])?;
                        l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                        l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                        l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                        l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                        l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                        l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                        l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                        if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                            + l_prin6.clone())
                            + l_prin7.clone())
                            == 14
                        {
                            ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn prince_eisen(ctx: &Ctx) -> Script {
    prince_eisen_body(ctx, Vec::new()).map(|_| ())
}

fn prince_ern_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_prin1 = Val::from(0);
    let mut l_prin2 = Val::from(0);
    let mut l_prin3 = Val::from(0);
    let mut l_prin4 = Val::from(0);
    let mut l_prin5 = Val::from(0);
    let mut l_prin6 = Val::from(0);
    let mut l_prin7 = Val::from(0);
    if ctx.call(Function::CheckQuest, vec![Val::from(10024)])? == 2 {
        ctx.lines_as(
            "Ernst",
            args![
                "...brother, if you want...",
                "...Ah, please leave here.",
                "I don't want to see anyone."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CheckQuest, vec![Val::from(10024)])? == 0 || ctx.call(Function::CheckQuest, vec![Val::from(10024)])? == 1)
    {
        ctx.lines_as("Ernst", args!["...Brother Ahrum..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nkprince_eisen").get()?.number()? > 2 {
        if ctx.var("nkprince_eisen").get()? == 3 {
            ctx.lines_as("Ernst", args!["Did you meet my brother Ahrum? He is a great man. Really."])?;
            ctx.next()?;
            ctx.mes("He tries to deal with things too tough that sometimes makes things into a problem. But even at those moments, he is willing to listen to my words; so we seldom seem to be in serious trouble.")?;
            ctx.next()?;
            ctx.lines_as(
                "Ernst",
                args!["That can only be that way, because we two have been so close to each other."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ernst",
                args![
                    "Brother Ahrum has been blaming me for that, I think unnecessarily too long. But he listens to me and acts as I advise."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ernst",
                args!["Isn't he a great person?", "He is a trustworthy and instructive person."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("nkprince_eisen").get()? == 5 {
                ctx.lines_as(
                    "Ernst",
                    args![
                        "Ah, now I am going to have a plan with Bro Ahrum",
                        "If you don't have a important issue with me, I shall go. "
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("nkprince_eisen").get()? == 6 {
                ctx.lines_as(
                    "Ernst",
                    args!["Ah, I heard person from Walter came in here. Have you seen him?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Ernst", args!["It seems that even brother Ahrum hasn't met with him. Hmm..."])?;
                ctx.var("nkprince_eisen").set(Val::from(7))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(10012), Val::from(10016)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("nkprince_eisen").get()? == 9 {
                ctx.lines_as(
                    "Ernst",
                    args!["Welcome, judge.", "We meet quite open", "Why don't you slow down your work?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Ernst", args!["Hmm...I don't have any more words to say. Moreover, Ahrum seems to be strange these days. I worry about that. I hope it's not a big deal."])?;
                ctx.var("nkprince_eisen").set(Val::from(10))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(10017), Val::from(10004)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ((ctx.var("nkprince_eisen").get()? == 10 || ctx.var("nkprince_eisen").get()? == 11)
                || ctx.var("nkprince_eisen").get()? == 12)
            {
                ctx.lines_as(
                    "Ernst",
                    args![
                        "Ahrum is getting weird.",
                        "It's not a normal change, but a real corruption. I feel uneasy about it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ernst",
                    args![
                        "This isn't happening.",
                        "I believe Ahrum will be restored as he was. However, I don't feel good."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("nkprince_eisen").get()? == 15 {
                ctx.lines_as("Ernst", args!["Ah...", "Brother Ahrum...", "Eigen Ahrum...yes, sir."])?;
                ctx.next()?;
                ctx.lines_as("Ernst", args!["......"])?;
                ctx.next()?;
                ctx.lines_as("Ernst", args!["Please leave here. I don't want to see anyone."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Ernst",
                    args![
                        "Welcome, appraiser.",
                        "Thank you for your hard work. It's hard to judge others, isn't it?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    if ctx.var("nk_prince").get()?.number()? < 5 {
        ctx.lines_as(
            "Prince",
            args!["You are not a adventurer judge", "I think you shouldn't be here."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("nk_prince").get()?.number()? > 4 {
        ctx.lines_as("Prince", args!["Hi", "you are the judege.", "I am Ernst, prince of Geoborg."])?;
        ctx.next()?;
        ctx.lines_as(
            "Ernst",
            args!["I think it's hard work that judges people's talents. I will do my best to help you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Ernst", args!["Well, thank you."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["(He seems to have a polite personality as a candidate for King)"],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                ctx.mes("-Well, what questions.-")?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "backgroud first.:I want to know your spirit of nationalism.: Your habbit and tastes.:I will be back.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Ernst", args!["Since I was young, I have grown up with a royal education. I haven't quarreled with others seriously, but I was not very polite either."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ernst",
                            args!["People thought that I was timid, but that's because I didn't want to be shun from others."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["Of course, we got an exception... Eigen."])?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["Eigen Ahrum can be the exception. He is not polite, but he makes sense from others with exact confirmation."])?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["I think that is a great ability, and I should learn it. But, it is not only about results, but also a courtesy matter. We should keep something that we have to do."])?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["For me, the process was more important than the result. Now, I take them as equal virtue. I didn't take results significantly. Ahrum taught me, who used to consider the result as nothing."])?;
                    }
                    2 => {
                        ctx.lines_as("Ernst", args!["The spirit of nationalism when I become King?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ernst",
                            args!["You might believe the power of a king is unlimited, but as one man, it has a limitation."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["Both normal public and prominent people take charge of the society that gathers that wisdom is easier than doing things on your own."])?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["Ahrum will help me to supplement any shortcomings. Not only Ahrum, but all ministers and cabinet members; and the public is my master."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ernst",
                            args!["It is my fortune that I got many masters beside Ahrum. It is also good for my country."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Ernst",
                            args!["II don't have special habits. I am just willing to learn and read books."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ernst",
                            args!["And I also love to talk with Ahrum. Ahrum is clever enough to generate new ideas that I can follow."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["Well, to tell the truth, going fishing with Ahrum is better. Although, we are too busy to go fishing because of all these kingdom issues."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ernst",
                            args!["We promised to go fishing after the taking of the throne... Let's see..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ernst", args!["Hmm...is that enough?"])?;
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as("Ernst", args!["Yes, then, see you later."])?;
                        if ctx.var("nkprince_eisen").get()? == 1 {
                            ctx.var("nkprince_eisen").set(Val::from(3))?;
                        } else {
                            ctx.var("nkprince_eisen").set(Val::from(2))?;
                        }
                        ctx.call(Function::CompleteQuest, vec![Val::from(10006)])?;
                        l_prin1 = ctx.call(Function::CheckQuest, vec![Val::from(10005)])?;
                        l_prin2 = ctx.call(Function::CheckQuest, vec![Val::from(10006)])?;
                        l_prin3 = ctx.call(Function::CheckQuest, vec![Val::from(10007)])?;
                        l_prin4 = ctx.call(Function::CheckQuest, vec![Val::from(10008)])?;
                        l_prin5 = ctx.call(Function::CheckQuest, vec![Val::from(10009)])?;
                        l_prin6 = ctx.call(Function::CheckQuest, vec![Val::from(10010)])?;
                        l_prin7 = ctx.call(Function::CheckQuest, vec![Val::from(10011)])?;
                        if ((((((l_prin1.clone() + l_prin2.clone()) + l_prin3.clone()) + l_prin4.clone()) + l_prin5.clone())
                            + l_prin6.clone())
                            + l_prin7.clone())
                            == 14
                        {
                            ctx.call(Function::SetQuest, vec![Val::from(10012)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn prince_ern(ctx: &Ctx) -> Script {
    prince_ern_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TwonobleStep {
    Start,
    OnTouch,
}

fn twonoble_run(ctx: &Ctx, mut step: TwonobleStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TwonobleStep::Start => {
                step = TwonobleStep::OnTouch;
                continue 'machine;
            }
            TwonobleStep::OnTouch => {
                if ctx.var("nkprince_eisen").get()? == 7 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Young Noble#valter::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Aged Noble#rihart::OnEnable")])?;
                    ctx.lines_as(
                        "Aged Noble",
                        args![
                            "You don't have to worry.",
                            "Once my Eigen Ahrum is chosen to be king, the Richard family will be a second magnate."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("A man of Riehart Family", args!["Hehe, Ahrum is the most favorable and influential candidate already. You would be so amazing were you to fulfill this."])?;
                    ctx.next()?;
                    ctx.lines_as("A man of Walter Family", args!["Although Ahrum is a strong candidate, the prince of Gaebolg has a high reputation for his virtues too. For sure, it is better to confirm things."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "A man of Walter Family",
                        args!["So, are you on the project of Prince Ernst's poisoning?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "A man of Riehart Family",
                        args!["Of course. Even an autopsy will not find it. ... You don't have to worry."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "A man of Walter Family",
                        args!["Ahrum will be elected, once the termination of prince Ernst happens, and then..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("A man of Riehart Family", args!["both family will lead this country, huhuhu."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "A man of Walter Family",
                        args!["You must know what Walter and Gaebolg did. Both families stayed on top of the country."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "A man of Riehart Family",
                        args!["The weakest one of the seven families, Gaebolg Family, took power by uniting with Schmidt and Walter."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("A man of Walter Family", args!["Yes, that's why the union of Walter and Gaebolg is perfect. Moreover, Ahrum and Ernst... It's obviously the second coming of Schmidt and Heinrich."])?;
                    ctx.next()?;
                    ctx.lines_as("A man of Walter Family", args!["That's why the Walter family and the Gaebolg Family can't exist together. Their co-existence will bring destruction as it has done in the past."])?;
                    ctx.next()?;
                    ctx.lines_as("A man of Riehart Family", args!["Thanks for that. It's good to go. All the people who know of the past will deny to be in league with Ahrum or Ernst."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "A man of Walter Family",
                        args!["Anyway, we got a little bit more time. Keep the secret thoroughly; not to tell anyone."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("A man of Riehart Family", args!["Yes. For the glory of Rune-Midgarts."])?;
                    ctx.next()?;
                    ctx.lines_as("A man of Walter Family", args!["All the glory for Rune-Midgarts...", "Hmm?"])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.lines_as("A man of Walter Family", args!["Quiet... Someone comes.", "Ok... take care."])?;
                    ctx.next()?;
                    ctx.lines_as("A man of Riehart Family", args!["Never mind..."])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Young Noble#valter::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Aged Noble#rihart::OnDisable")])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["(Walter is the name of Ahrum.", "Richard is the name of Poe.)"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["(Is this the conspiracy of two families? What the hell..)"],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(10016), Val::from(10017)])?;
                    ctx.var("nkprince_eisen").set(Val::from(8))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn twonoble(ctx: &Ctx) -> Script {
    twonoble_run(ctx, TwonobleStep::Start, Vec::new()).map(|_| ())
}

pub fn twonoble_ontouch(ctx: &Ctx) -> Script {
    twonoble_run(ctx, TwonobleStep::OnTouch, Vec::new()).map(|_| ())
}

fn young_noble_valter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Young Noble Walter", args!["Step aside.", "How dare you talk to him."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn young_noble_valter(ctx: &Ctx) -> Script {
    young_noble_valter_body(ctx, Vec::new()).map(|_| ())
}

fn young_noble_valter_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Young Noble#valter")])?;
    return Err(Stop::End);
}

pub fn young_noble_valter_oninit(ctx: &Ctx) -> Script {
    young_noble_valter_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn young_noble_valter_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Young Noble#valter")])?;
    return Err(Stop::End);
}

pub fn young_noble_valter_onenable(ctx: &Ctx) -> Script {
    young_noble_valter_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn young_noble_valter_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Young Noble#valter")])?;
    return Err(Stop::End);
}

pub fn young_noble_valter_ondisable(ctx: &Ctx) -> Script {
    young_noble_valter_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn aged_noble_rihart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Aged Noble Richard", args!["Hmm-hmm.", "What an indecorous person!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn aged_noble_rihart(ctx: &Ctx) -> Script {
    aged_noble_rihart_body(ctx, Vec::new()).map(|_| ())
}

fn aged_noble_rihart_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Aged Noble#rihart")])?;
    return Err(Stop::End);
}

pub fn aged_noble_rihart_oninit(ctx: &Ctx) -> Script {
    aged_noble_rihart_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn aged_noble_rihart_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Aged Noble#rihart")])?;
    return Err(Stop::End);
}

pub fn aged_noble_rihart_onenable(ctx: &Ctx) -> Script {
    aged_noble_rihart_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn aged_noble_rihart_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Aged Noble#rihart")])?;
    return Err(Stop::End);
}

pub fn aged_noble_rihart_ondisable(ctx: &Ctx) -> Script {
    aged_noble_rihart_ondisable_body(ctx, Vec::new()).map(|_| ())
}

pub fn prince_another_ern(ctx: &Ctx) -> Script {
    prince_another_ern_run(ctx, PrinceAnotherErnStep::Start, Vec::new()).map(|_| ())
}

pub fn prince_another_ern_oninit(ctx: &Ctx) -> Script {
    prince_another_ern_run(ctx, PrinceAnotherErnStep::OnInit, Vec::new()).map(|_| ())
}

pub fn prince_another_ern_onenable(ctx: &Ctx) -> Script {
    prince_another_ern_run(ctx, PrinceAnotherErnStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn prince_another_ern_ondisable(ctx: &Ctx) -> Script {
    prince_another_ern_run(ctx, PrinceAnotherErnStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn prince_another_ern1(ctx: &Ctx) -> Script {
    prince_another_ern1_run(ctx, PrinceAnotherErn1Step::Start, Vec::new()).map(|_| ())
}

pub fn prince_another_ern1_oninit(ctx: &Ctx) -> Script {
    prince_another_ern1_run(ctx, PrinceAnotherErn1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn prince_another_ern1_onenable(ctx: &Ctx) -> Script {
    prince_another_ern1_run(ctx, PrinceAnotherErn1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn prince_another_ern1_ondisable(ctx: &Ctx) -> Script {
    prince_another_ern1_run(ctx, PrinceAnotherErn1Step::OnDisable, Vec::new()).map(|_| ())
}

fn prince_eisen1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_er1 = Val::from(0);
    let mut l_er2 = Val::from(0);
    let mut l_er3 = Val::from(0);
    let mut l_er4 = Val::from(0);
    let mut l_er5 = Val::from(0);
    ctx.lines_as(
        "Ahrum",
        args!["Moron. You'll never be a king. All throughout your life... Don't you know it? Such a junk addict!!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Erich",
        args!["...I am not trying to be, but swear words don't make me feel very good. What's wrong with you all of a sudden?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args![
            "I am just fed up with you. Even a jerk like you is a candidate for king? I feel sick and nauseated about this king succession."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Erich",
        args!["Yeah, it's not that wrong. But I am the representative of the Nerious family. I cannot stand for your insults."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Erich",
        args!["Is your family aware of your crazy actions? You are ruining your name, and of your family."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args!["...! Yes... I give a crap. No... My existence too! Kukuku...hahahahaha!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args!["Yes, you and I are both disqualified! Hahahaha!! The freaking addict reveals the truth! Yes, hahahahahaha!!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Erich",
        args!["... Crazy jerk. Beat it. I don't want to talk with you anymore."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args!["Yes, I am going now. Family shame?!! Hahahaha... What is a family!! Kkkkkk!! Very interesting!"],
    )?;
    ctx.next()?;
    ctx.mes("-Ahrum, seems to be out of his mind, as he approaches the door-")?;
    ctx.next()?;
    ctx.lines_as("Erich", args!["...Nuts."])?;
    ctx.call(Function::CompleteQuest, vec![Val::from(10020)])?;
    l_er1 = ctx.call(Function::CheckQuest, vec![Val::from(10018)])?;
    l_er2 = ctx.call(Function::CheckQuest, vec![Val::from(10019)])?;
    l_er3 = ctx.call(Function::CheckQuest, vec![Val::from(10020)])?;
    l_er4 = ctx.call(Function::CheckQuest, vec![Val::from(10021)])?;
    l_er5 = ctx.call(Function::CheckQuest, vec![Val::from(10022)])?;
    if ((((l_er1.clone() + l_er2.clone()) + l_er3.clone()) + l_er4.clone()) + l_er5.clone()) == 10 {
        ctx.call(Function::SetQuest, vec![Val::from(10023)])?;
        ctx.var("nkprince_eisen").set(Val::from(11))?;
    }
    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen1::OnDisable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prince_eisen1(ctx: &Ctx) -> Script {
    prince_eisen1_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen1")])?;
    return Err(Stop::End);
}

pub fn prince_eisen1_oninit(ctx: &Ctx) -> Script {
    prince_eisen1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen1_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Prince#eisen1")])?;
    return Err(Stop::End);
}

pub fn prince_eisen1_onenable(ctx: &Ctx) -> Script {
    prince_eisen1_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen1_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen1")])?;
    return Err(Stop::End);
}

pub fn prince_eisen1_ondisable(ctx: &Ctx) -> Script {
    prince_eisen1_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_er1 = Val::from(0);
    let mut l_er2 = Val::from(0);
    let mut l_er3 = Val::from(0);
    let mut l_er4 = Val::from(0);
    let mut l_er5 = Val::from(0);
    ctx.lines_as(
        "Urugen",
        args!["What did you say? Ahrum?", "Hey! What did you just say to me? Ahrum?!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["I can say it a zillion times! Urugen, you are not beautiful at all! Not even a bit! In fact, your look is very ugly and obscene!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Urugen",
        args![
            "That's very rude! How could a dirty germ like you talk like that to me!",
            "you like germ can talk like that",
            "to me!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args!["Dirty? Yes, I am. I am dirty. But it is not a problem of mine. Everything is dirty! I cannot stand it anymore!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Urugen", args!["Hm! I can agree with your words, the world is obscene, but you cannot bring me down to that level! I cannot forgive your words! Never, ever!"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["What if you can't forgive me? What shall you do? That's good. Are you gonna kill me? Will you kill me? Kill me? Haha? You are being called beautiful only when calling yourself!"])?;
    ctx.next()?;
    ctx.lines_as("Urugen", args!["Akk! I cannot stand this! Such low behavior and words ...? You are no prince. I don't want to face you anymore! Get out of here!"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["You don't have enough quality. You don't have belief to eliminate evil. By will, you are rubbish. I don't want to be with rubbish either! Stay in your world forever and never come out of it!"])?;
    ctx.next()?;
    ctx.mes("-Ahrum strides out of the room, grumbling...-")?;
    ctx.next()?;
    ctx.lines_as(
        "Urugen",
        args!["...You are so low. I misjudged you. You are not of the few that have an artistic view. I totally saw you the wrong way."],
    )?;
    ctx.call(Function::CompleteQuest, vec![Val::from(10021)])?;
    l_er1 = ctx.call(Function::CheckQuest, vec![Val::from(10018)])?;
    l_er2 = ctx.call(Function::CheckQuest, vec![Val::from(10019)])?;
    l_er3 = ctx.call(Function::CheckQuest, vec![Val::from(10020)])?;
    l_er4 = ctx.call(Function::CheckQuest, vec![Val::from(10021)])?;
    l_er5 = ctx.call(Function::CheckQuest, vec![Val::from(10022)])?;
    if ((((l_er1.clone() + l_er2.clone()) + l_er3.clone()) + l_er4.clone()) + l_er5.clone()) == 10 {
        ctx.call(Function::SetQuest, vec![Val::from(10023)])?;
        ctx.var("nkprince_eisen").set(Val::from(11))?;
    }
    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen2::OnDisable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prince_eisen2(ctx: &Ctx) -> Script {
    prince_eisen2_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen2")])?;
    return Err(Stop::End);
}

pub fn prince_eisen2_oninit(ctx: &Ctx) -> Script {
    prince_eisen2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Prince#eisen2")])?;
    return Err(Stop::End);
}

pub fn prince_eisen2_onenable(ctx: &Ctx) -> Script {
    prince_eisen2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen2_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen2")])?;
    return Err(Stop::End);
}

pub fn prince_eisen2_ondisable(ctx: &Ctx) -> Script {
    prince_eisen2_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_er1 = Val::from(0);
    let mut l_er2 = Val::from(0);
    let mut l_er3 = Val::from(0);
    let mut l_er4 = Val::from(0);
    let mut l_er5 = Val::from(0);
    ctx.lines_as(
        "Ahrum",
        args![
            "Military maniac! Jerk! Do you think power can solve everything? In all situations, do you think people can live peacefully?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Helmut",
        args![
            "You are fully violent now.",
            "By your theory, you are also not eligible for the king's quality!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args![
            "Yes, you talk well. Bah!",
            "I don't have quality!!",
            "But you are worse than me! King candidate?? You are just a war monger!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Helmut",
        args!["You have come here to offend me? Do you think you can beat me, Eigen Ahrum?!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Fight? That's so rubbish. I don't care for it. I just came here to shoot what I want to say to you. I don't even feel like fighting with you. I am leaving!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Helmut",
        args!["Hold on, coward! Are you fleeing now?! Such a coward! Let's finish our battle!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Coward.. that's a very nice word. Yes, I am. I am leaving. Ta-tah! You are worse than a bad coward...like me! Bye-bye war monger!"])?;
    ctx.next()?;
    ctx.mes("-Ahrum grins with a vague feeling towards an irritated Helmut, and leaves the room.-")?;
    ctx.next()?;
    ctx.lines_as("Helmut", args!["That is... such a villain!!!"])?;
    ctx.call(Function::CompleteQuest, vec![Val::from(10022)])?;
    l_er1 = ctx.call(Function::CheckQuest, vec![Val::from(10018)])?;
    l_er2 = ctx.call(Function::CheckQuest, vec![Val::from(10019)])?;
    l_er3 = ctx.call(Function::CheckQuest, vec![Val::from(10020)])?;
    l_er4 = ctx.call(Function::CheckQuest, vec![Val::from(10021)])?;
    l_er5 = ctx.call(Function::CheckQuest, vec![Val::from(10022)])?;
    if ((((l_er1.clone() + l_er2.clone()) + l_er3.clone()) + l_er4.clone()) + l_er5.clone()) == 10 {
        ctx.call(Function::SetQuest, vec![Val::from(10023)])?;
        ctx.var("nkprince_eisen").set(Val::from(11))?;
    }
    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen3::OnDisable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prince_eisen3(ctx: &Ctx) -> Script {
    prince_eisen3_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen3")])?;
    return Err(Stop::End);
}

pub fn prince_eisen3_oninit(ctx: &Ctx) -> Script {
    prince_eisen3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen3_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Prince#eisen3")])?;
    return Err(Stop::End);
}

pub fn prince_eisen3_onenable(ctx: &Ctx) -> Script {
    prince_eisen3_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen3_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen3")])?;
    return Err(Stop::End);
}

pub fn prince_eisen3_ondisable(ctx: &Ctx) -> Script {
    prince_eisen3_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_er1 = Val::from(0);
    let mut l_er2 = Val::from(0);
    let mut l_er3 = Val::from(0);
    let mut l_er4 = Val::from(0);
    let mut l_er5 = Val::from(0);
    ctx.lines_as(
        "Ahrum",
        args!["You always care about gambling. But, why don't you care about your surrondings first, eh? Young master of Richard?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Poe",
        args!["Why are you teasing me, now? At least I feel I have dice with higher numbers than you... You don't think so?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Posh! You keep talking about dice. Listen. You can feel that I roll lower numbers of dice than you... But you have already failed before the roll of your dice! Do you know that?!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Poe",
        args!["What are you talking about? I can't get you. I've lost to whom? I don't know why you are talking like that."],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["You can't know it! So what?! If you know, what can you do? You have too much pride over everything. Do you think you can foster a deeply rotten tree, to become a healthy tree?!"])?;
    ctx.next()?;
    ctx.lines_as("Poe", args!["If you want to say something, you have to say it clearly. Are you announcing a loss and defeat? Is my luck so dreadful, Eigen Ahrum?"])?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Yes, I announce my defeat. But it is not to you. You are a loser like me. Before getting into important business, you are ruined by an environmental element."])?;
    ctx.next()?;
    ctx.lines_as(
        "Poe",
        args!["...What the hell are you talking about? Environment? I totally don't get you. It is never clear."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args![
            "You'd better not understand it. If you know this, you will lose your health. Don't care for it... for the rest of your life!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Poe", args!["Ho- hold! Look, Ahrum!", "Hold on! Eigen Ahrum! Hey!"])?;
    ctx.next()?;
    ctx.lines(args![
        "-Ahrum totally neglects the",
        "voice calling him and leaves the room, with a lonely countenance.-"
    ])?;
    ctx.next()?;
    ctx.lines_as("Poe", args!["What... What. It's not fun.", "I totally couldn't get him."])?;
    ctx.call(Function::CompleteQuest, vec![Val::from(10018)])?;
    l_er1 = ctx.call(Function::CheckQuest, vec![Val::from(10018)])?;
    l_er2 = ctx.call(Function::CheckQuest, vec![Val::from(10019)])?;
    l_er3 = ctx.call(Function::CheckQuest, vec![Val::from(10020)])?;
    l_er4 = ctx.call(Function::CheckQuest, vec![Val::from(10021)])?;
    l_er5 = ctx.call(Function::CheckQuest, vec![Val::from(10022)])?;
    if ((((l_er1.clone() + l_er2.clone()) + l_er3.clone()) + l_er4.clone()) + l_er5.clone()) == 10 {
        ctx.call(Function::SetQuest, vec![Val::from(10023)])?;
        ctx.var("nkprince_eisen").set(Val::from(11))?;
    }
    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen4::OnDisable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prince_eisen4(ctx: &Ctx) -> Script {
    prince_eisen4_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen4_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen4")])?;
    return Err(Stop::End);
}

pub fn prince_eisen4_oninit(ctx: &Ctx) -> Script {
    prince_eisen4_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen4_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Prince#eisen4")])?;
    return Err(Stop::End);
}

pub fn prince_eisen4_onenable(ctx: &Ctx) -> Script {
    prince_eisen4_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen4_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen4")])?;
    return Err(Stop::End);
}

pub fn prince_eisen4_ondisable(ctx: &Ctx) -> Script {
    prince_eisen4_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_er1 = Val::from(0);
    let mut l_er2 = Val::from(0);
    let mut l_er3 = Val::from(0);
    let mut l_er4 = Val::from(0);
    let mut l_er5 = Val::from(0);
    ctx.lines_as("Ahrum", args!["Peter, what will you do if your work doesn't mean anything?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Peter",
        args!["...I don't know what you are talking about, but meaningless things do not exist. I don't think you know about that."],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Buzzzz. You are wrong. There is a thing that is meaningless itself. Clearly speaking, the action becomes meaningless without knowing its meaning..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Peter",
        args![
            "Are you trying to explain... clearly? It's getting more complicated. What exactly do you want to say? Let me get it directly."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["...Whatever I do or do not do, I am just being manipulated by some thing. To make it worse, I hurt others as I am in that some thing only."])?;
    ctx.next()?;
    ctx.lines_as("Peter", args!["......"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ahrum",
        args!["The worst thing is, I feel I just become timid with knowing and understanding the some thing itself.."],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["But it is the same with you. If you really want to express gratitude towards the girl, you should have visited her in person. You couldn't do it, could you?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Peter",
        args!["...Ahrum, what's wrong with you? You are not the Ahrum I know. What's made you like this?"],
    )?;
    ctx.next()?;
    ctx.lines_as("Ahrum", args!["Hmph..The some thing. Nothing. It's meaningless. Reality, that's all. It was nice talking to you. Bye-bye, and live like what you used to live like, ever after."])?;
    ctx.next()?;
    ctx.lines_as("Peter", args!["......Eigen Ahrum. You..."])?;
    ctx.next()?;
    ctx.mes("-Ahrum shows a gloomy facial expression and leaves the room.-")?;
    ctx.next()?;
    ctx.lines_as("Peter", args!["......"])?;
    ctx.call(Function::CompleteQuest, vec![Val::from(10019)])?;
    l_er1 = ctx.call(Function::CheckQuest, vec![Val::from(10018)])?;
    l_er2 = ctx.call(Function::CheckQuest, vec![Val::from(10019)])?;
    l_er3 = ctx.call(Function::CheckQuest, vec![Val::from(10020)])?;
    l_er4 = ctx.call(Function::CheckQuest, vec![Val::from(10021)])?;
    l_er5 = ctx.call(Function::CheckQuest, vec![Val::from(10022)])?;
    if ((((l_er1.clone() + l_er2.clone()) + l_er3.clone()) + l_er4.clone()) + l_er5.clone()) == 10 {
        ctx.call(Function::SetQuest, vec![Val::from(10023)])?;
        ctx.var("nkprince_eisen").set(Val::from(11))?;
    }
    ctx.call(Function::DoNpcEvent, vec![Val::from("Prince#eisen5::OnDisable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prince_eisen5(ctx: &Ctx) -> Script {
    prince_eisen5_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen5_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Prince#eisen5")])?;
    return Err(Stop::End);
}

pub fn prince_eisen5_oninit(ctx: &Ctx) -> Script {
    prince_eisen5_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn prince_eisen5_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Prince#eisen5")])?;
    return Err(Stop::End);
}

pub fn prince_eisen5_onenable(ctx: &Ctx) -> Script {
    prince_eisen5_onenable_body(ctx, Vec::new()).map(|_| ())
}
