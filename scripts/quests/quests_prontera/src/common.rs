use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum MarjanaPoisonStep {
    Start,
    OnInit,
    OnEnable,
}
