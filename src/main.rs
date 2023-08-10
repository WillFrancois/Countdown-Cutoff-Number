use std::collections::HashMap;

mod functions;

fn main() {
    let small_nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let large_nums = [25, 50, 75, 100];

    let mut cur_number = 11;

    let mut complete_list: HashMap<u64, Vec<u32>> = HashMap::new();

    functions::preload(&mut complete_list, &small_nums, &large_nums);

    loop {
        functions::test_possible_number(cur_number, &mut complete_list);
        println!("{}", cur_number);
        if complete_list.get(&cur_number).is_none() {
            break;
        }
        cur_number += 1;
    }

    let mut smallest_solution = 0b1111111111111111111111111111111;

    for i in complete_list.get(&(cur_number - 1)).unwrap().into_iter() {
        if u32::count_ones(*i) < u32::count_ones(smallest_solution) {
            smallest_solution = *i;
        }
    }

    // println!("{complete_list:?}");
    println!("{:032b}", smallest_solution);
}
