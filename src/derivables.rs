// Import Bevy game engine essentials
use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;

// CONTENTS
// - Constants
// - States
// - Resources
// - Events
// - Components
// - Helper Functions

// CONSTANTS
pub const VIEW_SIZE: Vec2 = Vec2::new(1600.0, 900.0);

pub const SFX_VOLUME: f64 = 1.0;
pub const BGM_VOLUME: f64 = 1.0;

pub const GRID_SIZE: Vec2 = Vec2::new(1160.0, 480.0);
pub const GRID_CELLS: Vec2 = Vec2::new(GRID_SIZE.x/40.0, GRID_SIZE.y/40.0);
pub const CELL_SIZE: f32 = GRID_SIZE.x/GRID_CELLS.x;
pub const H_CELL_SIZE: f32 = CELL_SIZE/2.0;

pub const TILE_OFFSETS: [[Vec2; 4]; 4] = [
	[Vec2::new(-3.0*H_CELL_SIZE, 3.0*H_CELL_SIZE), Vec2::new(-3.0*H_CELL_SIZE, 1.0*H_CELL_SIZE), 
	Vec2::new(-3.0*H_CELL_SIZE, -1.0*H_CELL_SIZE), Vec2::new(-3.0*H_CELL_SIZE, -3.0*H_CELL_SIZE)],
	[Vec2::new(-1.0*H_CELL_SIZE, 3.0*H_CELL_SIZE), Vec2::new(-1.0*H_CELL_SIZE, 1.0*H_CELL_SIZE), 
	Vec2::new(-1.0*H_CELL_SIZE, -1.0*H_CELL_SIZE), Vec2::new(-1.0*H_CELL_SIZE, -3.0*H_CELL_SIZE)],
	[Vec2::new(1.0*H_CELL_SIZE, 3.0*H_CELL_SIZE), Vec2::new(1.0*H_CELL_SIZE, 1.0*H_CELL_SIZE), 
	Vec2::new(1.0*H_CELL_SIZE, -1.0*H_CELL_SIZE), Vec2::new(1.0*H_CELL_SIZE, -3.0*H_CELL_SIZE)],
	[Vec2::new(3.0*H_CELL_SIZE, 3.0*H_CELL_SIZE), Vec2::new(3.0*H_CELL_SIZE, 1.0*H_CELL_SIZE), 
	Vec2::new(3.0*H_CELL_SIZE, -1.0*H_CELL_SIZE), Vec2::new(3.0*H_CELL_SIZE, -3.0*H_CELL_SIZE)],
];

pub const NUM_1_JUNK: usize = 6;
pub const NUM_2_JUNK: usize = 6;
pub const NUM_3_JUNK: usize = 3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
	B,
	I,
	O,
	T,
	S,
	Z,
	L,
	J,
	V(usize),
	C(usize),
	W(usize),
}

impl TileType {
	pub fn get_layout(&self) -> Vec<(usize, usize)> {
		match *self {
			TileType::B => [(0,0), (1,0), (2,0), (3,0)].to_vec(),
			TileType::I => [(0,0), (0,1), (0,2), (0,3)].to_vec(),
			TileType::O | TileType::C(_) | TileType::W(2) => [(0,0), (1,0), (0,1), (1,1)].to_vec(),
			TileType::T => [(0,0), (1,0), (2,0), (1,1)].to_vec(),
			TileType::S => [(1,0), (2,0), (0,1), (1,1)].to_vec(),
			TileType::Z => [(0,0), (1,0), (1,1), (2,1)].to_vec(),
			TileType::L => [(0,0), (0,1), (0,2), (1,2)].to_vec(),
			TileType::J => [(1,0), (1,1), (1,2), (0,2)].to_vec(),
			TileType::V(_) => [(0,0), (1,0), (2,0), (0,1), (1,1), (2,1)].to_vec(),
			TileType::W(1) => [(0,0)].to_vec(),
			TileType::W(_) => [(0,0), (1,0), (2,0), (0,1), (1,1), (2,1), (0,2), (1,2), (2,2)].to_vec(),
		}
	}

	pub fn get_path(&self) -> String {
		match *self {
			TileType::B => "sprites/tiles/crowbar.png".to_string(),
			TileType::I => "sprites/tiles/swab.png".to_string(),
			TileType::O => "sprites/tiles/print.png".to_string(),
			TileType::T => "sprites/tiles/cctv.png".to_string(),
			TileType::S => "sprites/tiles/handcuffs.png".to_string(),
			TileType::Z => "sprites/tiles/lockpick.png".to_string(),
			TileType::L => "sprites/tiles/handgun.png".to_string(),
			TileType::J => "sprites/tiles/shells.png".to_string(),
			TileType::V(i) => format!("sprites/victim_{}.png", i),
			TileType::C(i) => format!("sprites/suspect_{}.png", i),
			TileType::W(s) => {
				match s {
					1 => {
						format!("sprites/junk/junk_1_{}.png", rand::Rng::gen_range(&mut rand::thread_rng(), 0..NUM_1_JUNK))
					},
					2 => {
						format!("sprites/junk/junk_2_{}.png", rand::Rng::gen_range(&mut rand::thread_rng(), 0..NUM_2_JUNK))
					},
					_ => {
						format!("sprites/junk/junk_3_{}.png", rand::Rng::gen_range(&mut rand::thread_rng(), 0..NUM_3_JUNK))
					},
				}
			}
		}
	}
}

