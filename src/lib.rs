#![deny(missing_docs)]

//! # Edit distance
//!
//! The [Levenshtein edit distance][wikipedia] between two strings is
//! the number of individual single-character changes (insert, delete,
//! substitute) necessary to change string `a` into `b`.
//!
//! This can be a used to order search results, for fuzzy
//! auto-completion, and to find candidates for spelling correction.
//!
//! ## References
//! [Wikipedia: Levenshtein distance][wikipedia]  
//! [NIST: Levenshtein distance][nist]
//!
//! [wikipedia]: http://en.wikipedia.org/wiki/Levenshtein_distance
//! [nist]: http://xlinux.nist.gov/dads/HTML/Levenshtein.html

/// Returns the edit distance between strings `a` and `b`.
///
/// The runtime complexity is `O(m*n)`, where `m` and `n` are the
/// strings' lengths.
///
/// # Examples
///
/// ```
/// use edit_distance::edit_distance;
///
/// edit_distance("kitten", "sitting"); // => 3
/// ```
pub fn edit_distance(a: &str, b: &str) -> usize {

    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let row: Vec<usize> = vec![0; len_b + 1];
    let mut matrix: Vec<Vec<usize>> = vec![row; len_a + 1];

    // initialize string a
    for i in 0..len_a {
        matrix[i+1][0] = matrix[i][0] + 1;
    }

    // initialize string b
    for i in 0..len_b {
        matrix[0][i+1] = matrix[0][i] + 1;
    }

    // calculate matrix
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let alternatives = [
                // deletion
                matrix[i][j+1] + 1,
                // insertion
                matrix[i+1][j] + 1,
                // match or substitution
                matrix[i][j] + if ca == cb { 0 } else { 1 }];
            matrix[i+1][j+1] = *alternatives.iter().min().unwrap();
        }
    }

    matrix[len_a][len_b]
}
