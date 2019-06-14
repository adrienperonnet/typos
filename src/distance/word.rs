extern crate edit_distance;

use crate::distance::path::{PathMultiCost, MAX_DIMENSION};
use pathfinding::num_traits::Bounded;
use std::cmp::min;

//This method returns a Path with ordering and additivity properties
//This is not a distance since it does not respect the triangular inequality
pub fn path_cost(w1: &str, w2: &str) -> PathMultiCost<usize> {
    match edit_distance::edit_distance(w1, w2) {
        0 => PathMultiCost::<usize>::min_value(),
        n => PathMultiCost::new(1, min(n, MAX_DIMENSION) - 1),
    }
}

pub fn edit_distance(w1: &str, w2: &str) -> PathMultiCost<usize> {
    PathMultiCost::new(
        min(edit_distance::edit_distance(w1, w2), MAX_DIMENSION - 1),
        0,
    )
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
}
