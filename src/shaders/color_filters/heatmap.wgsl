fn color_filter(x: f32) -> vec4<f32> {
    return heatmap_color(x);
}
fn heatmap_color(value: f32) -> vec4<f32> {
    var color: vec4<f32>;

    // Calculate heatmap color components
    color.r = (1.0-abs(-3.0+value*3.0));
    color.g = (1.0-abs(-2.0+value*3.0));
    color.b = (1.0-abs(-1.0+value*3.0));
    color.a = value;

    return color;
}