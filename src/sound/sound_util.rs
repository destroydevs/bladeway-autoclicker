use rodio::Decoder;
use std::io::Cursor;
use std::thread;

const ORB_SOUND: &[u8] = include_bytes!("orb.mp3");

pub fn play_orb_sound() {
    thread::spawn(move || {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        let cursor = Cursor::new(ORB_SOUND);
        let source = Decoder::new(cursor).unwrap();

        sink.append(source);

        sink.sleep_until_end();
    });
}
