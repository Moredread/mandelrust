use mandelrust::mandelbrot::*;
use rug::Float;

fn main() {
    // High-precision values require appropriate precision setting
    // Calculating required precision based on decimal digits
    let x_str = "-1.94156695046089411519723545042405785041614289882129276726563682132425517088701591901639765783026799492137462058533435865811646793455507945";
    let y_str = "-0.00017301617109765913618843129805085094212526763902256359889382674446838992551267030035919209576862092841923454016429306276671035659004195";

    let prec = (x_str.len() as f64 / 2f64.log10()).ceil() as u32;

    let x = Float::parse(x_str).unwrap().complete(prec);
    let y = Float::parse(y_str).unwrap().complete(prec);
    let max = 203416u32;

    iterate_all::<Float>(x, y, max);
}
