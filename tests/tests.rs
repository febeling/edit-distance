extern crate edit_distance;

#[test]
fn simple() {
    assert_eq!(edit_distance::edit_distance("kitten", "sitting"), 3);
    assert_eq!(edit_distance::edit_distance("Tier", "Tor"), 2);
}

#[test]
fn simple_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(
            &"kitten".chars().collect::<Vec<_>>(),
            &"sitting".chars().collect::<Vec<_>>()
        ),
        3
    );
    assert_eq!(
        edit_distance::edit_distance_sequences(&[1, 2, 3], &[2, 3, 4]),
        2
    );
}

#[test]
fn same() {
    assert_eq!(edit_distance::edit_distance("kitten", "kitten"), 0);
}

#[test]
fn same_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(
            &"kitten".chars().collect::<Vec<_>>(),
            &"kitten".chars().collect::<Vec<_>>()
        ),
        0
    );
    assert_eq!(
        edit_distance::edit_distance_sequences(&[1, 2, 3], &[1, 2, 3]),
        0
    );
}

#[test]
fn empty_a() {
    assert_eq!(edit_distance::edit_distance("", "kitten"), 6);
}

#[test]
fn empty_a_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(&[], &"kitten".chars().collect::<Vec<_>>()),
        6
    );
}

#[test]
fn empty_b() {
    assert_eq!(edit_distance::edit_distance("sitting", ""), 7);
}

#[test]
fn empty_b_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(&"sitting".chars().collect::<Vec<_>>(), &[]),
        7
    );
}

#[test]
fn empty_both() {
    assert_eq!(edit_distance::edit_distance("", ""), 0);
}

#[test]
fn empty_both_seq() {
    assert_eq!(edit_distance::edit_distance_sequences::<char>(&[], &[]), 0);
    assert_eq!(edit_distance::edit_distance_sequences::<i32>(&[], &[]), 0);
}

#[test]
fn unicode_misc() {
    assert_eq!(edit_distance::edit_distance("üö", "uo"), 2);
}

#[test]
fn unicode_misc_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(
            &"üö".chars().collect::<Vec<_>>(),
            &"uo".chars().collect::<Vec<_>>()
        ),
        2
    );
}

#[test]
fn unicode_thai() {
    assert_eq!(edit_distance::edit_distance("ฎ ฏ ฐ", "a b c"), 3);
}

#[test]
fn unicode_thai_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(
            &"ฎ ฏ ฐ".chars().collect::<Vec<_>>(),
            &"a b c".chars().collect::<Vec<_>>()
        ),
        3
    );
}

#[test]
fn unicode_misc_equal() {
    assert_eq!(edit_distance::edit_distance("☀☂☃☄", "☀☂☃☄"), 0);
}

#[test]
fn unicode_misc_equal_seq() {
    assert_eq!(
        edit_distance::edit_distance_sequences(
            &"☀☂☃☄".chars().collect::<Vec<_>>(),
            &"☀☂☃☄".chars().collect::<Vec<_>>()
        ),
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
fn at_least_size_difference_property_seq() {
    fn at_least_size_difference(a: Vec<u32>, b: Vec<u32>) -> bool {
        let size_a = a.len();
        let size_b = b.len();
        let diff = if size_a > size_b {
            size_a - size_b
        } else {
            size_b - size_a
        };
        edit_distance::edit_distance_sequences(&a, &b) >= diff
    }

    quickcheck(at_least_size_difference as fn(a: Vec<u32>, b: Vec<u32>) -> bool);
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
fn at_most_length_of_longer_property_seq() {
    fn at_most_size_of_longer(a: Vec<u32>, b: Vec<u32>) -> bool {
        let upper_bound = *[a.len(), b.len()].iter().max().unwrap();
        edit_distance::edit_distance_sequences(&a, &b) <= upper_bound
    }

    quickcheck(at_most_size_of_longer as fn(a: Vec<u32>, b: Vec<u32>) -> bool);
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
fn zero_iff_a_equals_b_property_seq() {
    fn zero_iff_a_equals_b(a: Vec<u32>, b: Vec<u32>) -> bool {
        let d = edit_distance::edit_distance_sequences(&a, &b);

        if a == b {
            d == 0
        } else {
            d > 0
        }
    }

    quickcheck(zero_iff_a_equals_b as fn(a: Vec<u32>, b: Vec<u32>) -> bool);
}

#[test]
fn triangle_inequality_property() {
    fn triangle_inequality(a: String, b: String, c: String) -> bool {
        edit_distance::edit_distance(&a, &b)
            <= edit_distance::edit_distance(&a, &c) + edit_distance::edit_distance(&b, &c)
    }

    quickcheck(triangle_inequality as fn(a: String, b: String, c: String) -> bool);
}

#[test]
fn triangle_inequality_property_sequences() {
    fn triangle_inequality(a: Vec<u32>, b: Vec<u32>, c: Vec<u32>) -> bool {
        edit_distance::edit_distance_sequences(&a, &b)
            <= edit_distance::edit_distance_sequences(&a, &c)
                + edit_distance::edit_distance_sequences(&b, &c)
    }

    quickcheck(triangle_inequality as fn(a: Vec<u32>, b: Vec<u32>, c: Vec<u32>) -> bool);
}
