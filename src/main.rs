fn main() {
    println!("Hello, world!");
    call_with_42(|x| 2 * x);

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

    // additional 1
    // ok
    let x = CopyableData(0);
    call_fn_once(|| {
        not_consume(x);
    });
    // ok
    let x = CopyableData(0);
    call_fn_mut(|| {
        not_consume(x);
    });
    // ok
    let x = CopyableData(0);
    call_fn(|| {
        not_consume(x);
    });

    // additional 2
    let x = Data(0);
    let c = move || {
        println!("in closure: {}", x.0);
    };
    // x はクロージャに move されるので,ここでは使えない.
    // error : borrow of moved value: `x`
    // println!("in main: {}", x.0);

    c();

    // move なしで, x の所有権も必要としないケース
    // struct Closure {
    //   x: &Data
    // }

    // move あり, もしくは x の所有権も必要とするケース
    // struct Closure {
    //   x: Data
    // }

    // * move クロージャかどうか
    //    * 環境 => クロージャへの所有権の移転に関係
    // * FnOnce, FnMut, Fn のいずれを実装しているか
    //    * クロージャ => 外部の関数などへの所有権の移動に関係

    // Fn < FnMut < FnOnce
    // 可能な限りより大きい trait を使った方がより多くのクロージャを受け取れる
    // 逆にクロージャを受け取るときに Fn で受け取るようにすると一部のクロージャが受け取れない
    // よって、可能な限り Fn より FnMut, また FnMut より FnOnce でクロージャを受け取るべき

    // クロージャを引数として受け取るときの指針
    // - 渡されたクロージャを一回しか呼び出さないなら FnOnce で受ける
    // - 渡されたクロージャを複数回呼び出す可能性があるなら FnMut で受け取る
    // - クロージャの immutable 性を要求したいなら Fn で受ける

    // クロージャを返すときの指針
    // Fn は FnMut でもあり, FnMut は FnOnce でもある
    // だから引数で受け取る時とは逆に、可能な限り FnOnce より FnMut, また FnMut より Fn として返すべき
    // そうすれば、呼び出し側でより広い範囲で返却されたクロージャを使うことができる

    let mut c = make_counter(5, 2);
    println!("{}", c()); // 5
    println!("{}", c()); // 7
    println!("{}", c()); // 9

    let mut c = make_counter_box(5, 2);
    println!("{}", c()); // 5
    println!("{}", c()); // 7
    println!("{}", c()); // 9
}

// an example of closure
fn call_with_42<F>(f: F)
where
    F: FnOnce(i32) -> i32,
{
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

fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn call_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

struct Data(i32);

fn consume(x: Data) {
    println!("{}", x.0);
}

#[derive(Clone, Copy)]
struct CopyableData(i32);

// Data が Copy を実装しているので, move ではなく copy される
fn not_consume(x: CopyableData) {
    println!("{}", x.0);
}

fn make_counter(init: i32, inc: i32) -> impl FnMut() -> i32 {
    let mut x = init;
    move || {
        x += inc;
        x - inc
    }
}

fn make_counter_box(init: i32, inc: i32) -> Box<dyn FnMut() -> i32> {
    let mut x = init;
    Box::new(move || {
        x += inc;
        x - inc
    })
}
