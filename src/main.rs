mod days;

fn main() {
    for day in days::get_days() {
        println!("{day}");
    }
}
