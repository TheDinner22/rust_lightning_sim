struct TwoDSpace {
    width: usize,
    height: usize,
    plane: Vec<Grid>,
}

struct Grid {
    is_wall: bool
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
