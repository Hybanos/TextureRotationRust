fn get_coordinate_random(x: i32, y: i32, z: i32) -> i64 {
    let l:i64 = (x * 3129871) as i64 ^ ((z as i64) * 116129781 ^ (y as i64)) ;

    let l2 = l * l * 42317861 + 11 * l;

    l2 >> 16 
}

fn mix(mut z:i64) -> i64 {
    z = (z ^ URS(z, 30)) * -4658895280553007687i64;
    z = (z ^ URS(z, 27)) * -7723592293110705685i64;

    z ^ URS(z, 31)
}

fn sodium_random(mut seed: i64) -> i32 {

    let PHI = -7046029254386353131i64;

    seed ^= URS(seed, 33);
    seed *= -49064778989728563i64;
    seed ^= URS(seed, 33);
    seed *= -4265267296055464877i64;
    seed ^= URS(seed, 33);
    
    seed += PHI;

    let rand1:i64 = mix(seed);
    let rand2:i64 = mix(seed + PHI);

    (rand1 + rand2) as i32
}

fn vanilla_random(mut seed: i64) -> i32 {

    let mask:i64 = (1 << 48) - 1;

    seed = (seed ^ 0x5DEECE66D) & mask;
    URS(seed * 0xBB20B4600A69 + 0x40942DE6BA, 16) as i32
}

fn URS(a:i64, b:i64) -> i64 {
    let bytes = a.to_be_bytes();
    (u64::from_be_bytes(bytes) >> b) as i64
}

pub fn sodium_textures(x: i32, y: i32, z: i32, m: i32) -> i32 {
    let rand = sodium_random(get_coordinate_random(x, y, z));
    rand.abs() % m
}

pub fn vanilla_textures(x: i32, y: i32, z: i32, m: i32) -> i32 {
    let rand = vanilla_random(get_coordinate_random(x, y, z));
    rand.abs() % m
}