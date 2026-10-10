#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args, runtime};

pub fn sign_post_god(ctx: &Ctx) -> Script {
    let mut l_i = Val::from(1);
    let mut l_seal_s: Vec<Val> = Vec::new();
    let mut l_status_s = Val::from("");
    let mut l_val = Val::from(0);
    ctx.mes("======== God Seal Status ========")?;
    runtime::local_set(&mut l_seal_s, &Val::from(1), Val::from("Sleipnir"), true);
    runtime::local_set(&mut l_seal_s, &Val::from(2), Val::from("Megingjard"), true);
    runtime::local_set(&mut l_seal_s, &Val::from(3), Val::from("Brisingamen"), true);
    runtime::local_set(&mut l_seal_s, &Val::from(4), Val::from("Mjolnir"), true);
    while l_i.number()? <= 4 {
        l_val = runtime::getd(
            ctx,
            &(Val::from("$God") + l_i.clone()),
            &[
                (".@i", runtime::Local::Scalar(&l_i)),
                (".@seal$", runtime::Local::Array(&l_seal_s)),
                (".@status$", runtime::Local::Scalar(&l_status_s)),
                (".@val", runtime::Local::Scalar(&l_val)),
            ],
        )?;
        if l_val == 0 {
            l_status_s = Val::from("Unseen");
        } else if runtime::op(&l_val, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
            l_status_s = Val::from("Active");
        } else if runtime::op(&l_val, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
            l_status_s = Val::from("Appeared");
        } else {
            l_status_s = Val::from("Released");
        }
        ctx.lines(args![
            runtime::local_get(&l_seal_s, &l_i, true) + Val::from(" Seal: ") + l_status_s.clone()
        ])?;
        l_i = l_i + Val::from(1);
    }
    ctx.lines(args![" ", "======= Your Seal Status ========"])?;
    seal_completion_line(ctx, "Sleipnir", "god_sl_1", 51)?;
    seal_completion_line(ctx, "Megingjard", "god_eremes", 28)?;
    seal_completion_line(ctx, "Brisingamen", "god_brising", 50)?;
    seal_completion_line(ctx, "Mjolnir", "god_mjo_0", 11)?;
    ctx.close()
}

fn seal_completion_line(ctx: &Ctx, seal: &str, var: &str, required: i32) -> Result<(), Stop> {
    if ctx.var(var).get()?.number()? < required {
        ctx.mes(&format!("^ff0000{seal} Seal: Not Completed^000000"))
    } else {
        ctx.mes(&format!("^00ff00{seal} Seal: Completed^000000"))
    }
}
