fn main() {
    println!("Hello, world!");
    call_with_42(|x| 2*x);
}

fn call_with_42<F>(f: F) where F: FnOnce(i32) -> i32 {
   println!("f(42) = {}", f(42))
}
