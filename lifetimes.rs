fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

fn main() {
    let z = "long";
    let zz = "longer";

    println!("{}", longest(z,zz));
}