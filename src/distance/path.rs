use pathfinding::num_traits::{Bounded, Zero};
use std::cmp::{min, Ord, Ordering};
use std::ops::{Add, AddAssign};

/*
In order to derive the Copy trait required by pathfinding api we use fixed size array for storage
Increasing this value with increase the memory required by the program but will support more hops.
*/
pub const MAX_DIMENSION: usize = 20;

/// A metric data for Path that supports different layers
#[derive(Debug, Copy, Clone, Eq)]
pub struct PathMultiCost<U> {
    data: [U; MAX_DIMENSION],
}

impl<U: Zero + PartialEq + Copy> PathMultiCost<U> {
    pub fn get_cost(self) -> Vec<(U, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter(|&u| *u.1 != U::zero())
            .map(|(k, v)| (*v, MAX_DIMENSION - k))
            .collect::<Vec<(U, usize)>>()
    }
}

impl<U: Zero + Copy> PathMultiCost<U> {
    pub fn new(cost: U, dimension: usize) -> PathMultiCost<U> {
        let mut data = [U::zero(); MAX_DIMENSION];
        data[min(MAX_DIMENSION - 1, MAX_DIMENSION - 1 - dimension)] = cost;
        return PathMultiCost { data };
    }
}

impl<U: Bounded + Copy + Zero> Bounded for PathMultiCost<U> {
    fn min_value() -> Self {
        return PathMultiCost::new(U::min_value(), 0);
    }
    fn max_value() -> Self {
        return PathMultiCost::new(U::max_value(), MAX_DIMENSION - 1);
    }
}

impl<U: PartialEq> PartialEq for PathMultiCost<U> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<U: Ord> Ord for PathMultiCost<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        for kv in self.data.iter().zip(other.data.iter()) {
            match kv.0.cmp(kv.1) {
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            }
        }
        return Ordering::Equal;
    }
}

impl<U: PartialOrd> PartialOrd for PathMultiCost<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.data.partial_cmp(&other.data);
    }
}

impl<U: Zero + Copy + AddAssign> Add for PathMultiCost<U> {
    type Output = PathMultiCost<U>;

    fn add(self, rhs: PathMultiCost<U>) -> Self::Output
    where
        U: Add,
    {
        let mut array = self.data;
        rhs.data.iter().enumerate().for_each(|(i, e)| {
            array[i] += *e;
        });
        return PathMultiCost::<U> { data: array };
    }
}

impl<U: Zero + Copy + AddAssign> Zero for PathMultiCost<U> {
    fn zero() -> Self {
        return PathMultiCost::new(U::zero(), 0);
    }
    fn is_zero(&self) -> bool {
        self.data
            .iter()
            .rev() //reverse to speedup because it's more likely that the last elements are not empty
            .all(|u| u.is_zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cost() {
        assert_eq!(cost([0, 5, 3]).get_cost(), vec![(5, 2), (3, 1)]);
        assert_eq!(cost([1, 5, 3]).get_cost(), vec![(1, 3), (5, 2), (3, 1)])
    }

    #[test]
    fn zero() {
        assert!(cost([0, 0, 0]).is_zero());
        assert!(!cost([0, 1, 0]).is_zero());
        assert!(!cost([1, 2, 3]).is_zero());
    }

    #[test]
    fn equality() {
        assert_ne!(cost([1, 2, 3]), cost([0, 1, 0]));
        assert_eq!(cost([0, 1, 0]), cost([0, 1, 0]));
    }

    #[test]
    fn zero_identity_element() {
        assert_eq!(PathMultiCost::zero() + cost([1, 2, 3]), cost([1, 2, 3]));
    }

    #[test]
    fn sum_commutative() {
        assert_eq!(
            cost([1, 2, 3]) + cost([4, 5, 6]),
            cost([4, 5, 6]) + cost([1, 2, 3])
        );
    }

    #[test]
    fn sum_associative() {
        assert_eq!(
            cost([1, 2, 3]) + (cost([4, 5, 6]) + cost([0, 1, 2])),
            (cost([1, 2, 3]) + cost([4, 5, 6])) + cost([0, 1, 2])
        );
    }

    #[test]
    fn sum() {
        assert_eq!(cost([0, 0, 1]) + cost([0, 0, 2]), cost([0, 0, 3]));
        assert_eq!(cost([0, 1, 0]) + cost([0, 5, 0]), cost([0, 6, 0]));
        assert_eq!(cost([1, 2, 3]) + cost([3, 2, 1]), cost([4, 4, 4]));
    }

    #[test]
    fn ordering_prefer_high_dimension() {
        assert!(cost([0, 0, 2]) > cost([0, 0, 1]));
        assert!(cost([0, 2, 0]) > cost([0, 0, 5]));
        assert!(cost([3, 0, 0]) > cost([2, 71, 88]));
    }

    #[test]
    fn subadditivity() {
        //f(x+y)<=f(x)+f(y)
        assert!(cost([3, 5, 4]) <= cost([2, 5, 4]) + cost([1, 5, 4]));
        assert!(cost([3, 5, 4]) <= cost([2, 3, 1]) + cost([1, 2, 3]));
    }

    fn cost(input: [i32; 3]) -> PathMultiCost<i32> {
        let mut data = [0; MAX_DIMENSION];
        data[MAX_DIMENSION - 1] = input[2];
        data[MAX_DIMENSION - 2] = input[1];
        data[MAX_DIMENSION - 3] = input[0];
        return PathMultiCost { data };
    }
}
