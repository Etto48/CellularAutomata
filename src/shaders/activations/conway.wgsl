fn activation(x: f32) -> f32
{
    if (x-floor(x)) != 0.0
    {
        return floor((x-floor(x))*2.0);
    }
    else if x == 3.0 || x == 11.0 || x == 12.0 {
        return 1.0;
    }
    else {
        return 0.0;
    }
}