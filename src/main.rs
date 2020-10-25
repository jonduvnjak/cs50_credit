const AE: [u32; 4] = [3, 4, 3, 7];
const MC: [u32; 10] = [5, 1, 5, 2, 5, 3, 5, 4, 5, 5];
const VISA: [u32; 1] = [4];

fn main() {
    let cc = vec![4, 0, 0, 3, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4];
    let cc_type_validity = cc_type_check(&cc);
    let cc_type = cc_type(cc_type_validity);
    let cc_length = cc.len();

    println!("{}", cc_type);
    println!("{}", card_validity(&cc_length, &cc_type, &cc));
}

//Runs luhn's algorithm on the credit card number
fn cc_check_sum(cc: &Vec<u32>) -> bool {
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

    //complex: multiply each digit of complex by 2
    let complex = complex.into_iter().map(|x| 2 * x).collect::<Vec<_>>();

    //if any of the digits in complex are > 9 then split that number into individual integers. eg: 14 -> 1 and 4.
    let split_complex = split_into_int(complex);

    //take the product of split_complex
    let split_complex: u32 = split_complex.iter().sum();

    //take the product of split_complex and the product of simple together
    let total = split_complex + simple;

    let check_valid = is_check_sum_valid(total);

    check_valid
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

fn is_check_sum_valid(sum: u32) -> bool {
    if sum % 10 == 0 {
        true
    } else {
        false
    }
}

fn cc_type_check(cc: &Vec<u32>) -> Option<String> {
    match cc {
        _ if cc[0] == VISA[0] => Some(String::from("Visa")),
        _ if cc[0..1] == AE[0..1] => Some(String::from("American Express")),
        _ if cc[0..1] == AE[2..3] => Some(String::from("American Express")),
        _ if cc[0..1] == MC[0..1] => Some(String::from("Mastercard")),
        _ if cc[0..1] == MC[2..3] => Some(String::from("Mastercard")),
        _ if cc[0..1] == MC[4..5] => Some(String::from("Mastercard")),
        _ if cc[0..1] == MC[6..7] => Some(String::from("Mastercard")),
        _ if cc[0..1] == MC[7..8] => Some(String::from("Mastercard")),
        _ => None,
    }
}

fn cc_type(cc_type: Option<String>) -> String {
    match cc_type {
        Some(cc_type) => cc_type,
        None => String::from(""),
    }
}

fn card_validity(cc_length: &usize, cc_type: &String, cc: &Vec<u32>) -> String {
    match (cc_length, cc_type.as_ref(), cc_check_sum(&cc)) {
        (15, "American Express", true) => {
            String::from("This 15 digit American Express card is valid")
        }
        (16, "Master Card", true) => String::from("This 16 digit Mastercard is valid"),
        (13, "Visa", true) => String::from("This 13 digit Visa card is valid"),
        (16, "Visa", true) => String::from("This 16 digit Visa card is valid"),
        _ => String::from("You have typed in an invalid card number"),
    }
}
