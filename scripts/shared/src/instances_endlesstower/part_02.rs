use script_sdk_2::{Ctx, Function, Script, Stop, Val, runtime};

pub fn f_tower_warp(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_level = Val::from(0);
    let mut l_map_s = Val::from("");
    l_level = runtime::arg(&args, 0, Val::from(0));
    l_map_s = runtime::arg(&args, 1, Val::from(0));
    let subject1 = l_level.clone();
    if subject1 == 2 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(354)])?;
    } else if subject1 == 3 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(354)])?;
    } else if subject1 == 4 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(354)])?;
    } else if subject1 == 5 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(354)])?;
    } else if subject1 == 6 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(270)])?;
    } else if subject1 == 7 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(270)])?;
    } else if subject1 == 8 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(270)])?;
    } else if subject1 == 9 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(270)])?;
    } else if subject1 == 10 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(270)])?;
    } else if subject1 == 11 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(183)])?;
    } else if subject1 == 12 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(183)])?;
    } else if subject1 == 13 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(183)])?;
    } else if subject1 == 14 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(183)])?;
    } else if subject1 == 15 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(183)])?;
    } else if subject1 == 16 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(99)])?;
    } else if subject1 == 17 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(99)])?;
    } else if subject1 == 18 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(99)])?;
    } else if subject1 == 19 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(99)])?;
    } else if subject1 == 20 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(99)])?;
    } else if subject1 == 21 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(12)])?;
    } else if subject1 == 22 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(12)])?;
    } else if subject1 == 23 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(12)])?;
    } else if subject1 == 24 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(12)])?;
    } else if subject1 == 25 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(12)])?;
    } else if subject1 == 27 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(354)])?;
    } else if subject1 == 28 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(354)])?;
    } else if subject1 == 29 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(354)])?;
    } else if subject1 == 30 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(354)])?;
    } else if subject1 == 31 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(270)])?;
    } else if subject1 == 32 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(270)])?;
    } else if subject1 == 33 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(270)])?;
    } else if subject1 == 34 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(270)])?;
    } else if subject1 == 35 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(270)])?;
    } else if subject1 == 36 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(183)])?;
    } else if subject1 == 37 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(183)])?;
    } else if subject1 == 38 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(183)])?;
    } else if subject1 == 39 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(183)])?;
    } else if subject1 == 40 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(183)])?;
    } else if subject1 == 41 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(99)])?;
    } else if subject1 == 42 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(99)])?;
    } else if subject1 == 43 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(99)])?;
    } else if subject1 == 44 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(99)])?;
    } else if subject1 == 45 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(99)])?;
    } else if subject1 == 46 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(12)])?;
    } else if subject1 == 47 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(12)])?;
    } else if subject1 == 48 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(12)])?;
    } else if subject1 == 49 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(12)])?;
    } else if subject1 == 50 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(12)])?;
    } else if subject1 == 52 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(354)])?;
    } else if subject1 == 53 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(354)])?;
    } else if subject1 == 54 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(354)])?;
    } else if subject1 == 55 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(354)])?;
    } else if subject1 == 56 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(270)])?;
    } else if subject1 == 57 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(270)])?;
    } else if subject1 == 58 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(270)])?;
    } else if subject1 == 59 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(270)])?;
    } else if subject1 == 60 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(270)])?;
    } else if subject1 == 61 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(183)])?;
    } else if subject1 == 62 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(183)])?;
    } else if subject1 == 63 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(183)])?;
    } else if subject1 == 64 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(183)])?;
    } else if subject1 == 65 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(183)])?;
    } else if subject1 == 66 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(99)])?;
    } else if subject1 == 67 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(99)])?;
    } else if subject1 == 68 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(99)])?;
    } else if subject1 == 69 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(99)])?;
    } else if subject1 == 70 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(99)])?;
    } else if subject1 == 71 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(12)])?;
    } else if subject1 == 72 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(12)])?;
    } else if subject1 == 73 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(12)])?;
    } else if subject1 == 74 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(12)])?;
    } else if subject1 == 75 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(12)])?;
    } else if subject1 == 77 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(354)])?;
    } else if subject1 == 78 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(354)])?;
    } else if subject1 == 79 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(354)])?;
    } else if subject1 == 80 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(354)])?;
    } else if subject1 == 81 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(270)])?;
    } else if subject1 == 82 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(270)])?;
    } else if subject1 == 83 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(270)])?;
    } else if subject1 == 84 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(270)])?;
    } else if subject1 == 85 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(270)])?;
    } else if subject1 == 86 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(183)])?;
    } else if subject1 == 87 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(183)])?;
    } else if subject1 == 88 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(183)])?;
    } else if subject1 == 89 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(183)])?;
    } else if subject1 == 90 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(183)])?;
    } else if subject1 == 91 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(99)])?;
    } else if subject1 == 92 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(99)])?;
    } else if subject1 == 93 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(99)])?;
    } else if subject1 == 94 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(310), Val::from(99)])?;
    } else if subject1 == 95 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(395), Val::from(99)])?;
    } else if subject1 == 96 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(52), Val::from(12)])?;
    } else if subject1 == 97 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(136), Val::from(12)])?;
    } else if subject1 == 98 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(224), Val::from(12)])?;
    } else if subject1 == 99 {
        ctx.call(Function::Warp, vec![l_map_s.clone(), Val::from(309), Val::from(12)])?;
    }
    return Ok(Val::from(0));
}
