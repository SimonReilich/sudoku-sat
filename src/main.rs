use std::io;
use cryptominisat::Solver;

pub mod tests;

#[cfg(feature = "test-bin")]
fn main() {
    tests::test();
}

#[cfg(not(feature = "test-bin"))]
fn main() {
    let mut solver = Solver::new();

    let sudoku = [
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
    ];

    sudoku::setup(&mut solver, &sudoku);

    let mut line = String::new();
    while io::stdin().read_line(&mut line).unwrap_or(0) > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() { break; }
        let solution = sudoku::solve(&sudoku, &mut solver, trimmed).unwrap();
        println!("{}", solution);
        line.clear();
    }
}

pub mod sudoku {
    use std::ops::Not;
    use cryptominisat::{Solver, Lit, Lbool};

    pub fn setup(solver: &mut Solver, sudoku: &[[[Lit; 9]; 9]; 9]) {
        for i in 0..9 {
            for j in 0..9 {
                solver.add_clause(&sudoku[i][j]);
                for n in 0..9 {
                    for n_prime in (n + 1)..9 {
                        solver.add_clause(&[sudoku[i][j][n].not(), sudoku[i][j][n_prime].not()]);
                    }
                    for j_prime in (j + 1)..9 {
                        solver.add_clause(&[sudoku[i][j][n].not(), sudoku[i][j_prime][n].not()]);
                    }
                    for i_prime in (i + 1)..9 {
                        solver.add_clause(&[sudoku[i][j][n].not(), sudoku[i_prime][j][n].not()]);
                    }
                    for i_prime in ((i / 3) * 3)..((i / 3 + 1) * 3) {
                        for j_prime in ((j / 3) * 3)..((j / 3 + 1) * 3) {
                            if i == i_prime && j == j_prime { continue; }
                            solver.add_clause(&[sudoku[i][j][n].not(), sudoku[i_prime][j_prime][n].not()]);
                        }
                    }
                }
            }
        }
    }

    pub fn solve(sudoku: &[[[Lit; 9]; 9]; 9], solver: &mut Solver, input: &str) -> Result<String, ()> {
        let mut assumptions = Vec::new();
        for (idx, c) in input.chars().enumerate() {
            if idx >= 81 { break; }
            if c != '0' {
                if let Some(digit) = c.to_digit(10) {
                    if digit >= 1 && digit <= 9 {
                        let i = idx / 9;
                        let j = idx % 9;
                        assumptions.push(sudoku[i][j][(digit - 1) as usize]);
                    }
                }
            }
        }

        match solver.solve_with_assumptions(&assumptions) {
            Lbool::True => {
                let mut result = String::new();
                for i in 0..9 {
                    for j in 0..9 {
                        for n in 0..9 {
                            if solver.is_true(sudoku[i][j][n]) {
                                result.push(char::from_digit((n + 1) as u32, 10).unwrap());
                                continue;
                            }
                        }
                    }
                }
                Ok(result)
            }
            Lbool::False => {
                println!("The sudoku was unsatisfyable");
                Err(())
            }
            Lbool::Undef => {
                println!("An Error occured");
                Err(())
            }
        }
    }
}
