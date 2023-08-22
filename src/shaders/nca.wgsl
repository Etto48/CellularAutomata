fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    let x = i32(global_id.x);
    let y = i32(global_id.y);
    let size = vec2<u32>(textureDimensions(input_texture));
    var filter_data = array<f32,{{filter_data_len}}>({{filter}});
    let dx = {{filter_size}};
    let dy = {{filter_size}};
    var sum = 0.0;
    for (var i = 0; i < dx; i++) {
	    for (var j = 0; j < dy; j++) {
	        let px = (x + i - dx/2 + i32(size.x)) % i32(size.x);
	        let py = (y + j - dy/2 + i32(size.y)) % i32(size.y);
	        sum += textureLoad(input_texture, vec2<i32>(px,py),0).a * filter_data[i+j*dx];
	    }
    }
    
    let pixel: vec4<f32> = color_filter(activation(sum));
    textureStore(output_texture, vec2<i32>(x,y), pixel);
}

{{{activation}}}

{{{color_filter}}}