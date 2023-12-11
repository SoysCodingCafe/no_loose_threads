#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
// #endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    
	// CHROMATIC ABERRATION

	// // Chromatic aberration strength
    // let offset_strength = settings.intensity;

    // // Sample each color channel with an arbitrary shift
    // return vec4<f32>(
    //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
    //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
    //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
    //     1.0
    // );

	// let resolution = vec2<f32>(textureDimensions(screen_texture));
	// let width_height_over_block_size = resolution / max(1.0, 3.0);


	// PIXELLATION

	// var uv = in.uv + 0.5;
    // uv *= width_height_over_block_size;
    // uv = floor(uv + 0.5);
    // uv /= width_height_over_block_size;
	// uv -= 0.5;

	// return vec4<f32>(textureSample(screen_texture, texture_sampler, uv));


	// PALETTE SWAP

	// var color_index = 0;
	// var palette_index = 0;
	// let col = textureSample(screen_texture, texture_sampler, in.uv);
	// if (col.r > 0.9 && col.g < 0.1 && col.b < 0.1) {palette_index = 1;}
	// else if (col.r > 0.1 && col.g < 0.9 && col.b < 0.1) {palette_index = 2;}
	// else if (col.r > 0.1 && col.g < 0.1 && col.b < 0.9) {palette_index = 3;}
	// else {palette_index = 0;};

	// if color_index < 0.05 {
	// 	palette_index = 0;
	// } else if color_index < 0.1 {
	// 	palette_index = 1;
	// } else if color_index < 0.5 {
	// 	palette_index = 2;
	// } else {
	// 	palette_index = 3;
	// }

	// let selected_palette = settings.intensity;
	// var r = textureSample(screen_texture, texture_sampler, in.uv).r; //vec4(0.0, 0.8, 0.9, 1.0);
	// var g = textureSample(screen_texture, texture_sampler, in.uv).g; //vec4(0.0, 0.3, 0.8, 1.0);
	// var b = textureSample(screen_texture, texture_sampler, in.uv).b; //vec4(0.0, 0.3, 0.8, 1.0);

	// if selected_palette < 1.0 {
	// 	// Gameboy
	// 	r = vec4(0.0, 0.239, 0.671, 0.976);
	// 	g = vec4(0.0, 0.502, 0.8, 1.0);
	// 	b = vec4(0.0, 0.149, 0.278, 0.702);
	// } else if selected_palette < 2.0 {
	// 	// Ice Cream
	// 	r = vec4(0.486, 0.922, 0.976, 1.0);
	// 	g = vec4(0.247, 0.420, 0.659, 0.965);
	// 	b = vec4(0.345, 0.435, 0.459, 0.827);
	// } else if selected_palette < 3.0 {
	// 	// Aqua
	// 	r = vec4(0.0, 0.0, 0.0, 0.624);
	// 	g = vec4(0.169, 0.373, 0.725, 0.957);
	// 	b = vec4(0.349, 0.549, 0.745, 0.898);
	// } else if selected_palette < 4.0 {
	// 	// Caramel Autumn
	// 	r = vec4(0.161, 0.635, 1.0, 1.0);
	// 	g = vec4(0.004, 0.184, 0.545, 0.957);
	// 	b = vec4(0.263, 0.788, 0.251, 0.722);
	// } else if selected_palette < 5.0 {
	// 	// Ace
	// 	r = vec4(0.0, 0.502, 0.639, 1.0);
	// 	g = vec4(0.0, 0.0, 0.639, 1.0);
	// 	b = vec4(0.0, 0.502, 0.639, 1.0);
	// } else if selected_palette < 6.0 {
	// 	// NB
	// 	r = vec4(0.0, 0.612, 0.988, 1.0);
	// 	g = vec4(0.0, 0.349, 0.957, 1.0);
	// 	b = vec4(0.0, 0.820, 0.204, 1.0);
	// } else if selected_palette < 7.0 {
	// 	// Trans
	// 	r = vec4(0.0, 0.357, 0.961, 1.0);
	// 	g = vec4(0.0, 0.808, 0.663, 1.0);
	// 	b = vec4(0.0, 0.980, 0.722, 1.0);
	// } else {
	// 	// Hollow
	// 	r = vec4(0.059, 0.337, 0.776, 0.980);
	// 	g = vec4(0.059, 0.353, 0.718, 0.984);
	// 	b = vec4(0.106, 0.459, 0.745, 0.965);
	// }
	// if selected_palette < 1.0 || palette_index == 3 {
	// 	return vec4<f32>(textureSample(screen_texture, texture_sampler, in.uv));
	// } else {
	// 	return vec4<f32>(
	// 		pow(r[palette_index], 2.2),
	// 		pow(g[palette_index], 2.2),
	// 		pow(b[palette_index], 2.2),
	// 		1.0,
	// 	);
	// }


	// PASSTHROUGH

	return vec4<f32>(textureSample(screen_texture, texture_sampler, in.uv));
}