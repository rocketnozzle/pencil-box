
/// 🔍 Returns the index of the **last** element in the slice that satisfies the predicate.
///
/// # Type Parameters
/// - `T`: The type of elements in the slice.
/// - `M`: A predicate function or closure that takes a reference to an element and returns `true` if it matches.
///
/// # Arguments
/// - `values`: A reference to a slice of elements to be searched.
/// - `matcher`: A predicate function applied to each element.
///
/// # Returns
/// - `Some(index)` of the **last** matching element, or
/// - `None` if no element satisfies the predicate.
///
/// # Behavior
/// - Iterates through the entire slice from start to finish.
/// - Keeps track of the most recent index where `matcher` returns `true`.
/// - Returns that index after completion, or `None` if no match was found.
///
/// # Performance
/// - ✅ Time complexity: **O(n)** — scans all elements regardless of early match.
/// - ✅ Space complexity: **O(1)** — constant space, no allocations.
/// - ✅ Zero heap allocations.
///
/// # Examples
/// 🔢 Numeric match:
/// ```rust
/// use pencil_box::array::find_last_index::find_last_index;
///
/// let values = [1, 4, 6, 7, 4];
/// let result = find_last_index(&values, |x| *x == 4);
/// assert_eq!(result, Some(4));
///
/// let none_result = find_last_index(&values, |x| *x > 10);
/// assert_eq!(none_result, None);
/// ```
///
/// 🧱 Struct field match:
/// ```rust
/// struct Log {
///     level: &'static str,
///     code: u16,
/// }
///
/// let logs = [
///     Log { level: "info", code: 200 },
///     Log { level: "error", code: 500 },
///     Log { level: "error", code: 501 },
/// ];
///
/// let last_error = find_last_index(&logs, |log| log.level == "error");
/// assert_eq!(last_error, Some(2));
/// ```
///
/// 🎭 Enum variant match:
/// ```rust
/// enum Action {
///     Create,
///     Update,
///     Delete,
/// }
///
/// let ops = [Action::Create, Action::Update, Action::Delete, Action::Update];
/// let last_update = find_last_index(&ops, |a| matches!(a, Action::Update));
/// assert_eq!(last_update, Some(3));
/// ```
///
/// # Panic Safety
/// ✅ Guaranteed panic-free for all valid input slices.
///
pub fn find_last_index<T, M: Fn(&T) -> bool>(values: &[T], matcher: M) -> Option<usize> {
    let mut last_found_at: usize = 0;
    let mut is_found = false;
    for (index, value) in values.iter().enumerate() {
        if matcher(value) {
            is_found = true;
            last_found_at = index;
        }
    }
    if is_found {
        return Some(last_found_at);
    }
    None
}
