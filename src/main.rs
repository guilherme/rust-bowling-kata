use std::cell::Cell;
struct Game {
  rolls: Vec<usize>,
  current_roll: Cell<usize>,
}

impl Game {

  pub fn new() -> Game {
    let rolls: Vec<usize> = vec![];
    return Game { rolls: rolls, current_roll: Cell::new(0) };
  }

  fn roll(&mut self, pins: usize) {
    self.rolls.push(pins);
    self.current_roll.set(self.current_roll.get() + 1);
  }

  fn score(&self) -> usize {
    let mut score = 0;
    let mut frame_index = 0;
    for frame in 0..10 {
      if self.rolls[frame_index] == 10 {
        score += 10 + self.rolls[frame_index+1] + self.rolls[frame_index + 2];
        frame_index += 1;
      } else {
        if self.rolls[frame_index] + self.rolls[frame_index+1] == 10 {
          score += 10 + self.rolls[frame + 2];
        } else {
          score += self.rolls[frame_index] + self.rolls[frame_index+1];
        };
        frame_index += 2;
      };
    };
    return score;
  }

}

fn main() {
}

#[cfg(test)]
mod tests {

  use super::Game;

  fn roll_many(game: &mut Game, times: usize, pin_value: usize)  {
    for _ in 0..times {
      game.roll(pin_value);
    }
  }

  #[test]
  fn it_runs_the_game() {
    let mut game = Game::new();
    roll_many(&mut game, 20, 0);
    assert_eq!(0, game.score());
  }

  #[test]
  fn it_test_all_ones() {
    let mut game = Game::new();
    roll_many(&mut game, 20, 1);
    assert_eq!(20, game.score());
  }

  #[test]
  fn test_one_spare() {
    let mut game = Game::new();
    game.roll(5);
    game.roll(5);
    game.roll(3);
    roll_many(&mut game, 17, 0);
    assert_eq!(16, game.score());
  }


  #[test]
  fn test_one_strike() {
    let mut game = Game::new();
    game.roll(10);
    game.roll(3);
    game.roll(4);
    roll_many(&mut game, 16, 0);
    assert_eq!(24, game.score());
  }


  #[test]
  fn test_perfect_game() {
    let mut game = Game::new();
    roll_many(&mut game, 12, 10);
    assert_eq!(300, game.score());
  }
}
