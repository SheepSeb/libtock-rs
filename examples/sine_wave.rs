#![no_std]
#![no_main]

use core::fmt::Write;
use libtock::alarm::{Alarm, Milliseconds};
use libtock::console::Console;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x2000}

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const PI: f32 = 3.14159265359;

fn clear_screen() {
    write!(Console::writer(), "\x1B[2J").unwrap();
}

fn move_cursor() {
    write!(Console::writer(), "\x1B[0;0H").unwrap();
}

fn print_horizontal_line() {
    for _ in 0..WIDTH {
        write!(Console::writer(), "-").unwrap();
    }
    writeln!(Console::writer()).unwrap();
}

fn print_y_axis_label() {
    for _ in 0..HEIGHT {
        write!(Console::writer(), "|").unwrap();
        for _ in 0..WIDTH {
            write!(Console::writer(), " ").unwrap();
        }
        writeln!(Console::writer(), "|\n").unwrap();
    }
}

fn sin(x: f32) -> f32 {
    let mut sum = 0.0;
    let mut term = x;

    for i in 1..=10 {
        sum += term;
        term *= -x * x / ((2 * i) * (2 * i + 1)) as f32;
    }

    sum
}

fn sin_optimized(x: f32) -> f32 {
    let mut x = x;

    // Reduce input to the range [-pi, pi]
    while x > PI {
        x -= 2.0 * PI;
    }

    while x < -PI {
        x += 2.0 * PI;
    }

    // Compute sine using Taylor series
    sin(x)
}

fn plot_data(time: f32, len: usize) {
    let mut data: [f32; WIDTH] = [0.0; WIDTH];

    for i in 0..len {
        data[i] = 0.0;
    }

    for i in 0..len {
        data[i] = sin_optimized(2.0 * PI * (time + i as f32 / WIDTH as f32));
    }

    print_horizontal_line();
    print_y_axis_label();
    print_horizontal_line();

    for i in 0..HEIGHT {
        write!(Console::writer(), "|").unwrap();
        for j in 0..WIDTH - 2 {
            if i == ((data[j] + 1.0) * HEIGHT as f32 / 2.0) as usize {
                write!(Console::writer(), "*").unwrap();
            } else {
                write!(Console::writer(), " ").unwrap();
            }
        }
        writeln!(Console::writer(), "|\n").unwrap();
    }

    print_horizontal_line();
}

fn main() {
    let mut time = 0.0;
    loop {
        plot_data(time, WIDTH);
        time += 0.1;
        Alarm::sleep_for(Milliseconds(1000)).unwrap();
    }
}
