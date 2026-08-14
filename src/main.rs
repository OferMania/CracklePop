
fn cracklepop(nn: i32) {
    for ii in 1..=nn {
        if ii % 15 == 0 {
            println!("CracklePop");
        } else if ii % 3 == 0 {
            println!("Crackle");
        } else if ii % 5 == 0 {
            println!("Pop");
        } else {
            println!("{}", ii);
        }
    }
}

fn main() {
    cracklepop(100);
}
