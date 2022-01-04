use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Square {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

fn main() {
    let mut square: Vec<Vec<Square>> = Vec::new();

    let mut square_size_str = String::new();

    io::stdin()
        .read_line(&mut square_size_str)
        .expect("Did not enter any values");

    println!("{}", square_size_str);

    let square_size = square_size_str.trim().parse::<usize>();

    match square_size {
        Result::Ok(parsed_number) => {
            println!("{}", parsed_number);
            for x in 0..(2 * parsed_number) {
                square.insert(0, Vec::new());
                for y in 0..(2 * parsed_number) {
                    square[0].insert(
                        0,
                        Square {
                            x1: x,
                            x2: x + 1,
                            y1: y,
                            y2: y + 1,
                        },
                    )
                }
            }
        }
        Result::Err(_) => (),
    }

    println!("{:?}", square);
}

fn petya_and_string() {
    let characters_orders = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
    ]);

    let mut first_string = String::new();
    let mut second_string = String::new();
    let mut first_count = 0;
    let mut second_count = 0;

    io::stdin()
        .read_line(&mut first_string)
        .expect("Did not enter any values");

    io::stdin()
        .read_line(&mut second_string)
        .expect("Did not enter any values");

    let up_first = first_string.to_lowercase();
    let mut first_chars = up_first.chars();

    let up_second = second_string.to_lowercase();
    let mut second_chars = up_second.chars();

    let len = (up_first.len() + up_second.len()) / 2;

    for _ in 0..len {
        if let Some(k1) = first_chars.next() {
            if let Some(first_number) = characters_orders.get(&k1) {
                if let Some(k2) = second_chars.next() {
                    if let Some(second_number) = characters_orders.get(&k2) {
                        if k1 != k2 {
                            if *first_number > *second_number {
                                first_count += 1;
                            } else {
                                second_count += 1;
                            }
                            break;
                        }
                    }
                };
            }
        };
    }

    if first_count < second_count {
        println!("{}", -1)
    } else if second_count < first_count {
        println!("{}", 1)
    } else {
        println!("{}", 0)
    }
}
