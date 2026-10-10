use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn father_bamph_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 13 {
        ctx.lines_as(
            "Father Bamph",
            args![
                "Welcome to",
                "Prontera Church.",
                "Please relax, and",
                "let your mind and",
                "spirit find rest in",
                "these hallowed halls."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Give him the herb pouch.")])?;
        ctx.var("@menu").set(choice)?;
        if ctx.call(Function::CountItem, vec![Val::from(7432)])?.number()? < 1 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Ack!", "Now, where did I put", "that pouch with the herbs?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Father Bamph",
            args!["May I ask...", "Why are you giving", "me this pouch of herbs?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, there was this",
                "lady living on Mount",
                "Mjolnir who wanted me",
                "to give this to you. Um, she",
                "has a son named Kaanu?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Oh, you must have met",
                "Bonnie Imbullea. It's",
                "been a long time since",
                "I've seen her. How is",
                "she doing, may I ask?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh...", "I guess she's fine.", "But she seems really...", "I dunno, tormented..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "Good! If she were happy",
                "and relaxed after what she",
                "did, not even God would forgive",
                "her! She deserves to live the",
                "rest of her life in agony!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Father Biscuss, how can",
                "you say something like that?",
                "She did her best to stop what",
                "had happened! We should pity",
                "her for the suffering she must be feeling. Where is your compassion?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "Bah! Don't speak to me",
                "about compassion! Weren't",
                "you the one who recommended",
                "her to perform the exorcism in",
                "the first place? You didn't",
                "forget that, did you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Father Bampf", args!["......", ".........", "............"])?;
        ctx.next()?;
        ctx.lines_as("Father Biscuss", args!["Hah!"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("About Bonnie Imbullea")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Well...",
                "I don't...",
                "I don't know if",
                "I should be telling ",
                "you about that incident..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "You know what? If Bonnie",
                "sent this adventurer here",
                "for a favor, it probably means",
                "she trusts this person. Besides, we need somebody to carry out"
            ],
        )?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.mes("this task for us. Why not him?")?;
        } else {
            ctx.mes("this task for us. Why not her?")?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Your words have the ring",
                "of truth, Father Biscuss.",
                "Well then, adventurer, give",
                "me the opportunity to tell you",
                "something very important. But, you must not tell anyone else..."
            ],
        )?;
        ctx.var("prt_curse").set(Val::from(14))?;
        ctx.call(Function::DelItem, vec![Val::from(7432), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("prt_curse").get()? == 14 {
            ctx.lines_as(
                "Father Bamph",
                args![
                    "What I am about to tell",
                    "you must be kept secret.",
                    "Now, do you know about",
                    "Jormungand, the great serpent",
                    "born from the god Loki and",
                    "the giantess, Angrboda?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "Jormungand was an evil",
                    "beast that, after the thousand",
                    "year war between the gods and",
                    "demons, began to attack humans.",
                    "It brought great chaos and much",
                    "suffering around the world..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "Finally, the first Tristram of",
                    "the Geoborg family defeated",
                    "Jormungand together with 6",
                    "other warriors, but only after",
                    "it killed his beloved father."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "Tristram would become the",
                    "first king of the Rune-Midgarts",
                    "Kingdom, but his family would",
                    "suffer from the curse placed",
                    "by Jormungand before its defeat. "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "To this day...",
                    "^FF0000Every first prince of",
                    "the Geoborg family dies",
                    "at a young age^000000. That is",
                    "Jormungand's curse and",
                    "the royal family's secret."
                ],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_HUK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "What...?!",
                    "Is this really true?!",
                    "Isn't there any way to",
                    "counter this horrible curse?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "Please lower your voice.",
                    "Yes, it's a shame that the",
                    "royal family must suffer like",
                    "this. But all the exorcisms",
                    "that have been attempted",
                    "over the years have failed..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "Although the situation",
                    "appeared hopeless, King",
                    "Tristram III tried once again to exorcise the curse. Following",
                    "his orders, the greatest priests and exorcists were all summoned."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "A series of tests was",
                    "performed to select the",
                    "best priests and exorcists",
                    "to remove this curse. In the end, ^3131FFBonnie Imbullea^000000 was chosen to",
                    "lead the exorcism ceremony."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Wait, but you mentioned",
                    "samething about an accident.",
                    "Does that mean that she failed",
                    "in performing the exorcism?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Bamph",
                args![
                    "Yes, unfortunately.",
                    "After the priests and",
                    "exorcists gathered in the",
                    "secret ceremonial grounds,",
                    "three princes were killed",
                    "instead of just the first."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Biscuss",
                args![
                    "Although we all swore to",
                    "keep that incident a secret,",
                    "Bonnie Imbullea took full",
                    "responsibility for the deaths.",
                    "That is why she is in self",
                    "imposed exile from the kingdom."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["What...?", "That's...", "That's crazy!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Biscuss",
                args![
                    "I know that this",
                    "is a very long story,",
                    "and that it's a little",
                    "complicated. Please",
                    "listen to what we have",
                    "to ask from here on..."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.var("prt_curse").set(Val::from(15))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("prt_curse").get()? == 15 {
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "Now, you should know that",
                        "all of the princes that were",
                        "killed that day bore strange",
                        "looking marks on their bodies.",
                        "It almost looked as if they",
                        "were growing snake scales."
                    ],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Wait, they grew snake",
                        "scales? That sounds just",
                        "like one of the versions of",
                        "this song I'm investigating.",
                        "I think it was the song about the Rune-Midgarts Kingdom's origin."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "Wait, that song?",
                        "Hm, I don't see how",
                        "it's related. Let's see",
                        "now, how did it go?",
                        "Rainbows... Eagles...",
                        "Ah! Now I remember!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "The great serpent",
                        "swallowed the sea.",
                        "The eagle of the rainbow",
                        "swallowed the serpent.",
                        "Then the eagle built its nest.",
                        "A nest upon the swallowed sea."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Hmm...",
                        "Well, I heard a",
                        "different version",
                        "of the same song.",
                        "It goes like..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "The great serpent",
                        "swallowed the sea.",
                        "The eagle of the rainbow",
                        "swallowed the serpent.",
                        "Then snake scales grew on",
                        "the eagle, and it slowly died."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "Oh, my. I learned the song",
                        "when I was a young boy from",
                        "my father. However, your version seems to reveal the secret curse",
                        "of the Geoborgs. Please tell me, where did you hear that song?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well, I first heard this",
                        "version from a historian,",
                        "and then I found out that",
                        "Bonnie Imbullea knows it as",
                        "well. Um, is there a problem?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "If those song lyrics are",
                        "spread, the secret of the",
                        "royal family curse could be",
                        "revealed to the public. Would",
                        "you please help us by speaking",
                        "to Bonnie Imbullea once again?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "If you can, please try to",
                        "see if you can learn of any",
                        "connection between the curse",
                        "and the song, or if she can",
                        "remember anything that happened",
                        "after the exorcism failed..."
                    ],
                )?;
                ctx.var("prt_curse").set(Val::from(16))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("prt_curse").get()? == 16 {
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "Please visit Bonnie",
                            "Imbullea in Mount Mjolnir",
                            "to see if you can learn more",
                            "about that song, or about what",
                            "happened after the attempted exorcism of the Jormungand curse."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("prt_curse").get()? == 17 {
                        ctx.lines_as(
                            "Father Bamph",
                            args![
                                "Ah, have you spoken",
                                "to Bonnie Imbullea?",
                                "Please tell me if you",
                                "have learned anything new."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou related everything",
                            "that Bonnie Imbullea spoke",
                            "about, including the discovery",
                            "of a fragment of Red Gemstone",
                            "in the secret ceremonial grounds. "
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Bamph",
                            args![
                                "Red Gemstone? There's",
                                "no reason for that to be in",
                                "a holy place. Hmm. There's",
                                "more to this case than meets",
                                "the eye. We better investigate",
                                "the truth, starting now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Bamph",
                            args![
                                "Come with me, adventurer,",
                                "to the secret ceremonial",
                                "grounds for the royal family.",
                                "Just use the switch hidden",
                                "within the bookshelf and",
                                "I will meet you there."
                            ],
                        )?;
                        ctx.var("prt_curse").set(Val::from(18))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("prt_curse").get()?.number()? > 17 && ctx.var("prt_curse").get()?.number()? < 22) {
                            ctx.lines_as(
                                "Father Bamph",
                                args![
                                    "Come with me to the",
                                    "secret ceremonial grounds",
                                    "so that we can investigate",
                                    "the possibility of sabotage",
                                    "with the exorcism that",
                                    "Bonnie Imbullea performed."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Father Bamph",
                                args![
                                    "There is a hidden switch",
                                    "in the bookshelf that you",
                                    "can use to transport yourself",
                                    "there. Go, and I will meet",
                                    "you in the ceremonial grounds."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("prt_curse").get()? == 22 {
                                ctx.lines_as(
                                    "Father Bamph",
                                    args![
                                        "This is a matter of grave",
                                        "importance, but we can't",
                                        "alert the royal family yet and",
                                        "cause a panic. It would be",
                                        "best to fully investigate this",
                                        "first and collect proof."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Father Bamph",
                                    args![
                                        "Brave adventurer, would",
                                        "you please visit the Assassin",
                                        "Guild in Morocc and see if you",
                                        "can learn anything about how",
                                        "we can verify whether poison",
                                        "was used to kill the princes?"
                                    ],
                                )?;
                                ctx.var("prt_curse").set(Val::from(23))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("prt_curse").get()? == 23 {
                                    ctx.lines_as(
                                        "Father Bamph",
                                        args![
                                            "Please see if you can",
                                            "learn anything about testing",
                                            "for the use of poison from",
                                            "a member of the Assassin",
                                            "Guild in Morocc."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("prt_curse").get()? == 31 {
                                        ctx.lines_as(
                                            "Father Bamph",
                                            args![
                                                "Ah, you've returned~",
                                                "Have you managed to learn",
                                                "anything about poison from",
                                                "the Assassin Guild? I know",
                                                "their members must be",
                                                "very difficult to find..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^3355FFYou relate all of the",
                                            "information you learned",
                                            "about poison to Father Bamph,",
                                            "including the method to test",
                                            "for the use of poison in murder. "
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Father Bamph",
                                            args![
                                                "Ah, I see. Then, would",
                                                "you please bring 1 Yellow",
                                                "Gemstone and 1 Green Potion",
                                                "to the secret ceremonial grounds as soon as you can? I will wait",
                                                "over there for you once again."
                                            ],
                                        )?;
                                        ctx.var("prt_curse").set(Val::from(32))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (((((ctx.var("prt_curse").get()? == 32 || ctx.var("prt_curse").get()? == 33)
                                            || ctx.var("prt_curse").get()? == 41)
                                            || ctx.var("prt_curse").get()? == 42)
                                            || ctx.var("prt_curse").get()? == 51)
                                            || ctx.var("prt_curse").get()? == 52)
                                        {
                                            ctx.lines_as("Father Bamph", args!["You can use the hidden", "switch in the bookshelf to", "enter the secret ceremonial", "grounds. Don't forget that we", "need^3131FF 1 Yellow Gemstone^000000 and ^3131FF1 Green Potion^000000 to test for poison."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("prt_curse").get()? == 34 {
                                                ctx.lines_as(
                                                    "Father Bamph",
                                                    args![
                                                        "I can't believe it!",
                                                        "If poison was used, then",
                                                        "the second and third princes",
                                                        "were murdered! Unbelievable...",
                                                        "How can anyone kill children?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Father Bamph",
                                                    args![
                                                        "At the very least, we now",
                                                        "know that their deaths were",
                                                        "not Bonnie Imbullea's fault.",
                                                        "Now I must discuss with Father",
                                                        "Biscuss and decide how to tell",
                                                        "the royal family about this..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Father Bamph",
                                                    args![
                                                        "Once again, let me remind",
                                                        "you that everything that has",
                                                        "transpired here must be kept",
                                                        "secret. The public cannot know",
                                                        "of Jormungand's curse! It would cause political turmoil and chaos!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Father Bamph",
                                                    args![
                                                        "Still, that does not mean",
                                                        "that I cannot personally thank",
                                                        "you for all of your help. I'm very grateful for what you have done,",
                                                        "adventurer. May safety accompany you on all of your journeys."
                                                    ],
                                                )?;
                                                ctx.var("prt_curse").set(Val::from(35))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ((ctx.var("prt_curse").get()?.number()? > 34
                                                    && ctx.var("prt_curse").get()?.number()? < 40)
                                                    && !(ctx.var("aru_monas").get()?.is_true()))
                                                {
                                                    ctx.lines_as(
                                                        "Father Bamph",
                                                        args![
                                                            "I'm at a loss at what",
                                                            "to do. Sometimes, even",
                                                            "I have doubts and believe",
                                                            "that Odin has abandoned us..."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("prt_curse").get()? == 40 {
                                                        ctx.lines_as(
                                                            "Father Bamph",
                                                            args![
                                                                "Ah, you've returned~",
                                                                "Have you managed to learn",
                                                                "anything about poison from",
                                                                "the Assassin Guild? I know",
                                                                "their members must be",
                                                                "very difficult to find..."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines(args![
                                                            "^3355FFYou relate all of the",
                                                            "information you learned",
                                                            "about poison to Father Bamph,",
                                                            "including the method to test",
                                                            "for the use of poison in murder. "
                                                        ])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Father Bamph",
                                                            args![
                                                                "Ah, I see. Then, would",
                                                                "you please bring^3131FF 1 Yellow",
                                                                "Gemstone^000000 and ^3131FF1 Green Potion^000000",
                                                                "to the secret ceremonial grounds as soon as you can? I will wait",
                                                                "over there for you once again."
                                                            ],
                                                        )?;
                                                        ctx.var("prt_curse").set(Val::from(41))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if (ctx.var("prt_curse").get()? == 43 || ctx.var("prt_curse").get()? == 53) {
                                                            ctx.lines_as(
                                                                "Father Bamph",
                                                                args![
                                                                    "I can't believe it!",
                                                                    "If poison was used, then",
                                                                    "the second and third princes",
                                                                    "were murdered! Unbelievable...",
                                                                    "How can anyone kill children?"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Father Bamph",
                                                                args![
                                                                    "At the very least, we now",
                                                                    "know that their deaths were",
                                                                    "not Bonnie Imbullea's fault.",
                                                                    "Now I must discuss with Father",
                                                                    "Biscuss and decide how to tell",
                                                                    "the royal family about this..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Father Bamph",
                                                                args![
                                                                    "Oh, and would you please",
                                                                    "inform ^3131FFBonnie Imbullea^000000 that",
                                                                    "the deaths of the princes were",
                                                                    "not her fault? I'm sure that news would bring her great relief",
                                                                    "from her burden of guilt..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Father Bamph",
                                                                args![
                                                                    "Once again, let me remind",
                                                                    "you that everything that has",
                                                                    "transpired here must be kept",
                                                                    "secret. The public cannot know",
                                                                    "of Jormungand's curse! It would cause political turmoil and chaos!"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Father Bamph",
                                                                args![
                                                                    "Still, that does not mean",
                                                                    "that I cannot personally thank",
                                                                    "you for all of your help. I'm very grateful for what you have done,",
                                                                    "adventurer. May safety accompany you on all of your journeys."
                                                                ],
                                                            )?;
                                                            if ctx.var("prt_curse").get()? == 43 {
                                                                ctx.var("prt_curse").set(Val::from(44))?;
                                                            } else {
                                                                ctx.var("prt_curse").set(Val::from(54))?;
                                                            }
                                                            ctx.call(Function::GetExperience, vec![Val::from(1600000), Val::from(0)])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ((ctx.var("prt_curse").get()?.number()? > 43
                                                                && ctx.var("prt_curse").get()?.number()? < 50)
                                                                && !(ctx.var("aru_monas").get()?.is_true()))
                                                            {
                                                                ctx.lines_as(
                                                                    "Father Bamph",
                                                                    args![
                                                                        "Have you spoken to",
                                                                        "Bonnie Imbullea yet?",
                                                                        "Try not to worry about the",
                                                                        "news about the princes'",
                                                                        "murder. Father Biscuss",
                                                                        "and I will handle it."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if ctx.var("prt_curse").get()? == 50 {
                                                                    ctx.lines_as(
                                                                        "Father Bamph",
                                                                        args![
                                                                            "Ah, you've returned~",
                                                                            "Have you managed to learn",
                                                                            "anything about poison from",
                                                                            "the Assassin Guild? I know",
                                                                            "their members must be",
                                                                            "very difficult to find..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args![
                                                                        "You relate all of the",
                                                                        "information you learned",
                                                                        "about poison to Father Bamph,",
                                                                        "including the method to test",
                                                                        "for the use of poison in murder. "
                                                                    ])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Father Bamph", args!["Ah, I see. Then, would", "you please bring ^3131FF1 Yellow", "Gemstone^000000 and ^3131FF1 Green Potion^000000", "to the secret ceremonial grounds as soon as you can? I will wait", "over there for you once again."])?;
                                                                    ctx.var("prt_curse").set(Val::from(51))?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if ctx.var("prt_curse").get()? == 60 {
                                                                        ctx.lines_as(
                                                                            "Father Bamph",
                                                                            args![
                                                                                ((Val::from("Oh, ")
                                                                                    + ctx.call(
                                                                                        Function::StrCharInfo,
                                                                                        vec![Val::from(0)]
                                                                                    )?)
                                                                                    + Val::from("~")),
                                                                                "Long time, no see.",
                                                                                "Have you spoken to",
                                                                                "Bonnie yet? Ah, and",
                                                                                "how may I help you today?"
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines(args![
                                                                            "^3355FFYou tell Father Bamph",
                                                                            "that rare herbs grow in",
                                                                            "a land to the west, although",
                                                                            "you do not mention that you",
                                                                            "told Rodafrian about the curse.^000000"
                                                                        ])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Father Biscuss",
                                                                            args![
                                                                                "Land to the west?",
                                                                                "That place is rumored to",
                                                                                "be populated by fanatics.",
                                                                                "Father Bamph, I think they",
                                                                                "may be prime suspects for",
                                                                                "the murder of the princes..."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Father Bamph", args!["Well, I'll admit that it's", "possible, but we haven't had", "contact with anyone from the", "land to the west for so long...", "It's still far too early to make those kinds of assumptions."])?;
                                                                        ctx.next()?;
                                                                        let choice =
                                                                            runtime::select_values(ctx, &[Val::from("Land to the west?")])?;
                                                                        ctx.var("@menu").set(choice)?;
                                                                        ctx.lines_as(
                                                                            "Father Bamph",
                                                                            args![
                                                                                "Not much is known about",
                                                                                "the land to the west. We",
                                                                                "did send several priests",
                                                                                "there once to spread our",
                                                                                "faith. However, they failed..."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Father Bamph",
                                                                            args![
                                                                                "From what I can tell,",
                                                                                "almost everyone there",
                                                                                "is a strong adherent of",
                                                                                "that region's native religion.",
                                                                                "Anyway, thank you for informing",
                                                                                "us. We'll handle it from here."
                                                                            ],
                                                                        )?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Father Bamph",
                                                                            args![
                                                                                "Ah, to preserve the secrets",
                                                                                "of the royal family, I ask that",
                                                                                "you refrain from entering the",
                                                                                "secret ceremonial grounds",
                                                                                ((Val::from("from now on. Thanks again for your help, ")
                                                                                    + ctx.call(
                                                                                        Function::StrCharInfo,
                                                                                        vec![Val::from(0)]
                                                                                    )?)
                                                                                    + Val::from("."))
                                                                            ],
                                                                        )?;
                                                                        ctx.var("prt_curse").set(Val::from(61))?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if (((ctx.var("prt_curse").get()? == 36
                                                                            || ctx.var("prt_curse").get()? == 45)
                                                                            || ctx.var("prt_curse").get()? == 56)
                                                                            || ctx.var("prt_curse").get()? == 61)
                                                                        {
                                                                            if ctx.var("aru_monas").get()? == 0 {
                                                                                ctx.lines_as(
                                                                                    "Father Bamph",
                                                                                    args![
                                                                                        "I'm sorry, but would it",
                                                                                        "be alright if we talked",
                                                                                        "later? I have to handle",
                                                                                        "a very important task now..."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else {
                                                                                if ctx.var("aru_monas").get()? == 1 {
                                                                                    ctx.lines_as(
                                                                                        "Father Bamph",
                                                                                        args![
                                                                                            "Oh, I've been waiting for",
                                                                                            "you, adventurer. I'm sorry",
                                                                                            "for giving you such short",
                                                                                            "notice, but it seems that",
                                                                                            "you've grown much stronger",
                                                                                            "since the last time we met."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Father Bamph",
                                                                                        args![
                                                                                            "The Church has received",
                                                                                            "a request from Prontera",
                                                                                            "Palace, but we don't have",
                                                                                            "the resources to handle it.",
                                                                                            "I believe you'd be well suited",
                                                                                            "to this task if you'll help us."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Father Bamph",
                                                                                        args![
                                                                                            "A man of high rank suddenly",
                                                                                            "disappeared a while ago.",
                                                                                            "Although Prontera Palace",
                                                                                            "asked us to find him, I can",
                                                                                            "only tell you who he is later."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as(
                                                                                        "Father Bamph",
                                                                                        args![
                                                                                            "All we know is that he was",
                                                                                            "last seen in Comodo. Please",
                                                                                            "speak to our informant, ^6B8E23Larjes^000000,",
                                                                                            "and he will assist you in your",
                                                                                            "search for the lost official."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.var("aru_monas").set(Val::from(2))?;
                                                                                    ctx.close_window()?;
                                                                                    return Err(Stop::End);
                                                                                } else {
                                                                                    if (ctx.var("aru_monas").get()? == 2
                                                                                        || ctx.var("aru_monas").get()? == 3)
                                                                                    {
                                                                                        ctx.lines_as(
                                                                                            "Father Bamph",
                                                                                            args![
                                                                                                "Please go to Comodo and",
                                                                                                "find our informant, ^6B8E23Larjes^000000,",
                                                                                                "in the Casino. He will help",
                                                                                                "you in your search for the",
                                                                                                "official who disappeared."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("aru_monas").get()? == 4 {
                                                                                        ctx.lines_as(
                                                                                            "Father Bamph",
                                                                                            args![
                                                                                                "Ah, so you found Larjes?",
                                                                                                "How has your investigation of",
                                                                                                "that official's disappearance",
                                                                                                "progressing? I hope that he",
                                                                                                "is still safe and sound",
                                                                                                "when you find him."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        let choice = runtime::select_values(
                                                                                            ctx,
                                                                                            &[Val::from("Tell him what Larjes said.")],
                                                                                        )?;
                                                                                        ctx.var("@menu").set(choice)?;
                                                                                        ctx.lines_as(
                                                                                            "Father Bamph",
                                                                                            args![
                                                                                                "I see. We've detected some",
                                                                                                "disquieting activity from",
                                                                                                "Arunafeltz lately, but I didn't",
                                                                                                "think they would make",
                                                                                                "their move so soon. Hmm...",
                                                                                                "Give me a moment to think."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.var("aru_monas").set(Val::from(5))?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("aru_monas").get()? == 5 {
                                                                                        if ctx.call(
                                                                                            Function::Rand,
                                                                                            vec![Val::from(1), Val::from(5)],
                                                                                        )? == 4
                                                                                        {
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "Well, I've considered all",
                                                                                                    "possible scenarios. I think",
                                                                                                    "it would work best if you",
                                                                                                    "continue your investigation",
                                                                                                    "on our behalf. That is, if",
                                                                                                    "you're willing to do it."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "We don't want to provoke",
                                                                                                    "an international conflict",
                                                                                                    "with our official involvement,",
                                                                                                    "but I also can't force you to",
                                                                                                    "work for us. Whether you",
                                                                                                    "can help is your choice."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            if Val::from(runtime::select_values(
                                                                                                ctx,
                                                                                                &[Val::from(
                                                                                                    "Let me think about it.:Of course, I'll help.",
                                                                                                )],
                                                                                            )?) == 1
                                                                                            {
                                                                                                ctx.lines_as(
                                                                                                    "Father Bamph",
                                                                                                    args![
                                                                                                        "I hope that you decide",
                                                                                                        "to help us. The safety of",
                                                                                                        "our nation depends on the",
                                                                                                        "success of this investigation.",
                                                                                                        "We could really use your help."
                                                                                                    ],
                                                                                                )?;
                                                                                                ctx.close_window()?;
                                                                                                return Err(Stop::End);
                                                                                            }
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "Thank you. I'm glad to hear",
                                                                                                    "that you'll help us. You may",
                                                                                                    "take the airship in Izlude to",
                                                                                                    "travel to Arunafeltz, where you",
                                                                                                    "must continue your investigation."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as("Father Bamph", args!["I believe that you should", "be able to learn more if you", "investigate the city of ^9370DBRachel^000000.", "Please accept this money", "to cover your Airship fee.", "Thank you, and good luck."])?;
                                                                                            ctx.var("aru_monas").set(Val::from(6))?;
                                                                                            ctx.var("Zeny").set(
                                                                                                (ctx.var("Zeny").get()? + Val::from(1500)),
                                                                                            )?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "Hmm... What's the best way",
                                                                                                    "for us to handle this? Let",
                                                                                                    "me think about our options.",
                                                                                                    "We can--no. That wouldn't",
                                                                                                    "work. This will be difficult."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        }
                                                                                    } else {
                                                                                        if ctx.var("aru_monas").get()? == 6 {
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "You might have problems",
                                                                                                    "in Rachel since the culture",
                                                                                                    "there is much different than",
                                                                                                    "our own. Their ways of doing",
                                                                                                    "things, their government...",
                                                                                                    "Everything is tied to religion."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        } else if (ctx.var("aru_monas").get()?.number()?
                                                                                            > 6
                                                                                            && ctx.var("aru_monas").get()?.number()? < 25)
                                                                                        {
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "Thank you so much for all",
                                                                                                    "of your hard work. Our agents",
                                                                                                    "will contact you whenever they",
                                                                                                    "uncover any new information.",
                                                                                                    "Remember that no one can",
                                                                                                    "know what we've been doing."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        } else if (ctx.var("aru_monas").get()? == 25
                                                                                            || ctx.var("aru_monas").get()? == 26)
                                                                                        {
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "Thank you for bring us such",
                                                                                                    "important information. With",
                                                                                                    "your help, we were able to",
                                                                                                    "clear a few unsolved cases.",
                                                                                                    "We expected something like",
                                                                                                    "this, but not this soon."
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.next()?;
                                                                                            ctx.lines_as("Father Bamph", args!["I fear that the Royal Court", "has been bickering over their", "own selfish needs. I pray that", "it does not grow worse, devolve", "into chaos. ^666666*Sigh*^000000 We'll see..."])?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        } else {
                                                                                            ctx.lines_as(
                                                                                                "Father Bamph",
                                                                                                args![
                                                                                                    "It's upsetting to me that",
                                                                                                    "Arunafeltz has been so quiet",
                                                                                                    "lately. You know the expression",
                                                                                                    "about there being a quiet lull",
                                                                                                    "before a raging storm, right?"
                                                                                                ],
                                                                                            )?;
                                                                                            ctx.close_window()?;
                                                                                            return Err(Stop::End);
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn father_bamph(ctx: &Ctx) -> Script {
    father_bamph_body(ctx, Vec::new()).map(|_| ())
}

fn father_biscuss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 54 {
        ctx.lines_as(
            "Father Biscuss",
            args![
                "Hmm, I still suspect that",
                "someone from the Assassin",
                "Guild may have killed the",
                "princes. Just in case, I'm",
                "going to send a spy. Keep",
                "that information secret."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "So yes. You didn't learn",
                "or hear anything from us,",
                "and you don't know anything",
                "about the royal family's curse.",
                "From here on, Father Bamph",
                "and I will handle this issue."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("prt_curse").get()? == 35 {
        ctx.lines_as(
            "Father Biscuss",
            args![
                "I've never seen Father",
                "Bamph this way before, but",
                "I can understand how he feels.",
                "As one of the leaders of this",
                "church, he feels responsible",
                "for these princes' deaths."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "I'm sure he'll feel",
                "better in a few days,",
                "but right now, he's in no",
                "condition to compile the",
                "valuable info that you've",
                "provided, so I'll do it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "Personally, I feel that",
                "what happened was tragic,",
                "but it should be avenged.",
                "Perhaps that why I've held",
                "a grudge against Imbullea",
                "for all this time. Anyway..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "Although we can't",
                "acknowledge it publicly,",
                "on behalf of the Prontera",
                "Church, I want to thank",
                "you for all of your help."
            ],
        )?;
        ctx.var("prt_curse").set(Val::from(36))?;
        ctx.call(Function::GetExperience, vec![Val::from(1600000), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("prt_curse").get()? == 36 {
        ctx.lines_as(
            "Father Biscuss",
            args![
                "No one can know light",
                "without experiencing",
                "darkness. Peace has",
                "no meaning until it is",
                "contrasted with chaos."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "Religion becomes even",
                "more important during times",
                "of chaos, and times of need.",
                "I must remain calm, especially",
                "when Father Bamph feels so bad about this whole incident..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Father Biscuss",
            args!["Please observe", "silence within the", "Priest Room. Thank", "you for cooperating."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn father_biscuss(ctx: &Ctx) -> Script {
    father_biscuss_body(ctx, Vec::new()).map(|_| ())
}

fn gototomb_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((((((ctx.var("prt_curse").get()?.number()? > 17 && ctx.var("prt_curse").get()?.number()? < 23)
        || (ctx.var("prt_curse").get()?.number()? > 31 && ctx.var("prt_curse").get()?.number()? < 35))
        || ctx.var("prt_curse").get()? == 41)
        || ctx.var("prt_curse").get()? == 42)
        || ctx.var("prt_curse").get()? == 44)
        || ctx.var("prt_curse").get()? == 51)
        || ctx.var("prt_curse").get()? == 52)
    {
        ctx.call(Function::Warp, vec![Val::from("prt_church"), Val::from(21), Val::from(91)])?;
    }
    return Err(Stop::End);
}

pub fn gototomb(ctx: &Ctx) -> Script {
    gototomb_body(ctx, Vec::new()).map(|_| ())
}

fn father_biscuss_tomb_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Father Biscuss", args!["Hm...?", "Are you ready to", "head back upstairs?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
        ctx.lines_as("Father Biscuss", args!["Please follow me."])?;
        ctx.next()?;
        ctx.call(Function::Warp, vec![Val::from("prt_church"), Val::from(178), Val::from(111)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Father Biscuss",
        args!["Please take your time", "and investigate this as", "thoroughly as you can."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn father_biscuss_tomb(ctx: &Ctx) -> Script {
    father_biscuss_tomb_body(ctx, Vec::new()).map(|_| ())
}

fn father_bamph_tomb_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 18 {
        ctx.lines_as(
            "Father Bamph",
            args![
                "There are the bodies",
                "of the Geoborg princes",
                "that were killed during",
                "the exorcism. Please take",
                "a look at the body to the left."
            ],
        )?;
        ctx.var("prt_curse").set(Val::from(19))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("prt_curse").get()? == 19 {
            ctx.lines_as(
                "Father Bamph",
                args!["Please take a look", "at the body to the", "far left, the first prince."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("prt_curse").get()? == 20 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "That weird mark looked",
                        "just like snake scales.",
                        "Is... Is that the mark left",
                        "behind by Jormungand's curse?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Father Bamph",
                    args![
                        "That's right.",
                        "Now, let's examine the",
                        "body of the second prince,",
                        "located in the middle."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("prt_curse").get()? == 21 {
                    ctx.lines_as("Father Bamph", args!["Now...", "Now we should", "examine the third prince."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("prt_curse").get()? == 22 {
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "Let's go upstairs where we",
                            "can continue this conversation.",
                            "Ah, you might want to ask",
                            "Father Biscuss to lead you."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ((ctx.var("prt_curse").get()? == 32 || ctx.var("prt_curse").get()? == 41) || ctx.var("prt_curse").get()? == 51) {
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "Do you have a ^3131FFYellow",
                            "Gemstone^000000 and ^3131FFGreen Potion^000000",
                            "ready? If so, you should begin",
                            "testing on the body of the third prince before the others."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ((ctx.var("prt_curse").get()? == 33 || ctx.var("prt_curse").get()? == 42) || ctx.var("prt_curse").get()? == 52) {
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "The mark disappeared?",
                            "Oh, this is just horrible!",
                            "That would mean that poison",
                            "was used to murder the other",
                            "princes! I almost... can't..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "I... I need some time",
                            "to recover from the shock",
                            "and to think about all of",
                            "this carefully. For now,",
                            "let's go back upstairs."
                        ],
                    )?;
                    if ctx.var("prt_curse").get()? == 33 {
                        ctx.var("prt_curse").set(Val::from(34))?;
                    } else if ctx.var("prt_curse").get()? == 42 {
                        ctx.var("prt_curse").set(Val::from(43))?;
                    } else {
                        ctx.var("prt_curse").set(Val::from(53))?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("prt_curse").get()? == 43 || ctx.var("prt_curse").get()? == 53) {
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "We've disturbed the",
                            "bodies of these poor",
                            "souls enough. We should",
                            "go back upstairs now..."
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

pub fn father_bamph_tomb(ctx: &Ctx) -> Script {
    father_bamph_tomb_body(ctx, Vec::new()).map(|_| ())
}

fn prince1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 19 {
        ctx.lines_as(
            "Father Bamph",
            args![
                "This is the body of",
                "the king's first son, the",
                "crown prince. Just as it has",
                "happened for generations,",
                "the curse took the life of",
                "the first born prince..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFBeneath one of the",
            "prince's sleeves, you",
            "notice a dark mark. Upon",
            "pulling up the sleeve, you",
            "note that the mark resembles",
            "the scales of a serpent.^000000"
        ])?;
        ctx.var("prt_curse").set(Val::from(20))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("prt_curse").get()? == 33
        || (ctx.var("prt_curse").get()?.number()? > 41 && ctx.var("prt_curse").get()?.number()? < 51))
        || ctx.var("prt_curse").get()? == 52)
    {
        ctx.lines(args![
            "^3355FFYou poured a little bit",
            "of the solution made from",
            "Yellow Gemstone and Green",
            "Potion on the mark on the skin.",
            "You waited a while, but there was no reaction from the solution.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFIt's the body of a male",
            "dressed in luxurious robes.",
            "Although deceased, the color",
            "of life has not yet left the body. "
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn prince1(ctx: &Ctx) -> Script {
    prince1_body(ctx, Vec::new()).map(|_| ())
}

fn prince2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 20 {
        ctx.lines_as(
            "Father Bamph",
            args![
                "This is the body of the",
                "second prince. The curse",
                "is only supposed to kill",
                "the firstborn prince, but",
                "all three princes of this",
                "generation were killed..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Despite our great efforts",
                "to exorcise this powerful",
                "curse, we all failed. Those",
                "involved have begun to believe",
                "that Odin may have abandoned us... "
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou examine the body of",
            "the second prince and notice",
            "that the scale marks on his",
            "skin are fainter, and slightly",
            "different in color, than the",
            "marks on the first prince.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Father Bamph...!",
                "Look, these marks are",
                "different on the second",
                "prince than on the first",
                "prince! See? They're different",
                "in darkness and coloration."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Goodness, you're right!",
                "How did we overlook this?",
                "Hm, this supports the idea",
                "that a conspiracy may be",
                "involved. Let's go check",
                "the body of the third prince."
            ],
        )?;
        ctx.var("prt_curse").set(Val::from(21))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("prt_curse").get()? == 33
        || (ctx.var("prt_curse").get()?.number()? > 41 && ctx.var("prt_curse").get()?.number()? < 51))
        || ctx.var("prt_curse").get()? == 52)
    {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONATTACK")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
        ctx.lines(args![
            "^3355FFYou poured a little of the",
            "solution made from Green",
            "Potion and Yellow Gemstone",
            "on the body's scale marks. The",
            "scale marks grow fainter and",
            "the solution bubbles on contact. "
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFIt's the body of a male",
            "dressed in luxurious robes.",
            "Although deceased, the color",
            "of life has not yet left the body. "
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn prince2(ctx: &Ctx) -> Script {
    prince2_body(ctx, Vec::new()).map(|_| ())
}

fn prince3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 21 {
        ctx.lines(args![
            "^3355FFYou and Father Bamph",
            "hurriedly inspect the third",
            "prince's body and find that the",
            "scale marks on his skin are",
            "a little darker than the marks",
            "on the second prince's skin.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "This is very suspicious...",
                "The deaths of the second and",
                "third princes might have been",
                "caused by murder, rather than",
                "the curse. However, what could",
                "possibly be used to kill them?"
            ],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                if Val::from(runtime::select_values(ctx, &[Val::from("A weapon!:Poison!")])?) == 1 {
                    ctx.lines_as(
                        "Father Bamph",
                        args![
                            "Hmm... But none of the",
                            "bodies had any wounds",
                            "or scarring. If that were the",
                            "case, I'm sure the king would",
                            "have declared war on someone.",
                            "It couldn't have been a weapon."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    break 'l1;
                }
            }
        }
        ctx.lines_as(
            "Father Bamph",
            args![
                "Poison...?",
                "Oh dear! No one in the",
                "Prontera Church would know",
                "the first thing about that. But",
                "maybe poison was used.",
                "How can we find out for sure?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Biscuss",
            args![
                "I say, you go straight to",
                "the experts. Someone in the",
                "^FF0000Assassin Guild in Morocc^000000",
                "ought to know. I hear they",
                "can make poison that can kill",
                "a man with just one drop!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Bamph",
            args![
                "Ah, that's a good idea!",
                "Hmm, but first, let's go",
                "continue this conversation",
                "outside, shall we? I'd prefer",
                "not to disturb these bodies..."
            ],
        )?;
        ctx.var("prt_curse").set(Val::from(22))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("prt_curse").get()? == 22 {
        ctx.lines_as(
            "Father Bamph",
            args![
                "Let's go upstairs where we",
                "can continue this conversation.",
                "Ah, you might want to ask",
                "Father Biscuss to lead you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("prt_curse").get()? == 32 || ctx.var("prt_curse").get()? == 41) || ctx.var("prt_curse").get()? == 51) {
        if (ctx.call(Function::CountItem, vec![Val::from(506)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(715)])?.number()? > 0)
        {
            ctx.lines(args![
                "^3355FFYou open a bottle of",
                "Green Potion and insert a",
                "Yellow Gemstone. The gem",
                "quickly dissolves, conveniently",
                "forming a solution to test for the presense of poison. You pour",
                "it on the prince's scale marks.^000000"
            ])?;
            ctx.next()?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONATTACK")?])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BUBBLE")?])?;
            ctx.mes("^3355FF*Pssssssssh*^000000")?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe solution bubbles",
                "once it touches the skin,",
                "and the serpent scale marks",
                "on the prince's body slowly",
                "fade until they disappear.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(506), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(715), Val::from(1)])?;
            if ctx.var("prt_curse").get()? == 32 {
                ctx.var("prt_curse").set(Val::from(33))?;
            } else if ctx.var("prt_curse").get()? == 41 {
                ctx.var("prt_curse").set(Val::from(42))?;
            } else {
                ctx.var("prt_curse").set(Val::from(52))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou'll need to have",
            "a Green Potion and",
            "a Yellow Gemstone in",
            "order to test and confirm",
            "whether poison killed the",
            "second and third princes.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("prt_curse").get()? == 33 || ctx.var("prt_curse").get()? == 42) || ctx.var("prt_curse").get()? == 52) {
        ctx.lines(args![
            "^3355FFThe serpent scale marks",
            "on this prince's body have",
            "vanished after you applied",
            "the Green Potion and Yellow",
            "Gemstone solution to the skin.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFIt's the body of a male",
            "dressed in luxurious robes.",
            "Although deceased, the color",
            "of life has not yet left the body. "
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn prince3(ctx: &Ctx) -> Script {
    prince3_body(ctx, Vec::new()).map(|_| ())
}

fn assassin_guildsman_poiso_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("prt_curse").get()? == 23 {
        ctx.lines_as("Assassin Guildsman", args!["What business", "brings you here?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Poison:Nothing")])?) == 1 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I need to talk to",
                    "a poison specialist.",
                    "I'm investigating",
                    "something for the",
                    "Prontera Church,",
                    "possibly a murder."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Assassin Guildsman", args!["...", "......", "........."])?;
            ctx.next()?;
            ctx.lines_as(
                "Assassin Guildsman",
                args!["Listen carefully.", "I will only tell this", "to you one time."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Assassin Guildsman",
                args![
                    "There is an",
                    "Assassin's Private Pub",
                    "in the 7 o'clock direction",
                    "in Morocc. Go find someone",
                    "named ^3131FFMarjana^000000 inside."
                ],
            )?;
            ctx.var("prt_curse").set(Val::from(24))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Assassin Guildsman",
            args![
                "No. That look in",
                "your eyes. I'm sure",
                "there is a reason that",
                "you have come here..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Assassin Guildsman",
            args![
                "It's very dry and",
                "windy today. I like this",
                "weather. It feels like it",
                "perfectly matches the soul",
                "of a true Assassin, the loner",
                "that hides in the shadows."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn assassin_guildsman_poiso(ctx: &Ctx) -> Script {
    assassin_guildsman_poiso_body(ctx, Vec::new()).map(|_| ())
}

fn marjana_poison_run(ctx: &Ctx, mut step: MarjanaPoisonStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MarjanaPoisonStep::Start => {
                if ctx.var("prt_curse").get()? == 24 {
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "What business brings",
                            "you here? I'm giving",
                            "you 4 minutes to speak",
                            "with me, so be direct."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I want to learn",
                            "more about poison",
                            "and confirm if it was",
                            "used to kill someone."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "Poison? You've come",
                            "to just the right person.",
                            "If I don't know the answer,",
                            "I doubt you'll find anyone",
                            "else that would. Ask away."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Is it true that an",
                            "Assassin's poison",
                            "can be so powerful,",
                            "that just one drop",
                            "can kill a person?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "It's true that such powerful",
                            "poison exists, but such deadly",
                            "poison is usually only used by",
                            "Assassin Crosses. Generally,",
                            "normal Assassins use poisons",
                            "that are much less potent."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Alright. Can you make", "a poison that leaves a", "specific mark on the body?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "Sure. There's all kinds of",
                            "poisons that exist that I'm",
                            "sure you can't even imagine.",
                            "However, poisons that leave",
                            "specific marks are difficult to",
                            "use, and few can handle them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "The types of marks",
                            "that are left behind all",
                            "depend on the materials",
                            "used to create the poison.",
                            "Since some materials are",
                            "exclusive to certain areas..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "Well, the type of mark",
                            "that was left behind could",
                            "actually serve as some kind",
                            "of clue. What was the mark",
                            "on the victim's body?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "We're not sure if",
                            "poison was used yet,",
                            "but there are marks that",
                            "look like snake scales",
                            "left on the bodies."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "Hmm. You know, if I had",
                            "to guess, I would say that",
                            "poison was probably used.",
                            "But the poison would have",
                            "to originate from outside of",
                            "the Rune-Midgarts Kingdom..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "Yeah...",
                            "The materials used to",
                            "make those snake scale",
                            "marks... Some of them can't",
                            "even be found here on the",
                            "Midgard continent."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "We're running out of time.",
                            "Listen, you can confirm whether",
                            "poison was used to kill someone",
                            "by mixing a Yellow Gemstone with a Green Potion, and sprinkling",
                            "the solution on the body."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marjana",
                        args![
                            "If poison was used, the",
                            "solution with react with",
                            "the body. But this method",
                            "won't work if too much time",
                            "has passed after the murder.",
                            "You better try this soon..."
                        ],
                    )?;
                    ctx.var("prt_curse").set(Val::from(25))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MarjanaPoisonStep::OnInit;
                continue 'machine;
            }
            MarjanaPoisonStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Marjana#poison")])?;
                return Err(Stop::End);
            }
            MarjanaPoisonStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Marjana#poison")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn marjana_poison(ctx: &Ctx) -> Script {
    marjana_poison_run(ctx, MarjanaPoisonStep::Start, Vec::new()).map(|_| ())
}

pub fn marjana_poison_oninit(ctx: &Ctx) -> Script {
    marjana_poison_run(ctx, MarjanaPoisonStep::OnInit, Vec::new()).map(|_| ())
}

pub fn marjana_poison_onenable(ctx: &Ctx) -> Script {
    marjana_poison_run(ctx, MarjanaPoisonStep::OnEnable, Vec::new()).map(|_| ())
}
