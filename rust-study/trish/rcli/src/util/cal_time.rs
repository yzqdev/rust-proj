use std::time::SystemTime;

pub fn cal_time(){
    let start = SystemTime::now();

    for a in 0..=1000 {
        for b in 0..=1000 {
            for c in 0..=1000 {
                if a * a + b * b == c * c && a + b + c == 1000 {
                    println!("a:{} b:{} c:{}", a, b, c);
                }
            }
        }
    }

    println!("{:?} seconds", start.elapsed().unwrap());
}