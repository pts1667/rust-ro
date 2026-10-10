use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum LuciaBrasilisStep {
    Start,
    LWhatHappen,
    HoistEnd1,
    HoistEnd2,
    OnInit,
    OnTimer7000,
}
