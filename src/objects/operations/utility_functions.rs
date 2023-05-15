use crate::prelude::*;
/// smooth maximum unit [wiki article](https://en.wikipedia.org/wiki/Smooth_maximum#Smooth_maximum_unit)
#[inline]
pub fn smooth_maximum_unit(a: f32, b: f32, epsilon: f32) -> f32 {
    (a + b + ((a - b).powi(2) + epsilon).sqrt()) / 2.
}

// finds either the two highest or smallest distances from the position to each of the objects. If
// looking_for_min it returns the smallest and second_smallest, if not, the largest and second
// largest
pub fn best_two_distances(
    objects: &Vec<Box<dyn Object3D>>,
    position: &Vector,
    looking_for_min: bool,
) -> (f32, f32) {
    let (best_distance, second_best_distance) = objects
        .iter()
        .map(|obj| {
            if looking_for_min {
                obj.signed_distance_function(position)
            } else {
                -obj.signed_distance_function(position)
            }
        })
        .fold(
            (f32::MAX, f32::MAX),
            |(best, second_best), current_distance| {
                let mut best_local = best;
                let mut second_best_local = second_best;

                if current_distance < best_local {
                    second_best_local = best_local;
                    best_local = current_distance;
                } else if current_distance < second_best_local {
                    second_best_local = current_distance;
                }
                (best_local, second_best_local)
            },
        );
    if looking_for_min {
        return (best_distance, second_best_distance);
    } else {
        (-best_distance, -second_best_distance)
    }
}
