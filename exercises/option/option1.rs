// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(99));
    print_number(Some(99));

    let mapper = |v| ((v * 1235) + 2) / (4 * 16);
    let reducer = |mut acc: Vec<i32>, curr: i32| {
        acc.push(mapper(curr)); acc };
    
    // let mut numbers: [Option<u16>; 5];
    let res: Vec<i32> = (0..5).fold(vec![], reducer);
    
    let res2: Vec<i32> = (0..5).map(mapper).collect();
    
    // for iter in 0..5 {
    //     let number_to_add: u16 = {
    //         ((iter * 1235) + 2) / (4 * 16)
    //     };
    //
    //     numbers[iter as usize] = Some(number_to_add);
    // }
    println!("{:?}", res);
    println!("{:?}", res2);
}
