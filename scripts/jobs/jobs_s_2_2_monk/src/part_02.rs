use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn touha_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_monk_t = Val::from(0);
    let mut l_rand = Val::from(0);
    if (ctx.var("monk_q").get()?.number()? >= 10 && ctx.var("monk_q").get()?.number()? < 14) {
        if ctx.var("monk_q").get()? == 10 {
            ctx.lines_as(
                "Touha",
                args!["What brings you to me.", "Do you wish to share a conversation with me?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args![
                    "Oh, I see. You're on the monk in training.",
                    "You already possess a similar spirit as a monk's."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args!["By the looks of you, it seems, you", "have already visited Sensei Moohae. Good."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args![
                    "Let me inform you about certain things you must know as a monk.",
                    "Then I will help you to strengthen your body so that you can bear your next training."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["Calm your mind.", "Relax your body...are you ready?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 2 {
                ctx.lines_as("Touha", args!["Please come back when you're ready."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Touha", args!["Ok...then."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["Please repeat after me."])?;
            ctx.next()?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3024), Val::from(3025)])?;
        } else {
            ctx.lines_as("Touha", args!["Now, pay attention this time..."])?;
            ctx.next()?;
        }
        ctx.mes("[Touha]")?;
        l_rand = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if (l_rand.clone() == 1 || ctx.var("monk_q").get()? == 11) {
            ctx.var("monk_q").set(Val::from(11))?;
            ctx.mes("I seek the path")?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["of enlightenment."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["We monks"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["shall hold true"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["to what we believe"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["and will help protect others"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["through the teachings"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["we learn through our lives."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["In nomine Patris, et Filii"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["et Spiritus Sancti."])?;
        } else if (l_rand.clone() == 2 || ctx.var("monk_q").get()? == 12) {
            ctx.var("monk_q").set(Val::from(12))?;
            ctx.mes("I commit myself to")?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["veritas and aequitas."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["I will follow my path"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["to enlightenment and purity."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["I will protect my"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["brothers with my life."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["Evil shall never be"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["victorious while I breathe."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["In nomine Patris, et Filii"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["et Spiritus Sancti."])?;
        } else if (l_rand.clone() == 3 || ctx.var("monk_q").get()? == 13) {
            ctx.var("monk_q").set(Val::from(13))?;
            ctx.mes("And shepherds we shall be,")?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["for thee my lord for thee."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["Power hath descended forth"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["from the hand"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["so our feet may swiftly carry"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["out thy command. And we shall"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["flow a river forth to thee and"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["teeming with souls shall it ever be"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["In nomine Patris, et Filii"])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["et Spiritus Sancti."])?;
        }
        ctx.next()?;
        if ctx.var("monk_q").get()? == 10 {
            ctx.lines_as(
                "Touha",
                args![
                    "Ok, that is all. Now repeat what I have spoken.",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", your turn."))
                ],
            )?;
            ctx.next()?;
        }
        if ctx.var("monk_q").get()? == 11 {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "shall hold true:We monks:and will help protect others:through the teachings:In nomine Patris, et Filii:to what we believe:I seek the path:we learn through our lives.:et Spiritus Sancti.:of enlightenment.",
                )],
            )? {
                1 => {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["shall hold true"])?;
                }
                2 => {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["We monks"])?;
                }
                3 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["and will help protect others"],
                    )?;
                }
                4 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["through the teachings"],
                    )?;
                }
                5 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["In nomine Patris, et Filii"],
                    )?;
                }
                6 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["to what we believe"],
                    )?;
                }
                7 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I seek the path"])?;
                }
                8 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["we learn through our lives."],
                    )?;
                }
                9 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["et Spiritus Sancti."],
                    )?;
                }
                10 => {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["of enlightenment."])?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "We monks:In nomine Patris, et Filii:I seek the path:shall hold true:of enlightenment.:and will help protect others:we learn through our lives.:through the teachings:to what we believe:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("We monks")?;
                }
                2 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                3 => {
                    ctx.mes("I seek the path")?;
                }
                4 => {
                    ctx.mes("shall hold true")?;
                }
                5 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("of enlightenment.")?;
                }
                6 => {
                    ctx.mes("and will help protect others")?;
                }
                7 => {
                    ctx.mes("we learn through our lives.")?;
                }
                8 => {
                    ctx.mes("through the teachings")?;
                }
                9 => {
                    ctx.mes("to what we believe")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "to what we believe:We monks:I seek the path:shall hold true:of enlightenment.:we learn through our lives.:In nomine Patris, et Filii:and will help protect others:through the teachings:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("to what we believe")?;
                }
                2 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("We monks")?;
                }
                3 => {
                    ctx.mes("I seek the path")?;
                }
                4 => {
                    ctx.mes("shall hold true")?;
                }
                5 => {
                    ctx.mes("of enlightenment.")?;
                }
                6 => {
                    ctx.mes("we learn through our lives.")?;
                }
                7 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                8 => {
                    ctx.mes("and will help protect others")?;
                }
                9 => {
                    ctx.mes("through the teachings")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "shall hold true:I seek the path:We monks:In nomine Patris, et Filii:of enlightenment.:et Spiritus Sancti.:to what we believe:we learn through our lives.:and will help protect others:through the teachings",
                )],
            )? {
                1 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("shall hold true")?;
                }
                2 => {
                    ctx.mes("I seek the path")?;
                }
                3 => {
                    ctx.mes("We monks")?;
                }
                4 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                5 => {
                    ctx.mes("of enlightenment.")?;
                }
                6 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                7 => {
                    ctx.mes("to what we believe")?;
                }
                8 => {
                    ctx.mes("we learn through our lives.")?;
                }
                9 => {
                    ctx.mes("and will help protect others")?;
                }
                10 => {
                    ctx.mes("through the teachings")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "of enlightenment.:I seek the path:We monks:shall hold true:and will help protect others:through the teachings:we learn through our lives.:In nomine Patris, et Filii:to what we believe:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("of enlightenment.")?;
                }
                2 => {
                    ctx.mes("I seek the path")?;
                }
                3 => {
                    ctx.mes("We monks")?;
                }
                4 => {
                    ctx.mes("shall hold true")?;
                }
                5 => {
                    ctx.mes("and will help protect others")?;
                }
                6 => {
                    ctx.mes("through the teachings")?;
                }
                7 => {
                    ctx.mes("we learn through our lives.")?;
                }
                8 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("to what we believe")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I seek the path:through the teachings:and will help protect others:of enlightenment.:shall hold true:et Spiritus Sancti.:In nomine Patris, et Filii:to what we believe:We monks:we learn through our lives.",
                )],
            )? {
                1 => {
                    ctx.mes("I seek the path")?;
                }
                2 => {
                    ctx.mes("through the teachings")?;
                }
                3 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("and will help protect others")?;
                }
                4 => {
                    ctx.mes("of enlightenment.")?;
                }
                5 => {
                    ctx.mes("shall hold true")?;
                }
                6 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                7 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                8 => {
                    ctx.mes("to what we believe")?;
                }
                9 => {
                    ctx.mes("We monks")?;
                }
                10 => {
                    ctx.mes("we learn through our lives.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "we learn through our lives.:In nomine Patris, et Filii:et Spiritus Sancti.:I seek the path:of enlightenment.:to what we believe:We monks:shall hold true:and will help protect others:through the teachings",
                )],
            )? {
                1 => {
                    ctx.mes("we learn through our lives.")?;
                }
                2 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                3 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                4 => {
                    ctx.mes("I seek the path")?;
                }
                5 => {
                    ctx.mes("of enlightenment.")?;
                }
                6 => {
                    ctx.mes("to what we believe")?;
                }
                7 => {
                    ctx.mes("We monks")?;
                }
                8 => {
                    ctx.mes("shall hold true")?;
                }
                9 => {
                    ctx.mes("and will help protect others")?;
                }
                10 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("through the teachings")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "we learn through our lives.:In nomine Patris, et Filii:through the teachings:I seek the path:We monks:shall hold true:to what we believe:and will help protect others:of enlightenment.:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("we learn through our lives.")?;
                }
                2 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                3 => {
                    ctx.mes("through the teachings")?;
                }
                4 => {
                    ctx.mes("I seek the path")?;
                }
                5 => {
                    ctx.mes("We monks")?;
                }
                6 => {
                    ctx.mes("shall hold true")?;
                }
                7 => {
                    ctx.mes("to what we believe")?;
                }
                8 => {
                    ctx.mes("and will help protect others")?;
                }
                9 => {
                    ctx.mes("of enlightenment.")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I seek the path:of enlightenment.:We monks:shall hold true:to what we believe:et Spiritus Sancti.:and will help protect others:through the teachings:we learn through our lives.:In nomine Patris, et Filii",
                )],
            )? {
                1 => {
                    ctx.mes("I seek the path")?;
                }
                2 => {
                    ctx.mes("of enlightenment.")?;
                }
                3 => {
                    ctx.mes("We monks")?;
                }
                4 => {
                    ctx.mes("shall hold true")?;
                }
                5 => {
                    ctx.mes("to what we believe")?;
                }
                6 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                7 => {
                    ctx.mes("and will help protect others")?;
                }
                8 => {
                    ctx.mes("through the teachings")?;
                }
                9 => {
                    ctx.mes("we learn through our lives.")?;
                }
                10 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I seek the path:of enlightenment.:We monks:shall hold true:to what we believe:and will help protect others:through the teachings:we learn through our lives.:In nomine Patris, et Filii:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("I seek the path")?;
                }
                2 => {
                    ctx.mes("of enlightenment.")?;
                }
                3 => {
                    ctx.mes("We monks")?;
                }
                4 => {
                    ctx.mes("shall hold true")?;
                }
                5 => {
                    ctx.mes("to what we believe")?;
                }
                6 => {
                    ctx.mes("and will help protect others")?;
                }
                7 => {
                    ctx.mes("through the teachings")?;
                }
                8 => {
                    ctx.mes("we learn through our lives.")?;
                }
                9 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                10 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
        } else if ctx.var("monk_q").get()? == 12 {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I will follow my path:veritas and aequitas.:to enlightenment and purity.:I commit myself to:I will protect my:victorious while I breathe.:brothers with my life.:Evil shall never be:In nomine Patris, et Filii:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I will follow my path"],
                    )?;
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["veritas and aequitas."],
                    )?;
                }
                3 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["to enlightenment and purity."],
                    )?;
                }
                4 => {
                    ctx.lines(args![
                        ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                    ])?;
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("I commit myself to")?;
                }
                5 => {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I will protect my"])?;
                }
                6 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["victorious while I breathe."],
                    )?;
                }
                7 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["brothers with my life."],
                    )?;
                }
                8 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Evil shall never be"],
                    )?;
                }
                9 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["In nomine Patris, et Filii"],
                    )?;
                }
                10 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["et Spiritus Sancti."],
                    )?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I will follow my path:I will protect my:brothers with my life.:to enlightenment and purity.:Evil shall never be:victorious while I breathe.:et Spiritus Sancti.:I commit myself to:veritas and aequitas.:In nomine Patris, et Filii",
                )],
            )? {
                1 => {
                    ctx.mes("I will follow my path")?;
                }
                2 => {
                    ctx.mes("I will protect my")?;
                }
                3 => {
                    ctx.mes("brothers with my life.")?;
                }
                4 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                5 => {
                    ctx.mes("Evil shall never be")?;
                }
                6 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                7 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                8 => {
                    ctx.mes("I commit myself to")?;
                }
                9 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("veritas and aequitas.")?;
                }
                10 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I will follow my path:veritas and aequitas.:I commit myself to:et Spiritus Sancti.:Evil shall never be:to enlightenment and purity.:In nomine Patris, et Filii:I will protect my:brothers with my life.:victorious while I breathe.",
                )],
            )? {
                1 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("I will follow my path")?;
                }
                2 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                3 => {
                    ctx.mes("I commit myself to")?;
                }
                4 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                5 => {
                    ctx.mes("Evil shall never be")?;
                }
                6 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                7 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                8 => {
                    ctx.mes("I will protect my")?;
                }
                9 => {
                    ctx.mes("brothers with my life.")?;
                }
                10 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "veritas and aequitas.:Evil shall never be:I will follow my path:I will protect my:victorious while I breathe.:to enlightenment and purity.:brothers with my life.:In nomine Patris, et Filii:et Spiritus Sancti.:I commit myself to",
                )],
            )? {
                1 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                2 => {
                    ctx.mes("Evil shall never be")?;
                }
                3 => {
                    ctx.mes("I will follow my path")?;
                }
                4 => {
                    ctx.mes("I will protect my")?;
                }
                5 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                6 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("to enlightenment and purity.")?;
                }
                7 => {
                    ctx.mes("brothers with my life.")?;
                }
                8 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                10 => {
                    ctx.mes("I commit myself to")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "victorious while I breathe.:I commit myself to:to enlightenment and purity.:brothers with my life.:Evil shall never be:In nomine Patris, et Filii:et Spiritus Sancti.:I will follow my path:veritas and aequitas.:I will protect my",
                )],
            )? {
                1 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                2 => {
                    ctx.mes("I commit myself to")?;
                }
                3 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                4 => {
                    ctx.mes("brothers with my life.")?;
                }
                5 => {
                    ctx.mes("Evil shall never be")?;
                }
                6 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                7 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                8 => {
                    ctx.mes("I will follow my path")?;
                }
                9 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                10 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("I will protect my")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "to enlightenment and purity.:I will follow my path:veritas and aequitas.:I commit myself to:brothers with my life.:I will protect my:victorious while I breathe.:Evil shall never be:et Spiritus Sancti.:In nomine Patris, et Filii",
                )],
            )? {
                1 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                2 => {
                    ctx.mes("I will follow my path")?;
                }
                3 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                4 => {
                    ctx.mes("I commit myself to")?;
                }
                5 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("brothers with my life.")?;
                }
                6 => {
                    ctx.mes("I will protect my")?;
                }
                7 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                8 => {
                    ctx.mes("Evil shall never be")?;
                }
                9 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                10 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "veritas and aequitas.:Evil shall never be:brothers with my life.:victorious while I breathe.:I will follow my path:to enlightenment and purity.:I will protect my:In nomine Patris, et Filii:et Spiritus Sancti.:I commit myself to",
                )],
            )? {
                1 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                2 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("Evil shall never be")?;
                }
                3 => {
                    ctx.mes("brothers with my life.")?;
                }
                4 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                5 => {
                    ctx.mes("I will follow my path")?;
                }
                6 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                7 => {
                    ctx.mes("I will protect my")?;
                }
                8 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                10 => {
                    ctx.mes("I commit myself to")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "victorious while I breathe.:to enlightenment and purity.:I will protect my:veritas and aequitas.:brothers with my life.:I will follow my path:Evil shall never be:In nomine Patris, et Filii:I commit myself to:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("victorious while I breathe.")?;
                }
                2 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                3 => {
                    ctx.mes("I will protect my")?;
                }
                4 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                5 => {
                    ctx.mes("brothers with my life.")?;
                }
                6 => {
                    ctx.mes("I will follow my path")?;
                }
                7 => {
                    ctx.mes("Evil shall never be")?;
                }
                8 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    ctx.mes("I commit myself to")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I commit myself to:I will follow my path:veritas and aequitas.:I will protect my:to enlightenment and purity.:brothers with my life.:Evil shall never be:In nomine Patris, et Filii:victorious while I breathe.:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("I commit myself to")?;
                }
                2 => {
                    ctx.mes("I will follow my path")?;
                }
                3 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                4 => {
                    ctx.mes("I will protect my")?;
                }
                5 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                6 => {
                    ctx.mes("brothers with my life.")?;
                }
                7 => {
                    ctx.mes("Evil shall never be")?;
                }
                8 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I commit myself to:veritas and aequitas.:I will follow my path:to enlightenment and purity.:I will protect my:brothers with my life.:Evil shall never be:victorious while I breathe.:In nomine Patris, et Filii:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("I commit myself to")?;
                }
                2 => {
                    ctx.mes("veritas and aequitas.")?;
                }
                3 => {
                    ctx.mes("I will follow my path")?;
                }
                4 => {
                    ctx.mes("to enlightenment and purity.")?;
                }
                5 => {
                    ctx.mes("I will protect my")?;
                }
                6 => {
                    ctx.mes("brothers with my life.")?;
                }
                7 => {
                    ctx.mes("Evil shall never be")?;
                }
                8 => {
                    ctx.mes("victorious while I breathe.")?;
                }
                9 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                10 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
        } else if ctx.var("monk_q").get()? == 13 {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "for thee my lord for thee.:And shepherds we shall be,:Power hath descended forth:out thy command. And we shall:from the hand:flow a river forth to thee and:so our feet may swiftly carry:teeming with souls shall it ever be:et Spiritus Sancti.:In nomine Patris, et Filii",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["for thee my lord for thee."],
                    )?;
                }
                2 => {
                    ctx.lines(args![
                        ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                    ])?;
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("And shepherds we shall be,")?;
                }
                3 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Power hath descended forth"],
                    )?;
                }
                4 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["out thy command. And we shall"],
                    )?;
                }
                5 => {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["from the hand"])?;
                }
                6 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["flow a river forth to thee and"],
                    )?;
                }
                7 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["so our feet may swiftly carry"],
                    )?;
                }
                8 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["teeming with souls shall it ever be"],
                    )?;
                }
                9 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["et Spiritus Sancti."],
                    )?;
                }
                10 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["In nomine Patris, et Filii"],
                    )?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "teeming with souls shall it ever be:flow a river forth to thee and:so our feet may swiftly carry:In nomine Patris, et Filii:et Spiritus Sancti.:Power hath descended forth:And shepherds we shall be,:for thee my lord for thee.:from the hand:out thy command. And we shall",
                )],
            )? {
                1 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                2 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                3 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                4 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                5 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                6 => {
                    ctx.mes("Power hath descended forth")?;
                }
                7 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                8 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("for thee my lord for thee.")?;
                }
                9 => {
                    ctx.mes("from the hand")?;
                }
                10 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "And shepherds we shall be,:for thee my lord for thee.:Power hath descended forth:from the hand:teeming with souls shall it ever be:et Spiritus Sancti.:In nomine Patris, et Filii:so our feet may swiftly carry:out thy command. And we shall:flow a river forth to thee and",
                )],
            )? {
                1 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                2 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                3 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("Power hath descended forth")?;
                }
                4 => {
                    ctx.mes("from the hand")?;
                }
                5 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                6 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                7 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                8 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                9 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                10 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "for thee my lord for thee.:And shepherds we shall be,:Power hath descended forth:so our feet may swiftly carry:from the hand:flow a river forth to thee and:out thy command. And we shall:In nomine Patris, et Filii:teeming with souls shall it ever be:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                2 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                3 => {
                    ctx.mes("Power hath descended forth")?;
                }
                4 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                5 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("from the hand")?;
                }
                6 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                7 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                8 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "And shepherds we shall be,:for thee my lord for thee.:Power hath descended forth:so our feet may swiftly carry:from the hand:so our feet may swiftly carry:flow a river forth to thee and:In nomine Patris, et Filii:teeming with souls shall it ever be:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                2 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                3 => {
                    ctx.mes("Power hath descended forth")?;
                }
                4 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("so our feet may swiftly carry")?;
                }
                5 => {
                    ctx.mes("from the hand")?;
                }
                6 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                7 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                8 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                9 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "for thee my lord for thee.:Power hath descended forth:And shepherds we shall be,:from the hand:so our feet may swiftly carry:flow a river forth to thee and:out thy command. And we shall:teeming with souls shall it ever be:In nomine Patris, et Filii:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                2 => {
                    ctx.mes("Power hath descended forth")?;
                }
                3 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                4 => {
                    ctx.mes("from the hand")?;
                }
                5 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                6 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                7 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("out thy command. And we shall")?;
                }
                8 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                9 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "for thee my lord for thee.:teeming with souls shall it ever be:flow a river forth to thee and:In nomine Patris, et Filii:et Spiritus Sancti.:Power hath descended forth:And shepherds we shall be,:so our feet may swiftly carry:from the hand:out thy command. And we shall",
                )],
            )? {
                1 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                2 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                3 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("flow a river forth to thee and")?;
                }
                4 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                5 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                6 => {
                    ctx.mes("Power hath descended forth")?;
                }
                7 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                8 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                9 => {
                    ctx.mes("from the hand")?;
                }
                10 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "teeming with souls shall it ever be:In nomine Patris, et Filii:And shepherds we shall be,:for thee my lord for thee.:Power hath descended forth:from the hand:so our feet may swiftly carry:out thy command. And we shall:flow a river forth to thee and:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                2 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                3 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                4 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                5 => {
                    ctx.mes("Power hath descended forth")?;
                }
                6 => {
                    ctx.mes("from the hand")?;
                }
                7 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                8 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                9 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Power hath descended forth:for thee my lord for thee.:And shepherds we shall be,:In nomine Patris, et Filii:so our feet may swiftly carry:from the hand:teeming with souls shall it ever be:flow a river forth to thee and:out thy command. And we shall:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("Power hath descended forth")?;
                }
                2 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                3 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                4 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                5 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                6 => {
                    ctx.mes("from the hand")?;
                }
                7 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                8 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                9 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                10 => {
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "And shepherds we shall be,:for thee my lord for thee.:Power hath descended forth:from the hand:out thy command. And we shall:so our feet may swiftly carry:flow a river forth to thee and:teeming with souls shall it ever be:In nomine Patris, et Filii:et Spiritus Sancti.",
                )],
            )? {
                1 => {
                    ctx.mes("And shepherds we shall be,")?;
                }
                2 => {
                    ctx.mes("for thee my lord for thee.")?;
                }
                3 => {
                    ctx.mes("Power hath descended forth")?;
                }
                4 => {
                    ctx.mes("from the hand")?;
                }
                5 => {
                    ctx.mes("out thy command. And we shall")?;
                }
                6 => {
                    ctx.mes("so our feet may swiftly carry")?;
                }
                7 => {
                    ctx.mes("flow a river forth to thee and")?;
                }
                8 => {
                    ctx.mes("teeming with souls shall it ever be")?;
                }
                9 => {
                    ctx.mes("In nomine Patris, et Filii")?;
                }
                10 => {
                    l_monk_t = (l_monk_t.clone() + Val::from(10));
                    ctx.mes("et Spiritus Sancti.")?;
                }
                _ => {}
            }
        }
        ctx.next()?;
        ctx.lines_as("Touha", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Touha", args!["Hmm..."])?;
        ctx.next()?;
        if l_monk_t.clone().number()? > 90 {
            ctx.var("monk_q").set(Val::from(14))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3025), Val::from(3026)])?;
            ctx.lines_as("Touha", args!["...well done, that was perfect. You pay attention well..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args!["However, now is not the time to relax. Your path is still long ahead of you."],
            )?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["Now as I promised, I will help strengthen your body."])?;
            ctx.next()?;
            ctx.mes("Focus your mind and do not move.")?;
            ctx.next()?;
            ctx.mes("^33CCFFYou feel wind all around your body.^000000")?;
            ctx.next()?;
            ctx.mes("^33CCFFAn energy within you grows.^000000")?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["I feel what grows within you.", "You may now continue on..."])?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["...the next course will be with Boohae."])?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args![
                    "I wish you well on your journey.",
                    "Don't forget, his name is ^CC0000Boohae^000000."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Touha",
                args!["I see you did not pay attention.. If you wish to become a monk, you must take this seriously."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args![
                    "Perhaps the path of a monk is too difficult for you?",
                    "You must take this seriously if you wish to continue..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Touha", args!["I will give you another chance."])?;
            ctx.next()?;
            ctx.lines_as(
                "Touha",
                args!["If you cannot pay attention and repeat what I ask you to, I will not allow you to continue your training here.."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("monk_q").get()? == 14 {
        ctx.lines_as("Touha", args!["Hmm... did you forget who to visit?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Touha",
            args!["I wonder about your abilities if you cannot remember such a simple thing."],
        )?;
        ctx.next()?;
        ctx.lines_as("Touha", args!["...are you testing my patience?"])?;
        ctx.next()?;
        ctx.lines_as("Touha", args!["You wear my patience thin...", "... go visit Boohae."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("monk_q").get()?.number()? > 14 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)) {
        ctx.lines_as("Touha", args!["...do your best for the final test."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Touha", args!["Never shall innocent blood be shed."])?;
        ctx.next()?;
        ctx.lines_as("Touha", args!["Yet the blood of the wicked shall flow like a river."])?;
        ctx.next()?;
        ctx.lines_as(
            "Touha",
            args!["We shall spread our blackened wings and be the vengeful striking hammer of god."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Touha",
            args!["We shall flow a river forth to thee, and teeming with souls shall it ever be."],
        )?;
        ctx.next()?;
        ctx.lines_as("Touha", args!["In nomine Patris, et Filii, et Spiritus Sancti."])?;
        ctx.next()?;
        ctx.lines_as("Touha", args!["...You don't have to be afraid of me..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn touha_mk(ctx: &Ctx) -> Script {
    touha_mk_body(ctx, Vec::new()).map(|_| ())
}

fn boohae_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("monk_q").get()? == 14 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)) {
        ctx.lines_as("Boohae", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["........."])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["............"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("...excuse me...?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Boohae",
            args!["...", "You just interrupted my meditation, I should break your legs..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boohae",
            args!["........", "I will give you a chance to explain why you interrupted me."],
        )?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["....."])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["Well, start explaining... or you'll be crawling soon..."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Touha sent me.:Sorry, nothing.")])?) == 2 {
            ctx.lines_as(
                "Boohae",
                args!["........", "...you must have a death wish to have interrupted me intentionally..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Boohae", args!["I see...", "Well then, let's see...."])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["....your body seems..", "strengthened. Good..."])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["What did you do with Touha?"])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Umm... well...ah..:We recited a holy pledge.:He diagnosed my physical status.",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Boohae",
                    args![
                        "You are not ready if you",
                        "cannot answer a simple question.",
                        "Leave me to my prayers."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Boohae", args!["... I see...", "Didn't he do anything for you?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Umm... well...ah..:He diagnosed my physical status.:He taught me about being a monk.:He modified my body.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Boohae",
                            args![
                                "You are not ready if you",
                                "cannot answer a simple question.",
                                "Leave me to my prayers."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Boohae",
                            args!["That is unimportant to me...", "Stop disturbing me and go away!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Boohae",
                            args![
                                "The teachings of becoming a monk are learned after becoming one.",
                                "This is not what I am looking for..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            "Boohae",
                            args![
                                "Very well, you seem to realize your body has something new inside.",
                                "Well then, we shall move on to the next step..."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            3 => {
                ctx.lines_as(
                    "Boohae",
                    args!["...You interrupted me to tell me that...?", "Get lost before I break your legs..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as(
            "Boohae",
            args!["Alright... well we have two tests...", "Choose which one you want to do..."],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Gathering mushrooms:Marathon")])?) == 1 {
            ctx.var("monk_q").set(Val::from(15))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3026), Val::from(3027)])?;
            ctx.lines_as(
                "Boohae",
                args![
                    "Hmm....gathering mushrooms. So you want to test your tolerance huh?",
                    "Go prepare and come back later when you're ready."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.var("monk_q").set(Val::from(16))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3026), Val::from(3028)])?;
        ctx.lines_as(
            "Boohae",
            args![
                "Good choice. Forcing your physical limits to their boundaries and grants a higher amount of self control.",
                "Go prepare and come back later when you're ready."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("monk_q").get()? == 15 {
        ctx.lines_as(
            "Boohae",
            args!["So, are you ready? You won't need anything but a great deal of determination."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boohae",
            args!["The gathering mushroom test is intended,", "to test your patience."],
        )?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["Go inside the building near this abbey."])?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["Other monk candidates will be with you for the same test,"])?;
        ctx.next()?;
        ctx.lines_as(
            "Boohae",
            args![
                "The more people that are there, the less mushrooms they'll find.",
                "So I hope you will understand that they are testing their patience, just like you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("monk_q").get()? == 16 {
        ctx.lines_as(
            "Boohae",
            args!["Welcome back, did you prepare? You won't need anything except strong legs."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Boohae",
            args!["The marathon is intended,", "to test your self-control ability."],
        )?;
        ctx.next()?;
        ctx.lines_as("Boohae", args!["Go inside the building near this abbey."])?;
        ctx.next()?;
        ctx.lines_as(
            "Boohae",
            args![
                "All you have to do is run around the building as many times as you're required.",
                "Well... get going."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("monk_q").get()? == 17 {
        ctx.lines_as(
            "Boohae",
            args![
                "Now, go visit 'Tomoon'. How many times should I tell you this?",
                "Now you have a lot of chance to damage your body because you're so exhausted right now.",
                "'Tomoon' is staying in a deepest place inside a building near this abbey."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("monk_q").get()?.number()? > 17 && ctx.var("monk_q").get()?.number()? < 24) {
        ctx.lines_as("Boohae", args!["..........."])?;
        ctx.next()?;
        ctx.mes("-He seems to be in meditation.-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Boohae", args!["Hmmmm....!!"])?;
        ctx.next()?;
        ctx.mes("-He seems to be in meditation.-")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn boohae_mk(ctx: &Ctx) -> Script {
    boohae_mk_body(ctx, Vec::new()).map(|_| ())
}

fn door_keeper_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Keeper Chorip",
        args!["....this place is for those", " in testing for becoming a monk."],
    )?;
    ctx.next()?;
    if ctx.var("monk_q").get()? == 14 {
        ctx.lines_as("Keeper Chorip", args!["Huh? Did you just say Boohae?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Keeper Chorip",
            args!["Boohae... tends to hide in some quite places, so you might not be able to find him. For instance... a corner..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("monk_q").get()?.number()? > 14 && ctx.var("monk_q").get()?.number()? < 25) {
        ctx.lines_as(
            "Keeper Chorip",
            args![((Val::from("Is your name ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?) == 1 {
            ctx.lines_as(
                "Keeper Chorip",
                args!["Alright you're cool... go on in. Your test is waiting for you. Good luck."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Keeper Chorip",
            args!["Yeah right, I know who you are... get in there... your test is ready."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Keeper Chorip", args!["...please be quiet inside."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn door_keeper_mk(ctx: &Ctx) -> Script {
    door_keeper_mk_body(ctx, Vec::new()).map(|_| ())
}

fn bashu_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("monk_q").get()?.number()? > 14 && ctx.var("monk_q").get()?.number()? < 25) {
        if ctx.var("monk_q").get()? == 15 {
            ctx.lines_as("Bashu", args!["So, which test do you want to do...?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bashu",
                args![
                    "From what I've heard, you chose the mushroom test...",
                    "Oh well, it's still your choice."
                ],
            )?;
        } else if ctx.var("monk_q").get()? == 16 {
            ctx.lines_as("Bashu", args!["Which test hall do you wish to enter?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Bashu",
                args![
                    "Well, as far as I've been told, you chose the marathon test...",
                    "Oh well, it's your choice."
                ],
            )?;
        } else {
            ctx.lines_as(
                "Bashu",
                args!["Which test hall do you wish to enter?", "You can choose which one you want."],
            )?;
            ctx.next()?;
        }
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Tolerance - Gathering Mushrooms:Self-Control - Marathon")],
        )?) == 1
        {
            ctx.mes("You have decided to take the test of tolerance by ^FF0000gathering mushrooms^000000.")?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("job_monk"), Val::from(226), Val::from(175)])?;
            return Err(Stop::End);
        }
        ctx.mes("You have decided to take the test of self control by taking a ^FF0000marathon^000000.")?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(386), Val::from(387)])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Bashu",
            args![
                "Welcome... this place is a training place for monks, Saint Capitolina Abbey.",
                "When you go inside....you will meet Tomoon the oldest monk who succeeds to the predecessors,"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bashu",
            args![
                "Please be advised and do not touch anything.",
                "And please avoid talking loud in front of Tomoon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Bashu", args!["I hope you will have a great time in here."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn bashu_mk(ctx: &Ctx) -> Script {
    bashu_mk_body(ctx, Vec::new()).map(|_| ())
}

fn apprentice_monk_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Monk Apprentice",
        args!["W... welcome!", "Th... this place is for testing the tolerance of monk candidates!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Monk Apprentice",
        args!["Ju... just run...", "until you're told to stop,", "Ru...ruu....run!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Monk Apprentice", args!["M... m... me? I'll run one of these days!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Monk Apprentice",
        args!["M... monk... are you going to be... a... m...m...monk??"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Monk Apprentice",
        args!["Ar...are...you...sure you are.. aren't... going to quit?"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Quit.:Keep running.")])?) == 1 {
        ctx.lines_as(
            "Monk Apprentice",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("...q.q..q. .quit! ...the marathon!! Y...you do not have what it takes to be a m... monk!"))
            ],
        )?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("monk_test"),
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from("...q.q..q. .quit! ...the marathon!! Y...you do not have what it takes to be a m... monk!")),
                ctx.constant("BC_MAP")?,
            ],
        )?;
        ctx.close_window()?;
        ctx.var("monk_q").set(Val::from(15))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3028), Val::from(3027)])?;
        ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(194), Val::from(168)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Monk Apprentice", args!["Until you're told to stop,", "Ru...ruu....run!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn apprentice_monk_mk(ctx: &Ctx) -> Script {
    apprentice_monk_mk_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SupervisorRaceMonkStep {
    Start,
    OnTouch,
}

fn supervisor_race_monk_run(ctx: &Ctx, mut step: SupervisorRaceMonkStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SupervisorRaceMonkStep::Start => {
                step = SupervisorRaceMonkStep::OnTouch;
                continue 'machine;
            }
            SupervisorRaceMonkStep::OnTouch => {
                if (ctx.var("monk_q").get()?.number()? >= 15 && ctx.var("monk_q").get()?.number()? <= 23) {
                    ctx.var("monk_q").set((ctx.var("monk_q").get()? + Val::from(1)))?;
                    ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(385), Val::from(388)])?;
                    return Err(Stop::End);
                } else if ctx.var("monk_q").get()? == 24 {
                    ctx.var("monk_q").set(Val::from(25))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3028), Val::from(3029)])?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("monk_test"),
                            Val::from("Now! This is the last lap!! If you make it you need to go visit Tomoon for the next test!"),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(385), Val::from(388)])?;
                    return Err(Stop::End);
                } else if ctx.var("monk_q").get()? == 25 {
                    ctx.lines_as(
                        "Supervisor",
                        args![
                            "Now...you may go visit Tomoon.",
                            "Tomoon is in the deepest room inside a building near this abbey."
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("monk_test"),
                            ((Val::from("Congratulations!") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("!! You completed the marathon!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(194), Val::from(168)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn supervisor_race_monk(ctx: &Ctx) -> Script {
    supervisor_race_monk_run(ctx, SupervisorRaceMonkStep::Start, Vec::new()).map(|_| ())
}

pub fn supervisor_race_monk_ontouch(ctx: &Ctx) -> Script {
    supervisor_race_monk_run(ctx, SupervisorRaceMonkStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TrapTMonk11Step {
    Start,
    OnTouch,
}

fn trap_t_monk1_1_run(ctx: &Ctx, mut step: TrapTMonk11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TrapTMonk11Step::Start => {
                step = TrapTMonk11Step::OnTouch;
                continue 'machine;
            }
            TrapTMonk11Step::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("monk_test"),
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(", you're trapped. You will be returned.")),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(387), Val::from(387)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn trap_t_monk1_1(ctx: &Ctx) -> Script {
    trap_t_monk1_1_run(ctx, TrapTMonk11Step::Start, Vec::new()).map(|_| ())
}

pub fn trap_t_monk1_1_ontouch(ctx: &Ctx) -> Script {
    trap_t_monk1_1_run(ctx, TrapTMonk11Step::OnTouch, Vec::new()).map(|_| ())
}
