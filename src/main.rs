fn main() {
    let cc = vec![4, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4];

    let check_sum = check_sum(cc);
    let is_valid = is_cc_valid(check_sum);
    println!("{}", is_valid);
    if is_valid == true {
        println!("Valid card")
    } else {
        println!("Not a valid card")
    }
}

fn is_cc_valid(sum: u32) -> bool {
    if sum % 10 == 0 {
        true
    } else {
        false
    }
}

fn check_sum(cc: Vec<u32>) -> u32 {
    let mut cc_iter = cc.iter();

    //Split the vector into two vectors based on the instructions
    let mut simple = 0;
    let mut complex = vec![];

    loop {
        //simple: starting with the last digit, take out each 2nd number and find the total product
        match cc_iter.next_back() {
            Some(d) => simple += d,
            None => break,
        }
        //complex: starting with the second to last digit, take out each 2nd number
        match cc_iter.next_back() {
            Some(d) => complex.push(d),
            None => break,
        };
    }
    println!("{}", simple);
    //complex: multiply each digit of complex by 2
    let complex = complex.into_iter().map(|x| 2 * x).collect::<Vec<_>>();

    //if any of the digits in complex are > 9 then split that number into individual integers. eg: 14 -> 1 and 4.
    let split_complex = split_into_int(complex);
    println!("{:?}", split_complex);

    //take the product of split_complex
    let split_complex: u32 = split_complex.iter().sum();
    println!("{:?}", split_complex);

    //take the product of split_complex and the product of simple together
    let total = split_complex + simple;

    //https://cs50.harvard.edu/x/2020/psets/1/credit/

    total
}

fn split_into_int(complex: Vec<u32>) -> Vec<u32> {
    let mut complex_split: Vec<u32> = Vec::new();
    for i in complex {
        if i >= 10 {
            let mut digits: Vec<_> = i
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            complex_split.append(&mut digits);
        } else {
            complex_split.push(i);
        }
    }
    complex_split
}
