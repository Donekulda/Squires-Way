//! Application shell — Bevy + egui composition root for the `app` conversion part.
//!
//! # C# reference (`IronyModManager.Program`)
//!
//! Irony [`Program.Main`](https://github.com/bcssov/IronyModManager/blob/master/src/IronyModManager/Program.cs)
//! runs (order matters): set cwd to exe dir → `InitDefaultCulture` → `InitAppEvents` → `InitDI` → `InitLogging` →
//! `ParseArguments` → optional single-instance gate → `BuildAvaloniaApp` / `InitAvaloniaOptions` → `Bootstrap.PostStartup`
//! → Avalonia desktop lifetime. Rust mirrors the **early startup** slice here; UI is **Bevy + egui** (re-platform).
//!
//! ## Parity vs re-platform (gap analysis)
//!
//! | C# step | SquiresWay | Parity note |
//! |---------|------------|-------------|
//! | `Environment.CurrentDirectory = BaseDirectory` | [`crate::game::set_working_directory_to_executable_dir`] in [`run`] | Same intent; failure is logged and startup continues. |
//! | `InitDefaultCulture` / `CurrentLocale.SetCurrent` + `ResourceManager.Init` | [`set_current`](crate::localization::set_current)([`DEFAULT_APP_CULTURE`]) then [`LocalizationManager`] in startup | Culture `"en"` matches Irony `Shared.Constants.DefaultAppCulture`. |
//! | `InitAppEvents` (unhandled exception + unobserved task) | [`install_panic_hook`] after tracing init | Logs panics via `tracing`; no `TaskScheduler` equivalent until async tasks are added. |
//! | `InitDI` (SimpleInjector + `Bootstrap.Setup`) | [`DiBootstrap`] + [`MessageBusResource`] | Explicit construction; no static service locator. |
//! | `InitLogging` (NLog + storage path) | `tracing_subscriber` in [`init_tracing`] | Structured logging; file targets / NLog layout not ported. |
//! | `ParseArguments` (CommandLine) | [`parse_args`](crate::game::parse_args) → [`ParsedCliArgs`] resource | Same argv slice as `GameHandler`; full CLI parity is incremental. |
//! | `InitSingleInstance` | [`try_single_instance`] | **Stub:** if `single_instance` is true, logs and still allows launch (Irony mutex not ported). |
//! | `BuildAvaloniaApp` / `InitAvaloniaOptions` (Linux X11/Wayland) | Bevy `DefaultPlugins` + window title | Linux GPU/Wayland app-id live in [`crate::platform::PlatformConfigurationOptions`] for future winit/bevy config. |
//! | `Bootstrap.PostStartup` / `DIContainer.Finish` | Startup systems (`init_app_shell_state`, `wire_message_bus_demo`) | Subset: message bus + shell state; full container verify deferred. |
//! | `StartWithClassicDesktopLifetime` | [`App::run`](bevy::app::App::run) | Desktop window via Bevy; not Avalonia. |
//! | `catch` / `LogError` / fatal subprocess | Not replicated | Would use `anyhow` + UI or `tracing` at app boundary when wired. |
//!
//! **Must match Irony (behavior):** default culture, argv parsing contract for shared flags, storage path usage where implemented.
//! **Re-platform acceptable:** Avalonia controls, Skia, NLog file format, single-instance IPC, fatal-error subprocess.
//!
//! [`run`]: fn@run

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy_egui::{EguiContexts, EguiPlugin, egui};

use crate::di::{DiBootstrap, MessageBusResource, SubscriptionGuard};
use crate::game::{GameHandlerArgs, parse_args};
use crate::localization::{FileLocalizationResourceProvider, LocalizationManager, set_current};
use crate::merging::{ScriptElement, merge_top_level_children};
use crate::platform::{AppOptions, PlatformConfigurationOptions};
use crate::services::{ModMergeService, ModMergeServiceStub};
use crate::storage::paths::default_data_dir;

/// Irony `IronyModManager.Shared.Constants.DefaultAppCulture` (`"en"`).
pub const DEFAULT_APP_CULTURE: &str = "en";

/// Parsed CLI as a Bevy resource (Irony `StaticResources.CommandLineOptions` / `GameHandler` argv).
#[derive(Resource, Clone, Debug)]
pub struct ParsedCliArgs(pub GameHandlerArgs);

/// Platform options snapshot for the shell (Irony `IPlatformConfiguration` / JSON settings).
#[derive(Resource, Clone, Debug)]
pub struct PlatformShellConfig(pub PlatformConfigurationOptions);

/// Demo event for [`InMemoryMessageBus::publish`] (proves subscribe + publish in the shell).
#[derive(Clone, Debug)]
pub struct AppShellPing {
    /// Monotonic tick from UI (optional; for display).
    pub tick: u32,
}

/// Keeps the message-bus subscription alive for the app lifetime (C# `IDisposable` subscription).
#[derive(Resource)]
#[expect(dead_code)] // guard only needed for `Drop`; field is intentionally unused
struct MessageBusDemoSubscription(SubscriptionGuard);

/// Shared counter updated by the bus demo handler.
#[derive(Resource)]
struct BusPingCounter(Arc<AtomicU32>);

/// UI + demo state for the first vertical slice (args, paths, localization, merge helper).
#[derive(Resource)]
struct AppShellState {
    parsed_args: GameHandlerArgs,
    data_dir: Option<PathBuf>,
    localized_title: Option<String>,
    merge_demo: String,
    bus_pings: Arc<AtomicU32>,
    next_ping_tick: u32,
    single_instance_enabled: bool,
}

