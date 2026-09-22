fn main() {
    println!("Hello, world!");
    /**
     * values: 
     * x length
     * y length
     * depth of cut
     * bit diameter
     * stepover
     * xy feed rate
     * z feed rate
     * z pass depth
     * safe z height?
     * perimeter margin
     * zero point
     * 
     * start: read values in app already
     * then: allow imperial to metric conversion
     * then: read external file with params
     * then: GUI web interface
     * then: web interface touch friendly
     * ideal: can directly talk to GRBL and run machine without having to export a file and import it
     * 
     */
    // have sidecar json file that can be generated with bit settings and read in at run time

    let x_length = 100;
    let y_length = 100;
    let x_margin = 10;
    let y_margin = 10;
    let x_speed = 10;
    let y_speed = 10;
    let z_speed = 10;
    let d_o_c = 2;
    let stepover = 5;
    let pass_depth = 1;
    

}

