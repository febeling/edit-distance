#![deny(missing_docs)]

//! # Edit distance
//!
//! The Levenshtein edit distance between two strings is the number of
//! individual single-character changes (insert, delete, substitute)
//! necessary for string `a` to change it into `b`.

/// This can be a helpful way to order search results or fuzzy
/// auto-completion.
///
/// The runtime complexity is O(m*n), where m and n are the strings'
/// length.
///
/// # Examples
///
/// ```
/// use edit_distance::edit_distance;
///
/// edit_distance("kitten", "sitting"); // => 3
/// ```
pub fn edit_distance(a: &str, b: &str) -> i32 {

    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let row: Vec<i32> = vec![0; len_b + 1];
    let mut matrix: Vec<Vec<i32>> = vec![row; len_a + 1];

    // initialize string a
    for i in (0..len_a) {
        matrix[i+1][0] = matrix[i][0] + 1;
    }

    // initialize string b
    for i in (0..len_b) {
        matrix[0][i+1] = matrix[0][i] + 1;
    }

    // calculate matrix
    let mut i = 0;
    let mut j = 0;
    for ca in a.chars() {
        for cb in b.chars() {
            let alternatives = [
                // deletion
                matrix[i][j+1] + 1,
                // insertion
                matrix[i+1][j] + 1,
                // match or substitution
                matrix[i][j] + if ca == cb { 0 } else { 1 }];
            matrix[i+1][j+1] = *alternatives.iter().min().unwrap();
            j = j + 1;
        }
        j = 0;
        i = i + 1;
    }

    matrix[len_a][len_b]
}
