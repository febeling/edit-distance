extern crate edit_distance;

#[test]
fn simple() {
    assert_eq!(edit_distance::edit_distance("kitten", "sitting"), 3);
    assert_eq!(edit_distance::edit_distance("Tier", "Tor"), 2);
}

#[test]
fn same() {
    assert_eq!(edit_distance::edit_distance("kitten", "kitten"), 0);
}

#[test]
fn empty_a() {
    assert_eq!(edit_distance::edit_distance("", "kitten"), 6);
}

#[test]
fn empty_b() {
    assert_eq!(edit_distance::edit_distance("sitting", ""), 7);
}

#[test]
fn empty_both() {
    assert_eq!(edit_distance::edit_distance("", ""), 0);
}

#[test]
fn unicode_misc() {
    assert_eq!(edit_distance::edit_distance("üö", "uo"), 2);
}

#[test]
fn unicode_thai() {
    assert_eq!(edit_distance::edit_distance("ฎ ฏ ฐ", "a b c"), 3);
}

#[test]
fn unicode_misc_equal() {
    assert_eq!(edit_distance::edit_distance("☀☂☃☄", "☀☂☃☄"), 0);
}

extern crate quickcheck;
use quickcheck::quickcheck;

#[test]
fn at_least_size_difference_property() {
    fn at_least_size_difference(a: String, b: String) -> bool {
        let size_a = a.chars().count() as i32;
        let size_b = b.chars().count() as i32;
        let diff = (size_a - size_b).abs();

        edit_distance::edit_distance(&a, &b) >= diff
    }

    quickcheck(at_least_size_difference as fn(a: String, b: String) -> bool);
}
