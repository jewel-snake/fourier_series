use project::{fourier,display_fourier,display_function};
use std::io::Write;

fn sawtooth(mut x: f64) -> f64 { if x <= std::f64::consts::PI && x >= -std::f64::consts::PI { x/2.0/std::f64::consts::PI } else {
    while x > std::f64::consts::PI {
        x-= std::f64::consts::PI*2.0;
    }
    while x < -std::f64::consts::PI {
        x+= std::f64::consts::PI*2.0;
    }
    sawtooth(x)
} }

fn pulse(x: f64) -> f64 { if x >0.0 {1.0} else {-1.0} }

fn tribyerr(x: f64) -> f64 { x.powi(3)*(-x.powi(2)).exp()}

fn tri(x: f64) -> f64 {x.powi(3)}

//fn ff(x: f64) -> f64 {if x != 0.0 {(x.recip().exp()-1.0)/(1.0+x.recip().exp())} else {1.0}}
fn errsq(x: f64) -> f64 {-(-x.powi(2)).exp()+x.powi(2)}
fn main() {
    let function = tri;
    let ans = fourier(function);
    let mut file = std::fs::File::create("fourier_args.csv").unwrap();
    ans.iter().for_each(|(a,b)| {
        write!(file, "{},{}\n",a,b).unwrap();
    });
    let fdots = display_fourier(ans);
    let dots = display_function(function);
    let mut file = std::fs::File::create("points.csv").unwrap();
    dots.iter().zip(fdots.iter()).for_each(|(&(x,y),&(_,fy))| {
        write!(file,"{},{},{}\n",x,y,fy).unwrap();
    });
    }
