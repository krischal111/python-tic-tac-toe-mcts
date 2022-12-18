// use std::io;
extern crate rand;
use rand::Rng;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
struct tttS {
    tictactoe_state: u32,
}
impl tttS {
    // use Self::tictactoe_state;
    fn new_empty() -> Self {
        return Self { tictactoe_state: 0 };
    }

    fn get_pos(&self, index: usize) -> u32 {
        return (self.tictactoe_state >> (index * 2)) & 3;
    }

    fn set_pos(&mut self, index: usize, player: u32) {
        self.tictactoe_state |= player << (index * 2);
    }

    fn is_valid(&self, index: usize) -> bool {
        return self.get_pos(index) == 0;
    }

    fn check_win(&self) -> Option<u32> {
        const ROW_WIN: u32 = 64 - 1;
        const COL_WIN: u32 = 3 + (3 << 6) + (3 << 12);
        const DIAG_1_WIN: u32 = 0x30303;
        const DIAG_2_WIN: u32 = 0x3330;
        const P_1_MASK: u32 = 0x55555555;
        const P_2_MASK: u32 = 0xaaaaaaaa;

        let player_win = |win_mask, player| -> bool {
            self.tictactoe_state & win_mask == {
                if player == 1 {
                    win_mask & P_1_MASK
                } else {
                    win_mask & P_2_MASK
                }
            }
        };
        for player in 1..=2 {
            for i in 0..3 {
                if player_win(ROW_WIN << (i * 6), player) || player_win(COL_WIN << (i * 2), player)
                {
                    return Some(player);
                }
            }
            if player_win(DIAG_1_WIN, player) || player_win(DIAG_2_WIN, player) {
                return Some(player);
            }
        }
        return None;
    }

    fn is_full(&self) -> bool {
        let mut st = self.tictactoe_state;
        for _i in 0..9 {
            if (st & 3) == 0 {
                return false;
            } else {
                st >>= 2;
            }
        }
        return true;
    }

    fn show_valid_state(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!(" {} ", self.is_valid(i * 3 + j));
            }
            println!();
        }
    }

    fn show_state(&self) {
        for i in 0..3 {
            for j in 0..3 {
                let p = self.get_pos(i * 3 + j);
                if p == 0 {
                    print!("   ");
                } else {
                    print!(" {} ", if p == 1 { 'o' } else { 'x' });
                }
                if j < 2 {
                    print!("|");
                } else {
                    print!(" ");
                }
            }
            if i < 2 {
                println!("\n---+---+---");
            } else {
                println!();
            }
        }
    }

    // ! cannot track if moves are remaining. Has to be tracked beforehand
    fn make_random_move(&mut self, player: u32) {
        let mut rng = rand::thread_rng();
        loop {
            let index = rng.gen_range(0..9);
            if self.is_valid(index) {
                self.set_pos(index, player);
                return;
            }
        }
    }

    fn random_game_example(&mut self) {
        for i in 0..9 {
            let p = (i & 1) + 1;
            println!("Move ({}) by player {p}", i + 1);
            self.make_random_move(p);
            self.show_state();
            println!();
            if let Some(player) = self.check_win() {
                println!("Won by player {} ", player);
                return;
            }
        }
        println!("The game is drawn");
    }

    fn monte_start(&self, player: u32, n: usize) {
        let mut record: [(u32, u32, u32); 9] = [(0, 0, 0); 9];
        for i in 0..9 {
            if self.is_valid(i) {
                let mut new_board = *self;
                new_board.set_pos(i, player);

                // yet to utilize the result.
                let (wins, ties, loss) = new_board.monte_carlo(player, n);
                record[i] = (wins, ties, loss);

                println!(
                    "For choice {i}, win:tie:loss ratio is: {:?}",
                    (wins, ties, loss)
                );
            } else {
                continue;
            }
        }
    }

    // should give the result (win, ties)
    fn monte_carlo(&self, player: u32, n: usize) -> (u32, u32, u32) {
        let mut wins = 0;
        let mut ties = 0;
        for _ in 0..n {
            let mut game = *self;
            let result = game.play_random_till_over(player);
            if let Some(winner) = result {
                if player == winner {
                    wins += 1;
                }
            } else {
                ties += 1;
            }
        }
        return (wins, ties, n as u32 - wins - ties);
    }

    fn play_random_till_over(&mut self, player: u32) -> Option<u32> {
        let mut player = player;
        loop {
            player = 3 - player;
            if !self.is_full() {
                self.make_random_move(player);
                if let Some(player) = self.check_win() {
                    return Some(player);
                }
            } else {
                return None;
            }
        }
    }

    fn play_game_with_hints(&mut self) {
        let mut player = 1;
        for _i in 0..9 {
            player = 3 - player;
            self.monte_start(player, 5000);
            self.show_state();
            let n: usize = loop {
                println!("Your choice = ");
                let mut n = String::new();
                use std::io;
                io::stdin()
                    .read_line(&mut n)
                    .expect("failed to read input.");
                break n.trim().parse().expect("invalid input");
            };
            self.set_pos(n as usize, player);
            let result = self.check_win();
            if let Some(winner) = result {
                println!("Player {winner} wins.");
                break;
            } else if self.is_full() {
                println!("The game is a tie");
                break;
            }
        }
        self.show_state();
    }
}

pub fn main() {
    let mut t = tttS::new_empty();
    // t.set_pos(0, 1);
    // t.set_pos(3,1);
    // t.set_pos(8,1);
    // t.set_pos(1,2);
    // t.set_pos(4,2);
    // t.set_pos(7,0);
    // t.show_state();
    // t.show_valid_state();
    // println!("Won by player {:?} ", t.check_win());
    // t.random_game_example();
    t.play_game_with_hints();
    println!();
}
