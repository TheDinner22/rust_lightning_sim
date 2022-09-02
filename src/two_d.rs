use rust_bfs::{HasId, RepresentsSpace, BfsAbleSpace};

// this shit is not working with bacon or neovide!!

struct TwoDSpace {
    width: i32,
    height: i32,
    squares = Vec<Square>
}

impl RepresentsSpace for TwoDSpace {

}

struct Square {
    isWall: bool
}
