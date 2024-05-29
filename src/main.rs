use numberswap_srikanth_kotturi::swap_numbers;

fn main() {
    let mut num1 = 10;
    let mut num2 = 20;

    println!("Before swapping:");
    println!("num1 = {}, num2 = {}", num1, num2);

    swap_numbers(&mut num1, &mut num2);

    println!("After swapping:");
    println!("num1 = {}, num2 = {}", num1, num2);
}
