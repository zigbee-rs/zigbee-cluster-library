//!
//! 4.4 Temperature Measurement Cluster
use core::convert::TryInto;
use heapless::Vec;

/// asdflkj
#[derive(Debug)]
pub struct TemperatureMeasurement {
    measured_value: i16,     // MeasuredValue in 0.01°C units
    min_measured_value: i16, // MinMeasuredValue
    max_measured_value: i16, // MaxMeasuredValue
    tolerance: u16,          // Tolerance (optional, set to 0 if not used)
}

impl TemperatureMeasurement {
    /// Create a new measurement with the minimum and maximum mea
    pub fn new(
        temperature_celsius: f32,
        min_temp: f32,
        max_temp: f32,
        tolerance: u16,
    ) -> Result<Self, &'static str> {
        if temperature_celsius < -273.15 || min_temp < -273.15 || max_temp < -273.15 {
            return Err("Temperature cannot be below absolute zero (-273.15°C)");
        }
        if min_temp > max_temp {
            return Err("Min temperature cannot be greater than max temperature");
        }
        if temperature_celsius < min_temp || temperature_celsius > max_temp {
            return Err("Measured temperature is out of the defined range");
        }

        let measured_value = (temperature_celsius * 100.0) as i16;
        let min_measured_value = (min_temp * 100.0) as i16;
        let max_measured_value = (max_temp * 100.0) as i16;

        Ok(Self {
            measured_value,
            min_measured_value,
            max_measured_value,
            tolerance,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8, 8> {
        let mut bytes = Vec::new();
        bytes
            .extend_from_slice(&self.measured_value.to_le_bytes())
            .unwrap();
        bytes
            .extend_from_slice(&self.min_measured_value.to_le_bytes())
            .unwrap();
        bytes
            .extend_from_slice(&self.max_measured_value.to_le_bytes())
            .unwrap();
        bytes
            .extend_from_slice(&self.tolerance.to_le_bytes())
            .unwrap();
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 8 {
            return Err("Invalid byte slice length");
        }

        let measured_value = i16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let min_measured_value = i16::from_le_bytes(bytes[2..4].try_into().unwrap());
        let max_measured_value = i16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let tolerance = u16::from_le_bytes(bytes[6..8].try_into().unwrap());

        Ok(Self {
            measured_value,
            min_measured_value,
            max_measured_value,
            tolerance,
        })
    }

    pub fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let bytes: Vec<u8, 8> = src.into_iter().collect();
        Self::from_bytes(&bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::measurement::temperature;

    use super::*;

    #[test]
    fn test_binary_as_measured_value() {
        // given
        let data = [0x0b, 0x8a];

        // when
        // let temperature = TemperatureMeasurement::unpack_from_slice(&data).unwrap();

        // then
        // assert_eq!(temperature, 29.54);
    }

    #[test]
    fn test_temperature_measurement() {
        let temp_measurement =
            TemperatureMeasurement::new(23.45, -10.0, 50.0, 5).expect("Initialization failed");
        let serialized = temp_measurement.to_bytes();
        let deserialized =
            TemperatureMeasurement::from_bytes(&serialized).expect("Deserialization failed");
        assert_eq!(temp_measurement.measured_value, deserialized.measured_value);
        assert_eq!(
            temp_measurement.min_measured_value,
            deserialized.min_measured_value
        );
        assert_eq!(
            temp_measurement.max_measured_value,
            deserialized.max_measured_value
        );
        assert_eq!(temp_measurement.tolerance, deserialized.tolerance);
    }
}
