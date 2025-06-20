use ahash::AHashSet;
use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate elements from a mutable vector using a standard `HashSet`.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement `Eq`, `Hash`, and `Clone`.
///
/// # Arguments
/// - `values`: A mutable reference to a vector of elements from which duplicates will be removed.
///
/// # Returns
/// This function does not return a value. It modifies the input vector in place by retaining only
/// the first occurrence of each unique item.
///
/// # Behavior
/// - Retains the first occurrence of each unique element and removes all subsequent duplicates.
/// - Preserves the relative order of the first occurrences.
/// - For empty vectors, no changes are made.
/// - Works with primitive, string, and complex types (e.g., structs, enums) that implement `Eq`, `Hash`, and `Clone`.
///
/// # Performance
/// - Uses the standard `HashSet`, which provides good general-purpose performance.
pub fn uniq<T: Eq + Hash + Clone>(values: &mut Vec<T>) {
    let mut seen = HashSet::with_capacity(values.len());
    values.retain(|item| seen.insert(item.clone()));
}

/// Removes duplicate elements from a mutable vector using `AHashSet` for faster hashing.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement `Eq`, `Hash`, and `Clone`.
///
/// # Arguments
/// - `values`: A mutable reference to a vector of elements from which duplicates will be removed.
///
/// # Returns
/// This function does not return a value. It modifies the input vector in place by retaining only
/// the first occurrence of each unique item.
///
/// # Behavior
/// - Retains the first occurrence of each unique element and removes all subsequent duplicates.
/// - Preserves the relative order of the first occurrences.
/// - For empty vectors, no changes are made.
/// - Works with primitive, string, and complex types (e.g., structs, enums) that implement `Eq`, `Hash`, and `Clone`.
///
/// # Performance
/// - Uses `AHashSet` from the `ahash` crate, offering faster hashing and better performance on large datasets or in performance-sensitive scenarios.
pub fn uniq_performant<T: Eq + Hash + Clone>(values: &mut Vec<T>) {
    let mut seen = AHashSet::with_capacity(values.len());
    values.retain(|item| seen.insert(item.clone()));
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::HashSet;

    fn random_string<R: Rng>(rng: &mut R, len: usize) -> String {
        (0..len).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    // i128 tests
    fn generate_random_i128_data(count: usize, seed: u64) -> Vec<i128> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut data = Vec::with_capacity(count);
        for _ in 0..count {
            let val: i128 = rng.gen();
            data.push(val);
            if rng.gen_bool(0.3) {
                data.push(val);
            }
        }
        data
    }

    #[test]
    fn test_uniq_with_random_i128() {
        let mut data = generate_random_i128_data(1000, 42);
        uniq(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    #[test]
    fn test_uniq_performant_with_random_i128() {
        let mut data = generate_random_i128_data(1000, 42);
        uniq_performant(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    // bool tests
    fn generate_random_bool_data(count: usize, seed: u64) -> Vec<bool> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..count).map(|_| rng.gen_bool(0.5)).collect()
    }

    #[test]
    fn test_uniq_with_random_bools() {
        let mut data = generate_random_bool_data(1000, 2025);
        uniq(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
        assert!(data.len() <= 2);
    }

    #[test]
    fn test_uniq_performant_with_random_bools() {
        let mut data = generate_random_bool_data(1000, 2025);
        uniq_performant(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
        assert!(data.len() <= 2);
    }

    // string tests
    fn generate_random_string_data(count: usize, seed: u64) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut data = Vec::with_capacity(count);
        for _ in 0..count {
            let s: String = random_string(&mut rng, 10);
            data.push(s.clone());
            if rng.gen_bool(0.3) {
                data.push(s);
            }
        }
        data
    }

    #[test]
    fn test_uniq_with_random_strings() {
        let mut data = generate_random_string_data(1000, 4242);
        uniq(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    #[test]
    fn test_uniq_performant_with_random_strings() {
        let mut data = generate_random_string_data(1000, 4242);
        uniq_performant(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    // nested struct tests
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Profile {
        bio: Option<String>,
        interests: Vec<String>,
        age: u8,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct NestedUser {
        id: u64,
        username: String,
        profile: Profile,
    }

    fn generate_random_nested_users(count: usize, seed: u64) -> Vec<NestedUser> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut users = Vec::with_capacity(count);
        for i in 0..count {
            let profile = Profile {
                bio: if rng.gen_bool(0.5) {
                    Some(random_string(&mut rng, 20))
                } else {
                    None
                },
                interests: (0..rng.gen_range(0..3))
                    .map(|_| random_string(&mut rng, 5))
                    .collect(),
                age: rng.gen_range(18..80),
            };
            let user = NestedUser {
                id: i as u64,
                username: random_string(&mut rng, 8),
                profile,
            };
            users.push(user.clone());
            if rng.gen_bool(0.3) {
                users.push(user);
            }
        }
        users
    }

    #[test]
    fn test_uniq_nested_struct() {
        let mut users = generate_random_nested_users(1000, 8888);
        uniq(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }

    #[test]
    fn test_uniq_performant_nested_struct() {
        let mut users = generate_random_nested_users(1000, 8888);
        uniq_performant(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }

    // nested enum tests
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum AuthMethod {
        OAuth(String),
        Password(String),
        Biometric,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum AccountType {
        Admin,
        Guest,
        Registered(AuthMethod),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct EnumUser {
        id: u64,
        username: String,
        account_type: AccountType,
    }

    fn generate_enum_users(count: usize, seed: u64) -> Vec<EnumUser> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut users = Vec::with_capacity(count);
        for i in 0..count {
            let method = match rng.gen_range(0..3) {
                0 => AuthMethod::OAuth(random_string(&mut rng, 10)),
                1 => AuthMethod::Password(random_string(&mut rng, 8)),
                _ => AuthMethod::Biometric,
            };
            let account_type = match rng.gen_range(0..3) {
                0 => AccountType::Admin,
                1 => AccountType::Guest,
                _ => AccountType::Registered(method),
            };
            let user = EnumUser {
                id: i as u64,
                username: random_string(&mut rng, 6),
                account_type,
            };
            users.push(user.clone());
            if rng.gen_bool(0.3) {
                users.push(user);
            }
        }
        users
    }

    #[test]
    fn test_uniq_with_nested_enum_struct() {
        let mut users = generate_enum_users(1000, 5555);
        uniq(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }

    #[test]
    fn test_uniq_performant_with_nested_enum_struct() {
        let mut users = generate_enum_users(1000, 5555);
        uniq_performant(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }
}
