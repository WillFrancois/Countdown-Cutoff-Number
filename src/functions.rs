use std::collections::HashMap;

pub fn select_number_to_binary(number: u32) -> u32 {
    match number {
        100 => 1 << 31,
        75 => 1 << 30,
        50 => 1 << 29,
        25 => 1 << 28,
        10 => 1 << 27,
        9 => 1 << 26,
        8 => 1 << 25,
        7 => 1 << 24,
        6 => 1 << 23,
        5 => 1 << 22,
        4 => 1 << 21,
        3 => 1 << 20,
        2 => 1 << 19,
        1 => 1 << 18,
        _ => panic!("undefined number!"),
    }
}

pub fn length_of_rep(rep: u32) -> u32 {
    return rep & 0b111;
}

pub fn return_rep_nums(rep: u32) -> u32 {
    return rep - (rep & 0b111);
}

pub fn preload(complete_list: &mut HashMap<u64, Vec<u32>>, small_nums: &[u32], large_nums: &[u32]) {
    let all_nums = [small_nums, large_nums].concat();

    //Add squares for small numbers
    for i in small_nums {
        complete_list.insert((*i * *i) as u64, vec![select_number_to_binary(*i) + 2]);
    }

    //Add all numbers to list
    for i in all_nums {
        complete_list.insert(i as u64, vec![select_number_to_binary(i) + 1]);
    }
}

pub fn test_possible_number(needed_number: u64, complete_list: &mut HashMap<u64, Vec<u32>>) {
    let mut possibility_vec: Vec<u32> = vec![];
    let mut lowest_path = 0b111;

    if complete_list.contains_key(&needed_number) {
        return;
    }

    //Test subtraction
    for i in 1..needed_number / 2 + 1 {
        if complete_list.get(&(needed_number - i)).is_none() || complete_list.get(&(i)).is_none() {
            continue;
        }

        let possibility_list_one = complete_list.get(&(needed_number - i)).unwrap();
        let possibility_list_two = complete_list.get(&i).unwrap();

        for j in possibility_list_one {
            for k in possibility_list_two {
                if return_rep_nums(*j) & return_rep_nums(*k) == 0
                    && length_of_rep(*j) + length_of_rep(*k) <= 6
                    && length_of_rep(*j) + length_of_rep(*k) <= lowest_path
                {
                    let length_of_solution = length_of_rep(*j) + length_of_rep(*k);
                    let reps = return_rep_nums(*j) | return_rep_nums(*k);
                    let new_rep = length_of_solution | reps;

                    lowest_path = length_of_rep(*j) + length_of_rep(*k);
                    possibility_vec.push(new_rep);

                    // println!(
                    //     "subtraction to reach {needed_number}: {i} {}",
                    //     needed_number - i
                    // );
                }
            }
        }
    }

    //Test division
    for i in 1..needed_number / 2 + 1 {
        if needed_number % i != 0 {
            continue;
        }
        if complete_list.get(&(needed_number / i)).is_none() || complete_list.get(&i).is_none() {
            continue;
        }

        let possibility_list_one = complete_list.get(&(needed_number / i)).unwrap();
        let possibility_list_two = complete_list.get(&i).unwrap();

        for j in possibility_list_one {
            for k in possibility_list_two {
                if return_rep_nums(*j) & return_rep_nums(*k) == 0
                    && length_of_rep(*j) + length_of_rep(*k) <= 6
                    && length_of_rep(*j) + length_of_rep(*k) <= lowest_path
                {
                    let length_of_solutions = length_of_rep(*j) + length_of_rep(*k);
                    let reps = return_rep_nums(*j) | return_rep_nums(*k);
                    let new_rep = length_of_solutions | reps;

                    lowest_path = length_of_rep(*j) + length_of_rep(*k);
                    possibility_vec.push(new_rep);

                    // println!(
                    //     "division to reach {needed_number}: {i} {}",
                    //     needed_number / i
                    // );
                }
            }
        }
    }

    //Add list to complete list
    possibility_vec.sort();
    possibility_vec.dedup();
    for i in 0..possibility_vec.len() - 1 {
        if possibility_vec.len() < 1 {
            return;
        }
        if length_of_rep(possibility_vec[i]) < lowest_path {
            possibility_vec.remove(possibility_vec[i] as usize);
        }
    }
    complete_list.insert(needed_number, possibility_vec);
}
