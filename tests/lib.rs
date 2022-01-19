use std::fs::File;

use vp9_parser::ivf::{Frame, Ivf};
use vp9_parser::{ColorDepth, ColorRange, ColorSpace, FrameType, Profile, Subsampling, Vp9Parser};

#[test]
pub fn parse_ivf() {
    let file = File::open("tests/data/320-24-crf.ivf").unwrap();
    let mut ivf = Ivf::new(file).unwrap();

    assert_eq!(ivf.width(), 320);
    assert_eq!(ivf.height(), 180);
    assert_eq!(ivf.frame_rate_rate(), 24);
    assert_eq!(ivf.frame_rate_scale(), 1);
    assert_eq!(ivf.frame_count(), 24);

    let mut first = true;

    let mut count = 0;
    while let Some(frame) = ivf.read_frame().unwrap() {
        if first {
            assert_eq!(frame.timestamp, 0);
            first = false;
        } else {
            assert_ne!(frame.timestamp, 0);
        }

        assert_ne!(frame.packet.len(), 0);
        count += 1;
    }

    assert_eq!(count, 24);
}

#[test]
pub fn parse_vp9_frames_and_super_frames() {
    // 320-24-cq.ivf contains super frames with reference frames.
    let file = File::open("tests/data/320-24-cq.ivf").unwrap();
    let mut ivf = Ivf::new(file).unwrap();
    let mut parser = Vp9Parser::default();

    let mut intra_frames = 0;
    let mut key_frames = 0;

    let mut last_frame_type = FrameType::NonKeyFrame;
    while let Some(ivf_frame) = ivf.read_frame().unwrap() {
        let Frame {
            timestamp: _timestamp,
            packet,
        } = ivf_frame;

        let frames = parser.parse_vp9_packet(&packet[..]).unwrap();
        for frame in frames.iter() {
            assert_ne!(frame.compressed_header_data().len(), 0);
            assert_ne!(frame.compressed_header_and_tile_data().len(), 0);
            assert_ne!(frame.tile_data().len(), 0);
            assert_eq!(frame.profile(), Profile::Profile0);
            assert_eq!(frame.color_depth(), ColorDepth::Depth8);
            assert_eq!(frame.color_range(), ColorRange::StudioSwing);
            assert_eq!(frame.color_space(), ColorSpace::Bt709);
            assert_eq!(frame.subsampling(), Subsampling::Yuv420);
            assert!(!frame.show_existing_frame());
            assert!(frame.frame_to_show_map_idx().is_none());
            assert_eq!(frame.last_frame_type(), last_frame_type);
            last_frame_type = frame.frame_type();

            if frame.frame_type() == FrameType::KeyFrame {
                key_frames += 1;
            }

            assert!(!frame.error_resilient_mode());
            if frame.intra_only() {
                intra_frames += 1;
            }
        }

        assert_eq!(key_frames, 1);
        assert_eq!(intra_frames, 0);
    }
}

#[test]
pub fn parse_vp9_10bit() {
    let file = File::open("tests/data/320-444-10bit.ivf").unwrap();
    let mut ivf = Ivf::new(file).unwrap();
    let mut parser = Vp9Parser::default();

    let mut last_frame_type = FrameType::NonKeyFrame;
    while let Some(ivf_frame) = ivf.read_frame().unwrap() {
        let Frame {
            timestamp: _timestamp,
            packet,
        } = ivf_frame;

        let frames = parser.parse_vp9_packet(&packet[..]).unwrap();
        for frame in frames.iter() {
            assert_ne!(frame.compressed_header_data().len(), 0);
            assert_ne!(frame.compressed_header_and_tile_data().len(), 0);
            assert_ne!(frame.tile_data().len(), 0);
            assert_eq!(frame.profile(), Profile::Profile3);
            assert_eq!(frame.color_depth(), ColorDepth::Depth10);
            assert_eq!(frame.color_range(), ColorRange::StudioSwing);
            assert_eq!(frame.color_space(), ColorSpace::Bt709);
            assert_eq!(frame.subsampling(), Subsampling::Yuv444);
            assert!(!frame.show_existing_frame());
            assert!(frame.frame_to_show_map_idx().is_none());
            assert_eq!(frame.last_frame_type(), last_frame_type);
            last_frame_type = frame.frame_type();
        }
    }
}

#[test]
pub fn parse_vp9_12bit() {
    let file = File::open("tests/data/320-444-12bit.ivf").unwrap();
    let mut ivf = Ivf::new(file).unwrap();
    let mut parser = Vp9Parser::default();

    let mut last_frame_type = FrameType::NonKeyFrame;
    while let Some(ivf_frame) = ivf.read_frame().unwrap() {
        let Frame {
            timestamp: _timestamp,
            packet,
        } = ivf_frame;

        let frames = parser.parse_vp9_packet(&packet[..]).unwrap();
        for frame in frames.iter() {
            assert_ne!(frame.compressed_header_data().len(), 0);
            assert_ne!(frame.compressed_header_and_tile_data().len(), 0);
            assert_ne!(frame.tile_data().len(), 0);
            assert_eq!(frame.profile(), Profile::Profile3);
            assert_eq!(frame.color_depth(), ColorDepth::Depth12);
            assert_eq!(frame.color_range(), ColorRange::StudioSwing);
            assert_eq!(frame.color_space(), ColorSpace::Bt709);
            assert_eq!(frame.subsampling(), Subsampling::Yuv444);
            assert!(!frame.show_existing_frame());
            assert!(frame.frame_to_show_map_idx().is_none());
            assert_eq!(frame.last_frame_type(), last_frame_type);
            last_frame_type = frame.frame_type();
        }
    }
}
