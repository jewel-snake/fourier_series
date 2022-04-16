use std::f64::consts::PI;
const A: f64 = -PI;
const B: f64 = PI;
const N_S: usize = 5000;
const N_F: usize = 100;
const N_A: usize = 1000;


fn simpson(mut f: impl FnMut(f64) -> f64) -> f64
{
  let mut ans = 0.0;
  let h = (B - A) / 2.0 / N_S as f64;
  for i in 0..N_S {
    ans += (f(A+h*(2*i) as f64)+4.0*f(A+h*(2*i+1) as f64) + f(A+h*(2*i+2) as f64))*h/3.0;
  }
  ans
}

pub fn fourier(f: fn(f64)-> f64) -> Vec<(f64,f64)> {
  let mut ans = Vec::new();
  //let a0 = 2.0*simpson(|x: f64| -> f64 {f(x)})/(B-A);
  for i in 0..=N_F {
    let ak = 2.0*simpson(|x: f64| -> f64 {
      f(x)*(2.0*PI/(B-A)*i as f64 *x).cos()
    })/(B-A);
    let bk = 2.0*simpson(|x: f64| -> f64 {
      f(x)*(2.0*PI/(B-A)*i as f64 *x).sin()
    })/(B-A);
    ans.push((ak,bk));
  }
  ans[0].0/=2.0;
  ans
}

pub fn display_fourier(inp: Vec<(f64,f64)>) -> Vec<(f64,f64)> {
  let dx = (B-A) / N_A as f64;
  let x_axis = (0..N_A).map(|i| A+i as f64 *dx).collect::<Vec<_>>();
  let y_axis = x_axis.iter().map(|x| inp.iter().enumerate().fold(0.0,|acc,(i,(a,b))| {
    acc+a*(i as f64 *x).cos()+b*(i as f64 *x).sin()
  })).collect::<Vec<_>>();
  x_axis.iter().zip(y_axis.iter()).map(|(&x,&y)| (x,y)).collect::<Vec<_>>()
}

pub fn display_function(f: fn(f64)->f64) -> Vec<(f64,f64)> {
  let dx = (B-A) / N_A as f64;
  let x_axis = (0..N_A).map(|i| A+i as f64 *dx).collect::<Vec<_>>();
  let y_axis = x_axis.iter().map(|&x| f(x)).collect::<Vec<_>>();
  x_axis.iter().zip(y_axis.iter()).map(|(&x,&y)|(x,y)).collect::<Vec<_>>()
}