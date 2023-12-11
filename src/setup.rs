use std::time::Duration;

// Setup module, used for initial game setup and initialising resources
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_kira_audio::prelude::*;

use crate::derivables::*;
use crate::post_processing::PostProcessSettings;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_state::<GameState>()
			.add_event::<WaitForJunkEvent>()
			.add_event::<SolveCaseEvent>()
			.add_event::<LevelSelectedEvent>()
			.insert_resource(ThreadCount(Vec3::ZERO))
			.insert_resource(ThreadColliding(false))
			.insert_resource(Level(0))
			.insert_resource(SplashCount(0))
			.insert_resource(VolumeToggle{bgm: true, sfx: true})
			.add_systems(Startup, (
				setup,
			))
			.add_systems(Update, (
				progress_splash_screens,
			).run_if(in_state(GameState::Boot)))
			.add_systems(OnEnter(GameState::Game), (
				setup_game,
			))
		;
	}
}

fn setup(
	mut commands: Commands, 
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
) {
	// Spawn camera
	commands.spawn((Camera2dBundle{
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
		projection: OrthographicProjection {
			scaling_mode: ScalingMode::Fixed{width: VIEW_SIZE.x, height: VIEW_SIZE.y},
			..default()
		},
		..default()
		},
		PostProcessSettings {
            intensity: 0.0,
            ..default()
        },
	));

	// Play BGM
	let bgm_handle = audio.play(asset_server.load("sounds/closing_the_loop.ogg")).looped().with_volume(BGM_VOLUME).handle();
	commands.insert_resource(MusicHandle(bgm_handle));

	// Spawn hidden loading text
	commands.spawn((Text2dBundle{
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		text_anchor: bevy::sprite::Anchor::Center,
		text: Text::from_section(
			"Loading...".to_string(),
			TextStyle {
				font: asset_server.load("fonts/XTypewriterBold.ttf"),
				font_size: 128.0,
				color: Color::rgb(0.9, 0.9, 0.9),
			}).with_alignment(TextAlignment::Center),
		..default()
		},
	));

	// Spawn splash screen
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(0.0, 0.0, 100.0),
		texture: asset_server.load("sprites/splash_image.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(VIEW_SIZE.x, VIEW_SIZE.y)),
			..default()
		},
		..default()
		},
		Splash(0),
	));

	// Spawn prologue screen
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(0.0, 0.0, 50.0),
		texture: asset_server.load("sprites/prologue.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(VIEW_SIZE.x, VIEW_SIZE.y)),
			..default()
		},
		..default()
		},
		Splash(1),
	));


	// Spawn click to start text
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(0.0, 0.0, 200.0),
		sprite: Sprite {
				custom_size: Some(Vec2::new(530.0, 64.0)),
				color: Color::rgba(0.0, 0.0, 0.0, 0.8),
			..default()
		},
		..default()
		},
		Splash(0),
	));
	commands.spawn((Text2dBundle{
		transform: Transform::from_xyz(0.0, 0.0, 210.0),
		text_anchor: bevy::sprite::Anchor::Center,
		text: Text::from_section(
			"Click to Start!".to_string(),
			TextStyle {
				font: asset_server.load("fonts/XTypewriter.ttf"),
				font_size: 64.0,
				color: Color:: rgb(0.9, 0.9, 0.9),
			}).with_alignment(TextAlignment::Center),
		..default()
		},
		Splash(0),
	));

	let pos = Vec2::new(-275.0, 15.0);
	let size = Vec2::new(900.0, 850.0);
	// Spawn prologue text box
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(pos.x, pos.y, 50.0),
		sprite: Sprite {
			custom_size: Some(size),
			color: Color::rgba(0.0, 0.0, 0.0, 0.99),
			..default()
		},
		..default()
		},
		Splash(1),
	)).with_children(|parent| {
		// Spawn prologue text
		parent.spawn((Text2dBundle{
			text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
				size.x - 5.0 * 2.0,
				size.y - 5.0 * 2.0,
			)},
			transform: Transform::from_xyz(-size.x/2.0, size.y/2.0, 5.0),
			text_anchor: bevy::sprite::Anchor::TopLeft,
			text: Text::from_sections([
				TextSection::new(
				format!("Act 1: Threads of Conflict\n\n"),
				TextStyle {
					font: asset_server.load("fonts/XTypewriterBold.ttf"),
					font_size: 48.0,
					color: Color::rgb(0.9, 0.7, 0.7),
				}),
				TextSection::new(
				format!("It's tough times in Crime City, and to make ends meet you've had to take on jobs from \
				a LOT of legal entities. But that means a lot of evidence, and in a bid to save the enviroment they've \
				shipped it all to your office in a single box.\n\nTypical.\n\nTime to get the evidence board out and see if \
				you	 can connect the evidence to the right crimes and piece together a narrative that will sway the courts. \
				\n\nSometimes you wonder if it's just the evidence that's messed up, or if it's this whole darn city.\n\n\n(Click to Begin...)"),
				TextStyle {
					font: asset_server.load("fonts/XTypewriterBold.ttf"),
					font_size: 36.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				}),
			]).with_alignment(TextAlignment::Left),
			..default()
			},
		));
	});

}

