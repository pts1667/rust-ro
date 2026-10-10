use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum PoringFortuneTellerStep {
    Start,
    LDisplaycutin,
}

fn poring_fortune_teller_run(ctx: &Ctx, mut step: PoringFortuneTellerStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_arg = Val::from(0);
    let mut l_card_2_buddy = Val::from(0);
    let mut l_card_2_fortune = Val::from(0);
    let mut l_card_2_future = Val::from(0);
    let mut l_card_2_love = Val::from(0);
    let mut l_card_2_study = Val::from(0);
    'machine: loop {
        match step {
            PoringFortuneTellerStep::Start => {
                ctx.lines_as(
                    "Chocarle",
                    args![
                        ((Val::from(" Welcome, welcome~!! ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("~!")),
                        " What brings you here today!? "
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(" I would like a Poring card reading. : What is a Poring card reading? ")],
                )?) == 2
                {
                    ctx.lines_as(
                        "Chocarle",
                        args![
                            " Poring card readings focus on cards with cute Porings..! ",
                            " Poke~poke~ they tell you your fortune! It is not traditional, but the liveliness is the best! "
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Chocarle", args![" The fortunes are simple and refreshing. It is perfect when you would like your fortune told with a light heart and not be burdensome! "])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Chocarle",
                    args![
                        " Ah~ is that so! ",
                        " Solving the wonders of the future with these cute and adorable Poring cards! ",
                        " It is 1000z for each fortune! Shall we begin? "
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(" Yes. : I would like to think about it once more. ")],
                )?) == 2
                {
                    ctx.lines_as(
                        "Chocarle",
                        args![" Ok then! Come again next time! We'll do a good job! Bye-bye~! "],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("Zeny").get()?.number()? < 1000 {
                    ctx.lines_as(
                        "Chocarle",
                        args!["You don't even have 1000 zeny!", "Nope, I can read cards for you!", "Nope!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                ctx.lines_as(
                    "Chocarle",
                    args![
                        " Thank you~! Then we shall look into your fortune! ",
                        " First, clear your mind! Just like washing when you wash dishes! "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Chocarle", args![" Then~ gently think of your wish! "])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        " (I'm curious about love!) : (I would like to do well in school!) : (Will my friendship be intact?) : (I want to know about my future self!) : (Will I make a lot of money?) ",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Chocarle", args![" Ohh! Cute love fortune! Heh! Lets look at the cards! "])?;
                        ctx.next()?;
                        l_card_2_love = ctx.call(Function::Rand, vec![Val::from(1), Val::from(33)])?;
                        poring_fortune_teller_run(ctx, PoringFortuneTellerStep::LDisplaycutin, vec![l_card_2_love.clone()])?;
                        ctx.mes("[Chocarle]")?;
                        let subject2 = l_card_2_love.clone();
                        if subject2 == 1 {
                            ctx.mes(" Oh! Your lover is about to leave your side! Hold on tight! ")?;
                        } else if subject2 == 2 {
                            ctx.mes(" Agh! You're about to give your soul to your loved one! Take it down a notch! ")?;
                        } else if subject2 == 3 {
                            ctx.mes(" No! Your heart is about to leave your love! Catch it! ")?;
                        } else if subject2 == 4 {
                            ctx.mes(" Love is like juice in the forest! It is refreshing! Take care of your loved one! ")?;
                        } else if subject2 == 5 {
                            ctx.mes(" Uh! You didn't share your juice with your loved one! What a shame! Take care of your loved one! ")?;
                        } else if subject2 == 6 {
                            ctx.mes(" Why not share with your loved one? Don't be too greedy and try to keep it all! ")?;
                        } else if subject2 == 7 {
                            ctx.mes(" Your love is going to have a crisis! Protect your love! ")?;
                        } else if subject2 == 8 {
                            ctx.mes(" Hee! Go after your love like a mole! Puhahaha! ")?;
                        } else if subject2 == 9 {
                            ctx.mes(" Oh no! Your love is in danger! Run away from the menace! ")?;
                        } else if subject2 == 10 {
                            ctx.mes(" Oh no! You lost your loved one. And, nobody similar around you. Cheer up! ")?;
                        } else if subject2 == 11 {
                            ctx.mes(" Ah! A sexy rival has appeared! Don't get sidetracked! ")?;
                        } else if subject2 == 12 {
                            ctx.mes(" You are desperately searching for love! If you go beyond this desert, you can find love! ")?;
                        } else if subject2 == 13 {
                            ctx.mes(" Prepare an event of love! They will be flying with happiness! ")?;
                        } else if subject2 == 14 {
                            ctx.mes(" Love is sweet! Be careful not to get cavities! ")?;
                        } else if subject2 == 15 {
                            ctx.mes(" Prepare a present for your love! They might get a heart attack because they will be overwhelmed with happiness! ")?;
                        } else if subject2 == 16 {
                            ctx.mes(" Don't be shy in front of your loved one! Love transcends everything! ")?;
                        } else if subject2 == 17 {
                            ctx.mes(" Embrace every aspect of your loved one! That is true love! ")?;
                        } else if subject2 == 18 {
                            ctx.mes(" Love will blossom with a totally different person! Wow, so cool! ")?;
                        } else if subject2 == 19 {
                            ctx.mes(" No matter what other people say, run towards your loved one! One-track love! ")?;
                        } else if subject2 == 20 {
                            ctx.mes(" Go to your loved one with a present! They will be very happy! ")?;
                        } else if subject2 == 21 {
                            ctx.mes(" Love requires health and strength! Exercise to become stronger! Power and love! ")?;
                        } else if subject2 == 22 {
                            ctx.mes(" Even if you wish to approach your loved one, your body is discomforted! Take a break and rest! ")?;
                        } else if subject2 == 23 {
                            ctx.mes(" Love is bending yourself for someone! Service! Sacrifice! ")?;
                        } else if subject2 == 24 {
                            ctx.mes(" You will be meeting your lover's parents! Prepare well! ")?;
                        } else if subject2 == 25 {
                            ctx.mes(
                                " Your relationship must be very happy as it seems you two are flying amongst the clouds! Very jealous! ",
                            )?;
                        } else if subject2 == 26 {
                            ctx.mes(" Make an opportunity to be alone with each other! It will bring much happiness! ")?;
                        } else if subject2 == 27 {
                            ctx.mes(" Uh oh! Someone is peeping at your love! Go scold them! ")?;
                        } else if subject2 == 28 {
                            ctx.mes(" Love blossoms from the foundation! Be true to the basics! ")?;
                        } else if subject2 == 29 {
                            ctx.mes(" When your loved one is hurt, be by them! They will be very happy! ")?;
                        } else if subject2 == 30 {
                            ctx.mes(" One day, when you wake up, your loved one will leave a present! Be happy! ")?;
                        } else if subject2 == 31 {
                            ctx.mes(" Give lots~~ of presents to your loved one! Then you will experience many~ many good things! ")?;
                        } else if subject2 == 32 {
                            ctx.mes(" Lean against a wall and await your love! *Boom* Love will appear! ")?;
                        } else if subject2 == 33 {
                            ctx.mes(" You must be lonely! It's ok, cheer up! ")?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Chocarle",
                            args![
                                " A fortune of love was told! What do you think! Do you like it? ",
                                " Even if you did not get the fortune you wished for, don't be do upset and simply go for your love! "
                            ],
                        )?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.next()?;
                        ctx.lines_as("Chocarle", args![" Then, see you next time~~~~~ "])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Chocarle",
                            args![" Issues about studying is a serious matter! Lets take a look! Yap! "],
                        )?;
                        ctx.next()?;
                        l_card_2_study = ctx.call(Function::Rand, vec![Val::from(1), Val::from(33)])?;
                        poring_fortune_teller_run(ctx, PoringFortuneTellerStep::LDisplaycutin, vec![l_card_2_study.clone()])?;
                        ctx.mes("[Chocarle]")?;
                        let subject3 = l_card_2_study.clone();
                        if subject3 == 1 {
                            ctx.mes(" Agh! Don't die from studying! Take breaks while you're at it! You need some rest! ")?;
                        } else if subject3 == 2 {
                            ctx.mes(" Oh, no! You're missing the main points! Figure out the essentials! ")?;
                        } else if subject3 == 3 {
                            ctx.mes(" Mmph! You're dying not to study! At times like this, a break is the best! ")?;
                        } else if subject3 == 4 {
                            ctx.mes(
                                " Studying is just like hunting a monster in the desert. Resting for a little is the best. Understand? ",
                            )?;
                        } else if subject3 == 5 {
                            ctx.mes(" A nice cup of cold juice under the hot sun! Studying should be done like that, too! ")?;
                        } else if subject3 == 6 {
                            ctx.mes(" Even though it is the desert, you seem to have a relaxed smile! Maybe you need to relax like this when studying as well! ")?;
                        } else if subject3 == 7 {
                            ctx.mes(" Your grades are in danger! You must study harder! ")?;
                        } else if subject3 == 8 {
                            ctx.mes(" There may be a big crisis! Don't get too stressed just because you can't study! ")?;
                        } else if subject3 == 9 {
                            ctx.mes(" It is saying studying in the dark night can be dangerous! Careful when studying at night! ")?;
                        } else if subject3 == 10 {
                            ctx.mes(" Studying came to a strange place! It needs to find its original path! You can do it! ")?;
                        } else if subject3 == 11 {
                            ctx.mes(" You're gazing at a smart friend with the eyes of a Poring! Heehee! Doing good! ")?;
                        } else if subject3 == 12 {
                            ctx.mes(" Eh? You're too smart! You're at a much higher level than your friends! Take a break! ")?;
                        } else if subject3 == 13 {
                            ctx.mes(" You will be rewarded for your accomplishments! It was worth the effort! ")?;
                        } else if subject3 == 14 {
                            ctx.mes(" Forget about studying for a moment and enjoy the party! It should be refreshing~! ")?;
                        } else if subject3 == 15 {
                            ctx.mes(" Study something that will make people happy! Future set! ")?;
                        } else if subject3 == 16 {
                            ctx.mes(
                                " If you're not sure of something, ask a friend! It's not something to be ashamed about! You can do it! ",
                            )?;
                        } else if subject3 == 17 {
                            ctx.mes(" Don't think that your head is empty! Because, you are smart! You can do it! ")?;
                        } else if subject3 == 18 {
                            ctx.mes(" There's a phenomenon you don't understand! Do a little more research! You'll be able to figure it out soon! ")?;
                        } else if subject3 == 19 {
                            ctx.mes(" Don't get lazy about studying wherever you go! Seek the road of truth and go down it to find the answer! ")?;
                        } else if subject3 == 20 {
                            ctx.mes(" You need strength to study! Get stronger! Running is a start! ")?;
                        } else if subject3 == 21 {
                            ctx.mes(" You will become an admirable person because of all the effort put into studying! You must feel very worth while! ")?;
                        } else if subject3 == 22 {
                            ctx.mes(" Try studying the history of our country! From when the tiger started smoking! ")?;
                        } else if subject3 == 23 {
                            ctx.mes(
                                " You must be having a hard time studying because of all the pressure! Go out and get some fresh air! ",
                            )?;
                        } else if subject3 == 24 {
                            ctx.mes(
                                " Your head seems to be heavy because of studying! You need a diversion! Put studying aside for a moment! ",
                            )?;
                        } else if subject3 == 25 {
                            ctx.mes(" Try studying aerospace or meteorology! Don't you think it would be fun? ")?;
                        } else if subject3 == 26 {
                            ctx.mes(" How about studying theology? You even get to study about angels! ")?;
                        } else if subject3 == 27 {
                            ctx.mes(
                                " Study little by little and make yourself feel lighter! You can't study if you're overloaded with words! ",
                            )?;
                        } else if subject3 == 28 {
                            ctx.mes(" When studying, you should have a snack! It may seem trivial, but it is rather important! ")?;
                        } else if subject3 == 29 {
                            ctx.mes(" Even when you are sick, don't forget about studying! Where there is effort, there is bound to be good results! ")?;
                        } else if subject3 == 30 {
                            ctx.mes(" Newton discovered gravity through a falling apple! Be wary of even the little things in your surroundings! ")?;
                        } else if subject3 == 31 {
                            ctx.mes(" Green is good for studying! Color your walls green or get a green drink! The lucky color! Green! ")?;
                        } else if subject3 == 32 {
                            ctx.mes(" There is a jewel in your mind! You need to bring all the wisdom to life! Don't study bad things! ")?;
                        } else if subject3 == 33 {
                            ctx.mes(" You will receive good results from studying! You can focus on your current studies! ")?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Chocarle",
                            args![
                                " You've been told your study fortune! Was it a good one? ",
                                " But in studying, effort is more important than any type of fortune! "
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.lines_as(
                            "Chocarle",
                            args![" Try your best. Then, see you again! Byyyyyyyyyyyyyyyyeeeee~~byyyyeeee~ "],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Chocarle", args![" Relationships among friends is more difficult than people think! Let's use the cards to try and solve this complicated puzzle of friendship! "])?;
                        ctx.next()?;
                        l_card_2_buddy = ctx.call(Function::Rand, vec![Val::from(1), Val::from(33)])?;
                        poring_fortune_teller_run(ctx, PoringFortuneTellerStep::LDisplaycutin, vec![l_card_2_buddy.clone()])?;
                        ctx.mes("[Chocarle]")?;
                        let subject4 = l_card_2_buddy.clone();
                        if subject4 == 1 {
                            ctx.mes(" Ah! Your friend has gone crazy! They need the heal of friendship! ")?;
                        } else if subject4 == 2 {
                            ctx.mes(" Mmm! You both have gone crazy. You must overcome it through conversations! ")?;
                        } else if subject4 == 3 {
                            ctx.mes(" Agh! You are suffering because of your friend! Try opening your heart and be more lenient! ")?;
                        } else if subject4 == 4 {
                            ctx.mes(" You even split a pea between friends! Don't be so cruel. Reflect upon yourself and apologize! ")?;
                        } else if subject4 == 5 {
                            ctx.mes(" Oh no! Your friend is ignoring you! Offer a bottle of juice to your friend! Your friendship may come back? ")?;
                        } else if subject4 == 6 {
                            ctx.mes(" Eh!? What kind of friendship is this?! Hurry up and make up! ")?;
                        } else if subject4 == 7 {
                            ctx.mes("Oh no?! This person is not your friend, but an enemy! You must be careful! ")?;
                        } else if subject4 == 8 {
                            ctx.mes(" Your friend is in danger! You must help your friend! ")?;
                        } else if subject4 == 9 {
                            ctx.mes(" Your friendship is on the verge of falling apart! Cast a shield around your friendship! ")?;
                        } else if subject4 == 10 {
                            ctx.mes(" One person is left out amongst your friends! Be more friendly! ")?;
                        } else if subject4 == 11 {
                            ctx.mes(" Your friend hit you! Don't cry even if it may hurt! Hurry and make up! ")?;
                        } else if subject4 == 12 {
                            ctx.mes(" You must be alone! It must be hard to make friends! Try putting on a brighter face! ")?;
                        } else if subject4 == 13 {
                            ctx.mes(" Give your friends a present! Your friendship will become deeper! ")?;
                        } else if subject4 == 14 {
                            ctx.mes(" Prepare a hat to cover your friend's large head! They will be moved by your care! ")?;
                        } else if subject4 == 15 {
                            ctx.mes(" A friend is waiting for you! Be nice to your friend! ")?;
                        } else if subject4 == 16 {
                            ctx.mes(" Being close friends despite differences is true friendship! Shelter each other's differences! ")?;
                        } else if subject4 == 17 {
                            ctx.mes(" Your misunderstandings will be removed if you take the time to talk! Go have a conversation with your friend! ")?;
                        } else if subject4 == 18 {
                            ctx.mes(" When a friend is sick, visit them! They will be happy! For sure! ")?;
                        } else if subject4 == 19 {
                            ctx.mes(" Go out and have some fun with your friend! Your friendship will surely get stronger! ")?;
                        } else if subject4 == 20 {
                            ctx.mes(" Give your friend a present! They'll go bragging around town?! ")?;
                        } else if subject4 == 21 {
                            ctx.mes(" Share your bone with your friend! Sacrifices in friendship are beautiful! ")?;
                        } else if subject4 == 22 {
                            ctx.mes(" Do anything for your friend! With all your heart and soul! Your friend will be delighted! ")?;
                        } else if subject4 == 23 {
                            ctx.mes(" No smoking, even with a friend! Stop smoking for your health! ")?;
                        } else if subject4 == 24 {
                            ctx.mes(" You and your friend will encounter hardships! Combine your powers and overcome it! ")?;
                        } else if subject4 == 25 {
                            ctx.mes(" Go on a trip with your friend! Friendship can get stronger in new environments! ")?;
                        } else if subject4 == 26 {
                            ctx.mes(" Give your friend a ride in a plane! They'll probably fly with joy!? ")?;
                        } else if subject4 == 27 {
                            ctx.mes(" Don't forget your friend in heaven! Friendship is eternal! ")?;
                        } else if subject4 == 28 {
                            ctx.mes(" Help your friend's scar. Your friend would greatly appreciate it! ")?;
                        } else if subject4 == 29 {
                            ctx.mes(" You even split a slice of an apple among friends! Share more things with your friend! Something good will happen! ")?;
                        } else if subject4 == 30 {
                            ctx.mes(" When your friend is sleeping, think of them! Then there will be progress in your friendship! What do you think~ it means~?! ")?;
                        } else if subject4 == 31 {
                            ctx.mes(" Don't measure friendship with money! If you do, be careful for there will be a crack in your friendship! ")?;
                        } else if subject4 == 32 {
                            ctx.mes(" It seems as if your friend will give you many presents! Look forward to it! ")?;
                        } else if subject4 == 33 {
                            ctx.mes(" You must be lonely without a friend! A good friend will come along soon! ")?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Chocarle",
                            args![
                                " Poring fortunetelling works very well when it is about friendship! ",
                                " It is the heart of the poring card that earnestly hopes for good friendships! "
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.lines_as(
                            "Chocarle",
                            args![" Did you like it? Next time invite your friend! Then byebye!! "],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            "Chocarle",
                            args![" If you say fortunetelling! Of course it is about the future! Let's try to figure this out! "],
                        )?;
                        ctx.next()?;
                        l_card_2_future = ctx.call(Function::Rand, vec![Val::from(1), Val::from(33)])?;
                        poring_fortune_teller_run(ctx, PoringFortuneTellerStep::LDisplaycutin, vec![l_card_2_future.clone()])?;
                        ctx.mes("[Chocarle]")?;
                        let subject5 = l_card_2_future.clone();
                        if subject5 == 1 {
                            ctx.mes(" Oh no! You are not confident about the future! Trust yourself a little more! Hope! ")?;
                        } else if subject5 == 2 {
                            ctx.mes(" Oh! Do you keep on thinking about your death in the future! First, forget about death and live your life! ")?;
                        } else if subject5 == 3 {
                            ctx.mes(" Cut! You are filled with uncertainty about the future! Let's get rid of this! Wee! It's gone! ")?;
                        } else if subject5 == 4 {
                            ctx.mes(" Life may seem like a hot and dry desert, but in the future, delicious juice and rest awaits you! ")?;
                        } else if subject5 == 5 {
                            ctx.mes(" Even in a hot and dry desert, isn't the future happier since you can encounter sweet juice? Put forth your strength! ")?;
                        } else if subject5 == 6 {
                            ctx.mes(
                                " Don't your troubles disappear watching a poring smile? Your future must be filled with good things! ",
                            )?;
                        } else if subject5 == 7 {
                            ctx.mes(" There is danger waiting ahead! You should avoid it for now! Take care of yourself! ")?;
                        } else if subject5 == 8 {
                            ctx.mes(" You must be fearing your future!? Don't worry too much! Think of happy thoughts! ")?;
                        } else if subject5 == 9 {
                            ctx.mes(" You may endanger someone in the future! Don't become a bad person~! ")?;
                        } else if subject5 == 10 {
                            ctx.mes(" You may have to stand alone in the future! Prepare yourself right now! ")?;
                        } else if subject5 == 11 {
                            ctx.mes(" Many hardships await you in the future! But you can overcome them! ")?;
                        } else if subject5 == 12 {
                            ctx.mes(" You might not even be able to buy summer clothes in the future! Don't waste your money! ")?;
                        } else if subject5 == 13 {
                            ctx.mes(" Something exciting may happen! What can it be? Fun! Fun! ")?;
                        } else if subject5 == 14 {
                            ctx.mes(" Something good might happen on this nice day! You can look forward to it! Yay! ")?;
                        } else if subject5 == 15 {
                            ctx.mes(" A splendid event will be held! Go get ready! Look forward to it! ")?;
                        } else if subject5 == 16 {
                            ctx.mes(" You will meet someone new! It will be very interesting! ")?;
                        } else if subject5 == 17 {
                            ctx.mes(" If you go little by little, something good will happen! Sit and take a look around you! ")?;
                        } else if subject5 == 18 {
                            ctx.mes(" You get a headache from thinking about the future? Empty your mind! You will feel refreshed! ")?;
                        } else if subject5 == 19 {
                            ctx.mes(" You will become a respectable person in the future! Good job! ")?;
                        } else if subject5 == 20 {
                            ctx.mes(" I can see you working hard in the future! What a lively future! ")?;
                        } else if subject5 == 21 {
                            ctx.mes(" You have a very busy future! Take care of your health! ")?;
                        } else if subject5 == 22 {
                            ctx.mes(" Pick a job where you can work with other people! It will be very rewarding, right? ")?;
                        } else if subject5 == 23 {
                            ctx.mes(" Try doing some volunteer work! It is worthwhile and you will feel good about it, too! ")?;
                        } else if subject5 == 24 {
                            ctx.mes(" You may become a commander! Mmm~! Kind of scary! ")?;
                        } else if subject5 == 25 {
                            ctx.mes(" Choose a job that involves flying! You show potential! ")?;
                        } else if subject5 == 26 {
                            ctx.mes(" Scrumptious ice cream that could even be eaten in heaven! Challenge yourself and get involved in a job making things of that sort! ")?;
                        } else if subject5 == 27 {
                            ctx.mes(
                                " In the future, things that cannot be done right now will be accomplished! You can look forward to it! ",
                            )?;
                        } else if subject5 == 28 {
                            ctx.mes(" Little things in life will bring you happiness and joy in the future! Even more than now! ")?;
                        } else if subject5 == 29 {
                            ctx.mes(" Do what you have to do. Live life to the fullest, even though the world may end tomorrow! ")?;
                        } else if subject5 == 30 {
                            ctx.mes(" You will deal with fruits in the future! How about preparing yourself? ")?;
                        } else if subject5 == 31 {
                            ctx.mes(
                                " Your future self will encounter a high wall! Although, I'm not sure what type of building it may be! ",
                            )?;
                        } else if subject5 == 32 {
                            ctx.mes(
                                " Green symbolizes peace! Your future seems as if it will be very peaceful! It's a good thing, right? ",
                            )?;
                        } else if subject5 == 33 {
                            ctx.mes(" Romance lays ahead in your future! Relax yourself and prepare yourself! ")?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Chocarle",
                            args![
                                " What do you think? Has your anxiety been relieved after hearing your fortune? ",
                                " This is only a small part of your future! You must discover the rest! "
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.lines_as(
                            "Chocarle",
                            args![" Am I cheesy? I'm just like that! Then, bye~~~~ye~~~ye~~~~!! "],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    5 => {
                        ctx.lines_as(
                            "Chocarle",
                            args![
                                " Mmm! The long awaited time has come! Asking a question about money to the cute Poring! ",
                                " Ok! Let's try it! "
                            ],
                        )?;
                        ctx.next()?;
                        l_card_2_fortune = ctx.call(Function::Rand, vec![Val::from(1), Val::from(33)])?;
                        poring_fortune_teller_run(ctx, PoringFortuneTellerStep::LDisplaycutin, vec![l_card_2_fortune.clone()])?;
                        ctx.mes("[Chocarle]")?;
                        let subject6 = l_card_2_fortune.clone();
                        if subject6 == 1 {
                            ctx.mes(" Uh oh! You're about to be robbed! You must save a little first! ")?;
                        } else if subject6 == 2 {
                            ctx.mes(" Kek! Your mind goes blank when you think about money! Think of other thoughts! Forget about money for a while! ")?;
                        } else if subject6 == 3 {
                            ctx.mes(" Bah! This isn't a time to think about money. Calm down and put your mind at ease! ")?;
                        } else if subject6 == 4 {
                            ctx.mes(" If you look carefully, it is not drinking juice, but underground water through a straw in the ground! Money is hidden where nobody expects! Good luck searching! ")?;
                        } else if subject6 == 5 {
                            ctx.mes(
                                " As you can quench your thirst in the dry desert, you can gather money even in this difficult world! ",
                            )?;
                        } else if subject6 == 6 {
                            ctx.mes(" Just like the juice inside the needles of a cactus, you can still gather wealth though you may be in anguish. You can do it! ")?;
                        } else if subject6 == 7 {
                            ctx.mes("Agh! Someone is after your possessions! Be careful! ")?;
                        } else if subject6 == 8 {
                            ctx.mes(" Uh oh! Someone is after your money! Take good care of it! ")?;
                        } else if subject6 == 9 {
                            ctx.mes(
                                " It says you might be tempted to do something bad to gather money! Don't forget about a kind heart! ",
                            )?;
                        } else if subject6 == 10 {
                            ctx.mes(" You have hidden money in your clothes! Hidden rich one! Be careful not to be caught! ")?;
                        } else if subject6 == 11 {
                            ctx.mes(" In the future, it seems like you will be kicking money around with your feet like those Porings! Congratulations! ")?;
                        } else if subject6 == 12 {
                            ctx.mes(" You must be exhausted! Making money is not the easiest thing to do! But it will be that much more valuable! ")?;
                        } else if subject6 == 13 {
                            ctx.mes(" Your tendency to spend money is growing! Be careful! Don't be left empty-handed! ")?;
                        } else if subject6 == 14 {
                            ctx.mes(" Use your money to buy something fun! Then good luck will be headed your way! ")?;
                        } else if subject6 == 15 {
                            ctx.mes(" Use your money on something exciting! It looks as if good luck will be headed your way! ")?;
                        } else if subject6 == 16 {
                            ctx.mes(" It seems like you will have two lucky offerings! Don't miss these two opportunities! ")?;
                        } else if subject6 == 17 {
                            ctx.mes(" Doesn't it seem like there should be money in the small wallet? You will have some small income! Save money wisely! ")?;
                        } else if subject6 == 18 {
                            ctx.mes(" So~ empty. Upsetting, but don't worry too much about money! Something better ought to happen! ")?;
                        } else if subject6 == 19 {
                            ctx.mes(" Someone will return something you lost! What a relief! ")?;
                        } else if subject6 == 20 {
                            ctx.mes(" No need to worry about osteoporosis! Wasn't it a good thing to eat so much calcium? Oops! This isn't about money?! ")?;
                        } else if subject6 == 21 {
                            ctx.mes(" Seeing that you work so hard, seems like you will make lots of money! Congratz~! ")?;
                        } else if subject6 == 22 {
                            ctx.mes(" Even if you save money, it disappears like smoke. Don't get too caught up with it! ")?;
                        } else if subject6 == 23 {
                            ctx.mes(" It's hard to make money, isn't it? But don't forget the good deeds in life! ")?;
                        } else if subject6 == 24 {
                            ctx.mes(" Going around to collect money might lead you to a scary person! Be very careful! ")?;
                        } else if subject6 == 25 {
                            ctx.mes(" Forget about money and fly~fly! You will feel very refreshed! ")?;
                        } else if subject6 == 26 {
                            ctx.mes(" Money is like clouds! Instead of money, think about a happy life! ")?;
                        } else if subject6 == 27 {
                            ctx.mes(
                                " Even if it may seem pointless, if you persist, you will be able to make lots of money! You can do it! ",
                            )?;
                        } else if subject6 == 28 {
                            ctx.mes(" Don't neglect the trivial things on the floor! If you keep an open heart like that, you will be able to collect money! ")?;
                        } else if subject6 == 29 {
                            ctx.mes(" If you work with something that is related to mushrooms or apples, a good fortune awaits you! What kind of work would that be? ")?;
                        } else if subject6 == 30 {
                            ctx.mes(" How can you think of money looking at such a peaceful card! That's not nice! Sniff..sniffles! Anyhow, it seems like you will gather much fortune! ")?;
                        } else if subject6 == 31 {
                            ctx.mes(" You have many fine stones! You will prosper if you are involved in this industry! ")?;
                        } else if subject6 == 32 {
                            ctx.mes(" It is an indication that you will prosper! No need to worry now! ")?;
                        } else if subject6 == 33 {
                            ctx.mes(" You will collect many treasures! But the problem is protecting all of it! ")?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Chocarle", args![" There is something you need to keep in mind after being told a fortune about wealth! ", " Money does not automatically come to one without effort! No matter how good the fortune, you must work diligently! "])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        ctx.lines_as("Chocarle", args![" Bear that in mind! Then, until we meet again.. *poof*! "])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = PoringFortuneTellerStep::LDisplaycutin;
                continue 'machine;
            }
            PoringFortuneTellerStep::LDisplaycutin => {
                l_arg = runtime::arg(&args, 0, Val::from(0));
                if l_arg.clone().number()? < 4 {
                    ctx.call(Function::Cutin, vec![Val::from("����Ʈ��ī��"), Val::from(4)])?;
                } else {
                    if l_arg.clone().number()? < 7 {
                        ctx.call(Function::Cutin, vec![Val::from("�������ī��"), Val::from(4)])?;
                    } else {
                        if l_arg.clone().number()? < 10 {
                            ctx.call(Function::Cutin, vec![Val::from("��Ƽ��ī��"), Val::from(4)])?;
                        } else {
                            if l_arg.clone().number()? < 13 {
                                ctx.call(Function::Cutin, vec![Val::from("����ī��"), Val::from(4)])?;
                            } else {
                                if l_arg.clone().number()? < 16 {
                                    ctx.call(Function::Cutin, vec![Val::from("��Ÿ����ī��"), Val::from(4)])?;
                                } else {
                                    if l_arg.clone().number()? < 19 {
                                        ctx.call(Function::Cutin, vec![Val::from("����������ī��"), Val::from(4)])?;
                                    } else if l_arg.clone().number()? < 22 {
                                        ctx.call(Function::Cutin, vec![Val::from("���̷���ī��"), Val::from(4)])?;
                                    } else if l_arg.clone().number()? < 25 {
                                        ctx.call(Function::Cutin, vec![Val::from("���尡ī��"), Val::from(4)])?;
                                    } else if l_arg.clone().number()? < 28 {
                                        ctx.call(Function::Cutin, vec![Val::from("������ī��"), Val::from(4)])?;
                                    } else if l_arg.clone().number()? < 31 {
                                        ctx.call(Function::Cutin, vec![Val::from("����ī��"), Val::from(4)])?;
                                    } else {
                                        ctx.call(Function::Cutin, vec![Val::from("������ī��"), Val::from(4)])?;
                                    }
                                }
                            }
                        }
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn poring_fortune_teller(ctx: &Ctx) -> Script {
    poring_fortune_teller_run(ctx, PoringFortuneTellerStep::Start, Vec::new()).map(|_| ())
}

fn ascetic_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Kissy-Kissy", args!["NyangNyangNyang~", "NyaNyangNyaNyag~"])?;
    ctx.next()?;
    ctx.lines_as(
        "Kissy-Kissy",
        args![
            "I decided to become a fortune teller",
            "when I was young and came to the city",
            "and devoted myself to training.",
            "I was in agony because of my slow progress."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kissy-Kissy",
        args![
            "I came because I heard many ",
            "fortunetellers gather here in Payon,",
            "but nobody will tell me anything~",
            "Waaah~ so sad~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Kissy-Kissy", args!["NyangNyangNyang~", "NyaNyangNyaNyang~"])?;
    ctx.next()?;
    ctx.lines_as("Kissy-Kissy", args!["Kiss me~~"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ascetic(ctx: &Ctx) -> Script {
    ascetic_body(ctx, Vec::new()).map(|_| ())
}

fn ascetic_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Kissy-Kissy", args!["NyangNyangNyang~", "NyaNyangNyaNyang~", "Kissy me~"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ascetic_ontouch(ctx: &Ctx) -> Script {
    ascetic_ontouch_body(ctx, Vec::new()).map(|_| ())
}
