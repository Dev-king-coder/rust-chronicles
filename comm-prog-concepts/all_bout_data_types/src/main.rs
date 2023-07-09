use std::io;

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
        let z = 'z';
        let lz: char = 'ℤ'; // with explicit type annotation for char(in '')
        let heart_eyed_cat = '😻';
        println!("{heart_eyed_cat}{z}{lz}")
    }

    // COMPOUND TYPES

    // Tuples
        // Grouping multiple data types into one compound
        // Have fixed length: cannot grow or shrink
        // written as comma separated value
        // type annotations not a necessity
    {
        let tup = (500, 6.4, 1);
        //let tup: (i32, f64, u8) = (500, 6.4, 1);
        // destructing
        {
            let (x, y, z) = tup; 
            println!("x={x},y={y},z={z}");
        }
        // access using indicies    
        {
            let five_hundred = tup.0;
            let six_point_four = tup.1;
            let one = tup.2;
            println!("x={five_hundred};y={six_point_four};z={one}") 
        }
        // empty tuple is called a UNIT
        let _empty_tup=(); 
    } 

    // Arrays
        // every ele of same type
        // fixed length
        // stack of data
        // not flexible or extendable
        // useful when we knw number of ele wont change
    {
        let _arr:[i32;5]=[1,2,3,4,5];
        // accessing array elements
        let first = _arr[0];
        let second = _arr[1];
        println!("{first},{second}");
        let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
        let _unit_arr=[1;5];// let a=[1,1,1,1,1];
        
        // User givin index
        let mut index = String::new();
        println!("Enter index: ");
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
        let element = _arr[index];
        println!("The value of the element at index {index} is: {element}");
    }
}
