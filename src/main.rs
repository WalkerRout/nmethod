const THRESHOLD: f64 = 0.0000000001;

fn newtons_method(mut x: f64, f: impl Fn(f64) -> f64, fp: impl Fn(f64) -> f64) -> f64 {
  let mut iterations = 0;
  loop {
    iterations += 1;
    if iterations > 2000 {
      println!("Unable to converge!");
      return f64::NAN;
    }
    
    let x1 = x - (f(x) / fp(x));
    let h = (x1 - x).abs();
    if h < THRESHOLD { break; }
    x = x1;
  }
  x
}

fn main() {
  let x = -2.0;
  
  let f = Box::new(|x: f64| {
    -(x.exp() * (1.0 + x + x.exp())) / ((x.exp() + 1.0).powf(2.0))
  });
  let fp = Box::new(|x: f64| {
    let p1 = ((x - 2.0) * (-x).exp()) / ((1.0 + (-x).exp()) * (1.0 + (-x).exp()));
    let p2 = (2.0*x * (-2.0 * x).exp()) / (1.0 + (-x).exp()).powf(3.0);
    p1 - p2
  });
  
  println!("Root: x = {}", newtons_method(x, f, fp));
}
