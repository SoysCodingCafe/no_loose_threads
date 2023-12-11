// Disable Windows console on release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import Bevy game engine essentials
use bevy::{prelude::*, asset::AssetMetaCheck};
use bevy_kira_audio::prelude::*;

// MODULES
mod buttons;
mod derivables;
mod post_processing;
mod setup;
mod threads;
mod casefiles;
mod tiles;


// Only include in debug builds
#[cfg(debug_assertions)]
mod debug;

// Can't forget main!!!
fn main() {
	let default_plugins = DefaultPlugins
		.set(WindowPlugin {
			primary_window: Some(Window {
				title: "No Loose Threads".to_string(),
				canvas: Some("#canvas".into()),
				fit_canvas_to_parent: true,
				..default()
			}), 
			..default()
		})
		.set(ImagePlugin::default_nearest())
	;

	let mut app: App = App::new();
	app
		// Prevents bug in wasm builds where it checks for a file
		// that doesn't exist.
		.insert_resource(AssetMetaCheck::Never)
		.add_plugins((
			default_plugins,
			AudioPlugin,
			buttons::ButtonsPlugin,
			post_processing::PostProcessingPlugin,
			setup::SetupPlugin,
			threads::ThreadsPlugin,
			casefiles::CasefilesPlugin,
			tiles::TilesPlugin,
		))
	;

	{
		#[cfg(debug_assertions)]
		app.add_plugins(debug::DebugPlugin);
	}

	app.run();
}