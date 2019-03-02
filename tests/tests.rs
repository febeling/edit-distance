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
    assert_eq!(
        edit_distance::edit_distance("☀☂☃☄", "☀☂☃☄"),
        0
    );
}

extern crate quickcheck;
use quickcheck::quickcheck;

#[test]
fn at_least_size_difference_property() {
    fn at_least_size_difference(a: String, b: String) -> bool {
        let size_a = a.chars().count();
        let size_b = b.chars().count();
        let diff = if size_a > size_b {
            size_a - size_b
        } else {
            size_b - size_a
        };
        edit_distance::edit_distance(&a, &b) >= diff
    }

    quickcheck(at_least_size_difference as fn(a: String, b: String) -> bool);
}

#[test]
fn at_most_length_of_longer_property() {
    fn at_most_size_of_longer(a: String, b: String) -> bool {
        let upper_bound = *[a.chars().count(), b.chars().count()].iter().max().unwrap();
        edit_distance::edit_distance(&a, &b) <= upper_bound
    }

    quickcheck(at_most_size_of_longer as fn(a: String, b: String) -> bool);
}

#[test]
fn zero_iff_a_equals_b_property() {
    fn zero_iff_a_equals_b(a: String, b: String) -> bool {
        let d = edit_distance::edit_distance(&a, &b);

        if a == b {
            d == 0
        } else {
            d > 0
        }
    }

    quickcheck(zero_iff_a_equals_b as fn(a: String, b: String) -> bool);
}

#[test]
fn triangle_inequality_property() {
    fn triangle_inequality(a: String, b: String, c: String) -> bool {
        edit_distance::edit_distance(&a, &b)
            <= edit_distance::edit_distance(&a, &c) + edit_distance::edit_distance(&b, &c)
    }

    quickcheck(triangle_inequality as fn(a: String, b: String, c: String) -> bool);
}
