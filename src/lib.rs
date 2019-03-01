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
pub fn edit_distance(a: &str, b: &str) -> i32 {

    // handle special case of 0 length
    if a.len() == 0 {
        return b.chars().count() as i32
    } else if b.len() == 0 {
        return a.chars().count() as i32
    }

    let len_b = b.chars().count() + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    for i in 1..len_b {
        cur[i] = i as i32;
    }

    // calculate edit distance
    for (i,ca) in a.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = (i as i32) + 1;
        for (j, cb) in b.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1, std::cmp::min(
                // insertion
                cur[j] + 1,
                // match or substitution
                pre + if ca == cb { 0 } else { 1 }));
            pre = tmp;
        }
    }
    cur[len_b - 1]
}
