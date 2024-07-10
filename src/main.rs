use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
};

use chess::Board;
use sysinfo::System;

fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("clear").status().unwrap();
    }
}

fn print_memory_usage(system: &mut System) {
    system.refresh_processes();
    if let Some(process) = system.process(sysinfo::get_current_pid().unwrap()) {
        let memory_in_mb = process.memory() as f64 / 1_048_576.0;
        println!("Memory usage: {:.2} MB", memory_in_mb);
    }
}

fn main() {
    let mut system = System::new_all();
    let mut board = Board::new();
    println!("{}", board);

    // Simulate a simple game where the computer makes the best moves
    loop {
        print_memory_usage(&mut system);
        let start_time = Instant::now();
        if let Some(best_move) = board.find_best_move() {
            board.make_move(best_move.from, best_move.to);
            clear_screen();
            println!("{:}", board);
            let duration = start_time.elapsed();
            println!(
                "Time taken to calculate move for {:?}: {:?}",
                board.turn, duration
            );
            // Add a small delay to make the moves more visible
            thread::sleep(Duration::from_millis(300));

            // Check for checkmate
            if board.is_checkmate(board.turn) {
                println!("{:?} wins!", board.turn);
                break;
            }
        } else {
            println!("Stalemate! No valid moves for {:?}", board.turn);
            break;
        }

        // Add a delay to update memory usage every second
        thread::sleep(Duration::from_secs(1));
    }
}
