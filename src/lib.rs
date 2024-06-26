#![deny(missing_docs)]

//! # Edit distance
//!
//! The [Levenshtein edit distance][wikipedia] between two sequences is
//! the number of individual single-element changes (insert, delete,
//! substitute) necessary to change sequence `a` into `b`.
//!
//! This can be used to order search results, for fuzzy
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
/// assert_eq!(edit_distance("kitten", "sitting"), 3);
/// ```
pub fn edit_distance(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    let len_b_plus_one = len_b + 1;

    if len_a < len_b {
        return edit_distance(b, a);
    }

    // handle special case of 0 length
    if len_a == 0 {
        return len_b;
    } else if len_b == 0 {
        return len_a;
    }

    let mut cur = vec![0; len_b_plus_one];

    // initialize string b
    (1..len_b_plus_one).for_each(|i| {
        cur[i] = i;
    });

    // calculate edit distance
    for (i, ca) in a.chars().enumerate() {
        // get first column for this row
        let mut pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let tmp = cur[j + 1];
            cur[j + 1] = (tmp + 1)
                .min(cur[j] + 1)
                .min(pre + if ca == cb { 0 } else { 1 });
            pre = tmp;
        }
    }

    cur[len_b]
}

/// Returns the edit distance between sequences `a` and `b`.
///
/// The runtime complexity is `O(m*n)`, where `m` and `n` are the
/// sequences' lengths.
///
/// # Examples
///
/// ```
/// use edit_distance::edit_distance_sequences;
///
/// assert_eq!(edit_distance_sequences(&"kitten".chars().collect::<Vec<_>>(), &"sitting".chars().collect::<Vec<_>>()), 3);
/// assert_eq!(edit_distance_sequences(&vec![1, 2, 3], &vec![2, 3, 4]), 2);
/// ```
pub fn edit_distance_sequences<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let len_a = a.len();
    let len_b = b.len();
    if len_a < len_b {
        return edit_distance_sequences(b, a);
    }

    // handle special case of 0 length
    if len_a == 0 {
        return len_b;
    } else if len_b == 0 {
        return len_a;
    }

    let mut cur = vec![0; len_b + 1];

    // initialize sequence b
    (1..=len_b).for_each(|i| {
        cur[i] = i;
    });

    // calculate edit distance
    for (i, ca) in a.iter().enumerate() {
        let mut pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let tmp = cur[j + 1];
            cur[j + 1] = (tmp + 1)
                .min(cur[j] + 1)
                .min(pre + if ca == cb { 0 } else { 1 });
            pre = tmp;
        }
    }

    cur[len_b]
}
