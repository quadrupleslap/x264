extern crate x264;

use std::io::Write;
use std::fs::File;
use x264::{Encoding, Image, Setup, Preset, Tune};

fn main() {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    // Initialize things.

    let mut encoder =
        Setup::preset(Preset::Ultrafast, Tune::None, false, false)
            .fps(60, 1)
            .build(Encoding::RGB, WIDTH as i32, HEIGHT as i32)
            .unwrap();
    let mut file = File::create("fade.h264").unwrap();
    let mut canvas = vec![0; WIDTH * HEIGHT * 3];

    println!("Initialized!");

    // Write the headers.

    {
        let headers = encoder.headers().unwrap();
        file.write_all(headers.entirety()).unwrap();
    }

    // Queue each frame.

    for i in 0..300 {
        frame(i as f64 / 300.0, &mut *canvas);
        let image = Image::rgb(WIDTH as i32, HEIGHT as i32, &canvas);
        let (data, _) = encoder.encode(i as i64, image).unwrap();
        file.write_all(data.entirety()).unwrap();
    }

    // Finally, flush any delayed frames.

    {
        let mut flush = encoder.flush();
        while let Some(result) = flush.next() {
            let (data, _) = result.unwrap();
            file.write_all(data.entirety()).unwrap();
        }
    }

    println!("Done! The output is at `fade.h264`.");
    println!("Try playing it with VLC, and prepare to be underwhelmed! ;)");
}

fn frame(p: f64, f: &mut [u8]) {
    let lum = (255.0 * p).floor() as u8;
    for x in f { *x = lum; }
}
