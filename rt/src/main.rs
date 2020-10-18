pub mod save;
pub mod structs;
const dimension: u32 = 100;
mod simple;

fn main() {
    let picture = simple::generate();
    save::save(&picture);
}
