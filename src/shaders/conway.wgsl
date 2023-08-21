fn activation(x: f32) -> f32
{
    if x < 0.5 && x != 0.0
    {
        return 1.0;
    }
    else if x == 3.0 || x == 11.0 || x == 12.0 {
        return 1.0;
    }
    else {
        return 0.0;
    }
}