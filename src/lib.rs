use openh264::decoder::Decoder;
use openh264::nal_units;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        let h264_in = include_bytes!("../data/test-25fps.h264");
        let mut decoder = Decoder::new().expect("create decoder fail");

        // Split H.264 into NAL units and decode each.
        for packet in nal_units(h264_in) {
            let yuv = decoder.decode(packet).expect("decode fail");
            log::error!("y: {:?}", yuv.y_with_stride());
        }
    }
}