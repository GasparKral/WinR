//! Tests unitarios de serialización para el proyecto WinR
//!
//! Este módulo contiene tests para verificar que todas las estructuras
//! serializables funcionen correctamente con serde.

pub mod background_tests;
pub mod border_tests;
pub mod color_tests;
pub mod gradient_tests;
pub mod margin_tests;
pub mod overflow_tests;
pub mod padding_tests;
pub mod position_tests;
pub mod size_tests;

#[cfg(test)]
mod test_utils {
    use serde::{Deserialize, Serialize};

    /// Utilidad para testear la serialización y deserialización de cualquier tipo
    pub fn test_serialization_roundtrip<T>(value: &T) -> Result<T, Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        // Serializar a JSON
        let json_string = serde_json::to_string(value)?;

        // Deserializar desde JSON
        let deserialized: T = serde_json::from_str(&json_string)?;

        Ok(deserialized)
    }

    /// Utilidad para testear la serialización a JSON y comparar con el resultado esperado
    pub fn test_json_serialization<T>(
        value: &T,
        expected_json: &str,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize + std::fmt::Debug,
    {
        let json_string = serde_json::to_string(value)?;
        let actual: serde_json::Value = serde_json::from_str(&json_string)?;
        let expected: serde_json::Value = serde_json::from_str(expected_json)?;

        assert_eq!(actual, expected, "JSON serialization mismatch");
        Ok(())
    }

    /// Utilidad para testear la deserialización desde JSON
    pub fn test_json_deserialization<T>(
        json: &str,
        expected_value: &T,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug + PartialEq,
    {
        let deserialized: T = serde_json::from_str(json)?;
        assert_eq!(
            &deserialized, expected_value,
            "JSON deserialization mismatch"
        );
        Ok(())
    }
}
