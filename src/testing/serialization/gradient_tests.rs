#[cfg(test)]
mod gradient_serialization_tests {
    use super::super::test_utils::*;
    use crate::core::components::properties::graphics::{color::Color, gradient::*};

    #[test]
    fn test_gradient_type_linear_serialization() {
        let gradient_type = GradientType::Linear(45);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient_type).unwrap();
        assert_eq!(gradient_type, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"Linear":45}"#;
        test_json_serialization(&gradient_type, expected_json).unwrap();
    }

    #[test]
    fn test_gradient_type_radial_serialization() {
        let gradient_type = GradientType::Radial((100, 200), 50);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient_type).unwrap();
        assert_eq!(gradient_type, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"Radial":[[100,200],50]}"#;
        test_json_serialization(&gradient_type, expected_json).unwrap();
    }

    #[test]
    fn test_gradient_type_conic_serialization() {
        let gradient_type = GradientType::Conic(90);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient_type).unwrap();
        assert_eq!(gradient_type, deserialized);

        // Test specific JSON output
        let expected_json = r#"{"Conic":90}"#;
        test_json_serialization(&gradient_type, expected_json).unwrap();
    }

    #[test]
    fn test_gradient_type_constructors() {
        // Test linear constructor
        let linear = GradientType::lienar(45);
        assert!(linear.is_linear());
        assert!(!linear.is_radial());
        assert!(!linear.is_conic());

        // Test radial constructor
        let radial = GradientType::radial((100, 200), 50);
        assert!(!radial.is_linear());
        assert!(radial.is_radial());
        assert!(!radial.is_conic());

        // Test conic constructor
        let conic = GradientType::conic(90);
        assert!(!conic.is_linear());
        assert!(!conic.is_radial());
        assert!(conic.is_conic());

        // Test serialization of constructed types
        let deserialized_linear = test_serialization_roundtrip(&linear).unwrap();
        assert_eq!(linear, deserialized_linear);

        let deserialized_radial = test_serialization_roundtrip(&radial).unwrap();
        assert_eq!(radial, deserialized_radial);

        let deserialized_conic = test_serialization_roundtrip(&conic).unwrap();
        assert_eq!(conic, deserialized_conic);
    }

    #[test]
    fn test_gradient_serialization() {
        let stops = vec![
            (Color::rgb(255, 0, 0), 0),
            (Color::rgb(0, 255, 0), 50),
            (Color::rgb(0, 0, 255), 100),
        ];

        let gradient = Gradient::new(GradientType::Linear(45), stops.clone());

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);

        // Test specific JSON structure
        let expected_json = r#"{"gradient_type":{"Linear":45},"stops":[{"RGB":{"r":255,"g":0,"b":0}},0],[{"RGB":{"r":0,"g":255,"b":0}},50],[{"RGB":{"r":0,"g":0,"b":255}},100]]}"#;
        test_json_serialization(&gradient, expected_json).unwrap();
    }

    #[test]
    fn test_gradient_with_radial_type() {
        let stops = vec![
            (Color::rgba(255, 255, 255, 255), 0),
            (Color::rgba(0, 0, 0, 0), 100),
        ];

        let gradient = Gradient::new(GradientType::Radial((50, 50), 25), stops);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);
    }

    #[test]
    fn test_gradient_with_conic_type() {
        let stops = vec![
            (Color::hex("#FF0000"), 0),
            (Color::hex("#00FF00"), 33),
            (Color::hex("#0000FF"), 66),
            (Color::hex("#FF0000"), 100),
        ];

        let gradient = Gradient::new(GradientType::Conic(0), stops);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);
    }

    #[test]
    fn test_gradient_with_mixed_color_types() {
        let stops = vec![
            (Color::rgb(255, 0, 0), 0),
            (Color::rgba(0, 255, 0, 128), 25),
            (Color::hsv(240, 100, 100), 50),
            (Color::hex("#FFFF00"), 75),
            (Color::rgb(255, 0, 255), 100),
        ];

        let gradient = Gradient::new(GradientType::Linear(90), stops);

        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);
    }

    #[test]
    fn test_gradient_deserialization_from_json() {
        // Test linear gradient deserialization
        let linear_json = r#"{"gradient_type":{"Linear":45},"stops":[{"RGB":{"r":255,"g":0,"b":0}},0],[{"RGB":{"r":0,"g":255,"b":0}},50],[{"RGB":{"r":0,"g":0,"b":255}},100]]}"#;
        let stops = vec![
            (Color::rgb(255, 0, 0), 0),
            (Color::rgb(0, 255, 0), 50),
            (Color::rgb(0, 0, 255), 100),
        ];
        let expected_gradient = Gradient::new(GradientType::Linear(45), stops);
        test_json_deserialization(linear_json, &expected_gradient).unwrap();

        // Test radial gradient deserialization
        let radial_json = r#"{"gradient_type":{"Radial":[[100,200],50]},"stops":[{"RGBA":{"r":255,"g":255,"b":255,"a":255}},0],[{"RGBA":{"r":0,"g":0,"b":0,"a":0}},100]]}"#;
        let stops = vec![
            (Color::rgba(255, 255, 255, 255), 0),
            (Color::rgba(0, 0, 0, 0), 100),
        ];
        let expected_gradient = Gradient::new(GradientType::Radial((100, 200), 50), stops);
        test_json_deserialization(radial_json, &expected_gradient).unwrap();

        // Test conic gradient deserialization
        let conic_json = r##"{"gradient_type":{"Conic":90},"stops":[{"HEX":{"value":"#FF0000"}},0],[{"HEX":{"value":"#00FF00"}},50],[{"HEX":{"value":"#0000FF"}},100]]}"##;
        let stops = vec![
            (Color::hex("#FF0000"), 0),
            (Color::hex("#00FF00"), 50),
            (Color::hex("#0000FF"), 100),
        ];
        let expected_gradient = Gradient::new(GradientType::Conic(90), stops);
        test_json_deserialization(conic_json, &expected_gradient).unwrap();
    }

    #[test]
    fn test_gradient_edge_cases() {
        // Test gradient with single color stop
        let single_stop = vec![(Color::rgb(255, 255, 255), 0)];
        let gradient = Gradient::new(GradientType::Linear(0), single_stop);
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);

        // Test gradient with many color stops
        let many_stops: Vec<(Color, u16)> = (0..10)
            .map(|i| (Color::rgb(i * 25, i * 25, i * 25), (i * 10) as u16))
            .collect();
        let gradient = Gradient::new(GradientType::Linear(180), many_stops);
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);

        // Test gradient with extreme angle values
        let extreme_angle_gradient = Gradient::new(
            GradientType::Linear(-360),
            vec![(Color::rgb(0, 0, 0), 0), (Color::rgb(255, 255, 255), 100)],
        );
        let deserialized = test_serialization_roundtrip(&extreme_angle_gradient).unwrap();
        assert_eq!(extreme_angle_gradient, deserialized);
    }

    #[test]
    fn test_gradient_type_boundary_values() {
        // Test with minimum and maximum i16 values
        let max_linear = GradientType::Linear(i16::MAX);
        let min_linear = GradientType::Linear(i16::MIN);

        let deserialized_max = test_serialization_roundtrip(&max_linear).unwrap();
        assert_eq!(max_linear, deserialized_max);

        let deserialized_min = test_serialization_roundtrip(&min_linear).unwrap();
        assert_eq!(min_linear, deserialized_min);

        // Test radial with extreme values
        let extreme_radial = GradientType::Radial((i16::MAX, i16::MIN), i16::MAX);
        let deserialized_radial = test_serialization_roundtrip(&extreme_radial).unwrap();
        assert_eq!(extreme_radial, deserialized_radial);
    }

    #[test]
    fn test_gradient_stops_boundary_values() {
        // Test with position values at boundaries (0 and 100)
        let boundary_stops = vec![(Color::rgb(255, 0, 0), 0), (Color::rgb(0, 255, 0), 100)];

        let gradient = Gradient::new(GradientType::Linear(45), boundary_stops);
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);
    }

    #[test]
    fn test_empty_gradient_stops() {
        // Test gradient with empty stops vector
        let empty_stops: Vec<(Color, u16)> = vec![];
        let gradient = Gradient::new(GradientType::Linear(0), empty_stops);
        let deserialized = test_serialization_roundtrip(&gradient).unwrap();
        assert_eq!(gradient, deserialized);
    }
}
