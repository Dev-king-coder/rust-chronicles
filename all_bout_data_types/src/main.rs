fn main() {
    // use _[varName] for unused variables

    // TYPES OF DATA TYPES IN RUST //

    // SCALAR TYPES

    // ints
    // isize/usize types depending on computer architecture(arch)
    {
        let _guess: u32 = "42".parse().expect("Not a number!");
    }
    
    // floats
    {
        let _x = 2.0; //f64
        let _y: f32 = 3.0; //f32
    }

    // bools
    {
        let _t = true;
        let _f: bool = false; // with explicit type annotation
    }

    // chars
    {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation for char(in '')
        let heart_eyed_cat = '😻';
        println!("{heart_eyed_cat}{c}{z}")
    }
}
