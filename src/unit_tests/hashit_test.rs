use super::*;
use crate::string::{HtString, StringHash};
#[test]
fn has_changed_given_new_items_is_true() {
    let input = "/this/is/new";
    let output = "/foo/bar";
    let mut hashit = Hashit {
        inner: HtString::new(),
        hasher: StringHash {},
    };
    let has_changed = hashit.has_changed(&vec![input][..], output);
    assert!(has_changed.unwrap());
}
#[test]
fn has_changed_given_existing_items_is_false() {
    let input = "/this/is/new";
    let output = "/this/is/new";
    let mut hashit = Hashit {
        inner: HtString::new(),
        hasher: StringHash {},
    };
    let has_changed = hashit.has_changed(&vec![input][..], output);
    assert_eq!(has_changed.unwrap(), false);
}
