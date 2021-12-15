#![cfg(test)]

use super::*;

#[test]
fn basic() {
	let (mut dc_offset, _ui_handle) = DcOffsetDSP::<64>::new(-1.0, 1.0, 1.0, SampleRate(44100.0));
	let mut buf = vec![0.0, 1.0, 184.0, -69.0];
	dc_offset.process_replacing_mono_fb(&mut buf);
	assert_eq!(buf, vec![1.0, 2.0, 185.0, -68.0]);
}
