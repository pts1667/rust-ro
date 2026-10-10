use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kellasus_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 13) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Keep up the",
                "good work. I get",
                "the feeling that",
                "our future might",
                "be depending on you..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 12) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Alright then, I'll begin",
                "by teaching you ^FF0000Bioethics^000000,",
                "the fundamental skill for",
                "creating Homunculi."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "In order to master Bioethics,",
                "you've got to appreciate the",
                "sweetness of life and genuinely",
                "care about living creatures.",
                "And so, you must always take",
                "care of your Homunculus."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "You must constantly check",
                "to see if your Homunculus",
                "is tired or hungry, or if it's",
                "been hurt. You must also take",
                "into account your Homunculus's feelings towards you, its owner."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "When that nurturing attitude",
                "becomes second nature to you,",
                "you will have mastered Bioethics. It sounds simple, but you can only",
                "understand a relationship with a Homunculus through experience."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Well, you should know enough",
                "by now to create and care for",
                "your own Homunculus. I trust",
                "that you will be responsible",
                "in your search for discoveries",
                "that may benefit mankind."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kellasus", args!["Before we part", "ways, would you", "tell me your name?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "My name is...",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(13))?;
        ctx.call(
            Function::Skill,
            vec![Val::from("AM_BIOETHICS"), Val::from(1), ctx.constant("SKILL_PERM")?],
        )?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Ah...",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "I'll remember that. Goodbye",
                "for now, and I hope that you",
                "make a great contribution to",
                "the world of Alchemy someday."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 11) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Our viewpoints may be",
                "different, but I'll agree that",
                "there's too much potential in",
                "studying Homunculi that I can",
                "no longer ignore as a scientist",
                "and as a humanitarian."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "If I teach you these skills,",
                "you may be treading a moral",
                "line. But I believe it should",
                "be alright if you work to learn",
                "something that will benefit",
                "the greater good."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Does...", "Does that mean", "you'll teach me?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Now don't get too excited.",
                "I'm still unsure if Homunculus",
                "Study should be open to all",
                "Alchemists, but for now,",
                "I believe I can trust you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "However, you've got to",
                "use the skills I teach you",
                "to ultimately benefit other",
                "people. It's not like I can force you to forget once you know how",
                "to make Homunculi, understand?"
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(12))?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Please give me a little",
                "time to prepare my lesson",
                "for you. It's been a very long",
                "time since I've done this..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 10) {
        ctx.var("bioeth").set(Val::from(11))?;
        ctx.lines_as(
            "Kellasus",
            args![
                "^333333*Sigh*^000000",
                "I'm so confused...",
                "There's no denying that",
                "everything you said is right.",
                "But now I don't know what",
                "I should believe..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 9) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Have you come to",
                "ask me again to teach",
                "you Homunculus skills?",
                "You're awfully persistent,",
                "but I suppose that might be",
                "how you became an Alchemist..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, I sort of came",
                "here to ask about that.",
                "But first, I heard a rumor",
                "that your son was really",
                "sick for a while..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Y-yes. It's really",
                "difficult for me to",
                "talk about that time.",
                "The thought of losing",
                "my boy is too much to bear.",
                "He means the world to me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "I was so desperate to",
                "keep my boy alive that",
                "I would have sold my soul",
                "without any regret. I'm more",
                "than willing to sacrifice my",
                "own life for his sake."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Luckily, my son",
                "recovered from his",
                "disease and he is very",
                "healthy nowadays. But",
                "why are you reminding",
                "me of something like this?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "You're right that the creation",
                "of Homunculi brings up a moral",
                "issue. But instead of morals,",
                "I want to appeal to your sense",
                "of humanitarianism if I can."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args!["I don't follow.", "What exactly", "do you mean by", "humanitarianism?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I believe that the",
                "study of Homunculi",
                "could help us discover",
                "the secret of life. There",
                "is just too much",
                "potential to do good to ignore."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I'm not interested in",
                "creating new creatures or",
                "playing God. What I do want",
                "is the knowledge to cure people",
                "regardless of the disease. No",
                "matter how desperate it may be."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "It may be immoral to",
                "create experimental life",
                "forms and to sacrifice them,",
                "but I'm willing to do that for",
                "the people who may need",
                "that cure I might discover..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "When your son was",
                "sick, you were willing",
                "to make sacrifices to see",
                "that he recover. But still,",
                "there are other people in the",
                "world with worse ailments..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I want to study",
                "Homunculi because",
                "there's a chance that",
                "I might be able to bring",
                "hope to someone's",
                "hopeless situation..."
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(10))?;
        ctx.lines_as(
            "Kellasus",
            args!["I've never...", "I never thought...", "I'm sorry. I need", "some time alone..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 8) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Although I admit that",
                "you're capabile of learning",
                "what I might be able to teach",
                "you about Homunculi, I can't",
                "in good conscience do it. I'm",
                "sorry, but please give up."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 7) {
        ctx.lines_as(
            "Kellasus",
            args![
                "How many times must I tell",
                "you? I'm sorry, but I can't teach you anything related to Homunculi.",
                "It especially bothers me that you know nothing at all about them..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "But while I was gone,",
                "I spent so much time",
                "studying about Homunculi",
                "from these Alchemy experts.",
                "I guarantee that I'm qualified",
                "to learn the fundamentals!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Experts...? Ah, you mean",
                "Skrajjad, Keshibien and",
                "Broncher. My old colleages.",
                "Alright. But I'll be the judge",
                "of whether or not you learned",
                "anything about Homunculi."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "I'm going to give you",
                "several questions, so",
                "I hope you answer all",
                "of these correctly.",
                "Are you ready?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "First question.",
                "What skill, mastered",
                "at Skill Level 1, allows",
                "the caster to retreat a",
                "Homunculus, putting it in",
                "an inert state of hibernation?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Vaporize" {
            ctx.lines_as(
                "Kellasus",
                args![
                    "Hmm, not bad. So you've",
                    "been studying. Now, the second",
                    "question. What is the item, whose name indicates a very early stage",
                    "of Homunculus development, used for the Call Homunculus skill?"
                ],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Embryo" {
                ctx.lines_as(
                    "Kellasus",
                    args![
                        "Huh. You got that one",
                        "right too. Alright, third",
                        "question. What is the",
                        "name of the skill, mastered",
                        "at Skill Level 5, which allows",
                        "you to resurrect Homunculi?"
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s.clone() == "Homunculus Resurrection" {
                    ctx.lines_as(
                        "Kellasus",
                        args![
                            "So you have been putting",
                            "a lot of effort in studying",
                            "this. I really admire your",
                            "dedication and it looks like",
                            "you've got the potential to",
                            "be a really great Alchemist."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kellasus",
                        args![
                            "Still, even though you're",
                            "qualified to learn all the",
                            "Homunculus skills, I choose",
                            "not to teach them based on",
                            "my own personal principles.",
                            "Nothing can change my mind."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kellasus",
                        args![
                            "I admit that I'm proud",
                            "of the progress you've",
                            "made, but I just can't",
                            "bring myself to be a part",
                            "of what I believe to be an",
                            "abuse of the gift of life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("bioeth").set(Val::from(8))?;
                    ctx.lines_as(
                        "Kellasus",
                        args![
                            "There's just so much",
                            "risk! I don't think I can",
                            "bear to be responsible for",
                            "any of the consequences that",
                            "may come with the existence",
                            "of artificially created life."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Kellasus",
                        args![
                            "I'm disappointed...",
                            "I really thought you",
                            "had learned everything",
                            "you could about Homunculi,",
                            "but it looks like there are a",
                            "few gaps in your knowledge."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as(
                    "Kellasus",
                    args![
                        "Hmm... It looks like",
                        "you still haven't learned",
                        "enough about Homunculi.",
                        "Even if I wanted to teach",
                        "you, it looks like you're",
                        "not quite ready to learn..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Kellasus",
                args![
                    "You'd be able to get",
                    "the first question right",
                    "if you were really serious",
                    "about studying Homunculi.",
                    "You'd better just give up and",
                    "find a new alchemic interest."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()?.number()? > 3) {
        ctx.lines_as(
            "Kellasus",
            args![
                "I'm sorry, but",
                "I refuse to teach",
                "you anything related",
                "to the Homunculus skills."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "I should feel insulted that",
                "you asked me, but once there",
                "was a time when I was more",
                "like you, so I can understand",
                "how you feel. However, my",
                "views are much different now..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 3) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Oh, hello, what",
                "brings you this way?",
                "As your senior in the",
                "field of Alchemy, I can",
                "give you some advice if you're stumped by a particular problem..."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Teach me the Homunculus Skills.:No, nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I want you to", "teach me how to", "create a Homunculus."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kellasus",
                args!["...", "Come again?", "I don't think I heard", "what you said quite right."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I said...", "I want you to", "teach me the skills", "for Homunculus creation."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kellasus",
                args![
                    "After all of my talk...",
                    "My warnings about knowledge",
                    "that should be forbidden to",
                    "Alchemists, you want to go",
                    "and create a Homunculus?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kellasus",
                args![
                    "Perhaps I never should",
                    "have mentioned that subject.",
                    "But I still don't understand why you would suddenly want to ",
                    "learn those skills. Yes, it's",
                    "true that I could teach you..."
                ],
            )?;
            ctx.next()?;
            ctx.var("bioeth").set(Val::from(4))?;
            ctx.lines_as(
                "Kellasus",
                args![
                    "But I refuse to pass down",
                    "my knowledge of Homunculi",
                    "to anyone for any reason.",
                    "Surely, you must know that",
                    "by now. I'm stupefied that you",
                    "even had the audacity to ask!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Kellasus",
            args![
                "No? Well, I suppose",
                "learning something on",
                "your own is good, but",
                "sometimes it's good to ask",
                "for help from those who are",
                "more experienced than you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 2) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Ah, you've returned...",
                "Now, you must be wondering",
                "why I have been warning you",
                "about the dangers of using",
                "Alchemy to obtain knowledge",
                "that should be forbidden."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "It is better that you know now",
                "than to find out later and not",
                "be prepared. There is a skill",
                "in Alchemy that transgresses",
                "the natural laws by creating",
                "life not intended by God."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "It is an extremely",
                "dangerous skill that was",
                "developed out of human",
                "arrogance. Life is a beautiful",
                "thing, a precious gift, that",
                "we dare not tamper with."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "My greatest shame in",
                "life is my contribution to",
                "that field of knowledge. At",
                "that time I was young and",
                "headstrong, too proud to",
                "admit I was playing God."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "I even went so far as to",
                "become the most skilled",
                "Alchemist in the creation",
                "of ^FF0000Homunculi^000000. But I've",
                "changed my ways. I don't...",
                "I'll never create one again."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Alchemy is an incredible",
                "field of study, and it's great",
                "that you've dedicated yourself",
                "to it, but always remember that",
                "this science has great potential for abuse, as well as for good."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "I only fear that your quest",
                "for knowledge, your insatiable",
                "curiosity to reveal the unknown, may lead you on the path of the",
                "Homunculus. For your own sake, don't bother considering the idea."
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(3))?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Remember that all life is",
                "precious. If your motives",
                "are good, and untainted by",
                "the desire for fame or fortune,",
                "and if you work for the greater good of mankind, you'll be fine."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 1) {
        ctx.lines_as(
            "Kellasus",
            args![
                "Ah, it's you again.",
                "Since you're here, let",
                "me ask you a question.",
                "What exactly do you",
                "think of Alchemy?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Wait. Don't answer",
                "that. As someone more",
                "experienced in this field,",
                "I believe that the answer",
                "to that question is more",
                "complicated than you think."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Most people see Alchemists",
                "as greedy scientists, assuming",
                "that we use our skills exclusively for^FFFFFF.^000000the transmutation of gold. Of",
                "course, learning how to do that",
                "is part of our basic training."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "But learning transmutation",
                "is part of something bigger.",
                "All Alchemists must strive to",
                "harness nature for the good",
                "of mankind. To do this requires",
                "the study of every science."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "The fields of Medicine,",
                "Chemistry, Biology, Geology,",
                "Astronomy, Astrology, Geomancy,",
                "Mathematics, Physics and even",
                "Philosophy must all be learned",
                "by the competent Alchemist."
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(2))?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Once you've entered the",
                "exciting vastness of Alchemy,",
                "there is no turning back. Do",
                "your best to learn all that you",
                "can, and never forget that you have responsibilities to mankind."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) {
        ctx.lines_as(
            "Kellasus",
            args![
                "You're another",
                "practitioner of",
                "Alchemy, aren't you?",
                "It's been a long time",
                "since I've spoken to",
                "such a young colleague..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "As your elder colleague,",
                "I encourage you to focus",
                "on your training. Enlighten",
                "yourself and satisfy your",
                "curiosity by solving the",
                "riddles of science."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kellasus",
            args![
                "But heed my words...",
                "There are secrets of",
                "science that must never",
                "be discovered by humans.",
                "Do not be so proud as to",
                "seek out forbidden truths."
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(1))?;
        ctx.lines_as(
            "Kellasus",
            args![
                "Good luck in your",
                "studies, and I hope",
                "you never use your",
                "search for answers as",
                "an excuse to give in to",
                "greed and obsession."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Kellasus",
        args![
            "Hmm. Who decides what",
            "is right and wrong? What is",
            "meant to be known and what",
            "secrets were never intended",
            "for mankind to understand?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kellasus",
        args![
            "There are many who would",
            "agree that God alone wields",
            "authority over life and death.",
            "But Alchemy does provide",
            "a way for the audacious..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kellasus_qsk_al(ctx: &Ctx) -> Script {
    kellasus_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn skrajjad_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()?.number()? > 4) {
        ctx.lines_as(
            "Skrajjad",
            args![
                "I've taught you everything",
                "I can about the Homunculi.",
                "I hope that I've been of some",
                "help in your quest for knowledge and understanding of our universe."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "If you haven't already done",
                "so, please visit Keshibien,",
                "master of the Call Homunculus",
                "skill. He's a kind person who's",
                "always willing to go out of his",
                "way to help young scientists."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 4) {
        ctx.lines_as(
            "Skrajjad",
            args![
                "Ah, you're the one",
                "who had the courage",
                "to ask Kellasus to teach",
                "you Alchemy. I must say",
                "that I admire your attitude,"
            ],
        )?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("young lad. Now, don't worry...")?;
        } else {
            ctx.mes("young lass. Now, don't worry...")?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "I understand how you",
                "must feel after Kellasus",
                "refused to teach you his",
                "methods, but that does not",
                "mean that you'll never learn",
                "how to create a Homunculus."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "I believe you'll be able to learn what you desire if you manage to",
                "train under all of the Homunculus experts. When you show how serious",
                "you are about this, then Kellasus might change his mind about you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "Let give teach you my",
                "Homunculus specialty,",
                "the ^FF0000Vaporize^000000 skill. Actually,",
                "it's useless without Kellasus's",
                "technique, but it should be a",
                "good enough starting point."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "The Vaporize skill",
                "is an Alchemist skill",
                "that deals solely with",
                "Homunculi. Vaporize is",
                "mastered at Skill Level 1."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "This skill allows you to",
                "reduce your Homunculus",
                "to vapor so that it is in an",
                "inert chemical state. It's",
                "very similar to hibernation",
                "when you think about it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "Now, if you want to",
                "bring back a vaporized",
                "Homunculus, you will need",
                "to know the ^FF0000Call Homunculus^000000",
                "which you can learn from",
                "Keshibien. Just remember..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "Homunculi are living",
                "creatures, so don't go",
                "and vaporize and recall",
                "them recklessly. That sort",
                "of behavior would be inhumane."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "''Show respect for all life",
                "forms.'' That's the creed",
                "for the Homuculus expert.",
                "Now go and find ^FF0000Keshibien^000000",
                "so that he can teach you",
                "the Call Homunculus skill."
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(5))?;
        ctx.lines_as(
            "Skrajjad",
            args![
                "Good luck in",
                "your studies",
                "and I hope you",
                "find the answers",
                "that you are seeking."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Skrajjad",
        args![
            "Alchemy is wondrous...",
            "It incorporates every",
            "science and many other",
            "fields of knowledge that",
            "it's not enough to be jack",
            "of all trades... No..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Skrajjad",
        args![
            "In a sense, you must",
            "be a master of all trades",
            "to be proficient in Alchemy.",
            "But it's incredibly rewarding to those of us who never stop asking",
            "how and why our world works."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn skrajjad_qsk_al(ctx: &Ctx) -> Script {
    skrajjad_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn keshibien_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()?.number()? > 5) {
        ctx.lines_as(
            "Keshibien",
            args![
                "I hope that you",
                "can make good use",
                "of what I've taught you.",
                "I'm glad that I was able",
                "to contribute something to",
                "your learning of Alchemy."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Now, the only person qualified",
                "to teach you the Homunculus",
                "Resurrection skill is Brocher.",
                "He's... He's a character, but",
                "he's also a great teacher."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Now...",
                "Where could he be?",
                "I know he likes to drink,",
                "so maybe you can find him",
                "some place where they serve",
                "alcohol in Lighthalzen?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 5) {
        ctx.lines_as(
            "Keshibien",
            args![
                "Oh! You must be the",
                "one Skrajjad told me",
                "about. So you've come",
                "so I can teach you the",
                "Call Homunculus skill?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "First of all, it's always",
                "a pleasure to meet another",
                "colleague. I'm Keshibien, and",
                "although I'm an expert on this",
                "particular skill, overall I'm just a run of the mill Alchemist."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Anyway, the Call Homunculus",
                "skill requires knowledge of the",
                "Vaporize skill and is mastered",
                "at Skill Level 1. This skill allows you to summon a Homunculus.",
                "Pretty simple so far, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Now, you can summon a",
                "vaporized Homunculus if",
                "you happen to have one.",
                "Otherwise, you can just",
                "create one through using",
                "this skill and these items..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "^FF00001 Embryo^000000,",
                "^FF00001 Glass Tube^000000,",
                "^FF00001 Seed of Life^000000 and",
                "^FF00001 Morning Dew of Yggdrasil^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Well, that's all the",
                "technical information",
                "I have to share with you.",
                "Always remember to treat",
                "your Homunculus well. Eh,",
                "once you summon one, anyway."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "But before you can do",
                "that, you need to learn",
                "the techniques that only",
                "Kellasus can teach you.",
                "For now, go to ^FF0000Broncher^000000",
                "and learn from him, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Broncher happens",
                "to specialize in the",
                "Homunculus Resurrection",
                "skill. You should be able to",
                "find him somewhere in town,",
                "though I'm unsure where..."
            ],
        )?;
        ctx.next()?;
        ctx.var("bioeth").set(Val::from(6))?;
        ctx.lines_as(
            "Keshibien",
            args![
                "Anyway, it was really",
                "nice to meet you. Hopefully,",
                "you'll be able to learn how",
                "to fully use a Homunculus",
                "someday soon. Still, you'll",
                "need to convince Kellasus..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Keshibien",
        args![
            "Hello there~",
            "You must be from",
            "Rune-Midgarts, right?",
            "It's nice to meet you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Keshibien",
        args![
            "I hear that the",
            "Alchemists from",
            "over there are pretty",
            "skilled. I wonder if I'll",
            "ever get the chance to",
            "collaborate with any of them..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn keshibien_qsk_al(ctx: &Ctx) -> Script {
    keshibien_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn broncher_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()?.number()? > 6) {
        ctx.lines_as(
            "Broncher",
            args![
                "You again? Didn't I already",
                "teach you the Homunculus",
                "Resurrection skill? Come",
                "on, there's nothing else",
                "I can do for you. Lemme",
                "enjoy myself here..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "If you didn't already,",
                "go and bother Kellasus",
                "and get him to teach you the",
                "basic Homunculus techniques.",
                "Now go away, you're sobering",
                "me up! Go on, get outta here!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 6) {
        ctx.lines_as(
            "Broncher",
            args![
                "^333333*Hiccup!*^000000",
                "Oh man...",
                "Oh man, this stuff",
                "is great! Yeah. Yeah,",
                "this really hits the spot."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Brocher",
            args![
                "Holy crap! Y-you're that",
                "Alchemist that Skrajjad",
                "told me to expect! Wha--?",
                "You already visited Keshibien?",
                "Crap! I thought you wouldn't",
                "get here for another... Hour!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "Oh, forget sobering.",
                "I'll just teach you the",
                "^FF0000Homunculus Resurrection^000000",
                "skill. It's one of the last",
                "skills you need to learn",
                "for Homunculi, anyway."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "Yes, you can literally",
                "bring Homunculi back",
                "from the dead. Oh, and you",
                "should know that this skill is",
                "mastered at Skill Level 5."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "To bring your",
                "Homunculus back from",
                "the dead, you just push",
                "this button, the one that's",
                "labeled, ''Homunculus",
                "Resurrection.'' Easy, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "Well, lesson's over.",
                "You know everything you",
                "need to know about using",
                "Homunculus Resurrection.",
                "But you still need Kellasus",
                "to teach you the fundamentals."
            ],
        )?;
        ctx.var("bioeth").set(Val::from(7))?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "Without those basic",
                "techiques, everything",
                "you've studied from us",
                "will be for nothing. So...",
                "Lots of luck convincing",
                "Kellasus to teach you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Broncher",
            args![
                "Now leave alone",
                "and let me enjoy",
                "all of this alcohol",
                "in solitude, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Broncher",
        args![
            "^333333*Hiccup~*^000000",
            "Oh yeah, this is",
            "the stuff! I call it...",
            "My ''Anti-Sobriety''",
            "formula! Can't think",
            "of a finer thing created..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn broncher_qsk_al(ctx: &Ctx) -> Script {
    broncher_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn koring_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 8)
        || ctx.var("bioeth").get()? == 9)
    {
        ctx.lines_as(
            "Koring",
            args![
                "My daddy is the bestest",
                "Alchemist! He's sooooo",
                "smart, and I never heard",
                "him be wrong before!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Koring",
            args![
                "Daddy's been thinking",
                "a lot lately and I don't",
                "know why. Daddy gets ",
                "really serious when he's ",
                "worried about something."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Koring",
            args![
                "He's always trying hard",
                "to make me happy and to ",
                "help me when I have problems.",
                "Like the time I was really sick..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Koring",
            args![
                "Daddy did his best to make",
                "some medicine to help me, ",
                "and he looked all over for any",
                "doctor who could cure me. For",
                "a while, I was really scared,",
                "but my Daddy never gave up."
            ],
        )?;
        ctx.next()?;
        if ctx.var("bioeth").get()? == 8 {
            ctx.var("bioeth").set(Val::from(9))?;
        }
        ctx.lines_as(
            "Koring",
            args![
                "He might be a little",
                "grumpy now, but my ",
                "Daddy is still the best!",
                "I hope he feels better soon."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Koring",
        args![
            "How did my Daddy get",
            "so smart? I wish I could",
            "learn things as fast as",
            "him so that I can be an",
            "Alchemist too when I grow up!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn koring_qsk_al(ctx: &Ctx) -> Script {
    koring_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn beninne_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) && ctx.var("bioeth").get()? == 8)
        || ctx.var("bioeth").get()? == 9)
    {
        ctx.lines_as(
            "Beninne",
            args![
                "You've met my husband,",
                "Kellasus? He's either a",
                "very stubborn or determined",
                "man, depending on how you",
                "want to look at it. Once his",
                "mind's made up, he won't budge!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Beninne",
            args![
                "He's a very good husband",
                "and father, but I feel like it",
                "takes the entire world to",
                "change his mind once he's ",
                "convinced that he's right",
                "about something."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Beninne",
            args![
                "Of course, if you manage",
                "to prove him wrong, he will",
                "do everything he can to make",
                "up for it. Kellasus has been",
                "like that since I first met him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Beninne",
            args![
                "It feels so nice to",
                "have someone like that",
                "to be protective of you.",
                "Kellasus loves his work,",
                "but he loves his family even",
                "more. Amazing, isn't it?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Beninne",
        args![
            "Most people think that",
            "being a scientist's wife",
            "would be so difficult. I must",
            "be lucky that my husband is",
            "such a good family man, even",
            "though he may be an Alchemist."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn beninne_qsk_al(ctx: &Ctx) -> Script {
    beninne_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn nannan_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Nannan",
        args![
            "You know, I always thought that all Alchemists were bookish,",
            "scholarly types, their faces always buried in books and studying. But",
            "I managed to find one who doesn't fit that nerdy stereotype at all."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nannan",
        args![
            "This guy, what's-his-face,",
            "Broncher, is always wasting",
            "his time drinking. I guess he discovered the secret of turning",
            "water into Grade A booze. But at least he's not the stuffy type."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nannan",
        args![
            "I don't know how much",
            "help he'd be to an aspiring",
            "Alchemist, but in my opinion,",
            "the man is a fully fledged",
            "genius! At least, compared",
            "to a street guy like me..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn nannan_qsk_al(ctx: &Ctx) -> Script {
    nannan_qsk_al_body(ctx, Vec::new()).map(|_| ())
}

fn alchemist_qsk_al_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Alchemist",
        args![
            "Out of all humans,",
            "I believe Kellasus is",
            "the one who has come",
            "closest to discovering",
            "the secrets of life. He",
            "never fails to amaze me..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alchemist",
        args![
            "I'm also impressed by the",
            "fact that he doesn't let his",
            "work keep him from being the",
            "best father and husband that",
            "he can for his family. He's",
            "an example for all of us."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alchemist",
        args![
            "Kellasus really is",
            "an amazing person.",
            "There isn't one Alchemist",
            "that I know who doesn't look",
            "up to him in the realms of",
            "both science and personal life."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn alchemist_qsk_al(ctx: &Ctx) -> Script {
    alchemist_qsk_al_body(ctx, Vec::new()).map(|_| ())
}