// STATES
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Game,
}

// RESOURCES
#[derive(Resource)]
pub struct Level(pub usize);

#[derive(Resource)]
pub struct SplashCount(pub usize);

#[derive(Resource)]
pub struct ThreadCount(pub Vec3);

#[derive(Resource)]
pub struct ThreadColliding(pub bool);

#[derive(Resource)]
pub struct MusicHandle(pub Handle<AudioInstance>);

#[derive(Resource)]
pub struct VolumeToggle {
	pub bgm: bool,
	pub sfx: bool,
}

// EVENTS
#[derive(Event)]
pub struct WaitForJunkEvent();

#[derive(Event)]
pub struct SolveCaseEvent();

#[derive(Event)]
pub struct LevelSelectedEvent{
	pub level: usize,
}

// COMPONENTS
#[derive(Component)]
pub struct RemoveOnReset;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct SolveText;

#[derive(Component)]
pub struct FailText;

#[derive(Component)]
pub struct CaseFileText(pub usize);

#[derive(Component)]
pub struct HintText;

#[derive(Component)]
pub struct UIButton {
	pub size: Vec2,
	pub function: usize,
}

#[derive(Component)]
pub struct Splash(pub usize);

#[derive(Component)]
pub struct Tile(pub TileType);

#[derive(Component)]
pub struct Tilette;

#[derive(Component)]
pub struct Tack {
	pub group: usize,
	pub end: bool,
	pub used: bool,
	pub links: i32,
	pub suspect: bool,
	pub tile_type: TileType,
}

#[derive(Component)]
pub struct Thread {
	pub group: usize,
	pub index: f32,
	pub tacks: Vec<Entity>,
	pub tacks_locs: Vec<Vec2>,
}

#[derive(Component)]
pub struct Loose;

#[derive(Component)]
pub struct Immovable;

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
pub struct Highlight;

#[derive(Component)]
pub struct CaseReport {
    pub number: usize,
	pub sfx: bool,
}

#[derive(Component)]
pub struct OpenCaseReport {
	pub number: usize,
}

#[derive(Component)]
pub struct Held {
	pub origin: Vec3,
	pub offset: Vec2,
}


// HELPER FUNCTIONS
pub fn cursor_to_screen(
	cursor_pos: Vec2,
	window: &Window,
) -> Vec2 {
	let screen_cursor = Vec2::new(
		cursor_pos.x/window.width() * VIEW_SIZE.x - VIEW_SIZE.x/2.0,
		-cursor_pos.y/window.height() * VIEW_SIZE.y + VIEW_SIZE.y/2.0,
	);
	screen_cursor
}

pub fn get_tack_color(
	tack_group: usize,
) -> Color {
	match tack_group {
		0 => Color::RED,
		1 => Color::GREEN,
		2 => Color::BLUE,
		3 => Color::WHITE,
		_ => Color::BLACK,
	}
}

pub fn get_case_report(
	level: usize,
	case: usize,
) -> String {
	match level {
		0 => match case {
			0 => "Victim found dead in their living room. Ironic. Suspect broke in through window, DNA sample and fingerprints acquired from broken glass.".to_string(),
			1 => "Suspect seen fleeing the scene of the crime. A lot of bullet casings were found around the victim.".to_string(),
			_ => "Autopsy reports victim was killed by blunt force to the head. No loud noises reported by witnesses. Firearm was acquired by police after suspect tried selling it in an auction lot.".to_string(),
		}
		1 => match case {
			0 => "Victim found shot dead after witnesses claim they were trying to summon a demonic entity in a parking lot. Suspect turned themselves in and were handcuffed.".to_string(),
			1 => "Victim killed in gang crime after being selected by lot. Footage of the crime was recovered by security camera.".to_string(),
			_ => "Lockpick set acquired from scene of the crime after suspect was killed in their home, no sign of forced entry.".to_string(),
		}
		_ => match case {
			0 => "Victim found with a lot of bullet holes in them. Several bullet casings found leading past their home.".to_string(),
			1 => "Door to victim's apartment found prized open. Fingerprints recovered from a dropped lottery ticket.".to_string(),
			_ => "No signs of murder weapon. Security footage shows suspect entering and leaving location of crime.".to_string(),
		}
	}
}

