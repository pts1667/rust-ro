use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum S11Step {
    Start,
    OnTouch,
}

fn s_1_1_run(ctx: &Ctx, mut step: S11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S11Step::Start => {
                step = S11Step::OnTouch;
                continue 'machine;
            }
            S11Step::OnTouch => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(200)])?;
                if subject1 == 2 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", yes you! Go back and start over!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 3 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" has failed!...well...you will if you don't start over...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 4 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", has failed me! Go back to where you started!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 5 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", you have blundered into a trap. I'm sorry, but for now, YOU LOSE.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 6 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", what are you doing!? Go back and do it again!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 7 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", come on! You can do better than this!! Try again!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 8 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", has fallen into a trap...again. But don't worry, you're getting better.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 9 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", fail, fail, fail... Go back to where you started!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 10 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("... aww~ Try again! You can do it!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 11 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", @#$%#^!@ and you wanna be a what? Hunter? Get back there!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 12 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" was defeated by the forces of evil. Don't give up, hero!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 13 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("I put a spell on you ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(" and now you're mine! Go start over now.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 14 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    ". G'day mate, welcome to the land down under. As a present, I'll send you to the starting point...",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 15 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("That's my nest ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("! dont step there, squawk~ Start over.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 16 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", another archer down. 3 more to go!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 17 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", have you got what it takes? Then try again.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 18 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Alas, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(
                                    ", you have fallen prey to the most feared of traps. The dreaded stink bomb! Run run run awaaaaaay!",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 19 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" had too much prune juice to drink today. Back to the starting point~")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 20 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("No.... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", you have fallen into a trap. You will be returned to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 21 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Meow *pounce* look my kitties we'll have some")
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("stew. Back to the starting point if you don't want to become stew!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 22 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Sorry ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", but you've actually found a BAD secret. You will be returned to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 23 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", silly Archer...your tenacity is touching. But it's back to the start for you...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 24 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", you've fallen and you can't get up. You'll be carried back to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 25 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Skip a turn ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", go back to start.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 26 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", nya nya, heh heh heh. You will be returned to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 27 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Practice is over ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(". This time show me for real.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 28 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    ", this is a Poring jellopy raid...Get out! Start from the beginning if you wish to continue.",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 29 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Disconnected from server. ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", you have to start over.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 30 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", what does this button do...oops! I'm afraid you have to start over.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 31 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    ", you have entered the bonus round! Aaaand, you lost. You will be returned to the starting point.",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 32 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Oh, no ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(
                                    ", you've stepped on a hive of bees. You narrowly escaped, all the way back to the beginning.",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 33 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", wait wait. That's a trap! Oh wait, that's dog. . . .")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 34 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Stop ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("! For stepping on that trap, I shall punish you!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 35 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", I'll be back. . . I hope you will too.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 36 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", Come with me if you want to live. . to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 37 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", my... precious! Go back to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 38 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Follow the yellow brick road... No wait, ")
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("!! Not that way...Back to the starting point it is.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 39 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", you've really go to get the hang of this 'in tune with nature' thing. Now, you're lost!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 40 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", it's only a story...not real. But you will be returned to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 41 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you have fallen into a trap. And the trap...has fallen into you. Returning you to the starting point.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 42 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), ((Val::from("Fear not ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", for you too, shall learn the powers of the dark side. You will now be returned to the starting point.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 43 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    ", fell into a trap. Quite easily too, I'm afraid. But this hero won't be beaten that easily!",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 44 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            Val::from(" Tip of the day: Do not step on the traps. You will be returned to the starting point."),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 45 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (((ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" fell into a trap. Once again, for clarity's sake, the name is "))
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(".")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 46 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Oy oy, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(".. I've seen blind Porings get further! Go back to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 47 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("MY EYES!! ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(
                                    ", you... you.. stepped on them. Go try again...I'd cry for you if I could even shed tears...",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 48 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", where are you? Right...NOT at the place where'd you be if you passed the test.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 49 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from(" Wow, they actually paid a guy to think of these comments. ")
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", you can go back to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 50 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from(" My word, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("! You've precariously fallen into a trap! You will be returned back to the start. Cheerio~!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 51 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", do you have any idea where you're going? Start over *sigh*.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 52 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", do you like green tea ice cream? No? Go back to the starting point...*hmph*")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 53 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Oh, no.. ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", are you hurt? Come now, let's go back to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 54 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Hi ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", how are you? Here's a present~ a free warp! Back to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 55 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(".......... did you know that that was a trap? I hope so, otherwise, I guess you'd be pretty embarassed.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 56 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", what do you think of the interior design? I worked very hard on it... and this lovely feature that sends you back to the starting point.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 57 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("BOOM BAM BOOM! ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", you have fallen into my trap! But I won't kill you...just...humiliate you...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 58 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(".. You can do better. Try again!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 59 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from("Hello, this is your local Kafra worker... ")
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", your mommy is waiting for you at the checkout line.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 60 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", if you do well. I'll have a nice present waiting for you... What is it? You'll see if you pass~ Go try again now!")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 61 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" faaaaaaaaaaaaaaaaailed.. Hehe, just kidding. You will be returned to the starting point so try again!")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 62 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you fell into a trap. You should watch your feet more. What? Invisible? That's baby talk! Now go try again!")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 63 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            ((Val::from(" Sorry, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", you have fallen into a trap. But I'll be nice and send you to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 64 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", HAHAHAHAHAHA!!!! I can't believe you fell for that one!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 65 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I was just about to tell you about that one...but, I didn't? Back to start.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 66 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("...I'm sorry my friend, but you lose. But don't worry, its not like this message is broadcast or anything.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 67 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I'm not so sure that you qualify for this anymore... Try again and prove me wrong.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 68 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", OW...now that's gotta hurt. Back to the start~")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 69 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I find your lack of faith disturbing... Let's start over.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 70 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    ", your eyes can deceive you. Don't trust them. Stretch out with your feelings. Try again now.",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 71 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", if you once start down the dark path, forever it will dominate your destiny. You shall be returned to the starting point.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 72 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    ", size doesn't matter. But, the number of traps that caught you do. Try again and win this time!",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 73 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", try not. Do or do not. There is no try.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 74 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", that is why you fail. Now try again!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 75 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", look. If its made of metal and looks like it has teeth, you shouldn't put your foot in it. So simple, really...")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 76 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", you are beaten. It is useless to resist.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 77 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I'm looking forward to completing your training. In time you will call me Master.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 78 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", your overconfidence is your weakness.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 79 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", watch your step. This place can be a little rough.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 80 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", oh I told you it was dangerous here.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 81 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", everything is proceeding as I have foreseen.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 82 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you should have not come back.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 83 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", there are alternatives to fighting. But not to losing to this test. Now try again!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 84 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", now that's a name I haven't heard in a while...let's hope I don't hear it again~!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 85 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", we do not train to be merciful here, mercy is for the weak!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 86 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", defeat does not exist here, does it?!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 87 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", concentrate. Focus your powers!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 88 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", am I going mad, or did the word -think- escape your lips?")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 89 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you're just stalling now.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 90 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", life isn't always fair.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 91 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(".... Uhhh...it's not OUR fault....")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 92 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", survivability takes priority.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 93 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", people have the strength to overcome their obstacles...everyone can.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 94 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", there is happiness for those who accept their fate, and there is glory for those who resist their fate.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 95 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", some things are real and some things are false. Can't you tell the difference?")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 96 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", we are having fun aren't we? Everyday here is like a dream. I really hope it's going to be like this forever.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 97 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I'll give you just one piece of advice... dying hurts like hell.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 98 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", don't give up the ghost! Just be more careful...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 99 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", death is a gift given at birth. But...these traps won't kill you, don't worry.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 100 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I was counting on you from the beginning. But I guess you're gonna make me wait longer...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 101 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", always with the end comes hope and rebirth. But it's back to the beginning for you~")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 102 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", have I ever told you about the power held in a single tear? Its okay to shed one now, I know its hard...")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 103 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", happiness often sneaks in through a door you didn't know you left open. Sort of like...hidden traps!")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 104 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", ....Chii?")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 105 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", Mine! Mine! Mine! Mine!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 106 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", it's time to duel!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 107 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", I choose you!...wait... uagh...not you!...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 108 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", this isn't the time to be complimenting it!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 109 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from("...Oh great, what else could go wrong today?")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 110 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", don't worry, I always keep a spare. We're not running out of traps anytime soon...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 111 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", when bad things happen, don't give up. The day will come when you will look back and laugh at them. But not today.")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 112 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", don't worry about it. We're good at fighting losing battles, remember?")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 113 {
                    ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you remember how it was like when you felt really slick? That feeling won't come back for a while...")), ctx.constant("BC_MAP")?])?;
                } else if subject1 == 114 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", you must forget about your past for the sake of your own happiness.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 115 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", if you can fool your friends, you can fool your enemies.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 116 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", this world is made up of love and peace!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 117 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", if you fail, you must drink this.....*evil grin*")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 118 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", don't say we didn't warn you!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 119 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", curiosity killed the cat. Lucky for me I'm not a cat. Oh, and um, you too of course.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 120 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", STUDY!STUDY!STUDY!STUDY!STUDY!STUDY!STUDY!STUDY!STUDY!STUDY!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 121 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", how you like me now? That's right, that was MY trap!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 122 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from("...brilliant...just absolutely...genius. Sorry, I was talking about all these crazy traps.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 123 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", it's time to bust out of that trap, Houdini-style! Hey, wait...come back!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 124 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", it looks like you could really be in trouble this time...no worries, just try this again!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 125 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from("...not smooth, dude. Go for it again, one more time!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 126 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(",it looks like you could really be in a pickle. Back to the starting point...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 127 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", just think of it as Karma. Someday, you'll be setting hundreds of traps of your own...")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 128 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", bad news dude...its another trap. You'll get the hang of this, I believe in you!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 129 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", don't give up! The world still needs you!")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 130 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", it's okay to cry. Just not too loudly.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else if subject1 == 131 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(
                                    "...oh man. It's tiring setting up all these traps. You guys have got to stop falling into them!",
                                )),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(", you have fallen into a trap. You will be returned to the starting point.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                }
                ctx.var("hntr_q").set(Val::from(13))?;
                ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#hnt::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_1_1(ctx: &Ctx) -> Script {
    s_1_1_run(ctx, S11Step::Start, Vec::new()).map(|_| ())
}

pub fn s_1_1_ontouch(ctx: &Ctx) -> Script {
    s_1_1_run(ctx, S11Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S574Step {
    Start,
    OnTouch,
}

fn s_57_4_run(ctx: &Ctx, mut step: S574Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S574Step::Start => {
                step = S574Step::OnTouch;
                continue 'machine;
            }
            S574Step::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                            + Val::from(", has failed me! Go back to where you started!")),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.var("hntr_q").set(Val::from(13))?;
                ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#hnt::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_57_4(ctx: &Ctx) -> Script {
    s_57_4_run(ctx, S574Step::Start, Vec::new()).map(|_| ())
}

pub fn s_57_4_ontouch(ctx: &Ctx) -> Script {
    s_57_4_run(ctx, S574Step::OnTouch, Vec::new()).map(|_| ())
}
