extern crate edit_distance;

use crate::distance::path::{PathMultiCost, MAX_DIMENSION};
use num_traits::Bounded;
use std::cmp::min;

pub type EditDistance = u8;

//This method returns a Path with ordering and additivity properties
//This is not a distance since it does not respect the triangular inequality
pub fn path_cost(w1: &str, w2: &str) -> PathMultiCost<EditDistance> {
    match edit_distance::edit_distance(w1, w2) {
        0 => PathMultiCost::<EditDistance>::min_value(),
        n => PathMultiCost::new(1 as EditDistance, min(n, MAX_DIMENSION) - 1),
    }
}

pub fn edit_distance(w1: &str, w2: &str) -> PathMultiCost<EditDistance> {
    PathMultiCost::new(edit_distance::edit_distance(w1, w2) as EditDistance, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // subadditivity is not respected for path cost
    // because we want to advantage path minimizing edit distance between each word
    // We prefer the path adrien -> adri -> adr to the path adrien -> adr
    fn triangular_inequality_not_true() {
        assert!(
            path_cost("adrien", "adri") + path_cost("adri", "adr") < path_cost("adrien", "adr")
        );
    }

    extern crate quickcheck;
    use quickcheck::quickcheck;

    quickcheck! {
        //Add some property-based testing on
        fn triangular_innequality_prop(a: String, b: String, c: String) -> bool {
            edit_distance(&a, &b) + edit_distance(&b, &c) >= edit_distance(&a, &c)
        }
        fn heuristic_prop(a: String, b: String) -> bool {
            path_cost(&a, &b) >= edit_distance(&a, &b)
        }
    }
}
