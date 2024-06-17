use csc411_image::Rgb;

/// A normalized RGB pixel.
#[derive(Clone)]
pub struct NormalizedRgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

/// Scale a set of RGB values between 0 and 1.
///
/// # Arguments
/// * `rgb`: the pixel to normalize
/// * `denominator`: the denominator of the corresponding PPM image
pub fn normalize(rgb: Rgb, denominator: u16) -> NormalizedRgb {
    NormalizedRgb {
        red: rgb.red as f32 / denominator as f32,
        green: rgb.green as f32 / denominator as f32,
        blue: rgb.blue as f32 / denominator as f32,
    }
}

/// Return a scaled RGB pixel to integer form. Assumes a PPM denominator of 255.
///
/// # Arguments
/// * `normalized`: the normalized RGB pixel
pub fn denormalize(normalized: NormalizedRgb) -> Rgb {
    let assumed_denominator = 255.0;

    Rgb {
        red: (normalized.red * assumed_denominator).round() as u16,
        green: (normalized.green * assumed_denominator).round() as u16,
        blue: (normalized.blue * assumed_denominator).round() as u16,
    }
}

#[cfg(test)]
mod tests {
    use csc411_image::Rgb;
    use crate::normalize::{normalize, denormalize, NormalizedRgb};

    #[test]
    fn _normalize() {
        let denominator = 100;

        let rgb = Rgb {
            red: 0,
            green: denominator / 2,
            blue: denominator,
        };

        assert_eq!(normalize(rgb.clone(), denominator).red, 0.0);
        assert_eq!(normalize(rgb.clone(), denominator).green, 0.5);
        assert_eq!(normalize(rgb.clone(), denominator).blue, 1.0);
    }

    #[test]
    fn _denormalize() {
        let normalized = NormalizedRgb {
            red: 0.2,
            green: 0.45,
            blue: 0.89,
        };

        assert_eq!(denormalize(normalized.clone()).red, 51);
        assert_eq!(denormalize(normalized.clone()).green, 115);
        assert_eq!(denormalize(normalized.clone()).blue, 227);
    }
}
