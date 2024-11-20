use number_lib::big_number::Bignum;

fn main() {
    let a = Bignum::new("012300");
    a.print();
    let b = Bignum::new(321123);
    b.print();
    let c = Bignum::new("0000000");
    c.print();
    println!("{}  {:?}", c.trailing_zeros(), c.valid_digits())
}