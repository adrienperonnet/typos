use crate::distance::path::PathMultiCost;
use pathfinding::directed::astar;
use pathfinding::directed::dijkstra;
use pathfinding::directed::fringe;
use pathfinding::directed::idastar;
use pathfinding::num_traits::Zero;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

mod path;
mod word;

pub fn find_shortest_path<'a>(
    start: &'a str,
    stop: &'a str,
    words: &'a [&str],
    algorithm: &PathFindingAlorithm,
) -> Option<(Vec<&'a str>, path::PathMultiCost<usize>)> {
    let get_successors = |&current_word: &&'a str| {
        words
            .iter()
            .map(move |&successor| (successor, word::path_cost(current_word, &successor)))
    };

    let heuristic = |word: &&str| word::edit_distance(word, stop);
    let stop_condition = |word: &&str| word == &stop;
    match algorithm {
        PathFindingAlorithm::Astar => {
            astar::astar(&start, get_successors, heuristic, stop_condition)
        }
        PathFindingAlorithm::Idastar => {
            idastar::idastar(&start, get_successors, heuristic, stop_condition)
        }
        PathFindingAlorithm::Fringe => {
            fringe::fringe(&start, get_successors, heuristic, stop_condition)
        }
        PathFindingAlorithm::Dijkstra => dijkstra::dijkstra(&start, get_successors, stop_condition),
    }
}

/// Pathfinding algorithm supported
pub enum PathFindingAlorithm {
    Astar,
    Fringe,
    Idastar,
    Dijkstra,
}

impl fmt::Display for PathFindingAlorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            PathFindingAlorithm::Astar => "astar",
            PathFindingAlorithm::Fringe => "fringe",
            PathFindingAlorithm::Idastar => "idastar",
            PathFindingAlorithm::Dijkstra => "dijkstra",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for PathFindingAlorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<PathFindingAlorithm, ()> {
        match s {
            "astar" => Ok(PathFindingAlorithm::Astar),
            "fringe" => Ok(PathFindingAlorithm::Fringe),
            "idastar" => Ok(PathFindingAlorithm::Idastar),
            "dijkstra" => Ok(PathFindingAlorithm::Dijkstra),
            _ => Err(()),
        }
    }
}

// Display number of letter-changes from a path between two words.
impl<U: Display + Zero + PartialEq + Copy> Display for PathMultiCost<U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_cost().as_slice() {
            [] => write!(f, "0 mutation"),
            cost => write!(
                f,
                "{}",
                cost.iter()
                    .rev()
                    .map(|(v, count)| format!("{} {}-letter mutation", v, count))
                    .collect::<Vec<String>>()
                    .join(" + ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        test_compare("adrien", "adrien", vec![], (vec!["adrien"], vec![]));
    }

    #[test]
    fn path_with_two_words() {
        test_compare(
            "banane",
            "banana",
            vec!["table", "chaise", "tabouret", "assiette"],
            (vec!["banane", "banana"], vec![(1, 1)]),
        );
    }

    #[test]
    fn long_words() {
        test_compare(
            "abracadabrantesques",
            "petit",
            vec!["abracadabra"],
            (
                vec!["abracadabrantesques", "abracadabra", "petit"],
                vec![(1, 11), (1, 8)],
            ),
        );
    }

    #[test]
    fn integration() {
        test_compare(
            "banane",
            "ano",
            vec!["banan", "table", "chaise", "lit", "banon"],
            (
                vec!["banane", "banan", "banon", "ano"],
                vec![(1, 2), (2, 1)],
            ),
        );
    }

    #[test]
    // heuristic function h is admissible
    // path cost will always be bigger than the edit_distance
    // This guaranteed to find the shortest path
    fn heuristic_property_is_admissible() {
        assert!(word::path_cost("adrien", "adri") >= word::edit_distance("adrien", "adri"));
        assert!(
            word::path_cost("adrien", "adri") + word::path_cost("adri", "adr")
                >= word::edit_distance("adrien", "adri")
        );
        assert_eq!(
            word::path_cost("adrien", "adrien"),
            word::edit_distance("adrien", "adrien")
        );
    }

    fn test_compare<'a>(
        start: &str,
        stop: &'a str,
        mut words: Vec<&'a str>,
        expected: (Vec<&str>, Vec<(usize, usize)>),
    ) {
        words.insert(0, stop);
        let (expected_path, expected_cost) = expected;
        [
            PathFindingAlorithm::Astar,
            PathFindingAlorithm::Fringe,
            PathFindingAlorithm::Idastar,
            PathFindingAlorithm::Dijkstra,
        ]
        .iter()
        .for_each(
            |alg| match find_shortest_path(start, stop, words.as_slice(), alg) {
                Some((path, cost)) => {
                    assert_eq!(path, expected_path);
                    assert_eq!(cost.get_cost(), expected_cost);
                }
                None => panic!("no path found"),
            },
        )
    }
}
