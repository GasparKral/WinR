#[cfg(test)]
mod margin_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::margin::Margin;

    #[test]
    fn test_margin_serialization() {
        let margin = Margin::new(10, 20, 30, 40);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&margin).unwrap();
        assert_eq!(margin, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"top":10,"right":20,"bottom":30,"left":40}"#;
        test_json_serialization(&margin, expected_json).unwrap();
    }

    #[test]
    fn test_margin_default() {
        let margin = Margin::default();

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&margin).unwrap();
        assert_eq!(margin, deserialized);

        // Test default values
        assert_eq!(margin.top, 0);
        assert_eq!(margin.right, 0);
        assert_eq!(margin.bottom, 0);
        assert_eq!(margin.left, 0);

        // Test default JSON
        let expected_json = r#"{"top":0,"right":0,"bottom":0,"left":0}"#;
        test_json_serialization(&margin, expected_json).unwrap();
    }

    #[test]
    fn test_margin_builder_methods() {
        let margin = Margin::new(5, 5, 5, 5)
            .top(10)
            .right(15)
            .bottom(20)
            .left(25);

        assert_eq!(margin.top, 10);
        assert_eq!(margin.right, 15);
        assert_eq!(margin.bottom, 20);
        assert_eq!(margin.left, 25);

        // Test serialization after builder usage
        let deserialized = test_serialization_roundtrip(&margin).unwrap();
        assert_eq!(margin, deserialized);
    }

    #[test]
    fn test_margin_horizontal_method() {
        let margin = Margin::new(10, 0, 20, 0).horizontal(30);

        // Should set left and right to 15 each (30/2)
        assert_eq!(margin.top, 10);
        assert_eq!(margin.right, 15);
        assert_eq!(margin.bottom, 20);
        assert_eq!(margin.left, 15);

        // Test serialization
        let deserialized = test_serialization_roundtrip(&margin).unwrap();
        assert_eq!(margin, deserialized);
    }

    #[test]
    fn test_margin_vertical_method() {
        let margin = Margin::new(0, 10, 0, 20).vertical(40);

        // Should set top and bottom to 20 each (40/2)
        assert_eq!(margin.top, 20);
        assert_eq!(margin.right, 10);
        assert_eq!(margin.bottom, 20);
        assert_eq!(margin.left, 20);

        // Test serialization
        let deserialized = test_serialization_roundtrip(&margin).unwrap();
        assert_eq!(margin, deserialized);
    }

    #[test]
    fn test_margin_deserialization_from_json() {
        let json = r#"{"top":5,"right":10,"bottom":15,"left":20}"#;
        let expected_margin = Margin::new(5, 10, 15, 20);
        test_json_deserialization(json, &expected_margin).unwrap();
    }

    #[test]
    fn test_margin_edge_cases() {
        // Test with zero values
        let zero_margin = Margin::new(0, 0, 0, 0);
        let deserialized = test_serialization_roundtrip(&zero_margin).unwrap();
        assert_eq!(zero_margin, deserialized);

        // Test with maximum values
        let max_margin = Margin::new(u16::MAX, u16::MAX, u16::MAX, u16::MAX);
        let deserialized = test_serialization_roundtrip(&max_margin).unwrap();
        assert_eq!(max_margin, deserialized);

        // Test with minimum values (should be 0 for u16)
        let min_margin = Margin::new(u16::MIN, u16::MIN, u16::MIN, u16::MIN);
        let deserialized = test_serialization_roundtrip(&min_margin).unwrap();
        assert_eq!(min_margin, deserialized);
    }

    #[test]
    fn test_margin_equality() {
        let margin1 = Margin::new(10, 20, 30, 40);
        let margin2 = Margin::new(10, 20, 30, 40);
        let margin3 = Margin::new(40, 30, 20, 10);

        assert_eq!(margin1, margin2);
        assert_ne!(margin1, margin3);

        // Test hash consistency
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        margin1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        margin2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_margin_chain_operations() {
        let initial = Margin::default();
        let final_margin = initial
            .top(10)
            .right(20)
            .bottom(30)
            .left(40)
            .top(50) // Should override previous top
            .right(60); // Should override previous right

        assert_eq!(final_margin.top, 50);
        assert_eq!(final_margin.right, 60);
        assert_eq!(final_margin.bottom, 30);
        assert_eq!(final_margin.left, 40);

        // Test serialization after chaining
        let deserialized = test_serialization_roundtrip(&final_margin).unwrap();
        assert_eq!(final_margin, deserialized);
    }

    #[test]
    fn test_margin_symmetrical_values() {
        // Test uniform margins
        let uniform = Margin::new(15, 15, 15, 15);
        let deserialized = test_serialization_roundtrip(&uniform).unwrap();
        assert_eq!(uniform, deserialized);

        // Test horizontal symmetry
        let horizontal_sym = Margin::new(10, 20, 30, 20);
        let deserialized = test_serialization_roundtrip(&horizontal_sym).unwrap();
        assert_eq!(horizontal_sym, deserialized);

        // Test vertical symmetry
        let vertical_sym = Margin::new(15, 10, 15, 25);
        let deserialized = test_serialization_roundtrip(&vertical_sym).unwrap();
        assert_eq!(vertical_sym, deserialized);
    }

    #[test]
    fn test_margin_horizontal_vertical_combinations() {
        let base = Margin::new(10, 20, 30, 40);

        // Test horizontal then vertical
        let margin1 = base.horizontal(50).vertical(60);
        assert_eq!(margin1.top, 30); // 60/2
        assert_eq!(margin1.right, 25); // 50/2
        assert_eq!(margin1.bottom, 30); // 60/2
        assert_eq!(margin1.left, 25); // 50/2

        // Test vertical then horizontal
        let margin2 = base.vertical(60).horizontal(50);
        assert_eq!(margin2.top, 30); // 60/2
        assert_eq!(margin2.right, 25); // 50/2
        assert_eq!(margin2.bottom, 30); // 60/2
        assert_eq!(margin2.left, 25); // 50/2

        // Both should be equal
        assert_eq!(margin1, margin2);

        // Test serialization
        let deserialized = test_serialization_roundtrip(&margin1).unwrap();
        assert_eq!(margin1, deserialized);
    }
}
