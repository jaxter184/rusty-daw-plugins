#![cfg(test)]

use super::*;

#[test]
fn basic_split_and_merge() {
	let (splitter, _ui_handle) = MidSideSplitterDSP::<64>::new();
	let (merger, _ui_handle)   = MidSideMergerDSP::<64>::new();
	let mut buf_l = vec![0.5, 0.684, 6.4, -20.1, 0.0, 1.0]; // these values have been selected to avoid floating point error
	let mut buf_r = vec![184.0, -12.812, -0.5, 2.5, 0.0, 1.0];
	splitter.process_replacing_stereo_fb(&mut buf_l[..], &mut buf_r[..]);
	assert_eq!(buf_l, vec![92.25, -6.064, 2.95, -8.8, 0.0, 1.0]); // mid
	assert_eq!(buf_r, vec![-91.75, 6.748, 3.45, -11.3, 0.0, 0.0]); // side
	merger.process_replacing_stereo_fb(&mut buf_l[..], &mut buf_r[..]);
	assert_eq!(buf_l, vec![0.5, 0.684, 6.4, -20.1, 0.0, 1.0]);
	assert_eq!(buf_r, vec![184.0, -12.812, -0.5, 2.5, 0.0, 1.0]);
}
