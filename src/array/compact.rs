/// A trait defining an `is_empty` method for various types.
///
/// This trait provides a generic way to determine if a value of a given type
/// can be considered "empty". The definition of "empty" is context-specific
/// and is implemented for common Rust types like collections, strings,
/// booleans, and numeric types.
///
/// # Implementations:
/// - `String` and `&str`: Returns `true` if the string contains no characters.
/// - `Vec<T>`: Returns `true` if the vector contains no elements.
/// - `bool`: Returns `true` if the boolean value is `false`.
/// - Numeric types (integers and floats): Returns `true` if the value is `0` or `0.0`.
/// - `Option<T>`: Returns `true` if the `Option` is `None` or if `Some(value)` and `value` is `is_empty()`.
///
/// # Usage
/// This trait is particularly useful for filtering or compacting collections
/// where the concept of "emptiness" applies to the elements.
pub trait IsEmpty {
    /// Checks if the value is considered empty.
    ///
    /// The specific definition of "empty" depends on the type implementing this trait.
    ///
    /// # Returns
    /// `true` if the value is empty, `false` otherwise.
    fn is_empty(&self) -> bool;
}

// --- Implementations for common Rust types ---

/// Implements `IsEmpty` for `String`.
///
/// A `String` is considered empty if its length is zero.
///
/// # Performance
/// This implementation directly calls the `String::is_empty()` method,
/// which is an efficient O(1) operation.
impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Implements `IsEmpty` for string slices (`&str`).
///
/// A `&str` is considered empty if its length is zero.
///
/// # Performance
/// This implementation directly calls the `&str::is_empty()` method,
/// which is an efficient O(1) operation.
impl IsEmpty for &str {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Implements `IsEmpty` for dynamic vectors (`Vec<T>`).
///
/// A `Vec<T>` is considered empty if it contains no elements.
///
/// # Type Parameters
/// - `T`: The type of elements within the vector.
///
/// # Performance
/// This implementation directly calls the `Vec::is_empty()` method,
/// which is an efficient O(1) operation.
impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Implements `IsEmpty` for boolean values (`bool`).
///
/// A `bool` is considered empty if its value is `false`.
/// This aligns with the common interpretation of `false` as "nothing" or "non-existent"
/// in a logical context.
///
/// # Performance
/// This is a direct comparison, an efficient O(1) operation.
impl IsEmpty for bool {
    fn is_empty(&self) -> bool {
        // Explicit comparison for clarity, though `!*self` would also work.
        *self == false
    }
}

/// Implements `IsEmpty` for `Option<T>`.
///
/// An `Option<T>` is considered empty if it is `None` or if it is `Some(value)`
/// and the `value` itself is `is_empty()`. This provides a recursive check for emptiness.
///
/// # Type Parameters
/// - `T`: The type contained within the `Option`, which must also implement `IsEmpty`.
///
/// # Performance
/// The performance depends on the `is_empty()` implementation of the inner type `T`.
impl<T: IsEmpty> IsEmpty for Option<T> {
    fn is_empty(&self) -> bool {
        match self {
            None => true,
            Some(value) => value.is_empty(),
        }
    }
}

/// A macro to automatically implement `IsEmpty` for various numeric types.
///
/// This macro reduces boilerplate by generating `IsEmpty` implementations
/// for both integer and floating-point types.
///
/// # How "Empty" is Defined for Numerics
/// - For integers, a value is considered empty if it is `0`.
/// - For floating-point numbers, a value is considered empty if it is `0.0`.
///
/// # Arguments
/// - `ints`: A comma-separated list of integer types (e.g., `i8, u16`).
/// - `floats`: A comma-separated list of floating-point types (e.g., `f32, f64`).
///
/// # Performance
/// All generated implementations are direct comparisons, which are efficient O(1) operations.
macro_rules! impl_is_empty_for_numerics {
    (
        ints: [$($int_ty:ty),*],
        floats: [$($float_ty:ty),*]
    ) => {
        $(
            /// Implements `IsEmpty` for integer type `$int_ty`.
            /// An integer is considered empty if its value is `0`.
            impl IsEmpty for $int_ty {
                fn is_empty(&self) -> bool {
                    *self == 0
                }
            }
        )*
        $(
            /// Implements `IsEmpty` for floating-point type `$float_ty`.
            /// A float is considered empty if its value is `0.0`.
            impl IsEmpty for $float_ty {
                fn is_empty(&self) -> bool {
                    *self == 0.0
                }
            }
        )*
    };
}

// Apply the macro to a comprehensive list of standard numeric types.
impl_is_empty_for_numerics!(
    ints: [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize],
    floats: [f32, f64]
);

/// Compacts a mutable vector by removing all elements that are considered "empty".
///
/// This function iterates through the vector and retains only those elements
/// for which the `is_empty()` method returns `false`.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. `T` must implement the `IsEmpty` trait.
///
/// # Arguments
/// - `values`: A mutable reference to the `Vec<T>` to be compacted.
///
/// # Behavior
/// - Modifies the `values` vector in-place.
/// - If `values` is empty initially, it remains empty.
/// - If all elements in `values` are considered empty, the vector will become empty.
/// - If no elements are considered empty, the vector remains unchanged.
///
/// # Performance
/// This function uses `Vec::retain()`, which operates in-place and is generally
/// efficient. It iterates over the vector once, moving non-empty elements
/// to the front and dropping the empty ones. Its complexity is O(N) where N
/// is the number of elements in the vector, as each element is visited once.
/// For types where `is_empty()` is O(1) (like primitive types, `String`, `Vec`),
/// the overall performance is excellent.
///
/// # Supported Types for `compact`
/// The `compact` function can be used with any type `T` that implements the `IsEmpty` trait.
/// Based on the current implementations, this includes:
/// - `String`
/// - `&str` (string slices)
/// - `Vec<T>` (vectors where `T` also implements `IsEmpty`)
/// - `bool`
/// - All integer types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
/// - All floating-point types: `f32`, `f64`
/// - `Option<T>` (optional values where `T` also implements `IsEmpty`)
pub fn compact<T: IsEmpty>(values: &mut Vec<T>) {
    values.retain(|v| !v.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Direct IsEmpty Trait Implementation Tests ---

    #[test]
    fn test_string_is_empty_impl() {
        assert!("".to_string().is_empty());
        assert!(! "hello".to_string().is_empty());
    }

    #[test]
    fn test_str_is_empty_impl() {
        assert!("".is_empty());
        assert!(! "world".is_empty());
    }

    #[test]
    fn test_vec_is_empty_impl() {
        let v_empty: Vec<i32> = vec![];
        assert!(v_empty.is_empty());
        let v_full = vec![1, 2, 3];
        assert!(! v_full.is_empty());
        // A vec containing empty elements is NOT considered empty by Vec<T>::is_empty()
        // unless it has no elements.
        let v_with_empty_elements = vec![0, 0, 0];
        assert!(! v_with_empty_elements.is_empty());
    }

    #[test]
    fn test_bool_is_empty_impl() {
        assert!(false.is_empty());
        assert!(! true.is_empty());
    }

    #[test]
    fn test_option_is_empty_impl() {
        // Option is None
        assert!(None::<i32>.is_empty());

        // Option is Some(value) where value is empty
        assert!(Some(0).is_empty()); // i32 is empty
        assert!(Some(0.0).is_empty()); // f64 is empty
        assert!(Some("".to_string()).is_empty()); // String is empty
        assert!(Some("").is_empty()); // &str is empty
        assert!(Some(false).is_empty()); // bool is empty
        assert!(Some(None::<i32>).is_empty()); // nested Option is empty

        // Option is Some(value) where value is NOT empty
        assert!(! Some(1).is_empty());
        assert!(! Some(3.14).is_empty());
        assert!(! Some("hello".to_string()).is_empty());
        assert!(! Some("world").is_empty());
        assert!(! Some(true).is_empty());
        assert!(! Some(Some(1)).is_empty()); // nested Option is not empty
    }

    #[test]
    fn test_numeric_is_empty_impls() {
        // Integers
        assert!(0_i8.is_empty());
        assert!(! 1_i8.is_empty());

        assert!(0_u8.is_empty());
        assert!(! 1_u8.is_empty());

        assert!(0_isize.is_empty());
        assert!(! 10_isize.is_empty());

        // Floats
        assert!(0.0_f32.is_empty());
        assert!(! 1.0_f32.is_empty());
        assert!(! 3.14_f64.is_empty());
    }

    // --- Compact Function Tests (Original and expanded) ---

    /// Tests `compact` with a vector of integers, including zeros.
    /// Expects zeros to be removed.
    #[test]
    fn test_compact_ints() {
        let mut v = vec![1, 0, 2, 3, 0, 4];
        compact(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    /// Tests `compact` with a vector of floats, including 0.0.
    /// Expects 0.0 to be removed.
    #[test]
    fn test_compact_floats() {
        let mut v = vec![1.1, 0.0, 2.2, 3.5, 0.0001, 4.5, 0.0];
        compact(&mut v);
        assert_eq!(v, vec![1.1, 2.2, 3.5, 0.0001, 4.5]);
    }

    /// Tests `compact` with a vector of string slices (`&str`), including empty strings.
    /// Expects empty string slices to be removed.
    #[test]
    fn test_compact_strings_refs() {
        let mut v = vec!["a", "", "b", "c", "", "d"];
        compact(&mut v);
        assert_eq!(v, vec!["a", "b", "c", "d"]);
    }

    /// Tests `compact` with a vector of `String` objects, including empty `String`s.
    /// Expects empty `String`s to be removed.
    #[test]
    fn test_compact_strings() {
        let mut values = vec![
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "d".to_string(),
        ];
        compact(&mut values);
        assert_eq!(
            values,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }

    /// Tests `compact` with a vector of booleans.
    /// Expects `false` values to be removed.
    #[test]
    fn test_compact_bools() {
        let mut v = vec![true, false, true, false, true, false];
        compact(&mut v);
        assert_eq!(v, vec![true, true, true]);
    }

    /// Tests `compact` with an initially empty vector of any type.
    /// Expects the vector to remain empty.
    #[test]
    fn test_compact_empty_vec() {
        let mut v: Vec<i32> = vec![];
        compact(&mut v);
        assert_eq!(v, vec![]);
    }

    /// Edge case: Tests `compact` with a vector containing only empty elements.
    /// Expects the vector to become completely empty.
    #[test]
    fn test_compact_all_empty_elements() {
        let mut v = vec![0, 0, 0, 0];
        compact(&mut v);
        assert_eq!(v, vec![]);

        let mut v_str = vec!["".to_string(), "".to_string()];
        compact(&mut v_str);
        assert_eq!(v_str, Vec::<String>::new());

        let mut v_bool = vec![false, false];
        compact(&mut v_bool);
        assert_eq!(v_bool, vec![]);
    }

    /// Edge case: Tests `compact` with a vector where no elements are empty.
    /// Expects the vector to remain unchanged.
    #[test]
    fn test_compact_no_empty_elements() {
        let mut v = vec![1, 2, 3, 4];
        compact(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);

        let mut v_str = vec!["hello".to_string(), "world".to_string()];
        compact(&mut v_str);
        assert_eq!(v_str, vec!["hello".to_string(), "world".to_string()]);
    }

    /// Tests `compact` with a vector of `Option<T>` values.
    /// Covers `None` and `Some(empty_value)` cases.
    #[test]
    fn test_compact_options() {
        let mut v = vec![
            Some(1),
            None,
            Some(0), // Inner integer is empty
            Some(2),
            None,
            Some(10),
            Some(0), // Inner boolean is empty
        ];
        compact(&mut v);
        assert_eq!(v, vec![Some(1), Some(2), Some(10)]);

        // Test with nested options
        let mut v_nested = vec![
            Some(Some(1)),
            Some(None), // Inner option is empty
            None,
            Some(Some(0)), // Inner value (i32) is empty
            Some(Some(2)),
        ];
        compact(&mut v_nested);
        assert_eq!(v_nested, vec![Some(Some(1)), Some(Some(2))]);
    }

    /// Test with `usize` and `isize` types in `compact`.
    #[test]
    fn test_compact_usize_isize() {
        let mut v_usize: Vec<usize> = vec![10, 0, 5, 0, 1];
        compact(&mut v_usize);
        assert_eq!(v_usize, vec![10, 5, 1]);

        let mut v_isize: Vec<isize> = vec![-1, 0, 2, -3, 0];
        compact(&mut v_isize);
        assert_eq!(v_isize, vec![-1, 2, -3]);
    }

    /// Test with `u8` and `i8` (smallest integer types) in `compact`.
    #[test]
    fn test_compact_small_ints() {
        let mut v_u8: Vec<u8> = vec![255, 0, 10, 0];
        compact(&mut v_u8);
        assert_eq!(v_u8, vec![255, 10]);

        let mut v_i8: Vec<i8> = vec![127, -128, 0, 5, 0];
        compact(&mut v_i8);
        assert_eq!(v_i8, vec![127, -128, 5]);
    }

    /// Test `compact` with a vector of `Vec<T>`
    /// As per the current `IsEmpty for Vec<T>` implementation, only truly empty vectors are removed.
    #[test]
    fn test_compact_vec_of_vecs() {
        let mut v = vec![
            vec![1, 2],
            vec![], // This one is empty
            vec![0, 0], // Not empty by Vec<T>::is_empty() definition
            vec![3],
            vec![], // This one is empty
        ];
        compact(&mut v);
        assert_eq!(v, vec![vec![1, 2], vec![0, 0], vec![3]]);
    }
}

