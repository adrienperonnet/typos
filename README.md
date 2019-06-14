# Context
(This tweet|https://twitter.com/dgryski/status/1125922800877883393) made me build this tool.

# Description
Compute the shortest distance between two words of a given set.
The result will be the path minimizing edit distance between each word in the path.
Result could be used to perform the social engineering idea of the tweet.

# Example
```shell
$wget https://raw.githubusercontent.com/smashew/NameDatabases/master/NamesDatabases/first%20names/all.txt

$./typos all.txt adrien pierre
Using input file: all.txt with astar algorithm to compute shortest path between adrien and pierre
19950 words loaded into memory
Shortest path found in 546.55645ms: adrien->adrian->adria->aria->iria->ira->iera->piera->piere->pierre (achieved in 9 1-letter mutation)

$./typos all.txt adrien maximilien
Using input file: all.txt with astar algorithm to compute shortest path between adrien and maximilien
19950 words loaded into memory
Shortest path found in 121.622294335s: adrien->adriel->ariel->mariel->marie->maxie->maxime->maxima->maximina->maximilia->maximilian->maximilien (achieved in 9 1-letter mutation + 2 2-letter mutation)

```

So, in practice, if you want to change a plane ticket from:
- Adrien to Pierre you'll have to make 9 phone calls to claim for a typo in your firstname
- Adrien to Maxime you'll have to make 11 phone calls to claim for a typo. 2 of those phone calls might be harder because they required a 2-letters change.


# Usage
```shell
$typos --help
Find a short path between two input words

USAGE:
    typos <INPUT> <START> <END> [ALGORITHM]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT>        Sets the input file to use
    <START>        starting word
    <END>          ending word
    <ALGORITHM>    algorithm to use to compute shortest path [default: astar]  [possible values: astar, idastar,
                   dijkstra, fringe]
```

# Disclaimer
This is a project done for fun, in order to learn Rust :)
Of course this is not intended for real-life use and **should not** be used to perform illegal activities.