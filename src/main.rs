fn main() {
    let cc = vec![1, 2, 3, 4, 5, 5, 6, 6, 7];

    get_every_other_digit(cc);
}

fn get_every_other_digit(cc: Vec<i32>) -> i32 {
    let mut cc_iter = cc.iter();
    let mut a = 0;
    let mut b = 0;

    a += cc_iter.next_back().unwrap();
    b += cc_iter.next_back().unwrap(); //note - if the number *2 is greater than 10 then I'll need to split that number into its constuent parts

    //https://cs50.harvard.edu/x/2020/psets/1/credit/

    32
}
