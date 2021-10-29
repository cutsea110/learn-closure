fn main() {
    println!("Hello, world!");
    call_with_42(|x| 2*x);
}

// an example of closure
fn call_with_42<F>(f: F) where F: FnOnce(i32) -> i32 {
   println!("f(42) = {}", f(42))
}

/*
// image of FnOnce/FnMut/Fn trait
pub trait FnOnce<Args> {
  type Output;
  extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
  type Output;
  extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args> : FnMut<Args> {
  type Output;
  extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
*/
