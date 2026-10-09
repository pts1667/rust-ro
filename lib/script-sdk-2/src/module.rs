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
            $crate::registry::run($crate::registry::Tables { npcs: NPCS, events: EVENTS, ..Default::default() })
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
            $crate::registry::run($crate::registry::Tables { items: ITEMS, bonuses: BONUSES, programs: PROGRAMS, ..Default::default() })
        }
    };
}

/// Exports a pet module: the passive bonus script, the support script and the automatic bonus programs of pets, by monster class
/// (programs by program id).
///
/// ```ignore
/// script_sdk_2::pet_module! {
///     pets { 1002 => pet_bonus_1002, }
///     supports { 1002 => pet_support_1002, }
///     programs { 1 => pet_program_1, }
/// }
/// ```
#[macro_export]
macro_rules! pet_module {
    (
        pets { $($pet:literal => $pet_script:path),* $(,)? }
        supports { $($support:literal => $support_script:path),* $(,)? }
        programs { $($program:literal => $program_script:path),* $(,)? }
    ) => {
        const _: () = assert!($crate::registry::is_strictly_sorted_ids(&[$($pet),*]), "Pet classes must be unique and sorted");
        const _: () = assert!($crate::registry::is_strictly_sorted_ids(&[$($support),*]), "Pet support classes must be unique and sorted");
        const _: () = assert!($crate::registry::is_strictly_sorted_ids(&[$($program),*]), "Pet program ids must be unique and sorted");

        #[no_mangle]
        pub extern "C" fn script_abi() -> u32 {
            $crate::registry::ABI
        }

        #[no_mangle]
        pub extern "C" fn script_run() -> i32 {
            const PETS: &[(u32, $crate::registry::ItemFn)] = &[$(($pet, $pet_script)),*];
            const SUPPORTS: &[(u32, $crate::registry::ItemFn)] = &[$(($support, $support_script)),*];
            const PROGRAMS: &[(u32, $crate::registry::ItemFn)] = &[$(($program, $program_script)),*];
            $crate::registry::run($crate::registry::Tables { pets: PETS, pet_supports: SUPPORTS, pet_programs: PROGRAMS, ..Default::default() })
        }
    };
}
