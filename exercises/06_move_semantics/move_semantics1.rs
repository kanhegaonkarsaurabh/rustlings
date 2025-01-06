fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // the underlying vector assigned to vec0 is "moved" to fill_vec(..)
        // below and vec1 essentially points to the same vector but only vec1 
        // can now reference the vector and calling vec0 after this would throw
        // a compiler error 
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
