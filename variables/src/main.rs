fn main() {
    let mut a = String::from("Hello");

    let r1 = &a;
    let r2 = &a;

    println!("{r1}, {r2}");

    let r3 = &mut a;

    r3.push_str(", world!");
    println!("{r3}");
}
