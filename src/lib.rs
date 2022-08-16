use openh264::encoder::{Encoder, EncoderConfig};
use openh264::decoder::Decoder;
use openh264::nal_units;
use openh264::formats::RBGYUVConverter;
#[cfg(test)]
mod tests {
    use openh264::formats::YUVSource;

    use super::*;

    #[test]
    fn test1() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        let h264_in = include_bytes!("../data/test-25fps.h264");
        let mut decoder = Decoder::new().expect("failed to create decoder");
        // Split H.264 into NAL units and decode each.
        for packet in nal_units(h264_in) {
            let yuv = decoder.decode(packet).expect("decode fail");
            // log::error!("y: {:?}", yuv.y_with_stride());     
            // log::error!("w: {:?}, h: {:?}", yuv.width(), yuv.height());
            if yuv.width() <= 0 || yuv.height() <= 0 {
                continue;
            }
            log::error!("w: {:?}, h: {:?}", yuv.width(), yuv.height());
            let config = EncoderConfig::new(320, 240);
            let mut encoder = Encoder::with_config(config).expect("failed to create encoder");
    
            // Encode YUV back into H.264.
            let _bitstream = encoder.encode(&yuv).expect("failed to encode");
        }
    }

    #[test]
    fn test2() {
        // let toYUV = RBGYUVConverter::new(320, 240);
        let mut to_yuv = RBGYUVConverter::new(320, 240);

        let mut rgb: Vec<u8> = Vec::new();

        for _ in 0..320*240*3 {
            rgb.push(255u8);
        }

        to_yuv.convert(&rgb);

        let config = EncoderConfig::new(320, 240);
        let mut encoder = Encoder::with_config(config).expect("failed to create encoder");

        // Encode YUV back into H.264.
        let _bitstream = encoder.encode(&to_yuv).expect("failed to encode");

    }
}