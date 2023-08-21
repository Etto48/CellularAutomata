@group(0) @binding(0) var input_texture : texture_2d<f32>;
@group(0) @binding(1) var output_texture : texture_storage_2d<rgba8unorm, write>;
@compute @workgroup_size(16,16)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    let x = i32(global_id.x);
    let y = i32(global_id.y);
    let size = vec2<u32>(textureDimensions(input_texture));
    var sum = 0.0;
    let filter_data = mat3x3<f32>({{filter}});
    // for (var i = 0; i < 3; i++) {
	//       for (var j = 0; j < 3; j++) {
	//           let x = (x + i - 1) % i32(size.x);
	//           let y = (y + j - 1) % i32(size.y);
	//           sum += textureLoad(input_texture, vec2<i32>(x,y),0).r * filter_data[i+j*3];
	//       }
    // }

    //fucking do it manually for God's sake, I swear I want to kill who made "filter_data[i+j*3]" illegal
    let x_values = vec3<i32>(
        (x - 1) % i32(size.x),
        (x) % i32(size.x),
        (x + 1) % i32(size.x),
    );
    let y_values = vec3<i32>(
        (y - 1) % i32(size.y),
        (y) % i32(size.y),
        (y + 1) % i32(size.y),
    );
    
    sum += textureLoad(input_texture, vec2<i32>(x_values[0],y_values[0]), 0).a * filter_data[2][0];
    sum += textureLoad(input_texture, vec2<i32>(x_values[1],y_values[0]), 0).a * filter_data[2][1];
    sum += textureLoad(input_texture, vec2<i32>(x_values[2],y_values[0]), 0).a * filter_data[2][2];
    sum += textureLoad(input_texture, vec2<i32>(x_values[0],y_values[1]), 0).a * filter_data[1][0];
    sum += textureLoad(input_texture, vec2<i32>(x_values[1],y_values[1]), 0).a * filter_data[1][1];
    sum += textureLoad(input_texture, vec2<i32>(x_values[2],y_values[1]), 0).a * filter_data[1][2];
    sum += textureLoad(input_texture, vec2<i32>(x_values[0],y_values[2]), 0).a * filter_data[0][0];
    sum += textureLoad(input_texture, vec2<i32>(x_values[1],y_values[2]), 0).a * filter_data[0][1];
    sum += textureLoad(input_texture, vec2<i32>(x_values[2],y_values[2]), 0).a * filter_data[0][2];
    
    let pixel: vec4<f32> = color_filter(activation(sum));
    textureStore(output_texture, vec2<i32>(x,y), pixel);
}

{{{activation}}}

{{{color_filter}}}