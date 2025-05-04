use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use rand::Rng;

const WIDTH: u16 = 20;
const HEIGHT: u16 = 10;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    snake1: Vec<(u16, u16)>,
    snake2: Vec<(u16, u16)>,
    apple: (u16, u16),
    direction1: Direction,
    direction2: Direction,
    game_over: bool,
    winner: Option<u8>,
    crashed1: bool,
    crashed2: bool,
}

impl SnakeGame {
    fn new() -> Self {
        let snake1 = vec![(10, 5)];
        let snake2 = vec![(8, 5)];
        let apple = SnakeGame::generate_apple(&snake1, &snake2);
        Self {
            snake1,
            snake2,
            apple,
            direction1: Direction::Right,
            direction2: Direction::Left,
            game_over: false,
            winner: None,
            crashed1: false,
            crashed2: false,
        }
    }

    fn generate_apple(snake1: &[(u16, u16)], snake2: &[(u16, u16)]) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        loop {
            let pos = (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT));
            if !snake1.contains(&pos) && !snake2.contains(&pos) {
                return pos;
            }
        }
    }

    fn update(&mut self) {
        let mut new_head1 = self.snake1[0];
        let mut new_head2 = self.snake2[0];

        match self.direction1 {
            Direction::Up => new_head1.1 = new_head1.1.saturating_sub(1),
            Direction::Down => new_head1.1 += 1,
            Direction::Left => new_head1.0 = new_head1.0.saturating_sub(1),
            Direction::Right => new_head1.0 += 1,
        }

        match self.direction2 {
            Direction::Up => new_head2.1 = new_head2.1.saturating_sub(1),
            Direction::Down => new_head2.1 += 1,
            Direction::Left => new_head2.0 = new_head2.0.saturating_sub(1),
            Direction::Right => new_head2.0 += 1,
        }

        if new_head1.0 >= WIDTH || new_head1.1 >= HEIGHT || self.snake1.contains(&new_head1) || self.snake2.contains(&new_head1) {
            self.crashed1 = true;
        }

        if new_head2.0 >= WIDTH || new_head2.1 >= HEIGHT || self.snake2.contains(&new_head2) || self.snake1.contains(&new_head2) {
            self.crashed2 = true;
        }

        if self.crashed1 || self.crashed2 {
            self.game_over = true;
            self.winner = if self.crashed1 && self.crashed2 {
                if self.snake1.len() > self.snake2.len() {
                    Some(1)
                } else if self.snake2.len() > self.snake1.len() {
                    Some(2)
                } else {
                    Some(0)
                }
            } else if self.crashed1 {
                Some(2)
            } else if self.crashed2 {
                Some(1)
            } else {
                None
            };
            return;
        }

        self.snake1.insert(0, new_head1);
        if new_head1 == self.apple {
            self.apple = SnakeGame::generate_apple(&self.snake1, &self.snake2);
        } else {
            self.snake1.pop();
        }

        self.snake2.insert(0, new_head2);
        if new_head2 == self.apple {
            self.apple = SnakeGame::generate_apple(&self.snake1, &self.snake2);
        } else {
            self.snake2.pop();
        }
    }

    fn draw(&self) {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();

        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        print!("+");
        for _ in 0..WIDTH {
            print!("--");
        }
        print!("+");

        for y in 0..HEIGHT {
            stdout.execute(cursor::MoveTo(0, y + 1)).unwrap();
            print!("|");
            for x in 0..WIDTH {
                if self.snake1.contains(&(x, y)) {
                    print!("██");
                } else if self.snake2.contains(&(x, y)) {
                    print!("██");
                } else if self.apple == (x, y) {
                    print!("@@");
                } else {
                    print!("  ");
                }
            }
            print!("|");
        }

        stdout.execute(cursor::MoveTo(0, HEIGHT + 1)).unwrap();
        print!("+");
        for _ in 0..WIDTH {
            print!("--");
        }
        print!("+");

        stdout.flush().unwrap();
    }

    fn change_direction(&mut self, player: u8, new_dir: Direction) {
        if player == 1 {
            if (self.direction1 == Direction::Up && new_dir != Direction::Down)
                || (self.direction1 == Direction::Down && new_dir != Direction::Up)
                || (self.direction1 == Direction::Left && new_dir != Direction::Right)
                || (self.direction1 == Direction::Right && new_dir != Direction::Left)
            {
                self.direction1 = new_dir;
            }
        } else if player == 2 {
            if (self.direction2 == Direction::Up && new_dir != Direction::Down)
                || (self.direction2 == Direction::Down && new_dir != Direction::Up)
                || (self.direction2 == Direction::Left && new_dir != Direction::Right)
                || (self.direction2 == Direction::Right && new_dir != Direction::Left)
            {
                self.direction2 = new_dir;
            }
        }
    }
}

fn main() {
    let mut game = SnakeGame::new();
    terminal::enable_raw_mode().unwrap();

    let mut last_frame = Instant::now();

    while !game.game_over {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => game.change_direction(1, Direction::Up),
                    KeyCode::Down => game.change_direction(1, Direction::Down),
                    KeyCode::Left => game.change_direction(1, Direction::Left),
                    KeyCode::Right => game.change_direction(1, Direction::Right),

                    KeyCode::Char('w') => game.change_direction(2, Direction::Up),
                    KeyCode::Char('s') => game.change_direction(2, Direction::Down),
                    KeyCode::Char('a') => game.change_direction(2, Direction::Left),
                    KeyCode::Char('d') => game.change_direction(2, Direction::Right),

                    KeyCode::Char('q') => {
                        game.game_over = true;
                    }
                    _ => {}
                }
            }
        }

        if last_frame.elapsed() >= Duration::from_millis(150) {
            game.update();
            game.draw();
            last_frame = Instant::now();
        }
    }

    println!("\nGame Over!");
    match game.winner {
        Some(1) => println!("Player Arrows wins!"),
        Some(2) => println!("Player WASD wins!"),
        Some(0) => println!("It's a tie!"),
        None => println!("No winner determined."),
        Some(n) => println!("Unknown winner: Player {n}"),
    }      

    terminal::disable_raw_mode().unwrap();
}
