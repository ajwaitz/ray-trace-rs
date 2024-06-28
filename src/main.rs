use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("test.ppm").unwrap();

    let h = 512;
    let w = 512;

    let mut buf = String::new();

    buf.push_str(format!("P3\n{} {}\n255\n", w, h).as_str());

    for j in 0..h {
        for i in 0..w {
            let r: i64 = (255.0 * ((i as f64) / (w as f64))).trunc() as i64;
            let g: i64 = (255.0 * ((j as f64) / (h as f64))).trunc() as i64;
            let b: i64 = 0;
            buf.push_str(format!("{} {} {} ", r, g, b).as_str());
        }
        buf.push_str("\n");
    }

    file.write_all(buf.as_ref()).unwrap();

    println!("Done!");
}
