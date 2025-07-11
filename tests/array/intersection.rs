#[cfg(test)]
mod tests {
    use pencil_box::array::intersection::intersection;

    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MyStruct {
        id: u8,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum MyEnum {
        A,
        B,
        C,
    }

    /// 🧪 Primitive integers (owned)
    ///
    /// Expect: common values `[2, 3]` from all slices.
    #[test]
    fn test_numbers_owned() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];
        let c = vec![2, 3, 5];

        let result = intersection(&[a, b, c]);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    /// 🧪 Primitive integers (references)
    ///
    /// Expect: common values `2` and `3` across all inputs.
    #[test]
    fn test_numbers_refs() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];
        let c = vec![2, 3, 5];

        let ra: Vec<&i32> = a.iter().collect();
        let rb: Vec<&i32> = b.iter().collect();
        let rc: Vec<&i32> = c.iter().collect();

        let result = intersection(&[ra, rb, rc]);

        assert_eq!(result.len(), 2);
        let values: HashSet<_> = result.into_iter().copied().collect();
        assert_eq!(values, HashSet::from([2, 3]));
    }

    /// 🧪 Booleans (owned)
    ///
    /// Expect: only `false` is present in all slices.
    #[test]
    fn test_bools_owned() {
        let a = vec![true, false];
        let b = vec![false, false];
        let c = vec![false];

        let result = intersection(&[a, b, c]);

        assert_eq!(result, vec![false]);
    }

    /// 🧪 Booleans (references)
    ///
    /// Expect: only `false` is common across all references.
    #[test]
    fn test_bools_refs() {
        let a = vec![true, false];
        let b = vec![false, true];
        let c = vec![false];

        let ra: Vec<&bool> = a.iter().collect();
        let rb: Vec<&bool> = b.iter().collect();
        let rc: Vec<&bool> = c.iter().collect();

        let result = intersection(&[ra, rb, rc]);

        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], &false));
    }

    /// 🧪 Strings (owned)
    ///
    /// Expect: "b" appears in all three input vectors.
    #[test]
    fn test_strings_owned() {
        let a = vec!["a".to_string(), "b".to_string()];
        let b = vec!["b".to_string(), "c".to_string()];
        let c = vec!["b".to_string(), "d".to_string()];

        let result = intersection(&[a, b, c]);

        assert_eq!(result, vec!["b".to_string()]);
    }

    /// 🧪 String slices (references)
    ///
    /// Expect: "b" appears in all references.
    #[test]
    fn test_strings_refs() {
        let a = vec!["a", "b"];
        let b = vec!["b", "c"];
        let c = vec!["b", "d"];

        let ra: Vec<&str> = a.iter().cloned().collect();
        let rb: Vec<&str> = b.iter().cloned().collect();
        let rc: Vec<&str> = c.iter().cloned().collect();

        let result = intersection(&[ra, rb, rc]);

        assert_eq!(result, vec!["b"]);
    }

    /// 🧪 Structs (owned)
    ///
    /// Expect: `MyStruct { id: 2 }` is common in all.
    #[test]
    fn test_structs_owned() {
        let a = vec![MyStruct { id: 1 }, MyStruct { id: 2 }];
        let b = vec![MyStruct { id: 2 }, MyStruct { id: 3 }];
        let c = vec![MyStruct { id: 2 }, MyStruct { id: 4 }];

        let result = intersection(&[a, b, c]);

        assert_eq!(result, vec![MyStruct { id: 2 }]);
    }

    /// 🧪 Structs (references)
    ///
    /// Expect: reference to `MyStruct { id: 2 }` in all collections.
    #[test]
    fn test_structs_refs() {
        let sa = [MyStruct { id: 1 }, MyStruct { id: 2 }];
        let sb = [MyStruct { id: 2 }, MyStruct { id: 3 }];
        let sc = [MyStruct { id: 2 }, MyStruct { id: 4 }];

        let ra: Vec<&MyStruct> = sa.iter().collect();
        let rb: Vec<&MyStruct> = sb.iter().collect();
        let rc: Vec<&MyStruct> = sc.iter().collect();

        let result = intersection(&[ra, rb, rc]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 2);
    }

    /// 🧪 Enums (owned)
    ///
    /// Expect: only `MyEnum::B` appears in all.
    #[test]
    fn test_enums_owned() {
        let a = vec![MyEnum::A, MyEnum::B];
        let b = vec![MyEnum::B, MyEnum::C];
        let c = vec![MyEnum::B, MyEnum::A];

        let result = intersection(&[a, b, c]);

        assert_eq!(result, vec![MyEnum::B]);
    }

    /// 🧪 Enums (references)
    ///
    /// Expect: reference to `MyEnum::B` is common in all inputs.
    #[test]
    fn test_enums_refs() {
        let ea = [MyEnum::A, MyEnum::B];
        let eb = [MyEnum::B, MyEnum::C];
        let ec = [MyEnum::B, MyEnum::A];

        let ra: Vec<&MyEnum> = ea.iter().collect();
        let rb: Vec<&MyEnum> = eb.iter().collect();
        let rc: Vec<&MyEnum> = ec.iter().collect();

        let result = intersection(&[ra, rb, rc]);

        assert_eq!(result.len(), 1);
        matches!(result[0], &MyEnum::B);
    }
}
