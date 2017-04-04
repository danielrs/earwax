extern crate earwax;
extern crate ao_rs as ao;

use earwax::Earwax;
use ao::*;

fn main() {
    let ao = Ao::new();
    let driver = Driver::new().unwrap();
    let format = Format::new();
    let device = Device::new(&driver, &format, None).unwrap();

    let mut earwax = Earwax::new("./tracks/Canon.mp3").unwrap();
    while let Some(chunk) = earwax.spit() {
        println!("Time: {}", chunk.time.pts());
        device.play(chunk.data);
    }
}
