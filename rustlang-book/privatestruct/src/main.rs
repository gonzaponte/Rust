mod module {
    pub struct Pvt{
        pub a_number: i32,
    }

    pub fn get() -> Pvt { Pvt{ a_number: 42 } }

}

use module::Pvt;

fn main() {
    module::get();
    Pvt{a_number : 42};
    ()
}
