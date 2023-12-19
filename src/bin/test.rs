fn main() {
    for x in 1..=4000 {
        for m in 1..=4000 {
            for a in 1..=4000 {
                for s in 1..=4000 {
                    println!("{}", x * m * a * s);
                }
            }
        }
    }
}
