use std::time::Duration;

// Buttons module for handling UI interaction
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{AudioInstance, PlaybackState, AudioTween, Audio, AudioControl};

use crate::{derivables::*, tiles::spawn_tile};

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
	fn build(&self, app: &mut App) {
		app
            .add_systems(Update, (
                handle_button_interaction,
                string_trash.before(load_level),
				load_level,
                solve_case,
				update_music,
            ).run_if(in_state(GameState::Game)))
		;
	}
}

fn handle_button_interaction(
    mut button_query: Query<(&Transform, &mut Sprite, &UIButton)>,
	mut hint_text_query: Query<(&mut Transform, (With<HintText>, Without<UIButton>))>,
	mut solve_text_query: Query<(&mut Transform, (With<SolveText>, Without<HintText>, Without<UIButton>))>,
    mut volume_toggle: ResMut<VolumeToggle>,
    mut ev_w_level: EventWriter<LevelSelectedEvent>,
    mut ev_w_solve: EventWriter<SolveCaseEvent>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
    level: Res<Level>,
    window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
) {
	for (mut hintbox_pos, _) in hint_text_query.iter_mut() {
		hintbox_pos.translation.z = -10.0;
	}
    let window = window_query.get_single().unwrap();
	if let Some(cursor_pos) = window.cursor_position() {
		let cursor_pos = cursor_to_screen(cursor_pos, window);
        for (button_pos, mut button_sprite, button) in button_query.iter_mut() {
            if (button_pos.translation.x - cursor_pos.x).abs() < button.size.x / 2.0
            && (button_pos.translation.y - cursor_pos.y).abs() < button.size.y / 2.0 {
				if button.function != 1 {
                	button_sprite.color = Color::rgb(1.0, 1.0, 0.4);
				}
				if button.function == 0 || button.function == 2 {
					for (mut solve_pos, _) in solve_text_query.iter_mut() {
						solve_pos.translation.z = -10.0;
					}
				}
				if button.function == 7 {
					for (mut hintbox_pos, _) in hint_text_query.iter_mut() {
						hintbox_pos.translation.z = 900.0;
					}
				}
                if mouse.just_pressed(MouseButton::Left) {
                    match button.function {
                        // Prev, Level, Next, Music, Sfx, Restart, Solve
                        0 => ev_w_level.send(LevelSelectedEvent{level: if level.0>0{level.0-1}else{0}}),
                        1 => {}, //ev_w_level.send(LevelSelectedEvent{level: level.0}),
                        2 => ev_w_level.send(LevelSelectedEvent{level: if level.0<2{level.0+1}else{2}}),
                        3 => volume_toggle.bgm = !volume_toggle.bgm,
                        4 => volume_toggle.sfx = !volume_toggle.sfx,
                        5 => ev_w_level.send(LevelSelectedEvent{level: level.0}),
                        6 => ev_w_solve.send(SolveCaseEvent()),
                        _ => {},
                    }
					if volume_toggle.sfx && button.function != 1 && button.function != 7 {
						audio.play(asset_server.load("sounds/basic_haptic.ogg")).with_volume(SFX_VOLUME);
					}
                }
            } else {
				if (button.function == 3 && !volume_toggle.bgm)
				|| (button.function == 4 && !volume_toggle.sfx) {
					button_sprite.color = Color::rgb(1.0, 0.4, 0.4)
				} else {
					button_sprite.color = Color::rgb(1.0, 1.0, 1.0);
				};
            }
        }
    }

}

struct TileInfo {
	location: Vec3,
	tile_type: TileType,
	group: usize,
	origin: bool,
	tack_tile: usize,
	links: i32,
	suspect: bool,
}

