
/*

# match finding algorithm
## match count
## multi-way match

# falling logic
## environmental effects
## physics: explosion, implosion, freezing

# piece placement
## random generation
## weighted piece selection
## optimize for large drops

*/

#[derive(Debug,Clone,Copy,PartialEq)]
enum MatchType {
	None,
	Sequential,
	Multiway,
	Bonus,
}

#[derive(Debug,Clone,Copy,PartialEq)]
struct PieceMatch {
	match_type: MatchType,
	coordinates: Vec<(int, int)>,
}


#[derive(Debug,Clone,Copy,PartialEq)]
enum FinderState {
	None,
	Ready,
	Done,
}

#[derive(Debug,Clone,Copy,PartialEq)]
struct MatchFinder {
	state: FinderState,
	matches: Vec<PieceMatch>,
}



mod tests {

}