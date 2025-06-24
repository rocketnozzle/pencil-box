#[cfg(test)]
mod tests {
    use pencil_box::array::uniq::{uniq, uniq_performant};

    use rand::distributions::Alphanumeric;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::HashSet;

    fn random_string<R: Rng>(rng: &mut R, len: usize) -> String {
        (0..len).map(|_| rng.sample(Alphanumeric) as char).collect()
    }

    // -------- i128 Tests --------

    /// Generates a vector of random i128 values with some deliberate duplicates.
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

    /// Tests `uniq` with randomly generated i128 values.
    ///
    /// # Expected
    /// All duplicates are removed. Only unique values remain.
    #[test]
    fn test_uniq_with_random_i128() {
        let mut data = generate_random_i128_data(1000, 42);
        uniq(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    /// Tests `uniq_performant` with randomly generated i128 values.
    ///
    /// # Expected
    /// All duplicates are removed. Only unique values remain.
    #[test]
    fn test_uniq_performant_with_random_i128() {
        let mut data = generate_random_i128_data(1000, 42);
        uniq_performant(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    // -------- Bool Tests --------

    fn generate_random_bool_data(count: usize, seed: u64) -> Vec<bool> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..count).map(|_| rng.gen_bool(0.5)).collect()
    }

    /// Tests `uniq` with a vector of boolean values.
    ///
    /// # Expected
    /// Vector contains at most 2 values: true and/or false.
    #[test]
    fn test_uniq_with_random_bools() {
        let mut data = generate_random_bool_data(1000, 2025);
        uniq(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
        assert!(data.len() <= 2);
    }

    /// Tests `uniq_performant` with a vector of boolean values.
    ///
    /// # Expected
    /// Vector contains at most 2 values: true and/or false.
    #[test]
    fn test_uniq_performant_with_random_bools() {
        let mut data = generate_random_bool_data(1000, 2025);
        uniq_performant(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
        assert!(data.len() <= 2);
    }

    // -------- String Tests --------

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

    /// Tests `uniq` with a vector of random `String` values.
    ///
    /// # Expected
    /// All duplicates are removed, keeping only the first occurrence.
    #[test]
    fn test_uniq_with_random_strings() {
        let mut data = generate_random_string_data(1000, 4242);
        uniq(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    /// Tests `uniq_performant` with a vector of random `String` values.
    ///
    /// # Expected
    /// All duplicates are removed, keeping only the first occurrence.
    #[test]
    fn test_uniq_performant_with_random_strings() {
        let mut data = generate_random_string_data(1000, 4242);
        uniq_performant(&mut data);
        let set: HashSet<_> = data.iter().cloned().collect();
        assert_eq!(data.len(), set.len());
    }

    // -------- Nested Struct Tests --------

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

    /// Tests `uniq` with nested user-defined struct values.
    ///
    /// # Expected
    /// All duplicates are removed based on full struct equality.
    #[test]
    fn test_uniq_nested_struct() {
        let mut users = generate_random_nested_users(1000, 8888);
        uniq(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }

    /// Tests `uniq_performant` with nested user-defined struct values.
    ///
    /// # Expected
    /// All duplicates are removed based on full struct equality.
    #[test]
    fn test_uniq_performant_nested_struct() {
        let mut users = generate_random_nested_users(1000, 8888);
        uniq_performant(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }

    // -------- Nested Enum Tests --------

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

    /// Tests `uniq` with deeply nested enum-containing struct values.
    ///
    /// # Expected
    /// All duplicates are removed based on structural equality of nested enums.
    #[test]
    fn test_uniq_with_nested_enum_struct() {
        let mut users = generate_enum_users(1000, 5555);
        uniq(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }

    /// Tests `uniq_performant` with deeply nested enum-containing struct values.
    ///
    /// # Expected
    /// All duplicates are removed based on structural equality of nested enums.
    #[test]
    fn test_uniq_performant_with_nested_enum_struct() {
        let mut users = generate_enum_users(1000, 5555);
        uniq_performant(&mut users);
        let set: HashSet<_> = users.iter().cloned().collect();
        assert_eq!(users.len(), set.len());
    }
}
