/// Exports a named-ABI script module: the NPC dialogues and events it provides, by manifest name.
///
/// ```ignore
/// script_sdk_2::script_module! {
///     npcs {
///         "prontera_guard" => prontera_guard,
///     }
///     events {
///         "prontera_guard::OnTouch" => prontera_guard_touch,
///     }
/// }
/// ```
///
/// The names must be unique and in ascending order. The macro checks this at compile time.
#[macro_export]
macro_rules! script_module {
    (
        npcs { $($npc:literal => $npc_script:path),* $(,)? }
        events { $($event:literal => $event_script:path),* $(,)? }
    ) => {
        const _: () = assert!($crate::registry::is_strictly_sorted(&[$($npc),*]), "NPC names must be unique and sorted");
        const _: () = assert!($crate::registry::is_strictly_sorted(&[$($event),*]), "Event names must be unique and sorted");

        #[no_mangle]
        pub extern "C" fn script_abi() -> u32 {
            $crate::registry::ABI
        }

        #[no_mangle]
        pub extern "C" fn script_run() -> i32 {
            const NPCS: &[(&str, $crate::registry::ScriptFn)] = &[$(($npc, $npc_script)),*];
            const EVENTS: &[(&str, $crate::registry::ScriptFn)] = &[$(($event, $event_script)),*];
            $crate::registry::run(NPCS, EVENTS)
        }
    };
}
