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

