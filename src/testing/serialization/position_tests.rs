#[cfg(test)]
mod position_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::position::Position;

    #[test]
    fn test_position_serialization() {
        let position = Position::new(100, 200);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&position).unwrap();
        assert_eq!(position, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"x":100,"y":200}"#;
        test_json_serialization(&position, expected_json).unwrap();
    }

    #[test]
    fn test_position_default() {
        let position = Position::default();

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&position).unwrap();
        assert_eq!(position, deserialized);

        // Test default values
        assert_eq!(position.x(), 0);
        assert_eq!(position.y(), 0);

        // Test default JSON
        let expected_json = r#"{"x":0,"y":0}"#;
        test_json_serialization(&position, expected_json).unwrap();
    }

    #[test]
    fn test_position_builder_methods() {
        let position = Position::new(50, 75).in_x(100).in_y(150);

        assert_eq!(position.x(), 100);
        assert_eq!(position.y(), 150);

        // Test serialization after builder usage
        let deserialized = test_serialization_roundtrip(&position).unwrap();
        assert_eq!(position, deserialized);
    }

    #[test]
    fn test_position_getters() {
        let position = Position::new(300, 400);

        assert_eq!(position.x(), 300);
        assert_eq!(position.y(), 400);

        // Test serialization preserves values
        let deserialized = test_serialization_roundtrip(&position).unwrap();
        assert_eq!(deserialized.x(), 300);
        assert_eq!(deserialized.y(), 400);
    }

    #[test]
    fn test_position_deserialization_from_json() {
        let json = r#"{"x":250,"y":350}"#;
        let expected_position = Position::new(250, 350);
        test_json_deserialization(json, &expected_position).unwrap();
    }

    #[test]
    fn test_position_edge_cases() {
        // Test with zero values
        let zero_position = Position::new(0, 0);
        let deserialized = test_serialization_roundtrip(&zero_position).unwrap();
        assert_eq!(zero_position, deserialized);

        // Test with maximum values
        let max_position = Position::new(u16::MAX, u16::MAX);
        let deserialized = test_serialization_roundtrip(&max_position).unwrap();
        assert_eq!(max_position, deserialized);

        // Test with minimum values (should be 0 for u16)
        let min_position = Position::new(u16::MIN, u16::MIN);
        let deserialized = test_serialization_roundtrip(&min_position).unwrap();
        assert_eq!(min_position, deserialized);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(100, 200);
        let pos2 = Position::new(100, 200);
        let pos3 = Position::new(200, 100);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);

        // Test hash consistency
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        pos1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        pos2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_position_chain_operations() {
        let initial = Position::default();
        let final_position = initial
            .in_x(50)
            .in_y(75)
            .in_x(100) // Should override previous x
            .in_y(125); // Should override previous y

        assert_eq!(final_position.x(), 100);
        assert_eq!(final_position.y(), 125);

        // Test serialization after chaining
        let deserialized = test_serialization_roundtrip(&final_position).unwrap();
        assert_eq!(final_position, deserialized);
    }
}
