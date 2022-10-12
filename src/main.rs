use std::{process, fs::File, io::Write};
//Quick and dirty sine table generator of asm projecys
fn main() {
    
    if std::env::args().len() != 4 {
        println!("{0} <{1}> <{2}> <{3}>\n f.i: {0} sine 100 10", std::env::args().nth(0).expect(""), "label", "nr_steps", "sine_amplitude");
        process::exit(1);
    }

    let label: String = std::env::args().nth(1).unwrap();
    let n: i32 = std::env::args().nth(2).unwrap().parse().unwrap();
    let a: f32 = std::env::args().nth(3).unwrap().parse().unwrap();
    let mut asm = File::create(format!("{}.s", label)).expect("Oops could not create file");

    println!("{}:", label);
    asm.write(format!("{}:\n", label).as_bytes()).unwrap();
    let mut b = 0;
    for i in 0..n {
        let m = i as f32 / n as f32 * (2.0 * std::f32::consts::PI);
        let p = f32::sin(m) * a;

        if b==0{
            print!("    dc.b {0}, ", p as i32);
            asm.write(format!("    dc.b {0}, ", p as i32).as_bytes()).unwrap();
            b+=1;
        }

        if b > 0 && b <7 {
            print!("{0}, ", p as i32);
            asm.write(format!("{0}, ", p as i32).as_bytes()).unwrap();
            b+=1;
        } 

        if b == 7 {
            println!("{0} ", p as i32);
            asm.write(format!("{0} \n", p as i32).as_bytes()).unwrap();
            b=0;
        }
    }
}
