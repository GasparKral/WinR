#[cfg(test)]
mod overflow_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::overflow::Overflow;

    #[test]
    fn test_overflow_variants_serialization() {
        let overflow_variants = vec![
            Overflow::Visible,
            Overflow::Hidden,
            Overflow::Scroll,
            Overflow::Auto,
        ];

        for overflow in overflow_variants {
            let deserialized = test_serialization_roundtrip(&overflow).unwrap();
            assert_eq!(overflow, deserialized);
        }
    }

    #[test]
    fn test_overflow_json_output() {
        // Test specific JSON outputs
        test_json_serialization(&Overflow::Visible, r#""Visible""#).unwrap();
        test_json_serialization(&Overflow::Hidden, r#""Hidden""#).unwrap();
        test_json_serialization(&Overflow::Scroll, r#""Scroll""#).unwrap();
        test_json_serialization(&Overflow::Auto, r#""Auto""#).unwrap();
    }

    #[test]
    fn test_overflow_default() {
        let default_overflow = Overflow::default();
        assert_eq!(default_overflow, Overflow::Visible);

        // Test serialization of default
        let deserialized = test_serialization_roundtrip(&default_overflow).unwrap();
        assert_eq!(default_overflow, deserialized);

        // Test default JSON
        let expected_json = r#""Visible""#;
        test_json_serialization(&default_overflow, expected_json).unwrap();
    }

    #[test]
    fn test_overflow_deserialization_from_json() {
        // Test individual overflow deserialization
        test_json_deserialization(r#""Visible""#, &Overflow::Visible).unwrap();
        test_json_deserialization(r#""Hidden""#, &Overflow::Hidden).unwrap();
        test_json_deserialization(r#""Scroll""#, &Overflow::Scroll).unwrap();
        test_json_deserialization(r#""Auto""#, &Overflow::Auto).unwrap();
    }

    #[test]
    fn test_overflow_as_str() {
        // Test the as_str method
        assert_eq!(Overflow::Visible.as_str(), "visible");
        assert_eq!(Overflow::Hidden.as_str(), "hidden");
        assert_eq!(Overflow::Scroll.as_str(), "scroll");
        assert_eq!(Overflow::Auto.as_str(), "auto");

        // Test serialization after using as_str
        let overflow = Overflow::Hidden;
        let _ = overflow.as_str(); // Use the method
        let deserialized = test_serialization_roundtrip(&overflow).unwrap();
        assert_eq!(overflow, deserialized);
    }

    #[test]
    fn test_overflow_equality() {
        let overflow1 = Overflow::Scroll;
        let overflow2 = Overflow::Scroll;
        let overflow3 = Overflow::Auto;

        assert_eq!(overflow1, overflow2);
        assert_ne!(overflow1, overflow3);

        // Test hash consistency
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        overflow1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        overflow2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_overflow_clone() {
        let original = Overflow::Hidden;
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Test serialization of cloned value
        let deserialized = test_serialization_roundtrip(&cloned).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_overflow_debug() {
        let overflow = Overflow::Auto;
        let debug_str = format!("{:?}", overflow);
        assert_eq!(debug_str, "Auto");

        // Test serialization after debug usage
        let deserialized = test_serialization_roundtrip(&overflow).unwrap();
        assert_eq!(overflow, deserialized);
    }

    #[test]
    fn test_overflow_copy_semantics() {
        let overflow1 = Overflow::Visible;
        let overflow2 = overflow1; // This should be a copy, not a move

        // Both should be usable
        assert_eq!(overflow1, Overflow::Visible);
        assert_eq!(overflow2, Overflow::Visible);
        assert_eq!(overflow1, overflow2);

        // Test serialization of both
        let deserialized1 = test_serialization_roundtrip(&overflow1).unwrap();
        let deserialized2 = test_serialization_roundtrip(&overflow2).unwrap();
        assert_eq!(deserialized1, deserialized2);
    }

    #[test]
    fn test_overflow_in_collections() {
        let overflow_vec = vec![
            Overflow::Visible,
            Overflow::Hidden,
            Overflow::Scroll,
            Overflow::Auto,
        ];

        // Test serialization of vector
        let deserialized_vec = test_serialization_roundtrip(&overflow_vec).unwrap();
        assert_eq!(overflow_vec, deserialized_vec);

        // Test serialization of individual elements
        for (original, deserialized) in overflow_vec.iter().zip(deserialized_vec.iter()) {
            assert_eq!(original, deserialized);
        }
    }

    #[test]
    fn test_overflow_pattern_matching() {
        let overflows = vec![
            Overflow::Visible,
            Overflow::Hidden,
            Overflow::Scroll,
            Overflow::Auto,
        ];

        for overflow in overflows {
            // Test pattern matching works
            let description = match overflow {
                Overflow::Visible => "visible",
                Overflow::Hidden => "hidden",
                Overflow::Scroll => "scroll",
                Overflow::Auto => "auto",
            };

            // Should match the as_str output
            assert_eq!(description, overflow.as_str());

            // Test serialization after pattern matching
            let deserialized = test_serialization_roundtrip(&overflow).unwrap();
            assert_eq!(overflow, deserialized);
        }
    }
}
