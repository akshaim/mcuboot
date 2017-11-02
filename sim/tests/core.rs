//! Core tests
//!
//! Run the existing testsuite as a Rust unit test.

extern crate bootsim;

use bootsim::{ALL_DEVICES, RunStatus};

#[test]
fn core_tests() {
    let mut status = RunStatus::new();

    for &dev in ALL_DEVICES {
        for &align in &[1, 2, 4, 8] {
            status.run_single(dev, align);
        }
    }

    assert!(status.failures() == 0);
}
