// an inverted gaussian function, 
// where f(0) = 0. 
// Graph: https://www.desmos.com/calculator/torawryxnq
fn inverted_gaussian(x: f32) -> f32{
    return -1./pow(2., (0.6*pow(x, 2.)))+1.;
}

fn activation(x: f32) -> f32{
    return inverted_gaussian(x);
}	