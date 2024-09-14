fn main() {
    println!("Hello, world!");
    println!("{}",is_even(20));
    println!("the number 5 in the sequence {}",find_fib(5));
}


fn is_even(num : i32) -> bool {
    if num%2 == 0 {
        return true;
    }
    return false;
}
        
fn find_fib(num : u32) -> u32 {
    if num == 1 || num == 2  {
        return num ;
    }else {
        return find_fib(num-1) + find_fib(num-2)
    }
}