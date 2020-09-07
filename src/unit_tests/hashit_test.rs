use super::*;
use serial_test::serial;

use crate::string::reset_resources;
use crate::string::{HtString, StringHash};

// This test verifies that the first time an input is checked, it shows up as having changed.
#[test]
#[serial]
fn has_changed_given_new_items_is_true() {
    reset_resources();
    let input = "/this/is/new";
    let output = "output";
    let mut hashit = Hashit {
        inner: HtString::new(),
        hasher: StringHash {},
    };
    let has_changed = hashit.has_changed(&vec![input][..], output);
    assert!(has_changed.unwrap());
}

// this test verifies that given an input, it will first show up as changed, and then, after a second
// inspection, show up as having not changed. The sequence of events is:
// - first call
//   - generate a hash from input
//   - check to see if the output exists - which it doesnt
//   - create output entry in the static RESOURCES map, setting its value to the hash generated from the input
//   - return true
// - second call
//   - generate a hash from input
//   - check to see if the output exists - it does
//   - load output hash from RESOURCES map
//   - compare hash and output hash - They match so
//   - return false
#[test]
#[serial]
fn has_changed_given_existing_item_is_false() {
    reset_resources();
    let input = "/this/is/new";
    let output = "output";
    let mut hashit = Hashit {
        inner: HtString::new(),
        hasher: StringHash {},
    };
    // first time we expect the output to
    let has_changed = hashit.has_changed(&vec![input][..], output);
    assert_eq!(has_changed.unwrap(), true);
    let has_changed = hashit.has_changed(&vec![input][..], output);
    assert_eq!(has_changed.unwrap(), false);
}

// The following test mimics the test above, but with multiple inputs
#[test]
#[serial]
fn has_changed_given_existing_items_is_false() {
    reset_resources();
    let input = "/this/is/new";
    let input2: &str = "/second/input";
    let output = "output";
    let mut hashit = Hashit {
        inner: HtString::new(),
        hasher: StringHash {},
    };
    // first time we expect the output to
    let has_changed = hashit.has_changed(&vec![input, input2][..], output);
    assert_eq!(has_changed.unwrap(), true);
    let has_changed = hashit.has_changed(&vec![input, input2][..], output);
    assert_eq!(has_changed.unwrap(), false);
}
