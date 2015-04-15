
pub fn edit_distance(a: &str, b: &str) -> i32 {

    let row: Vec<i32> = vec![0; b.len() + 1];
    let mut matrix: Vec<Vec<i32>> = vec![row; a.len() + 1];

    // initialize string a
    for i in (0..a.len()) {
        matrix[i+1][0] = matrix[i][0] + 1;
    }

    // initialize string b
    for i in (0..b.len()) {
        matrix[0][i+1] = matrix[0][i] + 1;
    }

    // calculate matrix
    for (i, ca) in a.char_indices() {
        for (j, cb) in b.char_indices() {

            let mut alternatives = [
                // deletion
                matrix[i][j+1] + 1,
                // insertion
                matrix[i+1][j] + 1,
                // match or substituion
                matrix[i][j] + if ca == cb { 0 } else { 1 }];
            alternatives.sort();
            matrix[i+1][j+1] = *alternatives.first().unwrap();
        }
    }

    return matrix[a.len()][b.len()];
}

#[cfg(test)]
mod test {
    use super::edit_distance;

    #[test]
    fn simple() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("Tier", "Tor"), 2);
    }
}
