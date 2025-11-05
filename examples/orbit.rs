use rug::Float;
use rug::ops::CompleteRound;
use mandelrust::mandelbrot::*;

fn main() {
    // High precision values for Mandelbrot iteration
    let prec = 512; // Use higher precision for these detailed coordinates
    let x = Float::parse("-1.94156695046089411519723545042405785041614289882129276726563682132425517088701591901639765783026799492137462058533435865811646793455507945")
        .unwrap()
        .complete(prec);
    let y = Float::parse("-0.00017301617109765913618843129805085094212526763902256359889382674446838992551267030035919209576862092841923454016429306276671035659004195")
        .unwrap()
        .complete(prec);
    let max = 203416u32;

    let result = iterate_all_float(x, y, max);
    println!("Computed {} iterations", result.len());
}
