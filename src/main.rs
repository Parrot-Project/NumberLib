use number_lib::big_number::Bignum;

fn main() {
    let a = Bignum::new("12345");
    a.print();
    let b = Bignum::new("654321");
    b.print();
}