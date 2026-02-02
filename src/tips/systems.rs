use super::components::{ScoreText, TipText};
use crate::core::components::Score;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_tiled::prelude::*;
use tiled::ObjectShape;

/// Spawns Text2d for text objects from Tiled.
pub fn spawn_tip_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_assets: Res<Assets<TiledMapAsset>>,
    maps: Query<&TiledMap, Added<TiledMap>>,
) {
    let font = asset_server.load("fonts/PixelOperator8.ttf");

    for map_handle in &maps {
        let Some(map_asset) = map_assets.get(&map_handle.0) else {
            continue;
        };

        let map = &map_asset.map;
        let map_width = map.width as f32 * map.tile_width as f32;
        let map_height = map.height as f32 * map.tile_height as f32;

        for layer in map.layers() {
            let Some(object_layer) = layer.as_object_layer() else {
                continue;
            };

            for object in object_layer.objects() {
                let ObjectShape::Text {
                    text,
                    pixel_size,
                    color,
                    ..
                } = &object.shape
                else {
                    continue;
                };

                // Convert Tiled coords (top-left origin) to Bevy centered coords
                // TilemapAnchor::Center puts map center at origin
                let x = object.x - map_width / 2.0;
                let y = map_height / 2.0 - object.y;

                // Check if this is a ScoreText
                let is_score_text = object.properties.contains_key("ScoreText");

                let mut entity = commands.spawn((
                    Name::new(format!("Tip: {}", text)),
                    TipText,
                    Text2d::new(text.clone()),
                    TextFont {
                        font: font.clone(),
                        font_size: *pixel_size as f32,
                        ..default()
                    },
                    TextColor(Color::srgba_u8(
                        color.red,
                        color.green,
                        color.blue,
                        color.alpha,
                    )),
                    Anchor::TOP_LEFT,
                    Transform::from_xyz(x, y, 5.0),
                ));

                if is_score_text {
                    entity.insert(ScoreText);
                }
            }
        }
    }
}

/// Updates score text to show current score.
pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text2d, With<ScoreText>>) {
    for mut text in &mut query {
        **text = format!("You collected {} coins.", score.0);
    }
}
