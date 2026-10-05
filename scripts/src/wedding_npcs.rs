use script_sdk::{Context, Function, Request, Value, Variable, VariableScope};

const DIAMOND_RING: i32 = 2613;
const WEDDING_DRESS: i32 = 2338;
const TUXEDO: i32 = 7170;
const MARRIAGE_COVENANT: i32 = 6026;
const MINIMUM_LEVEL: i32 = 45;
const BRIDE_FEE: i32 = 1_200_000;
const GROOM_FEE: i32 = 1_300_000;
const DIVORCE_FEE: i32 = 2_500_000;
const CEREMONY_TIMEOUT_SECONDS: i32 = 180;
const BABY_CLASS: i32 = 2;

fn number(ctx: &Context, function: Function, arguments: Vec<Value>) -> Result<i32, String> {
    ctx.call(function, arguments)?.number_value()
}

fn item_count(ctx: &Context, item: i32) -> i32 {
    number(ctx, Function::CountItem, vec![item.into()]).unwrap_or(0)
}

fn is_male(ctx: &Context) -> Result<bool, String> {
    Ok(ctx.read("Sex")?.number_value()? == 1)
}

fn is_baby(ctx: &Context) -> bool {
    ctx.read("Upper").and_then(|value| value.number_value()).is_ok_and(|upper| upper == BABY_CLASS)
}

fn ceremony_variable(ctx: &Context, name: &str) -> Result<Value, String> {
    ctx.request(Request::VariableRead { scope: VariableScope::ServerTemporary, name: name.into(), index: 0 })
}

fn set_ceremony_variable(ctx: &Context, name: &str, value: Value) -> Result<(), String> {
    ctx.request(Request::VariablesWrite(vec![Variable { scope: VariableScope::ServerTemporary, name: name.into(), index: 0, value }]))
        .map(|_| ())
}

fn clock(ctx: &Context) -> Result<(i32, i32), String> {
    let date = number(ctx, Function::GetTime, vec![9.into()])?;
    let seconds = number(ctx, Function::GetTime, vec![1.into()])?
        + 60 * number(ctx, Function::GetTime, vec![2.into()])?
        + 3600 * number(ctx, Function::GetTime, vec![3.into()])?;
    Ok((date, seconds))
}

fn announce(ctx: &Context, text: String) -> Result<(), String> {
    ctx.call(Function::Announce, vec![text.into(), ctx.constant("bc_map")?])?;
    Ok(())
}

fn reset_ceremony(ctx: &Context) -> Result<(), String> {
    set_ceremony_variable(ctx, "@wedding", 0.into())?;
    set_ceremony_variable(ctx, "@wed_groom$", "".into())?;
    set_ceremony_variable(ctx, "@wed_bride$", "".into())
}

fn expire_stale_ceremony(ctx: &Context) -> Result<(), String> {
    if ceremony_variable(ctx, "@wedding")?.number_value()? == 0 {
        return Ok(());
    }
    let (date, seconds) = clock(ctx)?;
    let started_date = ceremony_variable(ctx, "@wed_date")?.number_value()?;
    let started = ceremony_variable(ctx, "@wed_second")?.number_value()?;
    if started_date != date || seconds - started > CEREMONY_TIMEOUT_SECONDS {
        announce(ctx, "You've responded too slowly... Next couple, please proceed.".into())?;
        reset_ceremony(ctx)?;
    }
    Ok(())
}

