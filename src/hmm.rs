// HMM

use nalgebra::DMatrix;
use rand::Rng;
use tokio::time::{sleep, Duration};

struct MarkovProcess
{
  state: DMatrix<f64>,
  id: u128,
}

impl MarkovProcess {
  // Option 1: Create a `new()` associated function
  fn new(id: u128) -> Self {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    
    // Generate random values for a 7x7 matrix
    for _ in 0..49 {
      data.push(rng.gen_range(0.0..1.0));
    }
    
    MarkovProcess {
      state: DMatrix::from_row_slice(7, 7, &data),
      id: id,
    }
  }

  async fn compute(&mut self)
  {
    loop {
      match self.state.clone().try_inverse() {
        Some(inv) => {
          println!("Process {} - Inverse: {}", self.id, inv);
          self.state = inv;
          sleep(Duration::from_millis(1)).await;
        }
        None => {
          eprintln!("Process {} - Matrix is not invertible", self.id);
          break;
        }
      }
    }
  }
}

#[tokio::main]
async fn main()
{
  let mut handles = Vec::new();
  
  for i in 0..77 {
    // Option 1: Using the `new()` function
    let mut mp = MarkovProcess::new(i);
    // Spawn each computation asynchronously
    let handle = tokio::spawn(async move {
      mp.compute().await;
    });
    handles.push(handle);
  }
  
  // Wait for all tasks to complete
  for handle in handles {
    handle.await.unwrap();
  }
}
