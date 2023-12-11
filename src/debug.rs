// Debugging module, only used for features that should not get compiled into the final game
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;

use crate::{tiles::spawn_tile, derivables::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			//.add_plugins(EditorPlugin::default())
			.add_systems(Update, (
				spawn_random_tile,
				dump_locs,
			).run_if(in_state(GameState::Game)))
		;
	}
}

fn spawn_random_tile(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	keyboard: Res<Input<KeyCode>>,
	mut waste_count: Local<i32>,
	tack_query: Query<(Entity, &GlobalTransform, &Tack)>,
) {
	if keyboard.just_pressed(KeyCode::T) {
		let tile_types = 
		[TileType::I,
		TileType::O,
		TileType::T,
		TileType::S,
		TileType::Z,
		TileType::L,
		TileType::J,
		];
		let tile_type = &tile_types[rand::Rng::gen_range(&mut rand::thread_rng(), 0..tile_types.len())];
		let tack_tile = rand::Rng::gen_range(&mut rand::thread_rng(), 0..tile_type.get_layout().len());

		let loc = Vec3::new(-200.0, -200.0, 300.0);

		let tile = spawn_tile(&mut commands, loc, *tile_type, &asset_server, 3, false, -1, false, tack_tile);
	}
	if keyboard.just_pressed(KeyCode::W) {
		*waste_count += 1;
		if keyboard.pressed(KeyCode::E) {
			*waste_count += 1;
		}

		let tile_type = TileType::W(rand::Rng::gen_range(&mut rand::thread_rng(), 1..4));
		let tack_tile = rand::Rng::gen_range(&mut rand::thread_rng(), 0..tile_type.get_layout().len());

		let loc = Vec3::new(-200.0, -200.0, 300.0);

		let tile = spawn_tile(&mut commands, loc, tile_type, &asset_server, 4, false, *waste_count, false, tack_tile);
	}
	if keyboard.just_pressed(KeyCode::C) {
		let mut iter = tack_query.iter_combinations();
		while let Some([
			(tack_a_entity, tack_a_pos, tack_a),
			(tack_b_entity, tack_b_pos, tack_b),
		]) = iter.fetch_next() {
			if tack_a.links == -1 || tack_b.links == -1 {continue;};
			let tacks = [tack_a_entity, tack_b_entity].to_vec();
			let tacks_locs = [tack_a_pos.translation().xy(), tack_b_pos.translation().xy()].to_vec();
			if (tack_a.links-tack_b.links).abs() == 1 {
				let thread_entity = commands.spawn((
					SpriteBundle {
						transform: Transform::from_xyz(
							(tack_a_pos.translation().x + tack_b_pos.translation().x)/2.0,
							(tack_a_pos.translation().y + tack_b_pos.translation().y)/2.0,
							500.0,
						),
						sprite: Sprite {
							custom_size: Some(Vec2::new(6.0, 1.0)),
							color: get_tack_color(4),
							..default()
						},
						..default()
					},
					Thread {
						group: 4,
						index: -1.0,
						tacks: tacks,
						tacks_locs: tacks_locs,
					},
					RemoveOnReset,
				)).id();
			}
		}
	}
}

fn dump_locs(
	keyboard: Res<Input<KeyCode>>,
	tile_query: Query<(&Transform, &Tile)>,
	grid_query: Query<(&Transform, With<Grid>)>,
) {
	if keyboard.just_pressed(KeyCode::D) {
		for (tile_pos, tile) in tile_query.iter() {
			for (grid_pos, _) in grid_query.iter() {
				if (tile_pos.translation.y - grid_pos.translation.x).abs() < GRID_SIZE.x/2.0 
				&& (tile_pos.translation.y - grid_pos.translation.y).abs() < GRID_SIZE.y/2.0 {
					println!("{:?} at {}", tile.0, tile_pos.translation);
					println!("Coords: {:?}", grid_to_index([tile_pos.translation].to_vec()));
				}
			}
		}
	}
}

