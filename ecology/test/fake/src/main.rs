use fake::{Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
}


fn main() {
// type derived Dummy
let f: Foo = Faker.fake();
println!("{:?}", f);
}