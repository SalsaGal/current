use current::random::Noise;

fn main() {
    let noise = Noise::from_seed(500);
    for index in 0..20 {
        println!("{}", noise.get(index));
    }
}
