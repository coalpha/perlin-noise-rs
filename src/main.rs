use rand::prelude::*;

fn main() {
   let mut data = [0u8; 32];
   rand::thread_rng().fill_bytes(&mut data);
   data
      .into_iter()
      .map(|rand_u8| {
         let rand_f64 = *rand_u8 as f64;
         return (rand_f64.cos(), rand_f64.sin());
      })
      .for_each(|(x, y)| {
         println!("({}, {})", x, y);
      });
}
