mod camera;
mod killzone;
mod physics;
mod pickups;
mod platform;
mod player;
pub mod state;
mod tiled;

use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use killzone::KillZonePlugin;
use physics::PhysicsPlugin;
use pickups::PickupsPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use state::{GameState, restart_game};
use tiled::TiledPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Knight Jumper".to_string(),
                    resolution: WindowResolution::new(960, 640),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        TiledPlugin,
        PhysicsPlugin,
        PlayerPlugin,
        CameraPlugin,
        PlatformPlugin,
        PickupsPlugin,
        KillZonePlugin,
    ))
    .init_state::<GameState>()
    .add_systems(OnEnter(GameState::Reloading), restart_game);

    #[cfg(feature = "debug")]
    {
        use avian2d::prelude::*;
        use bevy_inspector_egui::bevy_egui::EguiPlugin;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(PhysicsDebugPlugin)
            .register_type::<PhysicsGizmos>();
    }

    app.run()
}
