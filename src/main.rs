use std::time::Instant;
use std::vec;
use std::thread;
mod texture_provider;

fn main() {

    let xmin = -100000;
    let xmax = -90000;

    let zmin = -90000;
    let zmax = -80000;
    
    let ymin = 65;
    let ymax = 70;

    let threads = 1;

    let mode = texture_provider::vanilla_textures;
    // sodium_textures

    let x_total = xmax - xmin;
    let per_x = x_total / threads;
    let mut threads = vec![];

    for start in (xmin..xmax).step_by(per_x as usize + 1) {
        threads.push(thread::spawn( move || {
            texture_finder(start, start + per_x, zmin, zmax, ymin, ymax, mode);
        }));
    }

    for t in threads {
        let _ = t.join();
    }

}

fn texture_finder(xmin:i32, xmax:i32, zmin:i32, zmax:i32, ymin:i32, ymax:i32, mode:fn(i32,i32,i32,i32)->i32) {

    let rotations = vec![
        RotationInfo{x: 1     , y: 0, z: 0, rotation: 0, is_side: true},
        RotationInfo{x: 1     , y: 1, z: 0, rotation: 1, is_side: true},
        RotationInfo{x: 0     , y: 2, z: 0, rotation: 1, is_side: true},
        RotationInfo{x: 1     , y: 2, z: 0, rotation: 1, is_side: true},
        RotationInfo{x: 2     , y: 2, z: 0, rotation: 1, is_side: true},
        RotationInfo{x: 1     , y: 3, z: 0, rotation: 1, is_side: true},
    ];

    let mut sides:Vec<RotationInfo> = vec![];
    let mut tops_and_bottoms:Vec<RotationInfo> = vec![];

    for i in 0..rotations.len() {
        let r:RotationInfo = RotationInfo { x: rotations[i].x, y: rotations[i].y, z: rotations[i].z, rotation: rotations[i].rotation, is_side: rotations[i].is_side};

        if r.is_side {
            sides.push(r);
        } else {
            tops_and_bottoms.push(r);
        }
    }

    let now = Instant::now();

    for x in xmin..xmax + 1  {
        for z in zmin..zmax + 1  {
            'nextattempt: 
            for y in ymin..ymax + 1 {

                // println!("{}", mode(x, y, z, 2));

                for b in &tops_and_bottoms {
                    if b.rotation != mode(x + b.x, y + b.y, z + b.z, 4) {
                        continue 'nextattempt;
                    }
                }

                for b in &sides {
                    if b.rotation % 2 != mode(x + b.x, y + b.y, z + b.z, 2) {
                        continue 'nextattempt;
                    }
                }
                println!("{x} {y} {z}");
            }
        }
    }

    let elapsed = now.elapsed();
    println!("took {} millis.", elapsed.as_millis());
}

struct RotationInfo {
    x: i32,
    y: i32,
    z: i32,
    rotation: i32,
    is_side: bool,
}

