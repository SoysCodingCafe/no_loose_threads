// Tiles module for handling tile placement and collisions
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};

use crate::derivables::*;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, (
				drag_and_drop,
			).run_if(in_state(GameState::Game)))
		;
	}
}

pub fn spawn_tile(
	commands: &mut Commands,
	location: Vec3,
	tile_type: TileType,
	asset_server: &Res<AssetServer>,
	tack_group: usize,
	origin: bool,
	links: i32,
	suspect: bool,
	tack_tile: usize,
) -> Entity {
	commands.spawn((SpriteBundle{
		transform: Transform::from_translation(location),
		texture: asset_server.load(tile_type.get_path()),
		sprite: Sprite {
			custom_size: Some(Vec2::new(CELL_SIZE*4.0, CELL_SIZE*4.0)),
			..default()
		},
		..default()
	},
	Tile(tile_type),
	RemoveOnReset,
	)).with_children(|parent| {
		let total_tilettes = tile_type.get_layout().len();
		let tack_tilette = if tack_tile == 99 {rand::Rng::gen_range(&mut rand::thread_rng(), 0..total_tilettes)}
			else {tack_tile};
		let mut tile_count = 0;
		for loc in tile_type.get_layout() {
			parent.spawn((
				SpriteBundle{
					transform: Transform::from_xyz(TILE_OFFSETS[loc.0][loc.1].x, TILE_OFFSETS[loc.0][loc.1].y, 0.0),
					sprite: Sprite {
						custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
						color: Color::rgba(1.0, 0.4, 0.4, 0.0),
						..default()
					},
					..default()
				},
				Tilette,
			));
			if tile_count == tack_tilette {
				parent.spawn((
					SpriteBundle{
						transform: Transform::from_xyz(TILE_OFFSETS[loc.0][loc.1].x, TILE_OFFSETS[loc.0][loc.1].y, 300.0),
						texture: asset_server.load(if tack_group == 4 {"sprites/pin.png"} else {"sprites/pin_alt.png"}),
						sprite: Sprite {
							custom_size: Some(Vec2::new(CELL_SIZE - 20.0, CELL_SIZE - 20.0)),
							color: get_tack_color(tack_group),
							..default()
						},
						..default()
					},
					Tack {
						group: tack_group,
						end: origin,
						used: false,
						links: links,
						suspect: suspect,
						tile_type: tile_type,
					},
				));
			}
			tile_count += 1;
		}
	}).id()
}

fn drag_and_drop(
	mut commands: Commands,
	mut held_query: Query<(Entity, &Children, &mut Transform, &Held)>,
	tile_query: Query<(Entity, &Transform, (With<Tile>, Without<Held>, Without<Immovable>))>,
	grid_query: Query<(&Transform, (With<Grid>, Without<Tile>, Without<Held>))>,
	tilette_query: Query<(&Parent, &GlobalTransform, (With<Tilette>, Without<Grid>, Without<Tile>, Without<Held>))>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	volume_toggle: Res<VolumeToggle>,
	thread_colliding: Res<ThreadColliding>,
) {
	let window = window_query.get_single().unwrap();
	
	for (tile_entity, children, mut tile_pos, held) in held_query.iter_mut() {
		if let Some(cursor_pos) = window.cursor_position() {
			let cursor_pos = cursor_to_screen(cursor_pos, window);
			tile_pos.translation.x = cursor_pos.x + held.offset.x;
			tile_pos.translation.y = cursor_pos.y + held.offset.y;

			if mouse.just_released(MouseButton::Left) {
				for (grid_pos, _) in grid_query.iter() {
					if (cursor_pos.x - grid_pos.translation.x).abs() < GRID_SIZE.x/2.0 
					&& (cursor_pos.y - grid_pos.translation.y).abs() < GRID_SIZE.y/2.0 
					&& !thread_colliding.0 {
						tile_pos.translation = Vec3::new(
							(tile_pos.translation.x/40.0).round() * 40.0,
							(tile_pos.translation.y/40.0).round() * 40.0,
							tile_pos.translation.z,
						);
						for &child in children.iter() {
							if let Ok((_, tilette_pos, _)) = tilette_query.get(child) {
								if (tilette_pos.translation().x - grid_pos.translation.x).abs() > GRID_SIZE.x/2.0
								|| (tilette_pos.translation().y - grid_pos.translation.y).abs() > GRID_SIZE.y/2.0 {
									tile_pos.translation = held.origin;
								} 
							}
						}
					} else if tile_pos.translation.y > -50.0 - 80.0 || thread_colliding.0 {
						tile_pos.translation = held.origin
					}
				}
				let mut iter = tilette_query.iter_combinations();
				while let Some([
					(_, pos_a, _),
					(_, pos_b, _),
				]) = iter.fetch_next() {
					if pos_a.translation().y > -50.0 - 80.0 && pos_b.translation().y > -50.0 - 80.0 
					&& (pos_a.translation().x - pos_b.translation().x).abs() < CELL_SIZE/1.5
					&& (pos_a.translation().y - pos_b.translation().y).abs() < CELL_SIZE/1.5 {
						tile_pos.translation = held.origin;
					}
				}
				commands.entity(tile_entity).remove::<Held>();
				if volume_toggle.sfx{
					audio.play(asset_server.load("sounds/basic_haptic.ogg")).with_volume(SFX_VOLUME);
				}
			}
		}
	}
	if mouse.just_pressed(MouseButton::Left) {
		if let Some(cursor_pos) = window.cursor_position() {
			let cursor_pos = cursor_to_screen(cursor_pos, window);
			for (parent, tilette_pos, _) in tilette_query.iter() {
				// println!("Cursor: {}", cursor_pos);
				// println!("tilette_pos.translation: {}", tilette_pos.translation().xy());
				if (cursor_pos.x - tilette_pos.translation().x).abs() < H_CELL_SIZE 
				&& (cursor_pos.y - tilette_pos.translation().y).abs() < H_CELL_SIZE {
					if let Ok((tile_entity, tile_pos, _)) = tile_query.get(parent.get()) {
						commands.entity(tile_entity).insert(Held{
							origin: Vec3::new(
								tile_pos.translation.x,
								tile_pos.translation.y,
								tile_pos.translation.z,
							),
							offset: Vec2::new(
								tile_pos.translation.x - cursor_pos.x,
								tile_pos.translation.y - cursor_pos.y,
							),
						});
						break;
					}
				}
			}
		}
	}
}