fn main() {
    println!("Hello, world!");

    // have sidecar json file that can be generated with bit settings and read in at run time

    // let x_length = 100;
    // let y_length = 100;
    // let x_margin = 10;
    // let y_margin = 10;
    // let x_speed = 10;
    // let y_speed = 10;
    // let z_speed = 10;
    // let d_o_c = 2;
    // let stepover = 5;
    // let pass_depth = 1;
    let units: &str = "in";
    println!("{}", units);

    let unit_code= get_unit_code(units);
    println!("{}", unit_code);

    let units: &str = "mm";
    println!("{}", units);

    // let unit_code: &str = if units == "mm" {
    //   "G21"
    // } else {
    //   "G20"
    // };

    

    let unit_code= get_unit_code(units);
    println!("{}", unit_code);

    
}

fn get_unit_code(units: &str) -> &str {
     let code: &str = if units == "mm" {
        "G21"
      } else {
        "G20"
      };
      code
    }