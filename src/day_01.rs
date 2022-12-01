extern crate utilities as utils;

use std::io::BufRead;


fn part1() -> i64 {
    let mut total_inv:i64 = 0;
    let mut elf_total: i64 = 0;
    for line in utils::open_input("./assets/input_day_01").lines() {


        if line.as_ref().unwrap().is_empty() {
            if elf_total > total_inv {
                total_inv = elf_total;

            }
            elf_total = 0;
        }else{
            let item :i64 = line.unwrap().parse().unwrap();
            elf_total = elf_total + item;
        }
    }
    return total_inv;

}

fn part2() -> i64 {
    let mut elfs_inventory:Vec<i64> = vec![];
    let mut elf_total: i64 = 0;
    for line in utils::open_input("./assets/input_day_01").lines() {

        if line.as_ref().unwrap().is_empty() {
            elfs_inventory.push(elf_total);
            elf_total = 0;
        }else{
            let item :i64 = line.unwrap().parse().unwrap();
            elf_total = elf_total + item;
        }
    }
    elfs_inventory.sort();

    return elfs_inventory[elfs_inventory.len()-3..elfs_inventory.len()].iter().sum();
}

fn main() {
    println!("Results for template");
    println!("-------------------");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
