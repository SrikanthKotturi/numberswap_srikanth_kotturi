pub fn swap_numbers(a: &mut i32, b: &mut i32) {
  let temp = *a;
  *a = *b;
  *b = temp;
}