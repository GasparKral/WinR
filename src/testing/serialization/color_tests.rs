#[cfg(test)]
mod color_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::graphics::color::*;

    #[test]
    fn test_rgba_serialization() {
        let rgba = RGBA::new(255, 128, 64, 200);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&rgba).unwrap();
        assert_eq!(rgba, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"r":255,"g":128,"b":64,"a":200}"#;
        test_json_serialization(&rgba, expected_json).unwrap();
    }

    #[test]
    fn test_rgb_serialization() {
        let rgb = RGB::new(255, 128, 64);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&rgb).unwrap();
        assert_eq!(rgb, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"r":255,"g":128,"b":64}"#;
        test_json_serialization(&rgb, expected_json).unwrap();
    }

    #[test]
    fn test_hsv_serialization() {
        let hsv = HSV::new(120, 100, 80);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&hsv).unwrap();
        assert_eq!(hsv, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"h":120,"s":100,"v":80}"#;
        test_json_serialization(&hsv, expected_json).unwrap();
    }

    #[test]
    fn test_hex_serialization() {
        let hex = HEX::new("#FF8040");

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&hex).unwrap();
        assert_eq!(hex, deserialized);

        // Test specific JSON output
        let expected_json = r##"{"value":"#FF8040"}"##;
        test_json_serialization(&hex, expected_json).unwrap();
    }

    #[test]
    fn test_color_enum_serialization() {
        // Test RGBA variant
        let color_rgba = Color::rgba(255, 128, 64, 200);
        let deserialized_rgba = test_serialization_roundtrip(&color_rgba).unwrap();
        assert_eq!(color_rgba, deserialized_rgba);

        // Test RGB variant
        let color_rgb = Color::rgb(255, 128, 64);
        let deserialized_rgb = test_serialization_roundtrip(&color_rgb).unwrap();
        assert_eq!(color_rgb, deserialized_rgb);

        // Test HSV variant
        let color_hsv = Color::hsv(120, 100, 80);
        let deserialized_hsv = test_serialization_roundtrip(&color_hsv).unwrap();
        assert_eq!(color_hsv, deserialized_hsv);

        // Test HEX variant
        let color_hex = Color::hex("#FF8040");
        let deserialized_hex = test_serialization_roundtrip(&color_hex).unwrap();
        assert_eq!(color_hex, deserialized_hex);
    }

    #[test]
    fn test_color_enum_json_structure() {
        let color_rgba = Color::rgba(255, 128, 64, 200);
        let expected_json = r#"{"RGBA":{"r":255,"g":128,"b":64,"a":200}}"#;
        test_json_serialization(&color_rgba, expected_json).unwrap();

        let color_rgb = Color::rgb(255, 128, 64);
        let expected_json = r#"{"RGB":{"r":255,"g":128,"b":64}}"#;
        test_json_serialization(&color_rgb, expected_json).unwrap();

        let color_hsv = Color::hsv(120, 100, 80);
        let expected_json = r#"{"HSV":{"h":120,"s":100,"v":80}}"#;
        test_json_serialization(&color_hsv, expected_json).unwrap();

        let color_hex = Color::hex("#FF8040");
        let expected_json = r##"{"HEX":{"value":"#FF8040"}}"##;
        test_json_serialization(&color_hex, expected_json).unwrap();
    }

    #[test]
    fn test_color_deserialization_from_json() {
        // Test RGBA deserialization
        let rgba_json = r#"{"RGBA":{"r":255,"g":128,"b":64,"a":200}}"#;
        let expected_rgba = Color::rgba(255, 128, 64, 200);
        test_json_deserialization(rgba_json, &expected_rgba).unwrap();

        // Test RGB deserialization
        let rgb_json = r#"{"RGB":{"r":255,"g":128,"b":64}}"#;
        let expected_rgb = Color::rgb(255, 128, 64);
        test_json_deserialization(rgb_json, &expected_rgb).unwrap();

        // Test HSV deserialization
        let hsv_json = r#"{"HSV":{"h":120,"s":100,"v":80}}"#;
        let expected_hsv = Color::hsv(120, 100, 80);
        test_json_deserialization(hsv_json, &expected_hsv).unwrap();

        // Test HEX deserialization
        let hex_json = r##"{"HEX":{"value":"#FF8040"}}"##;
        let expected_hex = Color::hex("#FF8040");
        test_json_deserialization(hex_json, &expected_hex).unwrap();
    }

    #[test]
    fn test_rgba_from_tuple() {
        let rgba = RGBA::from((255, 128, 64, 200));
        let expected = RGBA::new(255, 128, 64, 200);
        assert_eq!(rgba, expected);

        // Test serialization after creation from tuple
        let deserialized = test_serialization_roundtrip(&rgba).unwrap();
        assert_eq!(rgba, deserialized);
    }

    #[test]
    fn test_rgb_from_tuple() {
        let rgb = RGB::from((255, 128, 64));
        let expected = RGB::new(255, 128, 64);
        assert_eq!(rgb, expected);

        // Test serialization after creation from tuple
        let deserialized = test_serialization_roundtrip(&rgb).unwrap();
        assert_eq!(rgb, deserialized);
    }

    #[test]
    fn test_hsv_from_tuple() {
        let hsv = HSV::from((120, 100, 80));
        let expected = HSV::new(120, 100, 80);
        assert_eq!(hsv, expected);

        // Test serialization after creation from tuple
        let deserialized = test_serialization_roundtrip(&hsv).unwrap();
        assert_eq!(hsv, deserialized);
    }

    #[test]
    fn test_rgb_to_rgba_conversion() {
        let rgb = RGB::new(255, 128, 64);
        let rgba = rgb.to_rgba(200);
        let expected = RGBA::new(255, 128, 64, 200);
        assert_eq!(rgba, expected);

        // Test serialization after conversion
        let deserialized = test_serialization_roundtrip(&rgba).unwrap();
        assert_eq!(rgba, deserialized);
    }

    #[test]
    fn test_edge_cases() {
        // Test with minimum values
        let rgba_min = RGBA::new(0, 0, 0, 0);
        let deserialized_min = test_serialization_roundtrip(&rgba_min).unwrap();
        assert_eq!(rgba_min, deserialized_min);

        // Test with maximum values
        let rgba_max = RGBA::new(255, 255, 255, 255);
        let deserialized_max = test_serialization_roundtrip(&rgba_max).unwrap();
        assert_eq!(rgba_max, deserialized_max);
    }

    #[test]
    fn test_hex_various_formats() {
        // Test different valid hex formats
        let hex_formats = vec![
            "#FF8040",
            "#ABC",
            "#ABCD1234",
            "0xFF8040",
            "0xABC",
            "0xABCD1234",
        ];

        for format in hex_formats {
            let hex = HEX::new(format);
            let deserialized = test_serialization_roundtrip(&hex).unwrap();
            assert_eq!(hex, deserialized);
        }
    }
}
