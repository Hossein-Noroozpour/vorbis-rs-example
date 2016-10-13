extern crate vorbis;

fn main() {
	use std::fs::File;
    let f = File::open("/home/thany/Dropbox/Projects/Start/Music/back-1.ogg").unwrap();
    let mut decoder = vorbis::Decoder::new(f).unwrap();
    let packets = decoder.packets();
    let mut data: Vec<i16> = Vec::new();
	let mut channels = 0;
	let mut rate = 0;
	let mut bitrate_upper = 0;
	let mut bitrate_nominal = 0;
	let mut bitrate_lower = 0;
	let mut bitrate_window = 0;
    for p in packets {
        match p {
            Ok(mut packet) => {
				channels = packet.channels;
				rate = packet.rate;
				bitrate_upper = packet.bitrate_upper;
				bitrate_nominal = packet.bitrate_nominal;
				bitrate_lower = packet.bitrate_lower;
				bitrate_window = packet.bitrate_window;
                data.append(&mut packet.data);
            },
            _ => {}
        }

    }
    println!("PCM data size: {}", data.len());
	println!("channels: {:?}", channels);
	println!("rate: {:?}", rate);
	println!("bitrate_upper: {:?}", bitrate_upper);
	println!("bitrate_nominal: {:?}", bitrate_nominal);
	println!("bitrate_lower: {:?}", bitrate_lower);
	println!("bitrate_window: {:?}", bitrate_window);
}