/// Early startup aligned with Irony `Program.Main` before the Bevy loop (see module docs).
pub fn run() {
    if let Err(e) = crate::game::set_working_directory_to_executable_dir() {
        eprintln!("SquiresWay: could not set cwd to executable dir: {e}");
    }

    set_current(DEFAULT_APP_CULTURE);
    init_tracing();
    install_panic_hook();

    let argv: Vec<String> = std::env::args().skip(1).collect();
    let parsed_args = parse_args(&argv).unwrap_or_default();
    let platform = PlatformConfigurationOptions::default();

    if !try_single_instance(&platform.app) {
        return;
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SquiresWay".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(ParsedCliArgs(parsed_args))
        .insert_resource(PlatformShellConfig(platform.clone()))
        .insert_resource(MessageBusResource(DiBootstrap::new().into_message_bus()))
        .add_systems(Startup, (init_app_shell_state, wire_message_bus_demo))
        .add_systems(Update, app_shell_ui)
        .run();
}

/// Irony `InitSingleInstance`: returns `false` if a second instance should exit immediately.
///
/// **Current behavior:** always allows the process to continue. When `single_instance` is `true`, logs that
/// enforcement is not yet implemented (Irony uses a named mutex + second-instance activation).
#[must_use]
pub fn try_single_instance(app: &AppOptions) -> bool {
    if !app.single_instance {
        return true;
    }
    tracing::warn!(
        "single_instance is enabled in platform options but second-instance blocking is not implemented; \
         continuing (Irony parity pending)"
    );
    true
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("panic: {info}");
        default_hook(info);
    }));
}

fn merge_demo_string() -> String {
    let base = vec![
        ScriptElement::new_key("a"),
        ScriptElement::new_key("marker"),
        ScriptElement::new_key("z"),
    ];
    let insert = vec![ScriptElement::new_key("new1")];
    let out = merge_top_level_children(base, "marker", insert, false);
    out.iter()
        .filter_map(|e| e.key.as_deref())
        .collect::<Vec<_>>()
        .join(" → ")
}

fn init_app_shell_state(
    mut commands: Commands,
    cli: Res<ParsedCliArgs>,
    platform: Res<PlatformShellConfig>,
) {
    let parsed_args = cli.0.clone();
    let data_dir = default_data_dir();

    let locales_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/locales");
    let loc = LocalizationManager::new(vec![Box::new(FileLocalizationResourceProvider::new(
        locales_root,
    ))]);
    let localized_title = loc.get_resource("App.Title");

    let bus_pings = Arc::new(AtomicU32::new(0));
    commands.insert_resource(AppShellState {
        parsed_args,
        data_dir,
        localized_title,
        merge_demo: merge_demo_string(),
        bus_pings: Arc::clone(&bus_pings),
        next_ping_tick: 1,
        single_instance_enabled: platform.0.app.single_instance,
    });
    commands.insert_resource(BusPingCounter(bus_pings));
}

fn wire_message_bus_demo(
    mut commands: Commands,
    bus: Res<MessageBusResource>,
    counter: Res<BusPingCounter>,
) {
    let c = Arc::clone(&counter.0);
    let sub = bus.0.subscribe(move |_p: AppShellPing| {
        c.fetch_add(1, Ordering::SeqCst);
    });
    commands.insert_resource(MessageBusDemoSubscription(sub));
}

fn app_shell_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<AppShellState>,
    bus: Res<MessageBusResource>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(state.localized_title.as_deref().unwrap_or("SquiresWay"));
        ui.separator();
        ui.label(format!(
            "GameHandler args (parse_args): {:?}",
            state.parsed_args
        ));
        ui.label(format!("Default data dir: {:?}", state.data_dir));
        ui.label(format!(
            "Platform single_instance (options): {} — enforcement stubbed; see logs if true",
            state.single_instance_enabled
        ));
        ui.label(format!("MergeTopLevel demo keys: {}", state.merge_demo));
        ui.label(format!(
            "ModMergeService (stub) allow_mod_merge: {}",
            ModMergeServiceStub.allow_mod_merge("demo")
        ));
        ui.label(format!(
            "Message bus demo deliveries: {}",
            state.bus_pings.load(Ordering::SeqCst)
        ));
        if ui.button("Ping message bus").clicked() {
            let t = state.next_ping_tick;
            state.next_ping_tick = state.next_ping_tick.saturating_add(1);
            bus.0.publish(AppShellPing { tick: t });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::parse_args;
    use crate::localization::{FileLocalizationResourceProvider, LocalizationManager, set_current};
    use std::path::PathBuf;

    #[test]
    fn try_single_instance_allows_when_disabled() {
        let mut app = AppOptions::default();
        app.single_instance = false;
        assert!(try_single_instance(&app));
    }

    #[test]
    fn try_single_instance_stub_allows_when_enabled() {
        let mut app = AppOptions::default();
        app.single_instance = true;
        assert!(try_single_instance(&app));
    }

    #[test]
    fn default_app_culture_matches_irony() {
        assert_eq!(DEFAULT_APP_CULTURE, "en");
    }

    /// Golden-style check: merge list matches Irony-style insert-before-key behavior.
    #[test]
    fn merge_demo_matches_expected_key_order() {
        let s = merge_demo_string();
        assert_eq!(
            s, "a → new1 → marker → z",
            "merge_top_level_children insert-before-marker"
        );
    }

    /// Fixture under `assets/locales/en.json` resolves `App.Title`.
    #[test]
    fn localization_fixture_app_title() {
        set_current("en");
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/locales");
        let loc =
            LocalizationManager::new(vec![Box::new(FileLocalizationResourceProvider::new(root))]);
        assert_eq!(loc.get_resource("App.Title").as_deref(), Some("SquiresWay"));
    }

    #[test]
    fn parse_args_empty_is_default() {
        let a = parse_args(&[]).expect("parse");
        assert_eq!(a, GameHandlerArgs::default());
    }
}
