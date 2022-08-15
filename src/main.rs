use openh264::decoder::Decoder;
use openh264::nal_units;

fn main() {
    println!("Hello, world!");    
    let h264_in = include_bytes!("../data/test-25fps.h264");
    let mut decoder = match Decoder::new(){
        Err(_) => return,
        Ok(decoder) => decoder,
    };
    
    // Split H.264 into NAL units and decode each.
    for packet in nal_units(h264_in) {
        let _yuv = match decoder.decode(packet){
            Err(_) => return,
            Ok(yuv) => yuv,
        };
    }
}
