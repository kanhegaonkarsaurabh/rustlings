fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from-15
    // Vec implements From<[T; N]> which means that it has a method that can 
    // create a vecctor from a supplied array with type T of length N
    let v = Vec::from(a);
    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
