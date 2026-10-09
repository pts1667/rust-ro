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
            $crate::registry::run(NPCS, EVENTS, &[], &[], &[])
        }
    };
}

/// Exports an item module: the use scripts, passive bonuses and bonus programs of items, by item id, and the catalog
/// fingerprint the server checks against its item manifest.
///
/// ```ignore
/// script_sdk_2::item_module! {
///     catalog 1234567890,
///     items { 501 => red_potion, }
///     bonuses { 1 => bonus_1, }
///     programs { 1 => program_1, }
/// }
/// ```
#[macro_export]
macro_rules! item_module {
    (
        catalog $catalog:literal,
        items { $($item:literal => $item_script:path),* $(,)? }
        bonuses { $($bonus:literal => $bonus_script:path),* $(,)? }
        programs { $($program:literal => $program_script:path),* $(,)? }
    ) => {
        const _: () = assert!($crate::registry::is_strictly_sorted_ids(&[$($item),*]), "Item ids must be unique and sorted");
        const _: () = assert!($crate::registry::is_strictly_sorted_ids(&[$($bonus),*]), "Bonus ids must be unique and sorted");
        const _: () = assert!($crate::registry::is_strictly_sorted_ids(&[$($program),*]), "Program ids must be unique and sorted");

        #[no_mangle]
        pub extern "C" fn script_abi() -> u32 {
            $crate::registry::ABI
        }

        #[no_mangle]
        pub extern "C" fn script_catalog_hash() -> u64 {
            $catalog
        }

        #[no_mangle]
        pub extern "C" fn script_run() -> i32 {
            const ITEMS: &[(u32, $crate::registry::ItemFn)] = &[$(($item, $item_script)),*];
            const BONUSES: &[(u32, $crate::registry::BonusFn)] = &[$(($bonus, $bonus_script)),*];
            const PROGRAMS: &[(u32, $crate::registry::ItemFn)] = &[$(($program, $program_script)),*];
            $crate::registry::run(&[], &[], ITEMS, BONUSES, PROGRAMS)
        }
    };
}
