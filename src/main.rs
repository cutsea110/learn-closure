fn main() {
    println!("Hello, world!");
    call_with_42(|x| 2*x);

    // example 1
    // ok
    let x = Data(0);
    call_fn_once(|| {
      consume(x);
    });
    // error : cannot move ut of `x`, a captured variable in an `FnMut` closure
    // let x = Data(0);
    // call_fn_mut(|| {
    //   consume(x)
    // });
    // error : cannot move ut of `x`, a captured variable in an `Fn` closure
    // let x = Data(0);
    // call_fn(|| {
    //   consume(x);
    // });

    // example 2
    // ok
    let mut x = Data(0);
    call_fn_once(|| {
      x.0 = 1;
    });
    // ok
    let mut x = Data(0);
    call_fn_mut(|| {
      x.0 = 1;
    });
    // error : cannot assign to `x.0` as `Fn` closures cannot mutate their captured variables
    // let mut x = Data(0);
    // call_fn(|| {
    //   x.0 = 1;
    // });

    // example 3
    // ok
    let x = Data(0);
    call_fn_once(|| {
      println!("{}", x.0);
    });
    // ok
    let x = Data(0);
    call_fn_mut(|| {
      println!("{}", x.0);
    });
    // ok
    let x = Data(0);
    call_fn(|| {
      println!("{}", x.0);
    });
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

fn call_fn_once<F>(f: F) where F: FnOnce() {
   f();
}

fn call_fn_mut<F>(mut f: F) where F: FnMut() {
   f();
}

fn call_fn<F>(f: F) where F: Fn() {
   f();
}

struct Data(i32);

fn consume(x: Data) {
   println!("{}", x.0);
}
