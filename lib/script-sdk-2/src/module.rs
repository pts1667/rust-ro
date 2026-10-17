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

/// Exports a plugin (`docs/plugins.md`): its identity, its initialisation, the NPCs and events its manifests place, the hooks
/// it observes and the decisions it intercepts. The plugin's folder must carry its `name`.
///
/// ```ignore
/// script_sdk_2::plugin_module! {
///     info { name: "double_xp", version: "1.0.0", requires: [], capabilities: [Rates, Broadcast] }
///     init => init,
///     npcs { "Rate Clerk" => rate_clerk }
///     events { "Rate Clerk::OnTimer3600000" => announce }
///     hooks { MobKilled => on_mob_killed }
///     interceptors { ExpGain => exp_gain }
/// }
/// ```
///
/// `requires` and `capabilities` may be left out. NPC and event names must be unique and in ascending order.
#[macro_export]
macro_rules! plugin_module {
    (
        info {
            name: $name:literal,
            version: $version:literal
            $(, requires: [$($requires:literal),* $(,)?])?
            $(, capabilities: [$($capability:ident),* $(,)?])?
            $(,)?
        }
        init => $init:path,
        npcs { $($npc:literal => $npc_script:path),* $(,)? }
        events { $($event:literal => $event_script:path),* $(,)? }
        hooks { $($hook:ident => $hook_script:path),* $(,)? }
        interceptors { $($interceptor:ident => $interceptor_fn:path),* $(,)? }
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
            const HOOKS: &[($crate::HookKind, $crate::HookFn)] = &[$(($crate::HookKind::$hook, $hook_script)),*];
            $crate::registry::run($crate::registry::Tables {
                npcs: NPCS,
                events: EVENTS,
                plugin_init: Some($init),
                hooks: HOOKS,
                ..Default::default()
            })
        }

        #[no_mangle]
        pub extern "C" fn plugin_buffer() -> u32 {
            $crate::plugin::buffer_address()
        }

        #[no_mangle]
        pub extern "C" fn plugin_info(_length: u32) -> i32 {
            $crate::plugin::export_info(&$crate::PluginInfo {
                name: $name.into(),
                version: $version.into(),
                sdk_abi: $crate::registry::ABI,
                requires: vec![$($($requires.into()),*)?],
                capabilities: vec![$($($crate::Capability::$capability),*)?],
                hooks: vec![$($crate::HookKind::$hook),*],
                interceptors: vec![$($crate::InterceptorKind::$interceptor),*],
            })
        }

        #[no_mangle]
        pub extern "C" fn plugin_configure(length: u32) -> i32 {
            $crate::plugin::export_configure(length)
        }

        #[no_mangle]
        pub extern "C" fn plugin_intercept(length: u32) -> i32 {
            const INTERCEPTORS: &[($crate::InterceptorKind, $crate::InterceptFn)] = &[$(($crate::InterceptorKind::$interceptor, $interceptor_fn)),*];
            $crate::plugin::export_intercept(INTERCEPTORS, length)
        }
    };
}
