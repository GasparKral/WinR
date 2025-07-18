#[cfg(test)]
mod size_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::size::{Size, SizePolicy};

    #[test]
    fn test_size_policy_serialization() {
        let policies = vec![SizePolicy::Fixed, SizePolicy::Fill, SizePolicy::Fit];

        for policy in policies {
            let deserialized = test_serialization_roundtrip(&policy).unwrap();
            assert_eq!(policy, deserialized);
        }

        // Test specific JSON outputs
        test_json_serialization(&SizePolicy::Fixed, r#""Fixed""#).unwrap();
        test_json_serialization(&SizePolicy::Fill, r#""Fill""#).unwrap();
        test_json_serialization(&SizePolicy::Fit, r#""Fit""#).unwrap();
    }

    #[test]
    fn test_size_policy_default() {
        let default_policy = SizePolicy::default();
        assert_eq!(default_policy, SizePolicy::Fixed);

        // Test serialization of default
        let deserialized = test_serialization_roundtrip(&default_policy).unwrap();
        assert_eq!(default_policy, deserialized);
    }

    #[test]
    fn test_size_serialization() {
        let size = Size::new(800, 600);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&size).unwrap();
        assert_eq!(size, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"height":800,"width":600}"#;
        test_json_serialization(&size, expected_json).unwrap();
    }

    #[test]
    fn test_size_default() {
        let size = Size::default();

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&size).unwrap();
        assert_eq!(size, deserialized);

        // Test default values
        assert_eq!(size.height(), 0);
        assert_eq!(size.width(), 0);

        // Test default JSON
        let expected_json = r#"{"height":0,"width":0}"#;
        test_json_serialization(&size, expected_json).unwrap();
    }

    #[test]
    fn test_size_builder_methods() {
        let size = Size::new(100, 200).with_width(300).with_height(400);

        assert_eq!(size.width(), 300);
        assert_eq!(size.height(), 400);

        // Test serialization after builder usage
        let deserialized = test_serialization_roundtrip(&size).unwrap();
        assert_eq!(size, deserialized);
    }

    #[test]
    fn test_size_getters() {
        let size = Size::new(1920, 1080);

        assert_eq!(size.height(), 1920);
        assert_eq!(size.width(), 1080);

        // Test serialization preserves values
        let deserialized = test_serialization_roundtrip(&size).unwrap();
        assert_eq!(deserialized.height(), 1920);
        assert_eq!(deserialized.width(), 1080);
    }

    #[test]
    fn test_size_deserialization_from_json() {
        let json = r#"{"height":1024,"width":768}"#;
        let expected_size = Size::new(1024, 768);
        test_json_deserialization(json, &expected_size).unwrap();
    }

    #[test]
    fn test_size_edge_cases() {
        // Test with zero values
        let zero_size = Size::new(0, 0);
        let deserialized = test_serialization_roundtrip(&zero_size).unwrap();
        assert_eq!(zero_size, deserialized);

        // Test with maximum values
        let max_size = Size::new(u16::MAX, u16::MAX);
        let deserialized = test_serialization_roundtrip(&max_size).unwrap();
        assert_eq!(max_size, deserialized);

        // Test with minimum values (should be 0 for u16)
        let min_size = Size::new(u16::MIN, u16::MIN);
        let deserialized = test_serialization_roundtrip(&min_size).unwrap();
        assert_eq!(min_size, deserialized);
    }

    #[test]
    fn test_size_equality() {
        let size1 = Size::new(800, 600);
        let size2 = Size::new(800, 600);
        let size3 = Size::new(600, 800);

        assert_eq!(size1, size2);
        assert_ne!(size1, size3);

        // Test hash consistency
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        size1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        size2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_size_chain_operations() {
        let initial = Size::default();
        let final_size = initial
            .with_width(200)
            .with_height(300)
            .with_width(400) // Should override previous width
            .with_height(500); // Should override previous height

        assert_eq!(final_size.width(), 400);
        assert_eq!(final_size.height(), 500);

        // Test serialization after chaining
        let deserialized = test_serialization_roundtrip(&final_size).unwrap();
        assert_eq!(final_size, deserialized);
    }

    #[test]
    fn test_size_policy_deserialization_from_json() {
        // Test individual size policy deserialization
        test_json_deserialization(r#""Fixed""#, &SizePolicy::Fixed).unwrap();
        test_json_deserialization(r#""Fill""#, &SizePolicy::Fill).unwrap();
        test_json_deserialization(r#""Fit""#, &SizePolicy::Fit).unwrap();
    }

    #[test]
    fn test_size_common_resolutions() {
        let common_sizes = vec![
            Size::new(1920, 1080), // Full HD
            Size::new(1280, 720),  // HD
            Size::new(3840, 2160), // 4K
            Size::new(800, 600),   // SVGA
            Size::new(1024, 768),  // XGA
        ];

        for size in common_sizes {
            let deserialized = test_serialization_roundtrip(&size).unwrap();
            assert_eq!(size, deserialized);
        }
    }

    #[test]
    fn test_size_aspect_ratios() {
        // Test various aspect ratios
        let aspect_ratios = vec![
            Size::new(16, 9), // 16:9
            Size::new(4, 3),  // 4:3
            Size::new(21, 9), // 21:9 (ultrawide)
            Size::new(1, 1),  // 1:1 (square)
            Size::new(3, 2),  // 3:2
        ];

        for size in aspect_ratios {
            let deserialized = test_serialization_roundtrip(&size).unwrap();
            assert_eq!(size, deserialized);
        }
    }
}
