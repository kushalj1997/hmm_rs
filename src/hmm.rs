// HMM

use std::{thread, time};
use nalgebra::DMatrix;
use rand::Rng;

struct MarkovProcess
{
  state: DMatrix<f64>,
}

impl MarkovProcess {
  // Option 1: Create a `new()` associated function
  fn new() -> Self {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    
    // Generate random values for a 7x7 matrix
    for _ in 0..49 {
      data.push(rng.gen_range(0.0..1.0));
    }
    
    MarkovProcess {
      state: DMatrix::from_row_slice(7, 7, &data),
    }
  }

  fn compute(&mut self)
  {
    loop {
      match self.state.clone().try_inverse() {
        Some(inv) => {
          println!("Inverse: {}", inv);
          self.state = inv;
          thread::sleep(time::Duration::from_millis(1));
        }
        None => {
          eprintln!("Matrix is not invertible");
          break;
        }
      }
    }
  }
}

fn main()
{
  // Option 1: Using the `new()` function
  let mut mp = MarkovProcess::new();
  mp.compute();
  
  // Option 2: Direct struct literal initialization
  // let mut mp = MarkovProcess {
  //   state: Conventional::zero((7, 7)),
  // };
  
  // Option 3: If you implement Default trait, you can use:
  // let mut mp = MarkovProcess::default();
}
