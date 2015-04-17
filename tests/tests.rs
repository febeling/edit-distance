extern crate edit_distance;

#[test]
fn simple() {
    assert_eq!(edit_distance::edit_distance("kitten", "sitting"), 3);
    assert_eq!(edit_distance::edit_distance("Tier", "Tor"), 2);
}
