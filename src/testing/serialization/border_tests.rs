#[cfg(test)]
mod border_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::graphics::{border::*, color::Color};

    #[test]
    fn test_border_type_serialization() {
        let border_types = vec![
            BorderType::Solid,
            BorderType::Dashed,
            BorderType::Dotted,
            BorderType::Double,
        ];

        for border_type in border_types {
            let deserialized = test_serialization_roundtrip(&border_type).unwrap();
            assert_eq!(border_type, deserialized);
        }

        // Test specific JSON outputs
        test_json_serialization(&BorderType::Solid, r#""Solid""#).unwrap();
        test_json_serialization(&BorderType::Dashed, r#""Dashed""#).unwrap();
        test_json_serialization(&BorderType::Dotted, r#""Dotted""#).unwrap();
        test_json_serialization(&BorderType::Double, r#""Double""#).unwrap();
    }

    #[test]
    fn test_border_serialization() {
        let border = Border::new(Color::rgb(255, 0, 0), 2.5, BorderType::Solid);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&border).unwrap();
        assert_eq!(border, deserialized);

        // Test accessors
        assert_eq!(border.color(), &Color::rgb(255, 0, 0));
        assert_eq!(border.width(), 2.5);
        assert_eq!(border.border_type(), &BorderType::Solid);
    }

    #[test]
    fn test_border_with_various_colors() {
        let colors = vec![
            Color::rgb(255, 0, 0),
            Color::rgba(0, 255, 0, 128),
            Color::hsv(240, 100, 100),
            Color::hex("#FFFF00"),
        ];

        for color in colors {
            let border = Border::new(color, 1.0, BorderType::Solid);
            let deserialized = test_serialization_roundtrip(&border).unwrap();
            assert_eq!(border, deserialized);
        }
    }

    #[test]
    fn test_border_with_various_widths() {
        let widths = vec![0.0, 0.5, 1.0, 2.5, 10.0, 100.0, f32::MAX];

        for width in widths {
            let border = Border::new(Color::rgb(0, 0, 0), width, BorderType::Dashed);
            let deserialized = test_serialization_roundtrip(&border).unwrap();
            assert_eq!(border, deserialized);
        }
    }

    #[test]
    fn test_border_with_all_types() {
        let border_types = vec![
            BorderType::Solid,
            BorderType::Dashed,
            BorderType::Dotted,
            BorderType::Double,
        ];

        for border_type in border_types {
            let border = Border::new(Color::rgb(128, 128, 128), 2.0, border_type);
            let deserialized = test_serialization_roundtrip(&border).unwrap();
            assert_eq!(border, deserialized);
        }
    }

    #[test]
    fn test_border_json_structure() {
        let border = Border::new(Color::rgb(255, 128, 64), 3.5, BorderType::Dashed);

        let expected_json =
            r#"{"color":{"RGB":{"r":255,"g":128,"b":64}},"width":3.5,"border_type":"Dashed"}"#;
        test_json_serialization(&border, expected_json).unwrap();
    }

    #[test]
    fn test_border_deserialization_from_json() {
        // Test solid border deserialization
        let solid_json =
            r#"{"color":{"RGB":{"r":255,"g":0,"b":0}},"width":2.5,"border_type":"Solid"}"#;
        let expected_border = Border::new(Color::rgb(255, 0, 0), 2.5, BorderType::Solid);
        test_json_deserialization(solid_json, &expected_border).unwrap();

        // Test dashed border deserialization
        let dashed_json = r#"{"color":{"RGBA":{"r":0,"g":255,"b":0,"a":128}},"width":1.0,"border_type":"Dashed"}"#;
        let expected_border = Border::new(Color::rgba(0, 255, 0, 128), 1.0, BorderType::Dashed);
        test_json_deserialization(dashed_json, &expected_border).unwrap();

        // Test dotted border deserialization
        let dotted_json =
            r##"{"color":{"HEX":{"value":"#FF8040"}},"width":0.5,"border_type":"Dotted"}"##;
        let expected_border = Border::new(Color::hex("#FF8040"), 0.5, BorderType::Dotted);
        test_json_deserialization(dotted_json, &expected_border).unwrap();

        // Test double border deserialization
        let double_json =
            r#"{"color":{"HSV":{"h":240,"s":100,"v":100}},"width":4.0,"border_type":"Double"}"#;
        let expected_border = Border::new(Color::hsv(240, 100, 100), 4.0, BorderType::Double);
        test_json_deserialization(double_json, &expected_border).unwrap();
    }

    #[test]
    fn test_border_edge_cases() {
        // Test with zero width
        let zero_width = Border::new(Color::rgb(255, 255, 255), 0.0, BorderType::Solid);
        let deserialized = test_serialization_roundtrip(&zero_width).unwrap();
        assert_eq!(zero_width, deserialized);

        // Test with negative width (should still serialize/deserialize)
        let negative_width = Border::new(Color::rgb(0, 0, 0), -1.0, BorderType::Dashed);
        let deserialized = test_serialization_roundtrip(&negative_width).unwrap();
        assert_eq!(negative_width, deserialized);

        // Test with very small width
        let tiny_width = Border::new(
            Color::rgb(128, 128, 128),
            f32::MIN_POSITIVE,
            BorderType::Dotted,
        );
        let deserialized = test_serialization_roundtrip(&tiny_width).unwrap();
        assert_eq!(tiny_width, deserialized);
    }

    #[test]
    fn test_border_hash_consistency() {
        // Test that borders with same values hash the same
        let border1 = Border::new(Color::rgb(255, 0, 0), 2.5, BorderType::Solid);
        let border2 = Border::new(Color::rgb(255, 0, 0), 2.5, BorderType::Solid);

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        border1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        border2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
        assert_eq!(border1, border2);
    }

    #[test]
    fn test_border_inequality() {
        let base_border = Border::new(Color::rgb(255, 0, 0), 2.5, BorderType::Solid);

        // Different color
        let different_color = Border::new(Color::rgb(0, 255, 0), 2.5, BorderType::Solid);
        assert_ne!(base_border, different_color);

        // Different width
        let different_width = Border::new(Color::rgb(255, 0, 0), 3.0, BorderType::Solid);
        assert_ne!(base_border, different_width);

        // Different type
        let different_type = Border::new(Color::rgb(255, 0, 0), 2.5, BorderType::Dashed);
        assert_ne!(base_border, different_type);
    }

    #[test]
    fn test_border_complex_scenarios() {
        // Test border with RGBA color and decimal width
        let complex_border = Border::new(Color::rgba(255, 128, 64, 200), 1.75, BorderType::Double);

        let deserialized = test_serialization_roundtrip(&complex_border).unwrap();
        assert_eq!(complex_border, deserialized);

        // Verify all properties are preserved
        assert_eq!(deserialized.color(), &Color::rgba(255, 128, 64, 200));
        assert_eq!(deserialized.width(), 1.75);
        assert_eq!(deserialized.border_type(), &BorderType::Double);
    }

    #[test]
    fn test_border_type_from_json() {
        // Test individual border type deserialization
        test_json_deserialization(r#""Solid""#, &BorderType::Solid).unwrap();
        test_json_deserialization(r#""Dashed""#, &BorderType::Dashed).unwrap();
        test_json_deserialization(r#""Dotted""#, &BorderType::Dotted).unwrap();
        test_json_deserialization(r#""Double""#, &BorderType::Double).unwrap();
    }
}
