
/// 🔍 Returns the index of the **first** element in the slice that satisfies the predicate.
///
/// # Type Parameters
/// - `T`: The type of elements in the slice.
/// - `M`: A predicate function or closure that takes a reference to an element and returns `true` if it matches.
///
/// # Arguments
/// - `values`: A reference to a slice of elements to be scanned.
/// - `matcher`: A predicate function applied to each element.
///
/// # Returns
/// - `Some(index)` of the first matching element, or
/// - `None` if no element satisfies the predicate.
///
/// # Behavior
/// - Scans elements in order.
/// - Returns immediately on the first match.
/// - If no element matches, returns `None`.
///
/// # Performance
/// - ✅ Best-case: **O(1)** if the first element matches.
/// - ✅ Worst-case: **O(n)** if no elements match or match is last.
/// - 🚫 No allocations or cloning.
///
/// # Examples
/// 🔢 Integers:
/// ```rust
/// use pencil_box::array::find_index::find_index;
///
/// let values = [5, 8, 12, 7];
/// let result = find_index(&values, |x| x % 2 == 0);
/// assert_eq!(result, Some(1));
///
/// let result_none = find_index(&values, |x| *x > 100);
/// assert_eq!(result_none, None);
/// ```
///
/// 🧱 Structs:
/// ```rust
/// struct Task {
///     id: u32,
///     done: bool,
/// }
///
/// let tasks = [
///     Task { id: 1, done: false },
///     Task { id: 2, done: true },
/// ];
///
/// let first_done = find_index(&tasks, |t| t.done);
/// assert_eq!(first_done, Some(1));
/// ```
///
/// 🎭 Enums:
/// ```rust
/// enum Status {
///     Ready,
///     Failed,
/// }
///
/// let statuses = [Status::Ready, Status::Failed, Status::Failed];
/// let result = find_index(&statuses, |s| matches!(s, Status::Failed));
/// assert_eq!(result, Some(1));
/// ```
///
/// # Panic Safety
/// ✅ Guaranteed panic-free for all valid inputs.
///
pub fn find_index<T, M: Fn(&T) -> bool>(values: &[T], matcher: M) -> Option<usize> {
    for (index, value) in values.iter().enumerate() {
        if matcher(value) {
            return Some(index);
        }
    }
    None
}
