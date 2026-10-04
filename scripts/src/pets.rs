use script_sdk::{Context, Function, Value};
pub fn run_bonus(ctx: &Context, id: u32) -> Result<(), String> { match id {
1002 => pet_bonus_1002(ctx),
1113 => pet_bonus_1113(ctx),
1031 => pet_bonus_1031(ctx),
1063 => pet_bonus_1063(ctx),
1049 => pet_bonus_1049(ctx),
1011 => pet_bonus_1011(ctx),
1042 => pet_bonus_1042(ctx),
1035 => pet_bonus_1035(ctx),
1167 => pet_bonus_1167(ctx),
1107 => pet_bonus_1107(ctx),
1052 => pet_bonus_1052(ctx),
1014 => pet_bonus_1014(ctx),
1077 => pet_bonus_1077(ctx),
1019 => pet_bonus_1019(ctx),
1056 => pet_bonus_1056(ctx),
1057 => pet_bonus_1057(ctx),
1023 => pet_bonus_1023(ctx),
1026 => pet_bonus_1026(ctx),
1110 => pet_bonus_1110(ctx),
1170 => pet_bonus_1170(ctx),
1029 => pet_bonus_1029(ctx),
1155 => pet_bonus_1155(ctx),
1109 => pet_bonus_1109(ctx),
1101 => pet_bonus_1101(ctx),
1188 => pet_bonus_1188(ctx),
1200 => pet_bonus_1200(ctx),
1275 => pet_bonus_1275(ctx),
1815 => pet_bonus_1815(ctx),
1245 => pet_bonus_1245(ctx),
1519 => pet_bonus_1519(ctx),
1879 => pet_bonus_1879(ctx),
1122 => pet_bonus_1122(ctx),
1123 => pet_bonus_1123(ctx),
1125 => pet_bonus_1125(ctx),
1385 => pet_bonus_1385(ctx),
1382 => pet_bonus_1382(ctx),
1208 => pet_bonus_1208(ctx),
1963 => pet_bonus_1963(ctx),
1040 => pet_bonus_1040(ctx),
1143 => pet_bonus_1143(ctx),
1148 => pet_bonus_1148(ctx),
1179 => pet_bonus_1179(ctx),
1299 => pet_bonus_1299(ctx),
1370 => pet_bonus_1370(ctx),
1374 => pet_bonus_1374(ctx),
1379 => pet_bonus_1379(ctx),
1401 => pet_bonus_1401(ctx),
1404 => pet_bonus_1404(ctx),
1416 => pet_bonus_1416(ctx),
1495 => pet_bonus_1495(ctx),
1504 => pet_bonus_1504(ctx),
1505 => pet_bonus_1505(ctx),
1513 => pet_bonus_1513(ctx),
1586 => pet_bonus_1586(ctx),
1630 => pet_bonus_1630(ctx),
1837 => pet_bonus_1837(ctx),
2081 => pet_bonus_2081(ctx),
_ => Err(format!("Unknown pre-renewal pet class {id}")),
} }
pub fn run_support(ctx: &Context, id: u32) -> Result<(), String> { match id {
1002 => pet_support_1002(ctx),
1113 => pet_support_1113(ctx),
1031 => pet_support_1031(ctx),
1063 => pet_support_1063(ctx),
1049 => pet_support_1049(ctx),
1011 => pet_support_1011(ctx),
1042 => pet_support_1042(ctx),
1035 => pet_support_1035(ctx),
1167 => pet_support_1167(ctx),
1107 => pet_support_1107(ctx),
1052 => pet_support_1052(ctx),
1014 => pet_support_1014(ctx),
1077 => pet_support_1077(ctx),
1019 => pet_support_1019(ctx),
1056 => pet_support_1056(ctx),
1057 => pet_support_1057(ctx),
1023 => pet_support_1023(ctx),
1026 => pet_support_1026(ctx),
1110 => pet_support_1110(ctx),
1170 => pet_support_1170(ctx),
1029 => pet_support_1029(ctx),
1155 => pet_support_1155(ctx),
1109 => pet_support_1109(ctx),
1101 => pet_support_1101(ctx),
1188 => pet_support_1188(ctx),
1200 => pet_support_1200(ctx),
1275 => pet_support_1275(ctx),
1815 => pet_support_1815(ctx),
1245 => pet_support_1245(ctx),
1519 => pet_support_1519(ctx),
1879 => pet_support_1879(ctx),
1122 => pet_support_1122(ctx),
1123 => pet_support_1123(ctx),
1125 => pet_support_1125(ctx),
1385 => pet_support_1385(ctx),
1382 => pet_support_1382(ctx),
1208 => pet_support_1208(ctx),
1963 => pet_support_1963(ctx),
1040 => pet_support_1040(ctx),
1143 => pet_support_1143(ctx),
1148 => pet_support_1148(ctx),
1179 => pet_support_1179(ctx),
1299 => pet_support_1299(ctx),
1370 => pet_support_1370(ctx),
1374 => pet_support_1374(ctx),
1379 => pet_support_1379(ctx),
1401 => pet_support_1401(ctx),
1404 => pet_support_1404(ctx),
1416 => pet_support_1416(ctx),
1495 => pet_support_1495(ctx),
1504 => pet_support_1504(ctx),
1505 => pet_support_1505(ctx),
1513 => pet_support_1513(ctx),
1586 => pet_support_1586(ctx),
1630 => pet_support_1630(ctx),
1837 => pet_support_1837(ctx),
2081 => pet_support_2081(ctx),
_ => Err(format!("Unknown pre-renewal pet class {id}")),
} }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1002(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bLuk")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bCritical")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1002(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1113(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bHit")?, Value::Number(3)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtk")?, Value::Number(3)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1113(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1031(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bLuk")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubEle")?, ctx.constant("Ele_Poison")?, Value::Number(10)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1031(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1063(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bCritical")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtk")?, Value::Number(2)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1063(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1049(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bStr")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtk")?, Value::Number(5)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1049(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1011(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAgi")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee")?, Value::Number(2)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1011(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1042(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee")?, Value::Number(6)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAgi")?, Value::Number((Value::Number(1)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1042(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1035(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee")?, Value::Number((Value::Number(5)).number_value()?.wrapping_neg())])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee2")?, Value::Number(2)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1035(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1167(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bVit")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHP")?, Value::Number(50)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1167(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1107(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bInt")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxSP")?, Value::Number(50)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1107(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1052(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bHPrecovRate")?, Value::Number(5)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHP")?, Value::Number(25)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1052(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1014(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bHit")?, Value::Number(5)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtk")?, Value::Number((Value::Number(2)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1014(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1077(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bStr")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bInt")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1077(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1019(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHP")?, Value::Number(150)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxSP")?, Value::Number((Value::Number(10)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1019(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1056(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAgi")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee2")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1056(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1057(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bCritical")?, Value::Number(3)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bLuk")?, Value::Number((Value::Number(1)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1057(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1023(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtk")?, Value::Number(10)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDef")?, Value::Number((Value::Number(3)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1023(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1026(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bInt")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDef")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1026(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1110(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMatkRate")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtkRate")?, Value::Number((Value::Number(1)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1110(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1170(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bStr")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDex")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1170(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1029(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMatkRate")?, Value::Number((Value::Number(1)).number_value()?.wrapping_neg())])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtkRate")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1029(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1155(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDef")?, Value::Number((Value::Number(2)).number_value()?.wrapping_neg())])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMdef")?, Value::Number((Value::Number(2)).number_value()?.wrapping_neg())])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAspdRate")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1155(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1109(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMatkRate")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAtkRate")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHPrate")?, Value::Number((Value::Number(3)).number_value()?.wrapping_neg())])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxSPrate")?, Value::Number((Value::Number(3)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1109(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1101(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDef")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMdef")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bResEff")?, ctx.constant("Eff_Stun")?, Value::Number((Value::Number(100)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1101(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1188(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bVit")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bResEff")?, ctx.constant("Eff_Stun")?, Value::Number(100)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1188(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1200(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bAddRace")?, ctx.constant("RC_Demihuman")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bMagicAddRace")?, ctx.constant("RC_DemiHuman")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bAddRace")?, ctx.constant("RC_Player_Human")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bMagicAddRace")?, ctx.constant("RC_Player_Human")?, Value::Number(2)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1200(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1275(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMdef")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubRace")?, ctx.constant("RC_DemiHuman")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubRace")?, ctx.constant("RC_Player_Human")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1275(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1815(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubEle")?, ctx.constant("Ele_Neutral")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHPrate")?, Value::Number((Value::Number(1)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1815(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1245(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHP")?, Value::Number(30)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubEle")?, ctx.constant("Ele_Water")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1245(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1519(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDef")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubRace")?, ctx.constant("RC_DemiHuman")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubRace")?, ctx.constant("RC_Player_Human")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1519(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1879(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1879(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1122(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1122(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1123(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1123(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1125(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1125(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1385(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1385(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1382(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1382(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1208(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAgi")?, Value::Number(3)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDex")?, Value::Number(1)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1208(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1963(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1963(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1040(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHP")?, Value::Number(100)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee")?, Value::Number((Value::Number(5)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1040(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1143(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bSPrecovRate")?, Value::Number(3)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1143(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1148(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bVit")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bResEff")?, ctx.constant("Eff_Stone")?, Value::Number(500)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1148(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1179(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bFlee")?, Value::Number(7)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bDef")?, Value::Number((Value::Number(3)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1179(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1299(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bAddRace")?, ctx.constant("RC_DemiHuman")?, Value::Number(3)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bAddRace")?, ctx.constant("RC_Player_Human")?, Value::Number(3)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1299(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1370(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bHpDrainRate")?, Value::Number(50), Value::Number(5)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1370(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1374(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxSPRate")?, Value::Number(3)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1374(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1379(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bResEff")?, ctx.constant("Eff_Sleep")?, Value::Number(10000)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1379(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1401(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bAgi")?, Value::Number(2)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1401(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1404(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bInt")?, Value::Number(1)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bCastrate")?, Value::Number((Value::Number(3)).number_value()?.wrapping_neg())])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1404(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1416(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxSP")?, Value::Number(30)])?;
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bSPrecovRate")?, Value::Number(5)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1416(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1495(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubEle")?, ctx.constant("Ele_Fire")?, Value::Number(3)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1495(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1504(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bCritAtkRate")?, Value::Number(5)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1504(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1505(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxHPRate")?, Value::Number(3)])?;
let _ = ctx.call(Function::Bonus3, vec![ctx.constant("bAutoSpellWhenHit")?, Value::String("AL_HEAL".into()), Value::Number(1), Value::Number(10)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1505(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1513(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus, vec![ctx.constant("bMaxSP")?, Value::Number(10)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1513(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1586(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubRace")?, ctx.constant("RC_Brute")?, Value::Number(3)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubRace")?, ctx.constant("RC_Player_Doram")?, Value::Number(3)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1586(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1630(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1630(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_1837(ctx: &Context) -> Result<(), String> { let mut local_i = Value::default();
local_i = ctx.call(Function::GetPetInfo, vec![ctx.constant("PETINFO_INTIMATE")?])?;
if ((local_i.clone()).binary(">=", ctx.constant("PET_INTIMATE_LOYAL")?)?).truthy() { {
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bSubEle")?, ctx.constant("Ele_Fire")?, Value::Number(2)])?;
let _ = ctx.call(Function::Bonus2, vec![ctx.constant("bAddEle")?, ctx.constant("Ele_Fire")?, Value::Number(2)])?;
} }
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_1837(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_bonus_2081(ctx: &Context) -> Result<(), String> { 
Ok(()) }
#[inline(never)]
#[allow(unused_mut, unused_assignments, unused_variables, unreachable_code)]
fn pet_support_2081(ctx: &Context) -> Result<(), String> { 
Ok(()) }
