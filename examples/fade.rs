extern crate x264;

use std::io::Write;
use std::fs::File;
use x264::{Colorspace, Encoder, Image};

fn main() {
    const WIDTH: usize = 480;
    const HEIGHT: usize = 360;

    // Initialize things.

    let mut encoder =
        Encoder::builder()
            .fps(60, 1)
            .build(Colorspace::RGB, WIDTH as _, HEIGHT as _)
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
        frame(i as f64 / 300.0, &mut canvas);
        let image = Image::rgb(WIDTH as _, HEIGHT as _, &canvas);
        let (data, _) = encoder.encode((60 * i) as _, image).unwrap();
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
    println!("Good luck finding a H.264 viewer, though! ;)");
}

fn frame(p: f64, f: &mut [u8]) {
    let lum = (255.0 * p).floor().min(255.0) as u8;
    for x in f { *x = lum; }
}
