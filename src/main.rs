use ndarray::prelude::*;
use rand;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GridCell {
    None,
    Wall,
    Goal,
    Trap,
}

impl Default for GridCell {
    fn default() -> Self {
        GridCell::None
    }
}

impl GridCell {
    fn score(&self) -> f32 {
        match self {
            GridCell::None => 0.0,
            GridCell::Wall => 0.0,
            GridCell::Goal => 1.0,
            GridCell::Trap => -1.0,
        }
    }
}

#[derive(Debug, Clone)]
struct GridGame {
    player_pos: (usize, usize), // x, y
    board: Array2<GridCell>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl GridGame {
    fn new() -> Self {
        let mut board = Array2::default((3, 4));
        board[[1, 2]] = GridCell::Wall;
        board[[2, 3]] = GridCell::Trap;
        board[[0, 3]] = GridCell::Goal;

        Self {
            player_pos: (0, 2),
            board: board,
        }
    }

    fn act(&mut self, direction: Direction) {
        let p = self.player_pos;
        match direction {
            Direction::Left => {
                if self.player_pos.0 > 0 {
                    self.player_pos.0 -= 1;
                }
            }
            Direction::Right => {
                if self.player_pos.0 < 3 {
                    self.player_pos.0 += 1;
                }
            }
            Direction::Up => {
                if self.player_pos.1 > 0 {
                    self.player_pos.1 -= 1;
                }
            }
            Direction::Down => {
                if self.player_pos.1 < 2 {
                    self.player_pos.1 += 1;
                }
            }
        }
        if self.board[[self.player_pos.1, self.player_pos.0]] == GridCell::Wall {
            self.player_pos = p;
        }
    }

    fn reward(&self) -> f32 {
        self.board[[self.player_pos.1, self.player_pos.0]].score() - 0.001
    }

    fn render(&self) -> String {
        let mut s = "".to_string();
        for y in 0..3 {
            for x in 0..4 {
                if x == self.player_pos.0 && y == self.player_pos.1 {
                    s.push_str("@");
                } else {
                    match self.board[[y, x]] {
                        GridCell::None => s.push_str("."),
                        GridCell::Wall => s.push_str("#"),
                        GridCell::Goal => s.push_str("G"),
                        GridCell::Trap => s.push_str("T"),
                    }
                }
            }
            s.push_str("\n");
        }
        s
    }

    fn check_goal(&self) -> bool {
        self.board[[self.player_pos.1, self.player_pos.0]] == GridCell::Goal
    }
}
#[derive(Debug, Clone)]
struct Agent {
    q: HashMap<(usize, usize, Direction), f32>,
    pub alpha: f32,
    pub gamma: f32,
}

impl Default for Agent {
    fn default() -> Self {
        Self {
            q: Default::default(),
            alpha: 0.1,
            gamma: 0.99,
        }
    }
}

impl Agent {
    fn get_q(&mut self, pos: (usize, usize), dir: Direction) -> f32 {
        if self.q.contains_key(&(pos.0, pos.1, dir)) {
            return self.q[&(pos.0, pos.1, dir)];
        } else {
            self.q.insert((pos.0, pos.1, dir), 0.0);
            return 0.0;
        }
    }
    fn greedy(&mut self, pos: (usize, usize)) -> Direction {
        let dirs = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];
        let mut s: f32 = -10000.0;
        let mut d_out: Direction = Direction::Left;

        for d in dirs {
            if self.get_q((pos.0, pos.1), d) > s {
                s = self.q[&(pos.0, pos.1, d)];
                d_out = d;
            }
        }
        d_out
    }

    fn update_q(
        &mut self,
        s1: (usize, usize),
        a1: Direction,
        r: f32,
        s2: (usize, usize),
        a2: Direction,
    ) {
        let next_q = self.get_q((s2.0, s2.1), a2);
        let now_q = self.get_q((s1.0, s1.1), a1);
        let new_score = now_q * (1.0 - self.alpha) + (next_q * self.gamma + r) * self.alpha;
        self.q.insert((s1.0, s2.1, a1), new_score);
    }
}

fn main() {
    let mut agent = Agent::default();
    let mut a1 = Direction::Left;
    let mut s1 = (1, 1);
    for i in 0..1 {
        let mut game = GridGame::new();
        for j in 0..100 {
            println!("{}", game.render());
            let s2 = game.player_pos;
            let a2 = agent.greedy((s1.0, s1.1));
            println!("{:?}", agent);
            println!("Act: {:?}", a1);
            game.act(a1);
            let r = game.reward();
            println!("Rwd: {}", r);
            agent.update_q(s1, a1, r, s2, a2);
            s1 = s2;
            a1 = a2;
            if game.check_goal() {
                println!("{}", game.render());
                println!("GOAL!, t={}", j);
                break;
            }
        }
    }
}
