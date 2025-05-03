use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::Print,
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
    game_over: bool
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

        // Update snake1's head
        match self.direction1 {
            Direction::Up => new_head1.1 = new_head1.1.saturating_sub(1),
            Direction::Down => new_head1.1 += 1,
            Direction::Left => new_head1.0 = new_head1.0.saturating_sub(1),
            Direction::Right => new_head1.0 += 1,
        }

        // Update snake2's head
        match self.direction2 {
            Direction::Up => new_head2.1 = new_head2.1.saturating_sub(1),
            Direction::Down => new_head2.1 += 1,
            Direction::Left => new_head2.0 = new_head2.0.saturating_sub(1),
            Direction::Right => new_head2.0 += 1,
        }

        // Check for collisions with walls or self
        if new_head1.0 >= WIDTH || new_head1.1 >= HEIGHT || self.snake1.contains(&new_head1) || self.snake2.contains(&new_head1) {
            self.game_over = true;
            return;
        }

        if new_head2.0 >= WIDTH || new_head2.1 >= HEIGHT || self.snake2.contains(&new_head2) || self.snake1.contains(&new_head2) {
            self.game_over = true;
            return;
        }

        // Move snake1
        self.snake1.insert(0, new_head1);
        if new_head1 == self.apple {
            self.apple = SnakeGame::generate_apple(&self.snake1, &self.snake2);
        } else {
            self.snake1.pop();
        }

        // Move snake2
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

        // Draw top border
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        print!("+");
        for _ in 0..WIDTH {
            print!("--");
        }
        print!("+");

        // Draw game area
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

        // Draw bottom border
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
                    // Player 1: Arrow Keys
                    KeyCode::Up => game.change_direction(1, Direction::Up),
                    KeyCode::Down => game.change_direction(1, Direction::Down),
                    KeyCode::Left => game.change_direction(1, Direction::Left),
                    KeyCode::Right => game.change_direction(1, Direction::Right),

                    // Player 2: WASD
                    KeyCode::Char('w') => game.change_direction(2, Direction::Up),
                    KeyCode::Char('s') => game.change_direction(2, Direction::Down),
                    KeyCode::Char('a') => game.change_direction(2, Direction::Left),
                    KeyCode::Char('d') => game.change_direction(2, Direction::Right),

                    // Quit
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
    terminal::disable_raw_mode().unwrap();
}






// use crossterm::{
//     cursor::{self, MoveTo},
//     event::{self, Event, KeyCode},
//     execute,
//     style::Print,
//     terminal::{self, ClearType},
// };
// use rand::Rng;
// use std::{
//     collections::VecDeque,
//     io::{stdout, Write},
//     thread,
//     time::{Duration, Instant},
// };

// const BOARD_WIDTH: u16 = 28;
// const BOARD_HEIGHT: u16 = 13;

// #[derive(Clone, Copy, PartialEq, Debug)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// #[derive(Debug)]
// enum InputCommand {
//     ChangeDirection(Direction),
// }

// struct SnakeGame {
//     snake1: VecDeque<(u16, u16)>,
//     snake2: VecDeque<(u16, u16)>,
//     dir1: Direction,
//     dir2: Direction,
//     next_dir1: Direction,
//     next_dir2: Direction,
//     food: (u16, u16),
//     running: bool,
// }

// impl SnakeGame {
//     fn new() -> Self {
//         let mut snake1 = VecDeque::new();
//         snake1.push_back((BOARD_WIDTH / 4, BOARD_HEIGHT / 2)); // Player 1's starting position
//         let mut snake2 = VecDeque::new();
//         snake2.push_back((3 * BOARD_WIDTH / 4, BOARD_HEIGHT / 2)); // Player 2's starting position
//         let food = SnakeGame::generate_food(&snake1, &snake2);
//         SnakeGame {
//             snake1,
//             snake2,
//             dir1: Direction::Right,
//             dir2: Direction::Left,
//             next_dir1: Direction::Right,
//             next_dir2: Direction::Left,
//             food,
//             running: true,
//         }
//     }

//     fn generate_food(snake1: &VecDeque<(u16, u16)>, snake2: &VecDeque<(u16, u16)>) -> (u16, u16) {
//         let mut rng = rand::thread_rng();
//         loop {
//             let x = rng.gen_range(1..=BOARD_WIDTH);
//             let y = rng.gen_range(1..=BOARD_HEIGHT);
//             if !snake1.contains(&(x, y)) && !snake2.contains(&(x, y)) {
//                 return (x, y);
//             }
//         }
//     }

//     fn draw_board() {
//         let mut stdout = stdout();
//         for y in 0..=BOARD_HEIGHT + 1 {
//             for x in 0..=BOARD_WIDTH + 1 {
//                 let ch = if y == 0 || y == BOARD_HEIGHT + 1 || x == 0 || x == BOARD_WIDTH + 1 {
//                     '#'
//                 } else {
//                     ' '
//                 };
//                 execute!(stdout, MoveTo(x, y), Print(ch)).unwrap();
//             }
//         }
//         stdout.flush().unwrap();
//     }

