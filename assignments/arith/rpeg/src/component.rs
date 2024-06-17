use crate::normalize::NormalizedRgb;

/// A component (Y, Pb, Pr) pixel.
#[derive(Clone)]
pub struct Component {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

/// Transform a normalized RGB pixel into a component (Y, Pb, Pr) pixel.
///
/// Ranges:
/// * Y: [0, 1]
/// * Pb: [-0.5, 0.5]
/// * Pr: [-0.5, 0.5]
///
/// # Arguments
/// * `rgb`: a normalized RGB pixel
pub fn rgb_to_component(rgb: NormalizedRgb) -> Component {
    let NormalizedRgb { red, green, blue } = rgb;
    Component {
        y: 0.299 * red + 0.587 * green + 0.114 * blue,
        pb: -0.168736 * red - 0.331264 * green + 0.5 * blue,
        pr: 0.5 * red - 0.418688 * green - 0.081312 * blue,
    }
}

/// Transform a component pixel to normalized RGB.
///
/// # Arguments
/// * `component`: a component pixel
pub fn component_to_rgb(component: Component) -> NormalizedRgb {
    let Component { y, pb, pr } = component;
    NormalizedRgb {
        red: 1.0 * y + 0.0 * pb + 1.402 * pr,
        green: 1.0 * y - 0.344136 * pb - 0.714136 * pr,
        blue: 1.0 * y + 1.772 * pb + 0.0 * pr,
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;
    use crate::component::{Component, rgb_to_component, component_to_rgb};
    use crate::normalize::NormalizedRgb;

    #[test]
    fn _rgb_to_component() {
        let (red, green, blue) = (0.6, 0.35, 1.0);
        let rgb = NormalizedRgb { red, green, blue };

        assert_float_absolute_eq!(rgb_to_component(rgb.clone()).y, 0.299 * red + 0.587 * green + 0.114 * blue);
        assert_float_absolute_eq!(rgb_to_component(rgb.clone()).pb, -0.168736 * red - 0.331264 * green + 0.5 * blue);
        assert_float_absolute_eq!(rgb_to_component(rgb.clone()).pr, 0.5 * red - 0.418688 * green - 0.081312 * blue);
    }

    #[test]
    fn _component_to_rgb() {
        let (y, pb, pr) = (0.75, -0.2, 0.015);
        let component = Component { y, pb, pr };

        assert_float_absolute_eq!(component_to_rgb(component.clone()).red, 1.0 * y + 0.0 * pb + 1.402 * pr);
        assert_float_absolute_eq!(component_to_rgb(component.clone()).green, 1.0 * y - 0.344136 * pb - 0.714136 * pr);
        assert_float_absolute_eq!(component_to_rgb(component.clone()).blue, 1.0 * y + 1.772 * pb + 0.0 * pr);
    }
}
