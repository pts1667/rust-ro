use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum SpawnmanagerKitsuneStep {
    Start,
    OnInit,
    OnMyMobDead,
    OnMyMobDead2,
}

pub(super) fn spawnmanager_kitsune_run(ctx: &Ctx, mut step: SpawnmanagerKitsuneStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SpawnmanagerKitsuneStep::Start => {
                step = SpawnmanagerKitsuneStep::OnInit;
                continue 'machine;
            }
            SpawnmanagerKitsuneStep::OnInit => {
                ctx.var(".mymobs1").set(Val::from(1))?;
                ctx.var(".mymobs2").set(Val::from(1))?;
                return Err(Stop::End);
            }
            SpawnmanagerKitsuneStep::OnMyMobDead => {
                ctx.var(".mymobs1").set((ctx.var(".mymobs1").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs1").get()?.number()? < 1 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Nine Tails#Kitsune Man")])?;
                }
                return Err(Stop::End);
            }
            SpawnmanagerKitsuneStep::OnMyMobDead2 => {
                ctx.var(".mymobs2").set((ctx.var(".mymobs2").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs2").get()?.number()? < 1 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Nine Tails#Kitsune Mask")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}
