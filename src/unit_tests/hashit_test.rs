use super::*;
use serial_test::serial;

use crate::string::reset_resources;
use crate::string::{HtString, StringHash};

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