//     fn draw_at(pos: (u16, u16), ch: char) {
//         execute!(stdout(), MoveTo(pos.0, pos.1), Print(ch)).unwrap();
//     }

//     fn update(&mut self) {
//         // Update Player 1's snake
//         self.dir1 = self.next_dir1;
//         let head1 = *self.snake1.front().unwrap();
//         let new_head1 = match self.dir1 {
//             Direction::Up => (head1.0, head1.1.saturating_sub(1)),
//             Direction::Down => (head1.0, head1.1 + 1),
//             Direction::Left => (head1.0.saturating_sub(1), head1.1),
//             Direction::Right => (head1.0 + 1, head1.1),
//         };

//         if new_head1.0 == 0
//             || new_head1.0 == BOARD_WIDTH + 1
//             || new_head1.1 == 0
//             || new_head1.1 == BOARD_HEIGHT + 1
//             || self.snake1.contains(&new_head1)
//         {
//             self.running = false;
//             return;
//         }

//         self.snake1.push_front(new_head1);
//         SnakeGame::draw_at(new_head1, '█');

//         if new_head1 == self.food {
//             self.food = SnakeGame::generate_food(&self.snake1, &self.snake2);
//             SnakeGame::draw_at(self.food, 'O');
//         } else {
//             let tail = self.snake1.pop_back().unwrap();
//             SnakeGame::draw_at(tail, ' ');
//         }

//         // Update Player 2's snake
//         self.dir2 = self.next_dir2;
//         let head2 = *self.snake2.front().unwrap();
//         let new_head2 = match self.dir2 {
//             Direction::Up => (head2.0, head2.1.saturating_sub(1)),
//             Direction::Down => (head2.0, head2.1 + 1),
//             Direction::Left => (head2.0.saturating_sub(1), head2.1),
//             Direction::Right => (head2.0 + 1, head2.1),
//         };

//         if new_head2.0 == 0
//             || new_head2.0 == BOARD_WIDTH + 1
//             || new_head2.1 == 0
//             || new_head2.1 == BOARD_HEIGHT + 1
//             || self.snake2.contains(&new_head2)
//         {
//             self.running = false;
//             return;
//         }

//         self.snake2.push_front(new_head2);
//         SnakeGame::draw_at(new_head2, '█');

//         if new_head2 == self.food {
//             self.food = SnakeGame::generate_food(&self.snake1, &self.snake2);
//             SnakeGame::draw_at(self.food, 'O');
//         } else {
//             let tail = self.snake2.pop_back().unwrap();
//             SnakeGame::draw_at(tail, ' ');
//         }
//     }

//     fn handle_input(&mut self) {
//         if event::poll(Duration::from_millis(0)).unwrap() {
//             if let Event::Key(key) = event::read().unwrap() {
//                 match key.code {
//                     // Player 1 controls (Arrow keys)
//                     KeyCode::Up if self.dir1 != Direction::Down => self.next_dir1 = Direction::Up,
//                     KeyCode::Down if self.dir1 != Direction::Up => self.next_dir1 = Direction::Down,
//                     KeyCode::Left if self.dir1 != Direction::Right => self.next_dir1 = Direction::Left,
//                     KeyCode::Right if self.dir1 != Direction::Left => self.next_dir1 = Direction::Right,

//                     // Player 2 controls (WASD keys)
//                     KeyCode::Char('w') if self.dir2 != Direction::Down => self.next_dir2 = Direction::Up,
//                     KeyCode::Char('s') if self.dir2 != Direction::Up => self.next_dir2 = Direction::Down,
//                     KeyCode::Char('a') if self.dir2 != Direction::Right => self.next_dir2 = Direction::Left,
//                     KeyCode::Char('d') if self.dir2 != Direction::Left => self.next_dir2 = Direction::Right,

//                     // Quit the game
//                     KeyCode::Esc => {
//                         self.running = false;
//                         return;
//                     }
//                     _ => {}
//                 };
//             }
//         }
//     }
// }

// fn main() {
//     let mut stdout = stdout();
//     terminal::enable_raw_mode().unwrap();
//     execute!(
//         stdout,
//         terminal::EnterAlternateScreen,
//         cursor::Hide,
//         terminal::Clear(ClearType::All)
//     )
//     .unwrap();

//     let mut game = SnakeGame::new();
//     SnakeGame::draw_board();
//     SnakeGame::draw_at(*game.snake1.front().unwrap(), '█');
//     SnakeGame::draw_at(*game.snake2.front().unwrap(), '█');
//     SnakeGame::draw_at(game.food, 'O');

//     let mut last_frame = Instant::now();
//     let frame_duration = Duration::from_millis(120);

//     while game.running {
//         game.handle_input();
//         if last_frame.elapsed() >= frame_duration {
//             game.update();
//             last_frame = Instant::now();
//         }
//         thread::sleep(Duration::from_millis(5));
//     }

//     execute!(
//         stdout,
//         cursor::Show,
//         terminal::LeaveAlternateScreen,
//         terminal::Clear(ClearType::All)
//     )
//     .unwrap();
//     terminal::disable_raw_mode().unwrap();

//     println!("Game over!");
// }