pub fn get_level_layout(
	level: usize,
) -> Vec<Vec3> {
	let mut tiles = Vec::new();
	match level {
		0 => {
			tiles.append(&mut index_to_grid([
				// Victims
				(10, 3), (15, 3), (20, 3),
				// Suspects
				(10, 7), (20, 7), (15, 7),
				// Junk
				(0, 5), (5, 0),
				(26, 9), (25, 4), (16, 0),
			].to_vec()));
			tiles.append( &mut [
				// Evidence
				Vec3::new(-500.0, -350.0, 300.0),
				Vec3::new(-500.0, -150.0, 300.0),
				Vec3::new(-350.0, -350.0, 300.0),
				Vec3::new(-350.0, -150.0, 300.0),
			].to_vec());
			tiles
		}
		1 => {
			tiles.append(&mut index_to_grid([
				// Victims
				(0, 0), (15, 10), (25, 3), 
				// Suspects
				(3, 7), (9, 2), (2, 4),
				// Junk
				(3, 0), (4, 2),
				(23, 7), (13, 5), (3, 9)
			].to_vec()));
			tiles.append( &mut [
				// Evidence
				Vec3::new(-500.0, -350.0, 300.0),
				Vec3::new(-500.0, -150.0, 300.0),
				Vec3::new(-350.0, -350.0, 300.0),
				Vec3::new(-350.0, -150.0, 300.0),
			].to_vec());
			tiles
		}
		_ => {
			tiles.append(&mut index_to_grid([
				// Victims
				(0, 0), (4, 0), (8, 0), 
				// Suspects
				(0, 4), (4, 4), (8, 4),
				// Junk
				(0, 6), (8, 6),
				(22, 6), (8, 10), (11, 0)
			].to_vec()));
			tiles.append( &mut [
				// Evidence
				Vec3::new(-500.0, -350.0, 300.0),
				Vec3::new(-500.0, -150.0, 300.0),
				Vec3::new(-350.0, -350.0, 300.0),
				Vec3::new(-350.0, -150.0, 300.0),
				Vec3::new(-100.0, -350.0, 300.0),
			].to_vec());
			tiles
		}
	}
}

pub fn get_tile_types(
	level: usize,
) -> Vec<TileType> {
	match level {
		0 => {
			[TileType::V(0), TileType::V(1), TileType::V(2),
			TileType::C(0), TileType::C(1), TileType::C(2),
			TileType::W(1), TileType::W(1),
			TileType::W(3),	TileType::W(1), TileType::W(2),
			TileType::L, TileType::J, TileType::I,
			TileType::O,
			].to_vec()
		}
		1 => {
			[TileType::V(0), TileType::V(1), TileType::V(2),
			TileType::C(0), TileType::C(1), TileType::C(2),
			TileType::W(1), TileType::W(1),
			TileType::W(3),	TileType::W(1), TileType::W(2),
			TileType::T, TileType::S, TileType::L,
			TileType::Z,
			].to_vec()
		}
		_ => {
			[TileType::V(0), TileType::V(1), TileType::V(2),
			TileType::C(0), TileType::C(1), TileType::C(2),
			TileType::W(1), TileType::W(1),
			TileType::W(2),	TileType::W(2), TileType::W(2),
			TileType::O, TileType::J, TileType::B,
			TileType::L, TileType::T,
			].to_vec()
		}
	}
}

pub fn get_tile_group(
	level: usize,
) -> Vec<usize> {
	match level {
		0 | 1 => [
			0,1,2,
			3,3,3,
			4,4,4,4,4,
			3,3,3,3
		].to_vec(),
		_ => [
			0,1,2,
			3,3,3,
			4,4,4,4,4,
			3,3,3,3,3
		].to_vec(),
	}
}

pub fn get_junk_links(
	level: usize,
) -> Vec<i32> {
	match level {
		_ => [
			-1,-1,-1,
			-1,-1,-1,
			1,2,
			4,5,6,
			-1,-1,-1,-1,-1,
		].to_vec(),
	}
}

// 99 for random tack location
pub fn get_tack_tilette(
	level: usize,
) -> Vec<usize> {
	match level {
		0 | 1 => [
			0,3,5,
			0,1,3,
			0,0,
			7,0,2,
			0,2,3,2
		].to_vec(),
		_ => [
			0,3,5,
			0,1,3,
			0,0,
			2,1,3,
			0,2,3,2,1
		].to_vec(),			
	}
}

pub fn index_to_grid(
	indices: Vec<(usize, usize)>,
) -> Vec<Vec3> {
	let mut locs = Vec::new();
	for mut index in indices {
		index = index.clamp((0,0), (28, 11));
		locs.push(Vec3::new(
			-680.0 + index.0 as f32 * 40.0,
			360.0 - index.1 as f32 * 40.0,
			300.0,
		));
	}
	locs
}

pub fn grid_to_index(
	locs: Vec<Vec3>,
) -> Vec<(usize, usize)> {
	let mut indices = Vec::new();
	for loc in locs {
		indices.push((
			((loc.x + 680.0)/40.0) as usize,
			((loc.y - 360.0)/-40.0) as usize,
		));
	}
	indices
}