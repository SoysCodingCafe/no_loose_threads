// Threads module for handling thread placement and collisions
use bevy::{prelude::*, window::PrimaryWindow};

use crate::derivables::*;

pub struct ThreadsPlugin;

impl Plugin for ThreadsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, (
				update_thread_endpoints,
				draw_thread,
				detect_thread_collision,
				recolor_tacks,
			).chain().run_if(in_state(GameState::Game)))
		;
	}
}

fn update_thread_endpoints(
	mut thread_query: Query<(&mut Thread, Without<Loose>)>,
	tack_query: Query<(&GlobalTransform, With<Tack>)>,
) {
	for (mut thread, _) in thread_query.iter_mut() {
		if let Ok((tack_pos, _)) = tack_query.get(thread.tacks[0]) {
			thread.tacks_locs[0] = tack_pos.translation().xy();
		}
		if let Ok((tack_pos, _)) = tack_query.get(thread.tacks[1]) {
			thread.tacks_locs[1] = tack_pos.translation().xy();
		}
	}
}

fn detect_thread_collision(
	mut colliding: ResMut<ThreadColliding>,
	mut highlight_query: Query<(&GlobalTransform, &mut Sprite, With<Highlight>)>,
	thread_query: Query<(&Thread, Without<Highlight>)>,
	window_query: Query<&Window, With<PrimaryWindow>>,
) {
	let mut collision_locs = Vec::new();
	let mut iter = thread_query.iter_combinations();
	while let Some([
		(thread_a, _),
		(thread_b, _),
	]) = iter.fetch_next() {
		if thread_a.group == thread_b.group {continue;};
		let window = window_query.get_single().unwrap();
		if let Some(cursor_pos) = window.cursor_position() {
			let cursor_pos = cursor_to_screen(cursor_pos, window);

			let (a1, b1, c1) = if thread_a.tacks_locs.len() > 1 {
				let a1 = thread_a.tacks_locs[1].y - thread_a.tacks_locs[0].y;
				let b1 = thread_a.tacks_locs[0].x - thread_a.tacks_locs[1].x;
				let c1 = a1 * thread_a.tacks_locs[0].x + b1 * thread_a.tacks_locs[0].y;
				(a1, b1, c1)
			} else {
				let a1 = cursor_pos.y - thread_a.tacks_locs[0].y;
				let b1 = thread_a.tacks_locs[0].x - cursor_pos.x;
				let c1 = a1 * thread_a.tacks_locs[0].x + b1 * thread_a.tacks_locs[0].y;
				(a1, b1, c1)
			};

			let (a2, b2, c2) = if thread_b.tacks_locs.len() > 1 {
				let a2 = thread_b.tacks_locs[1].y - thread_b.tacks_locs[0].y;
				let b2 = thread_b.tacks_locs[0].x - thread_b.tacks_locs[1].x;
				let c2 = a2 * thread_b.tacks_locs[0].x + b2 * thread_b.tacks_locs[0].y;
				(a2, b2, c2)
			} else {
				let a2 = cursor_pos.y - thread_b.tacks_locs[0].y;
				let b2 = thread_b.tacks_locs[0].x - cursor_pos.x;
				let c2 = a2 * thread_b.tacks_locs[0].x + b2 * thread_b.tacks_locs[0].y;
				(a2, b2, c2)
			};

			let delta = a1 * b2 - a2 * b1;

			if delta == 0.0 {
				break;
			}

			let collision_loc = Vec2::new(
				(b2 * c1 - b1 * c2) / delta,
				(a1 * c2 - a2 * c1) / delta,
			);

			let line_a = if thread_a.tacks_locs.len() > 1 {
				(thread_a.tacks_locs[0], thread_a.tacks_locs[1])
			} else {
				(thread_a.tacks_locs[0], cursor_pos)
			};
			let line_b = if thread_b.tacks_locs.len() > 1 {
				(thread_b.tacks_locs[0], thread_b.tacks_locs[1])
			} else {
				(thread_b.tacks_locs[0], cursor_pos)
			};

			let perp_a = (collision_loc - line_a.0).dot(line_a.1 - line_a.0);
			let perp_b = (collision_loc - line_b.0).dot(line_b.1 - line_b.0);
			let mag_a = (line_a.1 - line_a.0).length_squared();
			let mag_b = (line_b.1 - line_b.0).length_squared();

			if 0.0 <= perp_a && perp_a <= mag_a
			&& 0.0 <= perp_b && perp_b <= mag_b {
				collision_locs.push(collision_loc);
			}

		}
	}

	if !collision_locs.is_empty() {
		colliding.0 = true;
		// println!("Total Collisions: {}", collision_locs.len());
		// println!("Colliding!");
		for (transform, mut highlight_sprite, _) in highlight_query.iter_mut() {
			for collision in collision_locs.iter() {
				//println!("Collision at: {}", collision);
				let quant_loc = (*collision/40.0).floor() * 40.0 + 20.0;
				//println!("Quant to: {}", quant_loc);
				//println!("Highlight at: {}", transform.translation().xy());
				if transform.translation().xy() == quant_loc {
					highlight_sprite.color = Color::rgba(1.0, 0.0, 0.0, 0.7);
					break;
				} else {
					highlight_sprite.color = Color::rgba(1.0, 1.0, 1.0, 0.0);
				}
			}
		}

	} else {
		colliding.0 = false;
		for (_, mut highlight_sprite, _) in highlight_query.iter_mut() {
			highlight_sprite.color = Color::rgba(1.0, 1.0, 1.0, 0.0);
		}
	}
}

