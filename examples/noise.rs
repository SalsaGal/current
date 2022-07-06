use current::random::Noise;

fn main() {
    let noise = Noise::from_seed(132);
    for index in 0..20 {
        println!("{:032b}", noise.get(index));
    }
    println!();
    let noise = Noise::from_seed(5);
    for index in 0..20 {
        println!("{:032b}", noise.get(index));
    }
    println!();
    let noise = Noise::new();
    for index in 0..20 {
        println!("{:032b}", noise.get(index));
    }
}