pub fn staff(ctx: &Context) -> Result<(), String> {
    ctx.mes("[Marry Happy]")?;
    if is_baby(ctx) {
        ctx.mes("Adopted characters aren't allowed to get married. For now, why don't you enjoy the simple pleasures of childhood?")?;
        return ctx.close();
    }
    ctx.mes("Marriage is the beautiful union of two souls that have chosen to be together forever. Is there a special someone like that in your life?")?;
    ctx.next()?;
    let menu = ["Ask about Wedding Ceremony", "Ask about Procedure", "Apply for Wedding", "Cancel"].map(String::from);
    match ctx.select(&menu)? {
        0 => {
            ctx.mes("[Marry Happy]")?;
            ctx.mes("Bishop Vomars, the bishop of love, officiates the marriage ceremony.\nWhen you marry someone, it's for the rest of your life. Keep in mind that a man can only marry a woman and vice versa.")?;
        }
        1 => {
            ctx.mes("[Marry Happy]")?;
            ctx.mes("First complete the application with me, then form a party of two.\nThe bridegroom speaks to Bishop Vomars first and tells him his bride's exact name. The bride then has 3 minutes to speak to the Bishop and confirm.\nOnce the rings are exchanged, you are bound in matrimony. Only one couple can be married at a time.")?;
        }
        2 => apply(ctx)?,
        _ => {}
    }
    ctx.close()
}

fn apply(ctx: &Context) -> Result<(), String> {
    ctx.mes("[Marry Happy]")?;
    let male = is_male(ctx)?;
    let fee = if male { GROOM_FEE } else { BRIDE_FEE };
    let outfit = if male { TUXEDO } else { WEDDING_DRESS };
    ctx.mes(format!(
        "The application needs a Diamond Ring, {} and {fee} zeny, or a Marriage Covenant in place of the zeny. You must also be at least level {MINIMUM_LEVEL}. Do you want to apply?",
        if male { "a Tuxedo" } else { "a Wedding Dress" }
    ))?;
    if ctx.select(&["Yes".to_string(), "No".into()])? != 0 {
        return Ok(());
    }
    ctx.mes("[Marry Happy]")?;
    if number(ctx, Function::GetPartnerId, vec![])? != 0 {
        ctx.mes("You are already married.")?;
        return Ok(());
    }
    if ctx.read("wedding_sign")?.number_value()? == 1 {
        ctx.mes("You have already applied for a wedding.")?;
        return Ok(());
    }
    let covenant = item_count(ctx, MARRIAGE_COVENANT) > 0;
    let zeny = ctx.read("Zeny")?.number_value()?;
    if ctx.read("BaseLevel")?.number_value()? < MINIMUM_LEVEL {
        ctx.mes(format!("You need to be at least level {MINIMUM_LEVEL} to get married."))?;
    } else if item_count(ctx, DIAMOND_RING) < 1 {
        ctx.mes("You need a Diamond Ring for the wedding.")?;
    } else if item_count(ctx, outfit) < 1 {
        ctx.mes(format!("You need {} for the wedding.", if male { "a Tuxedo" } else { "a Wedding Dress" }))?;
    } else if !covenant && zeny < fee {
        ctx.mes(format!("You need {fee} zeny, or a Marriage Covenant, for the wedding."))?;
    } else {
        ctx.mes("Please type your own name to confirm the application.")?;
        let name = ctx.call(Function::InputString, vec![])?.text();
        let own = ctx.call(Function::StrCharInfo, vec![0.into()])?.text();
        if name != own {
            return ctx.mes("That is not your name. The application was cancelled.");
        }
        if covenant {
            ctx.call(Function::DelItem, vec![MARRIAGE_COVENANT.into(), 1.into()])?;
        } else {
            ctx.write("Zeny", (zeny - fee).into())?;
        }
        ctx.call(Function::DelItem, vec![outfit.into(), 1.into()])?;
        ctx.call(Function::DelItem, vec![DIAMOND_RING.into(), 1.into()])?;
        ctx.write("wedding_sign", 1.into())?;
        ctx.mes("Your application is complete. Form a party of two and speak to Bishop Vomars.")?;
    }
    Ok(())
}

