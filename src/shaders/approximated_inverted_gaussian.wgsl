fn approx_inverted_gaussian(x: f32) -> f32{
    let x2 = x*x;
    let x4 = x2*x2;
    let x8 = x4*x4;
    return 1.0-(1.0+30.1*x2+2.0*x4)/(1.0+35.1*x2+20.7*x4+1.0*x8);
}

fn activation(x: f32) -> f32{
    return approx_inverted_gaussian(x);
}	