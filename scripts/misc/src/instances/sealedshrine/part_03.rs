use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Ins1fSpawnMobsStep {
    Start,
    OnInstanceInit,
}

fn ins_1f_spawn_mobs_run(ctx: &Ctx, mut step: Ins1fSpawnMobsStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            Ins1fSpawnMobsStep::Start => {
                step = Ins1fSpawnMobsStep::OnInstanceInit;
                continue 'machine;
            }
            Ins1fSpawnMobsStep::OnInstanceInit => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("1@cata")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Zombie Master"),
                        Val::from(1298),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wraith Dead"),
                        Val::from(1291),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Flame Skull"),
                        Val::from(1869),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Skeleton General"),
                        Val::from(1290),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Zombie Master"),
                        Val::from(1298),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Skeleton General"),
                        Val::from(1290),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Flame Skull"),
                        Val::from(1869),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wraith Dead"),
                        Val::from(1291),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wraith Dead"),
                        Val::from(1291),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Khalitzburg"),
                        Val::from(1132),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Khalitzburg"),
                        Val::from(1132),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Flame Skull"),
                        Val::from(1869),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Flame Skull"),
                        Val::from(1869),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Ancient Mimic"),
                        Val::from(1699),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Zombie Master"),
                        Val::from(1298),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Ancient Mimic"),
                        Val::from(1699),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Zombie Master"),
                        Val::from(1298),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wraith Dead"),
                        Val::from(1291),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Skeleton General"),
                        Val::from(1290),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Skeleton General"),
                        Val::from(1290),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wind Ghost"),
                        Val::from(1263),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wind Ghost"),
                        Val::from(1263),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Wind Ghost"),
                        Val::from(1263),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Lude"),
                        Val::from(1509),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Lude"),
                        Val::from(1509),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Evil Druid"),
                        Val::from(1117),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Evil Druid"),
                        Val::from(1117),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Evil Druid"),
                        Val::from(1117),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Banshee"),
                        Val::from(1867),
                        Val::from(10),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        l_map_s.clone(),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Dark Illusion"),
                        Val::from(1302),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_1f_spawn_mobs")])?],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_1f_spawn_mobs(ctx: &Ctx) -> Script {
    ins_1f_spawn_mobs_run(ctx, Ins1fSpawnMobsStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_1f_spawn_mobs_oninstanceinit(ctx: &Ctx) -> Script {
    ins_1f_spawn_mobs_run(ctx, Ins1fSpawnMobsStep::OnInstanceInit, Vec::new()).map(|_| ())
}
