extern crate rodio;

use std::io::BufReader;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let file = std::fs::File::open("./08_Battle_1_~Believe_in_Victory~.flac").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}