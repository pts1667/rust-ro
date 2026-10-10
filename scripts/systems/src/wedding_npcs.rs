use script_sdk_2::constants::{BC_MAP, DT_HOUR, DT_MINUTE, DT_SECOND, DT_YYYYMMDD};
use script_sdk_2::{Ctx, Script, Stop};

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

/// The ceremony in progress, shared by every player: `1` once a groom has proposed.
const WEDDING: &str = "$@wedding";
const WEDDING_DATE: &str = "$@wed_date";
const WEDDING_SECOND: &str = "$@wed_second";
const WEDDING_GROOM: &str = "$@wed_groom$";
const WEDDING_BRIDE: &str = "$@wed_bride$";
const WEDDING_PARTY: &str = "$@wed_party";

fn is_male(ctx: &Ctx) -> Result<bool, Stop> {
    Ok(ctx.var("Sex").get()?.number()? == 1)
}

fn is_baby(ctx: &Ctx) -> Result<bool, Stop> {
    Ok(ctx.var("Upper").get()?.number()? == BABY_CLASS)
}

/// Today's date and the seconds since midnight.
fn clock(ctx: &Ctx) -> Result<(i32, i32), Stop> {
    let date = ctx.time_field(DT_YYYYMMDD)?;
    let seconds = ctx.time_field(DT_SECOND)? + 60 * ctx.time_field(DT_MINUTE)? + 3600 * ctx.time_field(DT_HOUR)?;
    Ok((date, seconds))
}

fn announce(ctx: &Ctx, text: &str) -> Script {
    ctx.announce(text, BC_MAP)
}

fn reset_ceremony(ctx: &Ctx) -> Script {
    ctx.var(WEDDING).set(0)?;
    ctx.var(WEDDING_GROOM).set("")?;
    ctx.var(WEDDING_BRIDE).set("")
}

fn expire_stale_ceremony(ctx: &Ctx) -> Script {
    if ctx.var(WEDDING).get()?.number()? == 0 {
        return Ok(());
    }
    let (date, seconds) = clock(ctx)?;
    let started_date = ctx.var(WEDDING_DATE).get()?.number()?;
    let started = ctx.var(WEDDING_SECOND).get()?.number()?;
    if started_date != date || seconds - started > CEREMONY_TIMEOUT_SECONDS {
        announce(ctx, "You've responded too slowly... Next couple, please proceed.")?;
        reset_ceremony(ctx)?;
    }
    Ok(())
}

pub fn staff(ctx: &Ctx) -> Script {
    ctx.mes("[Marry Happy]")?;
    if is_baby(ctx)? {
        ctx.mes("Adopted characters aren't allowed to get married. For now, why don't you enjoy the simple pleasures of childhood?")?;
        return ctx.close();
    }
    ctx.mes("Marriage is the beautiful union of two souls that have chosen to be together forever. Is there a special someone like that in your life?")?;
    ctx.next()?;
    match ctx.menu(&["Ask about Wedding Ceremony", "Ask about Procedure", "Apply for Wedding", "Cancel"])? {
        0 => ctx.mes_as("Marry Happy", "Bishop Vomars, the bishop of love, officiates the marriage ceremony.\nWhen you marry someone, it's for the rest of your life. Keep in mind that a man can only marry a woman and vice versa.")?,
        1 => ctx.mes_as("Marry Happy", "First complete the application with me, then form a party of two.\nThe bridegroom speaks to Bishop Vomars first and tells him his bride's exact name. The bride then has 3 minutes to speak to the Bishop and confirm.\nOnce the rings are exchanged, you are bound in matrimony. Only one couple can be married at a time.")?,
        2 => apply(ctx)?,
        _ => {}
    }
    ctx.close()
}

fn apply(ctx: &Ctx) -> Script {
    ctx.mes("[Marry Happy]")?;
    let male = is_male(ctx)?;
    let fee = if male { GROOM_FEE } else { BRIDE_FEE };
    let (outfit, outfit_name) = if male { (TUXEDO, "a Tuxedo") } else { (WEDDING_DRESS, "a Wedding Dress") };
    ctx.mes(&format!("The application needs a Diamond Ring, {outfit_name} and {fee} zeny, or a Marriage Covenant in place of the zeny. You must also be at least level {MINIMUM_LEVEL}. Do you want to apply?"))?;
    if ctx.menu(&["Yes", "No"])? != 0 {
        return Ok(());
    }
    ctx.mes("[Marry Happy]")?;
    let player = ctx.player();
    if player.partner_id(None)? != 0 {
        return ctx.mes("You are already married.");
    }
    if ctx.var("wedding_sign").get()?.number()? == 1 {
        return ctx.mes("You have already applied for a wedding.");
    }
    let items = ctx.items();
    let covenant = items.count(MARRIAGE_COVENANT)? > 0;
    let zeny = player.zeny()?;
    if player.base_level()? < MINIMUM_LEVEL {
        ctx.mes(&format!("You need to be at least level {MINIMUM_LEVEL} to get married."))?;
    } else if items.count(DIAMOND_RING)? < 1 {
        ctx.mes("You need a Diamond Ring for the wedding.")?;
    } else if items.count(outfit)? < 1 {
        ctx.mes(&format!("You need {outfit_name} for the wedding."))?;
    } else if !covenant && zeny < fee {
        ctx.mes(&format!("You need {fee} zeny, or a Marriage Covenant, for the wedding."))?;
    } else {
        ctx.mes("Please type your own name to confirm the application.")?;
        let name = ctx.input_text(0, usize::MAX)?.value;
        if name != player.name()? {
            return ctx.mes("That is not your name. The application was cancelled.");
        }
        if covenant {
            items.take(MARRIAGE_COVENANT, 1)?;
        } else {
            player.set_zeny(zeny - fee)?;
        }
        items.take(outfit, 1)?;
        items.take(DIAMOND_RING, 1)?;
        ctx.var("wedding_sign").set(1)?;
        ctx.mes("Your application is complete. Form a party of two and speak to Bishop Vomars.")?;
    }
    Ok(())
}

