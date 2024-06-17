use array2::Array2;
use crate::component::Component;

/// The product of a discrete cosine transformation on component pixels.
#[derive(Clone)]
pub struct CosineBlock {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub pb_avg: f32,
    pub pr_avg: f32,
}

/// Use the discrete cosine transformation to convert a 2x2 block of component pixels to cosine space
/// (a, b, c, d, Pb_avg, Pr_avg).
///
/// Ranges:
/// * a: [0, 1]
/// * b: [-0.5, 0.5]
/// * c: [-0.5, 0.5]
/// * d: [-0.5, 0.5]
/// * Pb_avg: [-0.5, 0.5]
/// * Pr_avg: [-0.5, 0.5]
///
/// # Arguments
/// * `block`: a 2x2 `Array2` of component pixels
pub fn to_cosine_space(block: Array2<Component>) -> CosineBlock {
    let y = block.iter_row_major()
        .map(|component| component.y)
        .collect::<Vec<f32>>();

    let a = (y[3] + y[2] + y[1] + y[0]) / 4.0;
    let b = (y[3] + y[2] - y[1] - y[0]) / 4.0;
    let c = (y[3] - y[2] + y[1] - y[0]) / 4.0;
    let d = (y[3] - y[2] - y[1] + y[0]) / 4.0;

    let pb_avg = block.iter_row_major()
        .map(|component| component.pb)
        .reduce(|acc, e| acc + e)
        .unwrap()
        / 4.0;

    let pr_avg = block.iter_row_major()
        .map(|component| component.pr)
        .reduce(|acc, e| acc + e)
        .unwrap()
        / 4.0;

    CosineBlock { a, b, c, d, pb_avg, pr_avg }
}

/// Return a cosine space block to pixel space. Produces an `Array2` of component pixels.
///
/// # Arguments
/// * `cosine_block`: a cosine space block
pub fn to_pixel_space(cosine_block: CosineBlock) -> Array2<Component> {
    let CosineBlock { a, b, c, d, pb_avg, pr_avg } = cosine_block;

    let y: Vec<f32> = vec![
        a - b - c + d,
        a - b + c - d,
        a + b - c - d,
        a + b + c + d,
    ];

    Array2::from_row_major(
        y.iter().map(|y| Component { y: *y, pb: pb_avg, pr: pr_avg }).collect(),
        2,
    )
}

#[cfg(test)]
mod tests {
    use array2::Array2;
    use assert_float_eq::assert_float_absolute_eq;
    use crate::component::Component;
    use crate::dct::{CosineBlock, to_cosine_space, to_pixel_space};

    #[test]
    fn _dct() {
        let block = vec![
            Component { y: 0.5, pb: 0.0, pr: 0.3 },
            Component { y: 1.0, pb: -0.1, pr: 0.2 },
            Component { y: 0.0, pb: -0.5, pr: 0.1 },
            Component { y: 0.7, pb: 0.4, pr: 0.0 },
        ];

        let array = Array2::from_row_major(block, 2);

        let expected_output = CosineBlock {
            a: 0.55,
            b: -0.2,
            c: 0.3,
            d: 0.05,
            pb_avg: -0.05,
            pr_avg: 0.15,
        };

        let output = to_cosine_space(array);

        assert_float_absolute_eq!(expected_output.a, output.a);
        assert_float_absolute_eq!(expected_output.b, output.b);
        assert_float_absolute_eq!(expected_output.c, output.c);
        assert_float_absolute_eq!(expected_output.d, output.d);
        assert_float_absolute_eq!(expected_output.pb_avg, output.pb_avg);
        assert_float_absolute_eq!(expected_output.pr_avg, output.pr_avg);
    }

    #[test]
    fn _undo_dct() {
        let dct_block = CosineBlock {
            a: 0.55,
            b: -0.2,
            c: 0.3,
            d: 0.05,
            pb_avg: -0.05,
            pr_avg: 0.15,
        };

        let output = to_pixel_space(dct_block.clone());

        assert_float_absolute_eq!(output.get(0, 0).unwrap().y, 0.5);
        assert_float_absolute_eq!(output.get(0, 1).unwrap().y, 1.0);
        assert_float_absolute_eq!(output.get(1, 0).unwrap().y, 0.0);
        assert_float_absolute_eq!(output.get(1, 1).unwrap().y, 0.7);

        for component in output.iter_row_major() {
            assert_float_absolute_eq!(component.pb, &dct_block.pb_avg);
            assert_float_absolute_eq!(component.pr, &dct_block.pr_avg);
        }
    }
}
