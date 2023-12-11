// Casefiles module for interaction and displaying of casefiles
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};

use crate::derivables::*;

pub struct CasefilesPlugin;

impl Plugin for CasefilesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::Game), (
				spawn_casefiles,
			))
			.add_systems(Update, (
				mouse_hover,
			).run_if(in_state(GameState::Game)))
		;
	}
}

fn spawn_casefiles(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	for i in 0..3 {
		let case = format!("sprites/casefiles/casefile_{}.png", i);
		let opened_case = format!("sprites/casefiles/open_casefile_{}.png", i);
		let case_x = i as f32 * 180.0 + 360.0;
		commands.spawn((SpriteBundle{
			transform: Transform::from_xyz(case_x, -240.0, 50.0),
			texture: asset_server.load(case),
			sprite: Sprite {
				custom_size: Some(Vec2::new(220.0, 220.0)), // 220 x 165 collider
				..default()
			},
			..default()
			},
			CaseReport {number: i, sfx: false},
		));
		commands.spawn((SpriteBundle{
			transform: Transform::from_xyz(-200.0, 50.0, -1.0),
			texture: asset_server.load(opened_case),
			sprite: Sprite {
				custom_size: Some(Vec2::new(536.0, 608.0)),
				..default()
			},
			..default()
			},
			OpenCaseReport {number: i},
		)).with_children(|parent| {
			parent.spawn((Text2dBundle{
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					350.0 - 5.0 * 2.0,
					300.0 - 5.0 * 2.0,
				)},
				transform: Transform::from_xyz(50.0 - 350.0/2.0, -90.0 + 300.0/2.0, 0.5),
				text_anchor: bevy::sprite::Anchor::TopLeft,
				text: Text::from_sections([
					TextSection::new(
					format!("      Case {}\n", i+1),
					TextStyle {
						font: asset_server.load("fonts/XTypewriterBold.ttf"),
						font_size: 32.0,
						color: Color::rgb(0.1, 0.1, 0.1),
					}),
					TextSection::new(
					get_case_report(0, i),
					TextStyle {
						font: asset_server.load("fonts/XTypewriterBold.ttf"),
						font_size: 28.0,
						color: Color::rgb(0.1, 0.1, 0.1),
					}),
				]).with_alignment(TextAlignment::Left),
				..default()
				},
				CaseFileText(i),
			));
		});
	}
}

fn mouse_hover(
	mut case_report_query: Query<(&mut Transform, &mut CaseReport)>,
	mut open_case_report_query: Query<(&mut Transform, &OpenCaseReport, Without<CaseReport>)>,
	mut fail_text_query: Query<(&mut Transform, (With<FailText>, Without<OpenCaseReport>, Without<CaseReport>))>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	volume_toggle: Res<VolumeToggle>,
) {
	let window = window_query.get_single().unwrap();

	for (mut report_transform, mut case_report) in case_report_query.iter_mut() {
		if let Some(cursor_pos) = window.cursor_position() {
			let cursor_pos = cursor_to_screen(cursor_pos, window);
			if ((report_transform.translation.x - 27.5) - cursor_pos.x).abs() <= 82.5 && (report_transform.translation.y - cursor_pos.y).abs() <= 110.0 {
				for (mut fail_pos, _) in fail_text_query.iter_mut() {
					fail_pos.translation.z = -10.0;
				}
				for (mut open_transform, open_case_report, _) in open_case_report_query.iter_mut(){
					if case_report.number == open_case_report.number {
						if case_report.sfx == false {
							if volume_toggle.sfx{
								audio.play(asset_server.load("sounds/rustle.ogg")).with_volume(SFX_VOLUME);
							}
							case_report.sfx = true;
						}
						report_transform.translation.z = -1.0;
						open_transform.translation.z = 900.0;
					}
				}
			} else {
				for (mut open_transform, open_case_report, _) in open_case_report_query.iter_mut(){
					if case_report.number == open_case_report.number {
						case_report.sfx = false;
						report_transform.translation.z = 50.0;
						open_transform.translation.z = -1.0;
					}
				}
			}
		};
	}
}