fn load_level(
	mut commands: Commands,
	mut level: ResMut<Level>,
	mut ev_r_level: EventReader<LevelSelectedEvent>,
	mut ev_w_wait: EventWriter<WaitForJunkEvent>,
	asset_server: Res<AssetServer>,
	// keyboard: Res<Input<KeyCode>>,
	remove_on_reset: Query<(Entity, &RemoveOnReset)>,
	mut level_text_query: Query<(&mut Text, With<LevelText>)>,
	mut case_files_text_query: Query<(&mut Text, &CaseFileText, Without<LevelText>)>,
) {
	// let mut next_level = 999;
	// for key in keyboard.get_just_pressed() {
	// 	match key {
	// 		KeyCode::Key0 => next_level = 0,
	// 		KeyCode::Key1 => next_level = 1,
	// 		KeyCode::Key2 => next_level = 2,
	// 		KeyCode::Key3 => next_level = 3,
	// 		_ => (),
	// 	}
	// }
	for ev in ev_r_level.read() {
		level.0 = ev.level;
		for (entity, _) in remove_on_reset.iter() {
			commands.entity(entity).despawn_recursive();
		}

		let mut tile_infos = Vec::new();
		
		for i in 0..get_level_layout(level.0).len() {
			tile_infos.push(TileInfo {
				location: get_level_layout(level.0)[i],
				tile_type: get_tile_types(level.0)[i],
				group: get_tile_group(level.0)[i],
				origin: match get_tile_types(level.0)[i]{TileType::V(_) => true, _=> false},
				tack_tile: get_tack_tilette(level.0)[i],
				links: get_junk_links(level.0)[i],
				suspect: match get_tile_types(level.0)[i]{TileType::C(_) => true, _=> false},
			});
		}
		for tile_info in tile_infos.iter() {
			let tile = spawn_tile(&mut commands, tile_info.location, tile_info.tile_type, &asset_server,
				tile_info.group, tile_info.origin, tile_info.links, tile_info.suspect, tile_info.tack_tile);
			match tile_info.tile_type {
				TileType::C(_) | TileType::V(_) | TileType::W(_) => {commands.entity(tile).insert(Immovable);},
				_ => (),
			}
		}

		for (mut text, _) in level_text_query.iter_mut() {
			text.sections[0].value = format!("{}", ev.level + 1);
		}
		for (mut text, case, _) in case_files_text_query.iter_mut() {
			text.sections[1].value = get_case_report(ev.level, case.0);
		}

		ev_w_wait.send(WaitForJunkEvent());
	}
}


fn string_trash(
	mut commands: Commands,
	mut ev_r_wait: EventReader<WaitForJunkEvent>,
	tack_query: Query<(Entity, &GlobalTransform, &Tack)>,
) {
	for _ in ev_r_wait.read() {
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

fn solve_case(
	mut solve_text_query: Query<(&mut Transform, With<SolveText>)>,
	mut fail_text_query: Query<(&mut Transform, (With<FailText>, Without<SolveText>))>,
	mut ev_r_solve: EventReader<SolveCaseEvent>,
	tack_query: Query<&Tack>,
	level: Res<Level>,
) {
	for _ in ev_r_solve.read() {
		let mut evidence =[Vec::new(), Vec::new(), Vec::new()];
		let required_evidence = [
			[[TileType::C(0), TileType::I, TileType::O].to_vec(), [TileType::C(1), TileType::J].to_vec(), [TileType::C(2), TileType::L].to_vec()],
			[[TileType::C(0), TileType::L, TileType::S].to_vec(), [TileType::C(1), TileType::T].to_vec(), [TileType::C(2), TileType::Z].to_vec()],
			[[TileType::C(0), TileType::L, TileType::J].to_vec(), [TileType::C(1), TileType::B, TileType::O].to_vec(), [TileType::C(2), TileType::T].to_vec()],
		];

		for tack in tack_query.iter() {
			if tack.group < 3 {
				evidence[tack.group].push(tack.tile_type);
			}
		}
		let mut solved = [true, true, true];
		for case in 0..3{
			for required in &required_evidence[level.0][case] {
				if !evidence[case].contains(required) {
					solved[case] = false;
					continue;
				}
			}
		}
		if solved[0] && solved[1] && solved[2] {
			for (mut solve_pos, _) in solve_text_query.iter_mut() {
				solve_pos.translation.z = 990.0;
			}
		} else {
			for (mut fail_pos, _) in fail_text_query.iter_mut() {
				fail_pos.translation.z = 980.0;
			}
			// println!("Hmm, this evidence doesn't seem to add up...");
			// if !solved[0] {println!("Case 1 doesn't seem to have enough evidence connecting the suspect to the victim...")};
			// if !solved[1] {println!("Case 2 doesn't seem to have enough evidence connecting the suspect to the victim...")};
			// if !solved[2] {println!("Case 3 doesn't seem to have enough evidence connecting the suspect to the victim...")};
		}
	}
}

fn update_music(
	mut audio_instances: ResMut<Assets<AudioInstance>>,
	music_handle: ResMut<MusicHandle>,
	volume_toggle: Res<VolumeToggle>,
) {
	if let Some(instance) = audio_instances.get_mut(&music_handle.0) {
		match instance.state() {
			PlaybackState::Paused{ .. } => {
				if volume_toggle.bgm {
					instance.resume(AudioTween::linear(Duration::from_millis(500)));
				}
			}
			PlaybackState::Playing{ .. } => {
				if !volume_toggle.bgm {
					instance.pause(AudioTween::linear(Duration::from_millis(500)));
				}
			}
			_ => {}
		}
	}
}