use std::{fs::File, time::Duration};

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

pub fn play_sound(path: &str) {
    let path: String = String::from(path);

    std::thread::spawn(move || {
        let sink: MixerDeviceSink = DeviceSinkBuilder::open_default_sink().unwrap();
        let player: Player = Player::connect_new(sink.mixer());

        let file: File = std::fs::File::open(&path).unwrap();

        player.append(Decoder::try_from(file).unwrap());

        player.sleep_until_end();

        std::thread::sleep(Duration::from_millis(7000));
    });
}

pub fn try_play_sound(path: Option<&String>) -> Result<(), String> {
    return match path {
        Option::Some(path) => Result::Ok(play_sound(path)),
        Option::None => Result::Err(String::from("Path is None!"))
    };
}