fn progress_splash_screens(
	mut commands: Commands,
	mut splash_count: ResMut<SplashCount>,
	mut next_state: ResMut<NextState<GameState>>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
	music_handle: ResMut<MusicHandle>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	splash_query: Query<(Entity, &Splash)>,
	mouse: Res<Input<MouseButton>>,
) {
	if mouse.just_pressed(MouseButton::Left) {
		splash_count.0 += 1;
	}
	for (entity, splash) in splash_query.iter() {
		if splash.0 < splash_count.0 {
			commands.entity(entity).despawn_recursive();
		}
	}
	if splash_query.is_empty() {
		next_state.set(GameState::Game);
		if let Some(instance) = audio_instances.get_mut(&music_handle.0) {
			instance.stop(AudioTween::linear(Duration::from_millis(500)));
			let bgm_handle = audio.play(asset_server.load("sounds/picking_up_the_pieces.ogg"))
				.looped().with_volume(BGM_VOLUME).fade_in(AudioTween::linear(Duration::from_millis(500))).handle();
			commands.insert_resource(MusicHandle(bgm_handle));
		}
	}

}

struct UIInfo {
	loc: Vec3,
	size: Vec2,
	path: String,
	function: usize,
}

fn setup_game(
	mut commands: Commands, 
	mut ev_r_level: EventWriter<LevelSelectedEvent>,
	asset_server: Res<AssetServer>,
) {
	// Spawn background
	commands.spawn(SpriteBundle{
		transform: Transform::from_xyz(0.0, 0.0, 1.0),
		texture: asset_server.load("sprites/background.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(VIEW_SIZE.x, VIEW_SIZE.y)),
			..default()
		},
		..default()
	});

	// Spawn grid
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(-180.0, 200.0, 100.0),
		sprite: Sprite {
			custom_size: Some(GRID_SIZE),
			color: Color::rgba(0.0, 0.0, 0.0, 0.0),
			..default()
		},
		..default()
		}, 
		Grid,
	)).with_children(|parent| {
		for x in 0..GRID_CELLS.x as usize {
			for y in 0..GRID_CELLS.y as usize {
				parent.spawn((
					SpriteBundle {
						transform: Transform::from_xyz(
							x as f32 * CELL_SIZE - GRID_SIZE.x / 2.0 + CELL_SIZE / 2.0, 
							y as f32 * CELL_SIZE - GRID_SIZE.y / 2.0 + CELL_SIZE / 2.0,
							800.0),
						sprite: Sprite {
							custom_size: Some(Vec2::splat(CELL_SIZE-1.0)),
							color: Color::rgba(0.2, 0.2, 0.2, 0.0),
							..default()
						},
						..default()
					},
					Highlight,
				));
			}
		}
	})
	;

	let position = Vec2::new(-200.0, 50.0);
	let size = Vec2::new(550.0, 350.0);
	let margin = 15.0;
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(position.x, position.y, -10.0),
		sprite: Sprite {
			custom_size: Some(size),
			color: Color::rgba(0.0, 0.0, 0.0, 0.99),
			..default()
		},
		..default()
		},
		SolveText,
	)).with_children(|parent| {
		parent.spawn((Text2dBundle{
			text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
				size.x - margin * 2.0,
				size.y - margin * 2.0,
			)},
			transform: Transform::from_xyz(-size.x/2.0 + margin, size.y/2.0 - 5.0, 5.0),
			text_anchor: bevy::sprite::Anchor::TopLeft,
			text: Text::from_sections([
				TextSection::new(
				format!("Case Solved!"),
				TextStyle {
					font: asset_server.load("fonts/XTypewriterBold.ttf"),
					font_size: 64.0,
					color: Color::rgb(0.7, 0.9, 0.7),
				}),
				TextSection::new(
				format!("\nAfter presenting the connections between the evidence, the three suspects were found guilty! \
					Now to move onto the next case. Use the navigation arrows at the top right to move on to the next level!"),
				TextStyle {
					font: asset_server.load("fonts/XTypewriter.ttf"),
					font_size: 32.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				}),
			]).with_alignment(TextAlignment::Left),
			..default()
			},
		));
	});

	let position = Vec2::new(-200.0, 50.0);
	let size = Vec2::new(550.0, 250.0);
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(position.x, position.y, -10.0),
		sprite: Sprite {
			custom_size: Some(size),
			color: Color::rgba(0.0, 0.0, 0.0, 0.99),
			..default()
		},
		..default()
		},
		FailText,
	)).with_children(|parent| {
		parent.spawn((Text2dBundle{
			text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
				size.x - margin * 2.0,
				size.y - margin * 2.0,
			)},
			transform: Transform::from_xyz(-size.x/2.0 + margin, size.y/2.0 - 5.0, 5.0),
			text_anchor: bevy::sprite::Anchor::TopLeft,
			text: Text::from_sections([
				TextSection::new(
				format!("Mistrial!"),
				TextStyle {
					font: asset_server.load("fonts/XTypewriterBold.ttf"),
					font_size: 64.0,
					color: Color::rgb(0.9, 0.7, 0.7),
				}),
				TextSection::new(
				format!("\nThe victims aren't linked to the correct suspects, or with the right evidence! Check the case files by \
				hovering your cursor over them and try again!"),
				TextStyle {
					font: asset_server.load("fonts/XTypewriter.ttf"),
					font_size: 32.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				}),
			]).with_alignment(TextAlignment::Left),
			..default()
			},
		));
	});

	// // UI Buttons
	// let y = [390.0, 280.0, 170.0, 30.0];
	// for i in 0..4 {
	// 	commands.spawn(SpriteBundle{
	// 		transform: Transform::from_xyz(610.0, y[i], 100.0),
	// 		sprite: Sprite {
	// 			custom_size: Some(Vec2::new(340.0, 100.0)),
	// 			color: Color::rgba(0.0, 0.0, 0.0, 0.6),
	// 			..default()
	// 		},
	// 		..default()
	// 	});
	// }

	let mut buttons = Vec::new();
	let locs = [
		Vec3::new(506.5, 375.0, 55.0), Vec3::new(610.0, 375.0, 50.0), Vec3::new(713.5, 375.0, 55.0),
		Vec3::new(515.0, 265.0, 50.0), Vec3::new(705.0, 265.0, 50.0), Vec3::new(610.0, 155.0, 50.0),
		Vec3::new(610.0, 45.0, 50.0), Vec3::new((515.0 + 705.0)/2.0, 265.0, 50.0)
	];

	let sizes = [
		Vec2::new(105.0, 112.0), Vec2::new(102.0, 112.0), Vec2::new(105.0, 112.0),
		Vec2::new(80.0, 80.0), Vec2::new(80.0, 80.0), Vec2::new(312.0, 104.0),
		Vec2::new(312.0, 104.0), Vec2::new(80.0, 80.0),
	];

	let paths = [
		"sprites/UI/previous.png".to_string(), "sprites/UI/level_number.png".to_string(), 
		"sprites/UI/next.png".to_string(), "sprites/UI/music.png".to_string(), 
		"sprites/UI/sfx.png".to_string(), "sprites/UI/restart.png".to_string(), 
		"sprites/UI/solve.png".to_string(), "sprites/UI/help.png".to_string(),
	];

	for i in 0..8 {
		buttons.push(UIInfo {
			loc: locs[i],
			size: sizes[i],
			path: paths[i].clone(),
			function: i,
		})
	}
	
	for button in buttons {
		commands.spawn((SpriteBundle{
			transform: Transform::from_xyz(button.loc.x, button.loc.y, button.loc.z),
			texture: asset_server.load(button.path),
			sprite: Sprite {
				custom_size: Some(Vec2::new(button.size.x, button.size.y)),
				..default()
			},
			..default()
		},
		UIButton {
			size: Vec2::new(button.size.x, button.size.y),
			function: button.function,
		},
		));
	}


	commands.spawn((Text2dBundle{
		transform: Transform::from_xyz(610.0, 375.0, 60.0),
		text_anchor: bevy::sprite::Anchor::Center,
		text: Text::from_section(
			"1".to_string(),
			TextStyle {
				font: asset_server.load("fonts/XTypewriterBold.ttf"),
				font_size: 96.0,
				color: Color::rgb(0.9, 0.9, 0.9),
			}).with_alignment(TextAlignment::Center),
		..default()
		},
		LevelText,
	));

	let positions = [
		Vec2::new(-450.0, 160.0), Vec2::new(100.0, 300.0), 
		Vec2::new(-350.0, -280.0), Vec2::new(540.0, -240.0),
		Vec2::new(160.0, 20.0),
	];
	let sizes = [
		Vec2::new(500.0, 280.0), Vec2::new(300.0, 140.0),
		Vec2::new(400.0, 140.0), Vec2::new(300.0, 170.0),
		Vec2::new(440.0, 200.0),
	];
	let hints = [
		"This is the evidence board! Use right click, or right click and drag, to draw threads between tacks. Connect threads from the victims \
		to the suspects through the correct evidence.\n\nYou can hold shift and right click to unravel threads from the end. Don't tangle the threads!".to_string(),
		"Victims, suspects, and other notes from the legal entities sharing this board can't be moved.".to_string(),
		"This is the workbench where you've dumped the evidence. Left click and hold to drag tiles to and from the evidence board above.".to_string(),
		"These are the case files for the crimes. Use them to work out which evidence goes with which crime!".to_string(),
		"Once you think you have everything nicely tied up then press the Solve! button on the right to check! You can also freely navigate between levels \
		using the buttons at the top right.".to_string(),
	];

	for i in 0..hints.len() {
		generate_hint_textbox(&mut commands, &asset_server, positions[i], sizes[i], hints[i].clone());
	}
	
	ev_r_level.send(LevelSelectedEvent{level: 0});
}

