extern crate pkg_config;

fn main() {
    let x264 = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("x264")
        .expect("Is x264 installed?");

    let n: Option<u64> = x264.version
        .split('.')
        .nth(1)
        .and_then(|n| n.parse().ok());

    if let Some(n) = n {
        if n >= 149 {
            println!("rustc-cfg=yuyv");
        }
    }
}
