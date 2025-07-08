#[cfg(test)]
mod tests {
    use pencil_box::array::flatten::flatten;

    /// 🧪 Tests flattening a slice of slices (`&[&[T]]`)
    ///
    /// # ✅ Expected
    /// Returns all inner elements flattened into a single vector in order.
    #[test]
    fn test_slice_of_slices() {
        let a: &[&[i32]] = &[&[1, 2], &[3]];
        assert_eq!(flatten(a), vec![1, 2, 3]);
    }

    /// 🧪 Tests flattening a slice of vector references (`&[&Vec<T>]`)
    ///
    /// # ✅ Expected
    /// Returns a flat vector with all items cloned from the referenced vectors.
    #[test]
    fn test_slice_of_vec_refs() {
        let b: &[&Vec<i32>] = &[&vec![4], &vec![5]];
        assert_eq!(flatten(b), vec![4, 5]);
    }

    /// 🧪 Tests flattening a slice of boxed slices (`&[Box<[T]>]`)
    ///
    /// # ✅ Expected
    /// Returns a flat vector of the boxed elements.
    #[test]
    fn test_slice_of_boxed_slices() {
        let c: &[Box<[i32]>] = &[Box::new([6, 7]), Box::new([8])];
        assert_eq!(flatten(c), vec![6, 7, 8]);
    }

    /// 🧪 Tests flattening a slice of vectors (`&[Vec<T>]`)
    ///
    /// # ✅ Expected
    /// Returns a flat vector of all inner elements.
    #[test]
    fn test_slice_of_vecs() {
        let d: &[Vec<i32>] = &[vec![9], vec![10]];
        assert_eq!(flatten(d), vec![9, 10]);
    }

    /// 🧪 Tests flattening a reference to a Vec<Vec<T>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector of elements across all inner vectors.
    #[test]
    fn test_vec_of_vecs_by_ref() {
        let e: &Vec<Vec<i32>> = &vec![vec![11, 12], vec![13]];
        assert_eq!(flatten(e), vec![11, 12, 13]);
    }

    /// 🧪 Tests flattening a Vec<&[T]>
    ///
    /// # ✅ Expected
    /// Returns all elements from the borrowed slices.
    #[test]
    fn test_vec_of_slice_refs() {
        let f: Vec<&[i32]> = vec![&[14, 15], &[16]];
        assert_eq!(flatten(&f), vec![14, 15, 16]);
    }

    /// 🧪 Tests flattening a Vec<Vec<T>>
    ///
    /// # ✅ Expected
    /// Returns all values in order from the nested vectors.
    #[test]
    fn test_vec_of_vecs() {
        let g: Vec<Vec<i32>> = vec![vec![17, 18], vec![19]];
        assert_eq!(flatten(&g), vec![17, 18, 19]);
    }

    /// 🧪 Tests flattening a Vec<Box<[T]>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector with cloned boxed elements.
    #[test]
    fn test_vec_of_boxed_slices() {
        let h: Vec<Box<[i32]>> = vec![Box::new([20, 21]), Box::new([22])];
        assert_eq!(flatten(&h), vec![20, 21, 22]);
    }

    /// 🧪 Tests flattening a Vec<&Vec<T>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector of cloned values from referenced vectors.
    #[test]
    fn test_vec_of_vec_refs() {
        let vec1 = vec![23, 24];
        let vec2 = vec![25];
        let i: Vec<&Vec<i32>> = vec![&vec1, &vec2];
        assert_eq!(flatten(&i), vec![23, 24, 25]);
    }

    /// 🧪 Tests flattening an array of vectors (`&[Vec<T>; N]`)
    ///
    /// # ✅ Expected
    /// Returns all elements in the array of vectors as a single flat vector.
    #[test]
    fn test_array_of_vecs() {
        let j: &[Vec<i32>; 2] = &[vec![26, 27], vec![28]];
        assert_eq!(flatten(j), vec![26, 27, 28]);
    }

    /// 🧪 Tests flattening a Vec<Vec<String>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector of strings cloned from the nested vectors.
    #[test]
    fn test_flatten_strings() {
        let data = vec![
            vec!["foo".to_string(), "bar".to_string()],
            vec!["baz".to_string()],
        ];
        assert_eq!(flatten(&data), vec!["foo", "bar", "baz"]);
    }

    /// 🧪 Tests flattening a Vec<Vec<bool>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector of all boolean values.
    #[test]
    fn test_flatten_bools() {
        let data = vec![vec![true], vec![false, true]];
        assert_eq!(flatten(&data), vec![true, false, true]);
    }

    /// 🧪 Tests flattening a Vec<Vec<struct>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector of struct instances.
    #[test]
    fn test_flatten_structs() {
        #[derive(Debug, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let data = vec![
            vec![Point { x: 1, y: 2 }],
            vec![Point { x: 3, y: 4 }],
        ];

        assert_eq!(
            flatten(&data),
            vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }]
        );
    }

    /// 🧪 Tests flattening a Vec<Vec<enum>>
    ///
    /// # ✅ Expected
    /// Returns a flat vector of enum variants.
    #[test]
    fn test_flatten_enums() {
        #[derive(Debug, Clone, PartialEq)]
        enum Status {
            Ok,
            Error(String),
        }

        let data = vec![
            vec![Status::Ok],
            vec![Status::Error("fail".into())],
        ];

        assert_eq!(
            flatten(&data),
            vec![Status::Ok, Status::Error("fail".into())]
        );
    }

    /// 🧪 Tests flattening an empty outer vector
    ///
    /// # ✅ Expected
    /// Returns an empty vector.
    #[test]
    fn test_flatten_empty_outer() {
        let data: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten(&data), vec![]);
    }

    /// 🧪 Tests flattening with only empty inner containers
    ///
    /// # ✅ Expected
    /// Returns an empty vector.
    #[test]
    fn test_flatten_empty_inner() {
        let data: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        assert_eq!(flatten(&data), vec![]);
    }
}