pub fn bishop(ctx: &Context) -> Result<(), String> {
    if is_baby(ctx) || number(ctx, Function::GetPartnerId, vec![])? != 0 {
        return ctx.close();
    }
    ctx.mes("[Bishop Vomars]")?;
    if ctx.read("wedding_sign")?.number_value()? != 1 {
        ctx.mes("Please apply for a wedding with the Wedding Staff first.")?;
        return ctx.close();
    }
    let party = number(ctx, Function::GetCharacterId, vec![1.into()])?;
    if party == 0 {
        ctx.mes("The couple must form a party of two before the ceremony.")?;
        return ctx.close();
    }
    expire_stale_ceremony(ctx)?;
    let own = ctx.call(Function::StrCharInfo, vec![0.into()])?.text();
    let male = is_male(ctx)?;
    let state = ceremony_variable(ctx, "@wedding")?.number_value()?;
    if state == 0 && male {
        set_ceremony_variable(ctx, "@wedding", 1.into())?;
        let (date, seconds) = clock(ctx)?;
        set_ceremony_variable(ctx, "@wed_date", date.into())?;
        set_ceremony_variable(ctx, "@wed_second", seconds.into())?;
        announce(ctx, format!("It's the marriage proposal from the groom, Mr. {own}..."))?;
        ctx.mes("Please tell me the exact name of your bride.")?;
        let bride = ctx.call(Function::InputString, vec![])?.text();
        if ctx.select(&["I do.".to_string()])? == 0 {
            set_ceremony_variable(ctx, "@wed_bride$", bride.clone().into())?;
            set_ceremony_variable(ctx, "@wed_groom$", own.clone().into())?;
            set_ceremony_variable(ctx, "@wed_party", party.into())?;
            announce(ctx, format!("The groom, Mr. {own}, has made his vows to Miss {bride}..."))?;
        }
    } else if state == 1 && !male && own == ceremony_variable(ctx, "@wed_bride$")?.text() {
        let groom = ceremony_variable(ctx, "@wed_groom$")?.text();
        if groom.is_empty() || ceremony_variable(ctx, "@wed_party")?.number_value()? != party {
            ctx.mes("Your groom must be in your party for the ceremony.")?;
            return ctx.close();
        }
        announce(ctx, format!("Let's hear what the bride, Miss {own}, has to say..."))?;
        ctx.mes(format!("Do you take {groom} as your husband?"))?;
        if ctx.select(&["Yes, I do.".to_string(), "No.".into()])? == 0 {
            match ctx.call(Function::Marriage, vec![groom.clone().into()]).and_then(|value| value.number_value()) {
                Ok(1) => announce(ctx, format!("I now pronounce you, {groom} and {own}, husband and wife."))?,
                _ => announce(ctx, "The marriage could not be completed.".into())?,
            }
        } else {
            announce(ctx, format!("Alas! {own} has rejected {groom}'s marriage proposal!"))?;
        }
        reset_ceremony(ctx)?;
    } else {
        ctx.mes("Another couple is being married. Please wait for your turn.")?;
    }
    ctx.close()
}

pub fn divorce(ctx: &Context) -> Result<(), String> {
    ctx.mes("[Deviruchi]")?;
    if number(ctx, Function::GetPartnerId, vec![])? == 0 {
        ctx.mes("You are not married. There is nothing for me to break.")?;
        return ctx.close();
    }
    ctx.mes(format!("A divorce costs {DIVORCE_FEE} zeny and will take all of your life energy. Do you really want to divorce?"))?;
    if ctx.select(&["No thanks, I am happy.".to_string(), "Yes, please do.".into()])? == 0 {
        ctx.call(Function::Warp, vec!["niflheim".into(), 169.into(), 162.into()])?;
        return Ok(());
    }
    let zeny = ctx.read("Zeny")?.number_value()?;
    if zeny < DIVORCE_FEE {
        ctx.mes("You can't afford it. Come back when you have the zeny.")?;
        return ctx.close();
    }
    ctx.write("Zeny", (zeny - DIVORCE_FEE).into())?;
    ctx.write("wedding_sign", 0.into())?;
    ctx.call(Function::PercentHeal, vec![(-100).into(), (-100).into()])?;
    ctx.call(Function::Divorce, vec![])?;
    ctx.close()
}
