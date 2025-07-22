/*
    BladeWay Autoclicker Program
    Copyright (C) 2025  Evgeny K.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
