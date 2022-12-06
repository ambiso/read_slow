use rand::{distributions::Uniform, Rng};
fn main() {
    let path = "testfile";
    let mut rng = rand::thread_rng();
    let mut s = Vec::<u8>::new();
    for _ in 0..400_000_000 {
        s.push(rng.sample(Uniform::new(0, 26)) + 'a' as u8);
    }
    std::fs::write(path, s).unwrap();
}
