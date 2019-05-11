/*
 * conway's game of life implemented entirely in rust
 * 
 * GAME RULES:
 * 1. Any live cell with fewer than two live neighbours dies.
 * 2. Any live cell with more than three live neighbours dies.
 * 3. Any live cell with two or three live neighbours lives, unchanged, to the next generation.
 * 4. Any dead cell with exactly three live neighbours will come to life.
 */


mod cell;
mod board;


use std::{thread, time};
use std::process::Command;
use std::io;
use colored::*;
use rand::Rng;
use cell::CellState;
use board::Board;

fn main() {
	/* declare variables to hold the entries */
	let mut width = String::new();
	let mut height = String::new();

	/* get the board width and store it */
	println!("enter board width: ");
	io::stdin().read_line(&mut width)
		.expect("failed to read line!");

	/* and parse it as a u32 */
	let width: u32 = width.trim().parse()
		.expect("invalid board width");

	/*get the board height and store it */
	println!("enter board height: ");
	io::stdin().read_line(&mut height)
		.expect("failed to read line!");

	/* and parse it as a u32 */
	let height: u32 = height.trim().parse()
		.expect("invalid board height");

	/* create the board and seed it */
	let mut b = Board::new(width, height);
	seed_board(&mut b);

	loop {
		print_board(&b);
		apply_rules(&mut b);
		thread::sleep(time::Duration::from_millis(100));
	}
}

fn clear_screen() {
	let output = Command::new("clear").output().unwrap();
  	println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn print_board(b: &Board) {
	clear_screen();
	for y in 0..b.height() - 1 {
		for x in 0..b.width() - 1 {
			let x = x as i32;
			let y = y as i32;

			match b.at(x, y).state {
				CellState::Alive => print!("{}", "O".green()),
				CellState::Dead => print!("#"),
				CellState::WillBeRevived => print!("{}", "#".yellow()),
				CellState::WillBeKilled => print!("{}", "O".red())
			}
		}
		println!();
	}
}

fn apply_rules(b: &mut Board) {
	for y in 0..b.height() - 1 {
		for x in 0..b.width() - 1 {
			let x = x as i32;
			let y = y as i32;
			b.get(x, y).set_mark();
		}
	}

	for y in 0..b.height() - 1 {
		for x in 0..b.width() - 1 {
			let x = x as i32;
			let y = y as i32;
			let living_neighbors = b.living_neighbors(x, y);

			/* Any live cell with fewer than two live neighbours dies. */
			if living_neighbors < 2 && b.at(x, y).is_alive() {
				b.get(x, y).mark_death();
			}

			/* Any live cell with more than three live neighbours dies. */
			if living_neighbors > 3 && b.at(x, y).is_alive() {
				b.get(x, y).mark_death();
			}

			/* Any dead cell with exactly three live neighbours will come to life. */
			if living_neighbors == 3 && b.at(x, y).is_dead() {
				b.get(x, y).mark_revive();
			}
		}
	}
}

fn seed_board(b: &mut Board) {
	let mut r = rand::thread_rng();
	for y in 0..b.height() - 1 {
		for x in 0..b.width() - 1 {
			let x = x as i32;
			let y = y as i32;

			if r.gen_range(0, 2) == 1 {
				b.get(x, y).mark_revive();
			}
		}
	}
}