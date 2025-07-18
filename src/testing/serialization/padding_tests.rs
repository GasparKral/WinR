#[cfg(test)]
mod padding_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::padding::Padding;

    #[test]
    fn test_padding_serialization() {
        let padding = Padding::new(10, 20, 30, 40);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&padding).unwrap();
        assert_eq!(padding, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"top":10,"right":20,"bottom":30,"left":40}"#;
        test_json_serialization(&padding, expected_json).unwrap();
    }

    #[test]
    fn test_padding_default() {
        let padding = Padding::default();

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&padding).unwrap();
        assert_eq!(padding, deserialized);

        // Test default values
        assert_eq!(padding.top, 0);
        assert_eq!(padding.right, 0);
        assert_eq!(padding.bottom, 0);
        assert_eq!(padding.left, 0);

        // Test default JSON
        let expected_json = r#"{"top":0,"right":0,"bottom":0,"left":0}"#;
        test_json_serialization(&padding, expected_json).unwrap();
    }

    #[test]
    fn test_padding_builder_methods() {
        let padding = Padding::new(5, 5, 5, 5)
            .top(10)
            .right(15)
            .bottom(20)
            .left(25);

        assert_eq!(padding.top, 10);
        assert_eq!(padding.right, 15);
        assert_eq!(padding.bottom, 20);
        assert_eq!(padding.left, 25);

        // Test serialization after builder usage
        let deserialized = test_serialization_roundtrip(&padding).unwrap();
        assert_eq!(padding, deserialized);
    }

    #[test]
    fn test_padding_horizontal_method() {
        let padding = Padding::new(10, 0, 20, 0).horizontal(30);

        // Should set left and right to 15 each (30/2)
        assert_eq!(padding.top, 10);
        assert_eq!(padding.right, 15);
        assert_eq!(padding.bottom, 20);
        assert_eq!(padding.left, 15);

        // Test serialization
        let deserialized = test_serialization_roundtrip(&padding).unwrap();
        assert_eq!(padding, deserialized);
    }

    #[test]
    fn test_padding_vertical_method() {
        let padding = Padding::new(0, 10, 0, 20).vertical(40);

        // Should set top and bottom to 20 each (40/2)
        assert_eq!(padding.top, 20);
        assert_eq!(padding.right, 10);
        assert_eq!(padding.bottom, 20);
        assert_eq!(padding.left, 20);

        // Test serialization
        let deserialized = test_serialization_roundtrip(&padding).unwrap();
        assert_eq!(padding, deserialized);
    }

    #[test]
    fn test_padding_deserialization_from_json() {
        let json = r#"{"top":5,"right":10,"bottom":15,"left":20}"#;
        let expected_padding = Padding::new(5, 10, 15, 20);
        test_json_deserialization(json, &expected_padding).unwrap();
    }

    #[test]
    fn test_padding_edge_cases() {
        // Test with zero values
        let zero_padding = Padding::new(0, 0, 0, 0);
        let deserialized = test_serialization_roundtrip(&zero_padding).unwrap();
        assert_eq!(zero_padding, deserialized);

        // Test with maximum values
        let max_padding = Padding::new(u16::MAX, u16::MAX, u16::MAX, u16::MAX);
        let deserialized = test_serialization_roundtrip(&max_padding).unwrap();
        assert_eq!(max_padding, deserialized);

        // Test with minimum values (should be 0 for u16)
        let min_padding = Padding::new(u16::MIN, u16::MIN, u16::MIN, u16::MIN);
        let deserialized = test_serialization_roundtrip(&min_padding).unwrap();
        assert_eq!(min_padding, deserialized);
    }

    #[test]
    fn test_padding_equality() {
        let padding1 = Padding::new(10, 20, 30, 40);
        let padding2 = Padding::new(10, 20, 30, 40);
        let padding3 = Padding::new(40, 30, 20, 10);

        assert_eq!(padding1, padding2);
        assert_ne!(padding1, padding3);

        // Test hash consistency
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        padding1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        padding2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_padding_chain_operations() {
        let initial = Padding::default();
        let final_padding = initial
            .top(10)
            .right(20)
            .bottom(30)
            .left(40)
            .top(50) // Should override previous top
            .right(60); // Should override previous right

        assert_eq!(final_padding.top, 50);
        assert_eq!(final_padding.right, 60);
        assert_eq!(final_padding.bottom, 30);
        assert_eq!(final_padding.left, 40);

        // Test serialization after chaining
        let deserialized = test_serialization_roundtrip(&final_padding).unwrap();
        assert_eq!(final_padding, deserialized);
    }

    #[test]
    fn test_padding_symmetrical_values() {
        // Test uniform padding
        let uniform = Padding::new(15, 15, 15, 15);
        let deserialized = test_serialization_roundtrip(&uniform).unwrap();
        assert_eq!(uniform, deserialized);

        // Test horizontal symmetry
        let horizontal_sym = Padding::new(10, 20, 30, 20);
        let deserialized = test_serialization_roundtrip(&horizontal_sym).unwrap();
        assert_eq!(horizontal_sym, deserialized);

        // Test vertical symmetry
        let vertical_sym = Padding::new(15, 10, 15, 25);
        let deserialized = test_serialization_roundtrip(&vertical_sym).unwrap();
        assert_eq!(vertical_sym, deserialized);
    }

    #[test]
    fn test_padding_horizontal_vertical_combinations() {
        let base = Padding::new(10, 20, 30, 40);

        // Test horizontal then vertical
        let padding1 = base.horizontal(50).vertical(60);
        assert_eq!(padding1.top, 30); // 60/2
        assert_eq!(padding1.right, 25); // 50/2
        assert_eq!(padding1.bottom, 30); // 60/2
        assert_eq!(padding1.left, 25); // 50/2

        // Test vertical then horizontal
        let padding2 = base.vertical(60).horizontal(50);
        assert_eq!(padding2.top, 30); // 60/2
        assert_eq!(padding2.right, 25); // 50/2
        assert_eq!(padding2.bottom, 30); // 60/2
        assert_eq!(padding2.left, 25); // 50/2

        // Both should be equal
        assert_eq!(padding1, padding2);

        // Test serialization
        let deserialized = test_serialization_roundtrip(&padding1).unwrap();
        assert_eq!(padding1, deserialized);
    }

    #[test]
    fn test_padding_common_ui_values() {
        // Test common UI padding values
        let common_paddings = vec![
            Padding::new(4, 4, 4, 4),     // xs
            Padding::new(8, 8, 8, 8),     // sm
            Padding::new(12, 12, 12, 12), // md
            Padding::new(16, 16, 16, 16), // lg
            Padding::new(20, 20, 20, 20), // xl
            Padding::new(24, 24, 24, 24), // 2xl
        ];

        for padding in common_paddings {
            let deserialized = test_serialization_roundtrip(&padding).unwrap();
            assert_eq!(padding, deserialized);
        }
    }

    #[test]
    fn test_padding_responsive_values() {
        // Test responsive-like padding values
        let responsive_paddings = vec![
            Padding::new(8, 16, 8, 16), // Mobile: smaller vertical, larger horizontal
            Padding::new(16, 32, 16, 32), // Tablet: medium vertical, large horizontal
            Padding::new(24, 48, 24, 48), // Desktop: large vertical, extra large horizontal
        ];

        for padding in responsive_paddings {
            let deserialized = test_serialization_roundtrip(&padding).unwrap();
            assert_eq!(padding, deserialized);
        }
    }
}
