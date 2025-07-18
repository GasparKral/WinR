#[cfg(test)]
mod background_serialization_tests {
    use crate::core::components::properties::graphics::{
        background::*,
        color::Color,
        gradient::{Gradient, GradientType},
    };
    use super::super::test_utils::*;

    #[test]
    fn test_background_shape_serialization() {
        // Test Rectangle
        let rectangle = BackgroundShape::Rectangle;
        let deserialized = test_serialization_roundtrip(&rectangle).unwrap();
        assert_eq!(rectangle, deserialized);
        
        let expected_json = r#""Rectangle""#;
        test_json_serialization(&rectangle, expected_json).unwrap();
        
        // Test RoundedRectangle
        let rounded = BackgroundShape::RoundedRectangle { radius: 10.5 };
        let deserialized = test_serialization_roundtrip(&rounded).unwrap();
        assert_eq!(rounded, deserialized);
        
        let expected_json = r#"{"RoundedRectangle":{"radius":10.5}}"#;
        test_json_serialization(&rounded, expected_json).unwrap();
    }

    #[test]
    fn test_background_solid_serialization() {
        let background = Background::new_solid(
            Color::rgb(255, 128, 64),
            BackgroundShape::Rectangle
        );
        
        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&background).unwrap();
        assert_eq!(background, deserialized);
        
        // Verify the color and shape are accessible
        assert_eq!(background.shape(), &BackgroundShape::Rectangle);
    }

    #[test]
    fn test_background_gradient_serialization() {
        let gradient = Gradient::new(
            GradientType::Linear(45),
            vec![
                (Color::rgb(255, 0, 0), 0),
                (Color::rgb(0, 0, 255), 100),
            ]
        );
        
        let background = Background::new_gradient(
            gradient,
            BackgroundShape::RoundedRectangle { radius: 15.0 }
        );
        
        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&background).unwrap();
        assert_eq!(background, deserialized);
        
        // Verify the shape is accessible
        assert_eq!(background.shape(), &BackgroundShape::RoundedRectangle { radius: 15.0 });
    }

    #[test]
    fn test_background_with_various_colors() {
        let colors = vec![
            Color::rgb(255, 0, 0),
            Color::rgba(0, 255, 0, 128),
            Color::hsv(240, 100, 100),
            Color::hex("#FFFF00"),
        ];
        
        for color in colors {
            let background = Background::new_solid(color, BackgroundShape::Rectangle);
            let deserialized = test_serialization_roundtrip(&background).unwrap();
            assert_eq!(background, deserialized);
        }
    }

    #[test]
    fn test_background_with_various_shapes() {
        let shapes = vec![
            BackgroundShape::Rectangle,
            BackgroundShape::RoundedRectangle { radius: 0.0 },
            BackgroundShape::RoundedRectangle { radius: 5.5 },
            BackgroundShape::RoundedRectangle { radius: 100.0 },
        ];
        
        for shape in shapes {
            let background = Background::new_solid(Color::rgb(255, 255, 255), shape);
            let deserialized = test_serialization_roundtrip(&background).unwrap();
            assert_eq!(background, deserialized);
        }
    }

    #[test]
    fn test_background_with_complex_gradient() {
        let gradient = Gradient::new(
            GradientType::Radial((50, 50), 25),
            vec![
                (Color::rgba(255, 255, 255, 255), 0),
                (Color::rgba(255, 0, 0, 200), 25),
                (Color::rgba(0, 255, 0, 150), 50),
                (Color::rgba(0, 0, 255, 100), 75),
                (Color::rgba(0, 0, 0, 0), 100),
            ]
        );
        
        let background = Background::new_gradient(
            gradient,
            BackgroundShape::RoundedRectangle { radius: 20.0 }
        );
        
        // Test roundtrip
        let deserialized = test_serialization_roundtrip(&background).unwrap();
        assert_eq!(background, deserialized);
    }

    #[test]
    fn test_background_deserialization_from_json() {
        // Test solid background deserialization
        let solid_json = r#"{"color":{"Solid":{"RGB":{"r":255,"g":128,"b":64}}},"shape":"Rectangle"}"#;
        let expected_background = Background::new_solid(
            Color::rgb(255, 128, 64),
            BackgroundShape::Rectangle
        );
        test_json_deserialization(solid_json, &expected_background).unwrap();
        
        // Test gradient background deserialization
        let gradient_json = r#"{"color":{"Gradient":{"gradient_type":{"Linear":45},"stops":[{"RGB":{"r":255,"g":0,"b":0}},0],[{"RGB":{"r":0,"g":0,"b":255}},100]]}},"shape":{"RoundedRectangle":{"radius":15.0}}}"#;
        let gradient = Gradient::new(
            GradientType::Linear(45),
            vec![
                (Color::rgb(255, 0, 0), 0),
                (Color::rgb(0, 0, 255), 100),
            ]
        );
        let expected_background = Background::new_gradient(
            gradient,
            BackgroundShape::RoundedRectangle { radius: 15.0 }
        );
        test_json_deserialization(gradient_json, &expected_background).unwrap();
    }

    #[test]
    fn test_background_edge_cases() {
        // Test with extreme radius values
        let extreme_radius = BackgroundShape::RoundedRectangle { radius: f32::MAX };
        let background = Background::new_solid(Color::rgb(0, 0, 0), extreme_radius);
        let deserialized = test_serialization_roundtrip(&background).unwrap();
        assert_eq!(background, deserialized);
        
        // Test with negative radius (should still serialize/deserialize)
        let negative_radius = BackgroundShape::RoundedRectangle { radius: -10.0 };
        let background = Background::new_solid(Color::rgb(255, 255, 255), negative_radius);
        let deserialized = test_serialization_roundtrip(&background).unwrap();
        assert_eq!(background, deserialized);
    }

    #[test]
    fn test_background_shape_hash_consistency() {
        // Test that shapes with same values hash the same
        let shape1 = BackgroundShape::RoundedRectangle { radius: 10.0 };
        let shape2 = BackgroundShape::RoundedRectangle { radius: 10.0 };
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        shape1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        shape2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        assert_eq!(hash1, hash2);
        
        // Test serialization consistency
        let background1 = Background::new_solid(Color::rgb(100, 100, 100), shape1);
        let background2 = Background::new_solid(Color::rgb(100, 100, 100), shape2);
        assert_eq!(background1, background2);
    }
}