pub struct Board {
    pub grid: [[(i32, bool); 5]; 5],
    score: i32
}

impl Board {
    pub fn new(numbers: [[i32; 5]; 5]) -> Board {
        let mut grid : [[(i32, bool); 5]; 5] = [
            [(0, false), (0, false), (0, false), (0, false), (0, false)],
            [(0, false), (0, false), (0, false), (0, false), (0, false)],
            [(0, false), (0, false), (0, false), (0, false), (0, false)],
            [(0, false), (0, false), (0, false), (0, false), (0, false)],
            [(0, false), (0, false), (0, false), (0, false), (0, false)],
        ];

        for i in 0..5 {
            for j in 0..5 {
                grid[i][j] = (numbers[i][j], false);
            }
        }

        return Board {
            grid,
            score: 0
        };
    }


    fn check_number(&mut self, number: i32) {
        for i in 0..5 {
            for j in 0..5 {
                let (val, _) = self.grid[i][j];

                if val == number {
                    self.grid[i][j] = (val, true);
                }
            }
        }

        self.calculate_score();
    }

    /**
     * Works out the largest score on the board and sets it to the board's score parameter
     */
    fn calculate_score(&mut self) {
        let mut scores: Vec<i32> = vec![];

        let accumulator = |acc, (_, marked): &(i32, bool)| {
            if *marked {
                return acc + 1;
            }
            return acc;
        };
        for i in 0..5 {
            let horizontal = self.grid[i].iter().fold(0, accumulator);

            scores.push(horizontal);

            let vertical_line = [
                self.grid[0][i],
                self.grid[1][i],
                self.grid[2][i],
                self.grid[3][i],
                self.grid[4][i]
            ];
            let vertical = vertical_line.iter().fold(0, accumulator);

            scores.push(vertical);
        }

        scores.sort();
        scores.reverse();
        self.score = scores[0];
    }

    fn unmarked_sum(&self) -> i32 {
        return self.grid
            .iter()
            .fold(0, |acc, line| acc + line.iter().fold(0, |acc, (val, marked)| {
                if !*marked {
                    return acc + val;
                }
                return acc;
            }));
    }
}

pub fn find_winning_score(mut boards: Vec<Board>, numbers: Vec<i32>) -> i32 {
    for number in numbers {
        boards = boards.into_iter().map(|mut b| {
            b.check_number(number);
            return b;
        }).collect();

        boards.sort_by_cached_key(|f| f.score);
        boards.reverse();

        if boards[0].score == 5 {
            return boards[0].unmarked_sum() * number;
        }
    }
    return 0;
}

pub fn find_last_winning_score(mut boards: Vec<Board>, numbers: Vec<i32>) -> i32 {
    for number in numbers {
        boards = boards.into_iter().map(|mut b| {
            b.check_number(number);
            return b;
        }).collect();
        
        boards.sort_by_key(|f| f.score);

        if boards[0].score == 5 {
            return boards[0].unmarked_sum() * number
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_marks_a_value_as_checked_if_the_number_is_in_the_grid_and_calculates_a_score() {
        let mut board = super::Board {
            grid: [
                [(22, false), (13, false), (17, false), (11, false), (0, false)],
                [( 8, false), ( 2, false), (23, false), ( 4, false), (24, false)],
                [(21, false), ( 9, false), (14, false), (16, false), ( 7, false)],
                [( 6, false), (10, false), ( 3, false), (18, false), ( 5, false)],
                [( 1, false), (12, false), (20, false), (15, false), (19, false)]
            ],
            score: 0
        };

        board.check_number(4);
        assert_eq!(board.grid[1][3], (4, true));
        assert_eq!(board.score, 1);

        board.check_number(9);
        assert_eq!(board.grid[2][1], (9, true));
        assert_eq!(board.score, 1);

        board.check_number(20);
        assert_eq!(board.grid[4][2], (20, true));
        assert_eq!(board.score, 1);

        board.check_number(23);
        assert_eq!(board.grid[1][2], (23, true));
        assert_eq!(board.score, 2);
    }

    #[test]
    fn it_returns_the_sum_of_unmarked_tiles() {
        let mut board = super::Board::new([
            [22, 13, 17, 11,  0],
            [ 8,  2, 23,  4, 24],
            [21,  9, 14, 16,  7],
            [ 6, 10,  3, 18,  5],   
            [ 1, 12, 20, 15, 19]
        ]);

        board.check_number(16);
        board.check_number(5);
        board.check_number(22);

        assert_eq!(board.unmarked_sum(), 257);
    }

    #[test]
    fn it_plays_bingo_against_several_boards_and_returns_the_final_score_of_the_winner() {
        let boards: Vec<super::Board> = vec![
            super::Board::new([
                [22, 13, 17, 11,  0],
                [ 8,  2, 23,  4, 24],
                [21,  9, 14, 16,  7],
                [ 6, 10,  3, 18,  5],   
                [ 1, 12, 20, 15, 19]
            ]),
            super::Board::new([
               [ 3, 15,  0,  2, 22],
               [ 9, 18, 13, 17,  5],
               [19,  8,  7, 25, 23],
               [20, 11, 10, 24, 14],
               [14, 21, 16, 12,  6]
            ]),
            super::Board::new([
                [14, 21, 17, 24,  4],
                [10, 16, 15,  9, 19],
                [18,  8, 23, 26, 20],
                [22, 11, 13,  6,  5],
                [ 2,  0, 12,  3,  7]
            ])
        ];

        let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let score = super::find_winning_score(boards, numbers);
        assert_eq!(score, 4512);
    }

    #[test]
    fn it_finds_the_last_winner() {
        let boards: Vec<super::Board> = vec![
            super::Board::new([
                [22, 13, 17, 11,  0],
                [ 8,  2, 23,  4, 24],
                [21,  9, 14, 16,  7],
                [ 6, 10,  3, 18,  5],   
                [ 1, 12, 20, 15, 19]
            ]),
            super::Board::new([
               [ 3, 15,  0,  2, 22],
               [ 9, 18, 13, 17,  5],
               [19,  8,  7, 25, 23],
               [20, 11, 10, 24, 14],
               [14, 21, 16, 12,  6]
            ]),
            super::Board::new([
                [14, 21, 17, 24,  4],
                [10, 16, 15,  9, 19],
                [18,  8, 23, 26, 20],
                [22, 11, 13,  6,  5],
                [ 2,  0, 12,  3,  7]
            ])
        ];

        let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let score = super::find_last_winning_score(boards, numbers);
        assert_eq!(score, 1924);
    }
}