/// Weds the couple in two steps: the groom proposes and names his bride, then she accepts within the timeout.
pub fn bishop(ctx: &Ctx) -> Script {
    let player = ctx.player();
    if is_baby(ctx)? || player.partner_id(None)? != 0 {
        return ctx.close();
    }
    ctx.mes("[Bishop Vomars]")?;
    if ctx.var("wedding_sign").get()?.number()? != 1 {
        ctx.mes("Please apply for a wedding with the Wedding Staff first.")?;
        return ctx.close();
    }
    let party = player.party_id()?;
    if party == 0 {
        ctx.mes("The couple must form a party of two before the ceremony.")?;
        return ctx.close();
    }
    expire_stale_ceremony(ctx)?;
    let own = player.name()?;
    let male = is_male(ctx)?;
    let state = ctx.var(WEDDING).get()?.number()?;
    if state == 0 && male {
        ctx.var(WEDDING).set(1)?;
        let (date, seconds) = clock(ctx)?;
        ctx.var(WEDDING_DATE).set(date)?;
        ctx.var(WEDDING_SECOND).set(seconds)?;
        announce(ctx, &format!("It's the marriage proposal from the groom, Mr. {own}..."))?;
        ctx.mes("Please tell me the exact name of your bride.")?;
        let bride = ctx.input_text(0, usize::MAX)?.value;
        if ctx.menu(&["I do."])? == 0 {
            ctx.var(WEDDING_BRIDE).set(bride.as_str())?;
            ctx.var(WEDDING_GROOM).set(own.as_str())?;
            ctx.var(WEDDING_PARTY).set(party)?;
            announce(ctx, &format!("The groom, Mr. {own}, has made his vows to Miss {bride}..."))?;
        }
    } else if state == 1 && !male && own == ctx.var(WEDDING_BRIDE).get()?.text() {
        let groom = ctx.var(WEDDING_GROOM).get()?.text();
        if groom.is_empty() || ctx.var(WEDDING_PARTY).get()?.number()? != party {
            ctx.mes("Your groom must be in your party for the ceremony.")?;
            return ctx.close();
        }
        announce(ctx, &format!("Let's hear what the bride, Miss {own}, has to say..."))?;
        ctx.mes(&format!("Do you take {groom} as your husband?"))?;
        if ctx.menu(&["Yes, I do.", "No."])? == 0 {
            match player.marry(&groom) {
                Ok(true) => announce(ctx, &format!("I now pronounce you, {groom} and {own}, husband and wife."))?,
                _ => announce(ctx, "The marriage could not be completed.")?,
            }
        } else {
            announce(ctx, &format!("Alas! {own} has rejected {groom}'s marriage proposal!"))?;
        }
        reset_ceremony(ctx)?;
    } else {
        ctx.mes("Another couple is being married. Please wait for your turn.")?;
    }
    ctx.close()
}

pub fn divorce(ctx: &Ctx) -> Script {
    let player = ctx.player();
    ctx.mes("[Deviruchi]")?;
    if player.partner_id(None)? == 0 {
        ctx.mes("You are not married. There is nothing for me to break.")?;
        return ctx.close();
    }
    ctx.mes(&format!("A divorce costs {DIVORCE_FEE} zeny and will take all of your life energy. Do you really want to divorce?"))?;
    if ctx.menu(&["No thanks, I am happy.", "Yes, please do."])? == 0 {
        return ctx.warp("niflheim", 169, 162);
    }
    let zeny = player.zeny()?;
    if zeny < DIVORCE_FEE {
        ctx.mes("You can't afford it. Come back when you have the zeny.")?;
        return ctx.close();
    }
    player.set_zeny(zeny - DIVORCE_FEE)?;
    ctx.var("wedding_sign").set(0)?;
    player.percent_heal(-100, -100)?;
    player.divorce()?;
    ctx.close()
}
