fn color_filter(x: f32) -> vec4<f32> {
    return heatmap_color(x);
}
fn heatmap_color(value: f32) -> vec4<f32> {
    var color1 = vec4<f32>(0.0, 0.1, 0.2, 0.0);
    var color2 = vec4<f32>(1.0, 0.8, 0.8, 1.0);

    return mix(color1, color2, value);
}