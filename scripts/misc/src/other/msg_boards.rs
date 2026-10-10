#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args};

pub fn sign_post_prt1(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Post Reads -^000000",
        "North to Prontera Castle",
        "North to Al De Baran",
        "Northwest to Geffen",
        "East to Prontera Fields",
        "South to Prontera Fields",
        "Southeast to Alberta",
        "Southwest to Morocc",
        "Southwest to Comodo",
        "West to Prontera Fields"
    ])?;
    ctx.close()
}

pub fn prontera_bulletin_prt2(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Prontera Bulletin Reads -^000000",
        "Wanted: Two Clip Accessories",
        "Please contact...",
        "- Name appears to be worn off -",
        "Selling: Used Bastard Sword",
        "Will take any offer!",
        "Contact Abramulious",
        "Help Wanted: Buying or selling a used Peco Peco?",
        "Contact Grasisium in Morocc now!"
    ])?;
    ctx.close()
}

pub fn sign_prt3(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "Please help keep Prontera a clean place."
    ])?;
    ctx.close()
}

pub fn billboard_prt5(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "~WANTED~",
        "iROGM01",
        "DEAD or ALIVE",
        "*Kill Stealing in Glast Heim*",
        "~REWARD~",
        "50,000 Zeny ",
        "Contact: iROGM02"
    ])?;
    ctx.close()
}

pub fn billboard_prt6(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "We hope you enjoy your stay in Prontera."
    ])?;
    ctx.close()
}

pub fn sign_prt7(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "Note:",
        "I lost my cart in Mt. Mjolnir, if someone finds it please tell me, my life was in that bucket of goods!"
    ])?;
    ctx.close()
}

pub fn sign_moc1(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "Wanted: Body guard to protect my shop from thieves",
        "Please contact Butcher"
    ])?;
    ctx.close()
}

pub fn sign_moc2(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "Selling, well groomed Peco Peco!",
        "This beautiful specimen has only been ridden by myself, comes with a saddle, a harness and..."
    ])?;
    ctx.close()
}

pub fn bulletin_moc3(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Bulletin Reads -^000000",
        "^0099FFMorocc women up in arms!^000000",
        "A recent study has shown that the majority of male citizens in",
        "Morocc prefer the women of Geffen. 90 of the 100 male citizens",
        "of Morocc claimed that they have had numerous relationships",
        "with Geffen women outside of the Morocc Region."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^993300- The Bulletin Continued -^000000",
        "'I just prefer their company better, that's all...' said one Morocc man.",
        "'it's not like I'm against Morocc women or anything, so what's the problem...'",
        "Besides emotional and stressful issues in regards to the daily",
        "activities of these males.",
        "Hunting still seems to be their number one priority",
        "over finding decent woman within the region...."
    ])?;
    ctx.close()
}

pub fn sign_moc5(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "Welcome to Morocc."])?;
    ctx.close()
}

pub fn billboard_moc6(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "^CC0033Battle Royal!^000000",
        "Do you have what it takes to battle someone in a no holds barred, player vs. player game of death!",
        "Head to Prontera if you think you have what it takes!"
    ])?;
    ctx.close()
}

pub fn sign_moc7(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "Welcome to Morocc."])?;
    ctx.close()
}

pub fn geffen_bulletin_gef1(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Geffen Bulletin Reads -^000000",
        "Remember Wizard's...It's not how many skills you know, it's the magic that counts!"
    ])?;
    ctx.close()
}

pub fn billboard_gef3(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "Selling: Brand new Chon Chon Doll!",
        "What a great gift to give to a loved one, contact me now!",
        "- Name seems to be smeared -",
        " ",
        "Buying: Manteau!",
        "I'm freezing and I have no zeny, please help me!",
        "Contact Edionyus"
    ])?;
    ctx.close()
}

pub fn sign_post_gef4(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Post Reads -^000000",
        "North to Geffen Fields",
        "Northeast to Al De Baran",
        "Northwest to Glast Heim",
        "East to Geffen Fields",
        "South to Morocc",
        "Southeast to Prontera",
        "Southeast to Alberta",
        "Southwest to Comodo",
        "West to Geffen Fields"
    ])?;
    ctx.close()
}

pub fn sign_gef5(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "''Your always welcomed in Geffen''"])?;
    ctx.close()
}

pub fn sign_gef6(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "Welcome."])?;
    ctx.close()
}

pub fn billboard_alde1(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "In Search of:",
        "I lost my Bongun pet, it wasn't my fault, it just ran away...",
        "If you see him, please let me know. Reward to whomever finds him!"
    ])?;
    ctx.close()
}

pub fn al_de_baran_bulletin_al2(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Al De Baran Bulletin Reads -^000000",
        "''Enjoy your stay in Al De Baran''"
    ])?;
    ctx.close()
}

pub fn billboard_alde3(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "Help Wanted:",
        "We are looking for young, strong and athletic people who are",
        "interested in a full time career as a Blacksmith. If your interested, please contact Altiregen",
        "in Geffen!"
    ])?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum Alde4Step {
    Start,
    OnTouch,
}

fn alde4_run(ctx: &Ctx, mut step: Alde4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Alde4Step::Start => {
                step = Alde4Step::OnTouch;
                continue 'machine;
            }
            Alde4Step::OnTouch => {
                ctx.lines_as("Home Owner", args!["Get off my roof you no good leecher!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alde4(ctx: &Ctx) -> Script {
    alde4_run(ctx, Alde4Step::Start, Vec::new()).map(|_| ())
}

pub fn alde4_ontouch(ctx: &Ctx) -> Script {
    alde4_run(ctx, Alde4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn sign_alde5(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "I saw Santa Claus in Lutie!",
        "- The rest looks like scribble -"
    ])?;
    ctx.close()
}

pub fn billboard_alb1(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Billboard Reads -^000000",
        "Welcome to Alberta, the Merchant's paradise."
    ])?;
    ctx.close()
}

pub fn billboard_alb2(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Billboard Reads -^000000", "Welcome."])?;
    ctx.close()
}

pub fn sign_alb3(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "Tools by the Cart full!",
        "You need tools? We got'em!",
        "Come on in, we never close!"
    ])?;
    ctx.close()
}

pub fn sign_alb4(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^993300- The Sign Reads -^000000",
        "Docking and Shipment times very on load. For information regarding",
        "Shipping and Receiving, please...",
        "- You can't make out the rest -"
    ])?;
    ctx.close()
}

pub fn sign_alb5(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "Welcome."])?;
    ctx.close()
}