fn draw_thread(
	mut commands: Commands,
	mut thread_count: ResMut<ThreadCount>,
	mut tack_query: Query<(Entity, &GlobalTransform, &mut Tack)>,
	mut thread_query: Query<(Entity, &mut Transform, &mut Thread, (Without<Loose>, Without<Tack>))>,
	mut loose_thread_query: Query<(Entity, &mut Transform, &mut Thread, (With<Loose>, Without<Tack>))>,
	grid_query: Query<(&Transform, (With<Grid>, Without<Tack>, Without<Thread>, Without<Loose>))>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
	keyboard: Res<Input<KeyCode>>,
	colliding: Res<ThreadColliding>,
) {
	let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
	let window = window_query.get_single().unwrap();
	if let Some(cursor_pos) = window.cursor_position() {
		let cursor_pos = cursor_to_screen(cursor_pos, window);
		// if mouse.just_pressed(MouseButton::Right) {
		// 	for (tack_entity, tack_pos, mut tack) in tack_query.iter_mut() {
		// 		if (cursor_pos.x - tack_pos.translation().x).abs() < CELL_SIZE/2.0
		// 		&& (cursor_pos.y - tack_pos.translation().y).abs() < CELL_SIZE/2.0 {
		// 			if loose_thread_query.is_empty() {
		// 				println!("Thread created!");
		// 				let mut thread = Vec::new();
		// 				thread.push(tack_entity);
		// 				let thread_entity = commands.spawn((
		// 					SpriteBundle {
		// 						transform: Transform::from_xyz(
		// 							tack_pos.translation().x,
		// 							tack_pos.translation().y,
		// 							500.0,
		// 						),
		// 						sprite: Sprite {
		// 							custom_size: Some(Vec2::new(8.0, 1.0)),
		// 							color: Color::rgba(0.8, 0.0, 0.0, 1.0),
		// 							..default()
		// 						},
		// 						..default()
		// 					},
		// 					Thread(thread),
		// 					Loose,
		// 				)).id();
		// 				break;
		// 			} else {
		// 				for (thread_entity, _, mut thread, _) in loose_thread_query.iter_mut() {
		// 					thread.0.push(tack_entity);
		// 					tack.0.push(thread_entity);
		// 					commands.entity(thread_entity).remove::<Loose>();
		// 				}
		// 				break;
		// 			}
		// 		}
		// 	}
		// } else if mouse.just_pressed(MouseButton::Middle) {
		// 	for (_, tack_pos, mut tack) in tack_query.iter_mut() {
		// 		if (cursor_pos.x - tack_pos.translation().x).abs() < CELL_SIZE/2.0
		// 		&& (cursor_pos.y - tack_pos.translation().y).abs() < CELL_SIZE/2.0 {
		// 			while let Some(thread_entity) = tack.0.pop() {
		// 				commands.entity(thread_entity).despawn_recursive();
		// 			}
		// 		}
		// 	}
		// }
		for (grid_pos, _) in grid_query.iter() {
			let on_grid = (cursor_pos.x - grid_pos.translation.x).abs() < GRID_SIZE.x/2.0
				&& (cursor_pos.y - grid_pos.translation.y).abs() < GRID_SIZE.y/2.0;
			if mouse.just_pressed(MouseButton::Right) || mouse.just_released(MouseButton::Right) {
				let mut tack_clicked = false;
				if on_grid {
					for (tack_entity, tack_pos, mut tack) in tack_query.iter_mut() {
						if (cursor_pos.x - tack_pos.translation().x).abs() < CELL_SIZE/2.0
						&& (cursor_pos.y - tack_pos.translation().y).abs() < CELL_SIZE/2.0 {
							// Click on valid tack to create thread
							if loose_thread_query.is_empty() && tack.end && tack.group < 3 && !tack.suspect && mouse.just_pressed(MouseButton::Right) && !shift {
								thread_count.0[tack.group] += 1.0;
								// println!("Thread created! There are now {} threads in group {}", thread_count.0[tack.group], tack.group);
								let mut tacks = Vec::new();
								tacks.push(tack_entity);
								let mut tacks_locs = Vec::new();
								tacks_locs.push(tack_pos.translation().xy());
								let thread_entity = commands.spawn((
									SpriteBundle {transform: Transform::from_xyz(
											tack_pos.translation().x,
											tack_pos.translation().y,
											500.0,),
										sprite: Sprite {custom_size: Some(Vec2::new(6.0, 1.0)),
											color: get_tack_color(tack.group),
											..default()},
										..default()},
									Thread {group: tack.group,
										index: thread_count.0[tack.group],
										tacks: tacks,
										tacks_locs: tacks_locs},
									Loose,
									RemoveOnReset
								)).id();
								tack.end = false;
								tack.used = true;
								tack_clicked = true;
								break;
								
								// Click on valid tack to place thread
							} else if !loose_thread_query.is_empty() && !tack.used && !colliding.0 && tack.group == 3 && !shift {
								// println!("Loose threads");
								for (thread_entity, _, mut thread, _) in loose_thread_query.iter_mut() {
									if tack_entity != thread.tacks[0] {
										tack.group = thread.group;
										thread.tacks.push(tack_entity);
										thread.tacks_locs.push(tack_pos.translation().xy());
										commands.entity(thread_entity).remove::<Loose>();
										tack.end = true;
										tack.used = true;
										tack_clicked = true;
										break;
									} else {
										tack_clicked = true;
									}
								}

								// If not final tack, generate additional thread for chaining
								if mouse.just_pressed(MouseButton::Right) && !tack.suspect {
									thread_count.0[tack.group] += 1.0;
									// println!("Thread created! There are now {} threads in group {}", thread_count.0[tack.group], tack.group);
									let mut tacks = Vec::new();
									tacks.push(tack_entity);
									let mut tacks_locs = Vec::new();
									tacks_locs.push(tack_pos.translation().xy());
									let thread_entity = commands.spawn((
										SpriteBundle {transform: Transform::from_xyz(
												tack_pos.translation().x,
												tack_pos.translation().y,
												500.0,),
											sprite: Sprite {custom_size: Some(Vec2::new(6.0, 1.0)),
												color: get_tack_color(tack.group),
												..default()},
											..default()},
										Thread {group: tack.group,
											index: thread_count.0[tack.group],
											tacks: tacks,
											tacks_locs: tacks_locs},
										Loose,
										RemoveOnReset
									)).id();
									tack.end = false;
									tack.used = true;
									tack_clicked = true;
									break;
								}

								if tack_clicked {
									break;
								}
								// Click on an invalid tack
							} else if !loose_thread_query.is_empty() && tack.used && !shift {
								for (_, _, thread, _) in loose_thread_query.iter_mut() {
									if tack_entity == thread.tacks[0] {
										tack_clicked = true;
									}
								}
								// Keep presses valid for dragging threads
							} else if mouse.just_pressed(MouseButton::Right) && !shift {
								tack_clicked = true;
							}
						}
					}
				}
				// If no tacks clicked then delete thread
				if !tack_clicked {
					for (thread_entity, _, thread, _) in loose_thread_query.iter_mut() {
						if let Ok((_, _, mut tack)) = tack_query.get_mut(thread.tacks[0]) {
							tack.end = true;
							tack.used = false;
						}
						commands.entity(thread_entity).despawn_recursive();
						thread_count.0[thread.group] -= 1.0;
					}
				}
			}
		}

		if shift && mouse.just_pressed(MouseButton::Right) {
			for (thread_entity, _, thread, _) in thread_query.iter_mut() {
				if thread.index == -1.0 {continue;};
				for endpoint in &thread.tacks_locs {
					if (cursor_pos.x - endpoint.x).abs() < CELL_SIZE/2.0
					&& (cursor_pos.y - endpoint.y).abs() < CELL_SIZE/2.0 {
						if thread.index == thread_count.0[thread.group] {
							if let Ok((_, _, mut tack)) = tack_query.get_mut(thread.tacks[1]) {
								tack.end = false;
								tack.used = false;
								tack.group = 3;
							}
							if let Ok((_, _, mut tack)) = tack_query.get_mut(thread.tacks[0]) {
								tack.end = true;
							}
							commands.entity(thread_entity).despawn_recursive();
							thread_count.0[thread.group] -= 1.0;
						}
					}
				}
			}
		}

		for (_, mut thread_pos, thread, _) in loose_thread_query.iter_mut() {
			if let Ok((_, tack_pos_a, _)) = tack_query.get(thread.tacks[0]) {
				thread_pos.translation.x = (tack_pos_a.translation().x + cursor_pos.x)/2.0;
				thread_pos.translation.y = (tack_pos_a.translation().y + cursor_pos.y)/2.0;
				let direction = (cursor_pos - tack_pos_a.translation().xy()).normalize();
				thread_pos.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.0));
				thread_pos.scale.y = cursor_pos.distance(tack_pos_a.translation().xy()); // Divide by sprite height
			}
		}

		for (_, mut thread_pos, thread, _) in thread_query.iter_mut() {
			if let Ok((_, tack_pos_a, _)) = tack_query.get(thread.tacks[0]) {
				if let Ok((_, tack_pos_b, _)) = tack_query.get(thread.tacks[1]) {
					thread_pos.translation.x = (tack_pos_a.translation().x + tack_pos_b.translation().x)/2.0;
					thread_pos.translation.y = (tack_pos_a.translation().y + tack_pos_b.translation().y)/2.0;
					let direction = (tack_pos_b.translation().xy() - tack_pos_a.translation().xy()).normalize();
					thread_pos.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.0));
					thread_pos.scale.y = tack_pos_b.translation().xy().distance(tack_pos_a.translation().xy()); // Divide by sprite height
				}
			}
		}
	}
}

fn recolor_tacks(
	mut tack_query: Query<(&mut Sprite, &Tack)>,
) {
	for (mut sprite, tack) in tack_query.iter_mut() {
		sprite.color = get_tack_color(tack.group);
	}
}