fn generate_hint_textbox(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	position: Vec2,
	size: Vec2,
	hint: String,
) -> Entity {
	let margin = 10.0;
	commands.spawn((SpriteBundle{
		transform: Transform::from_xyz(position.x, position.y, -10.0),
		sprite: Sprite {
			custom_size: Some(size),
			color: Color::rgba(0.0, 0.0, 0.0, 0.99),
			..default()
		},
		..default()
		},
		HintText,
	)).with_children(|parent| {
		parent.spawn((Text2dBundle{
			text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
				size.x - margin * 2.0,
				size.y - margin * 2.0,
			)},
			transform: Transform::from_xyz(-size.x/2.0 + margin, size.y/2.0, 5.0),
			text_anchor: bevy::sprite::Anchor::TopLeft,
			text: Text::from_sections([
				// TextSection::new(
				// format!("Hint Title\n\n"),
				// TextStyle {
				// 	font: asset_server.load("fonts/XTypewriterBold.ttf"),
				// 	font_size: 48.0,
				// 	color: Color::rgb(0.9, 0.7, 0.7),
				// }),
				TextSection::new(
					hint,
				TextStyle {
					font: asset_server.load("fonts/XTypewriter.ttf"),
					font_size: 26.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				}),
			]).with_alignment(TextAlignment::Left),
			..default()
			},
		));
	}).id()
}