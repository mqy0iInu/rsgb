mod cpu;
use cpu::{cpu_reset, cpu_main};

fn main() {
    cpu_reset();

    loop {
        cpu_main();
    }
}