use std::fs;
use std::path::Path;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    enum ZeroZ {
      Bottom,
      Top,
    }

    enum ZeroXY {
      BottomLeft,
      CenterLeft,
      TopLeft,
      Center
    }

    // bit variables
    // let bit_name = "";
    let bit_number = 10;
    let bit_diameter = 0.25;
    let bit_feed = 0.25;
    let bit_pass_depth = 0.25;
    let bit_plunge = 0.25;
    let bit_rpm = 20000;
    // let bit_units = "in";
    
    // // project variables
    let project_units: &str = "in"; // should be predefined list of options
    // let zero_z: &str = "Bottom";
    let zero_z = ZeroZ::Bottom;
    // let zero_xy: &str = "bottom-left"; // should be predefined list of options
    let zero_xy = ZeroXY::BottomLeft;
    let angle: i32 = 0;
    let depth: f64 = 0.100;

    // material variables
    let material_width: f64 = 10.25;
    let material_height: f64 = 5.5;
    let material_thickness: f64 = 0.75;
    let retract_height: f64 = 0.25;

    
    // // load prefs and bits from json
    // let file_path = Path::new("./data.json");
    // let content = fs::read_to_string(file_path)?;
    // let data: Value = serde_json::from_str(&content)?;
    
    // // bits come in as an array and first one is loaded by default
    // struct Bit {
      //   bit_name: String,
      //   bit_diameter: f64,
      //   bit_feed:f64,
      //   bit_pass_depth:f64,
      //   bit_plunge:f64,
      //   bit_rpm:i32,
      //   bit_units: String,
      // }
      
      
fn get_start_x(zero_xy: ZeroXY, material_width:&f64) -> f64{
  match zero_xy {
    ZeroXY::BottomLeft => 0.0,
    ZeroXY::CenterLeft => 0.0,
    ZeroXY::TopLeft => 0.0,
    ZeroXY::Center => 0.000 - (0.5 * material_width),
  }
}

fn get_start_y(zero_xy: ZeroXY, material_height:&f64) -> f64{
  match zero_xy {
    ZeroXY::BottomLeft => 0.0,
    ZeroXY::CenterLeft => 0.0 - (0.5 * material_height),
    ZeroXY::TopLeft => 0.0 - material_height,
    ZeroXY::Center => 0.0 - (0.5 * material_height),
  }
}

  fn get_z_with_retract(zero_z: ZeroZ, material_thickness:&f64, retract_height:&f64) -> f64{
  match zero_z {
    ZeroZ::Bottom => material_thickness + retract_height,
    ZeroZ::Top => 0.0 + retract_height,
  }
}

  fn get_z_cutting_height(zero_z: ZeroZ, material_thickness:&f64, bit_pass_depth:&f64) -> f64{
  match zero_z {
    ZeroZ::Bottom => material_thickness + bit_pass_depth,
    ZeroZ::Top => 0.0 + bit_pass_depth,
  }
}      
      
      
      // output variables
    let mut gcode = String::new();
  
    // initial setup values in gcode
    gcode = format!("G90\n{}\nG53G0Z-0.197\nM05\nM6T{}\nM03S{}\n",get_unit_code(project_units),&bit_number.to_string(),&bit_rpm.to_string());
    
    // check xy start point option to determine the math that needs to be done
    // if bottom-left - stay at 0:0
    // if center-left - x stays 0, y is 0 - (1/2 height)
    // if top-left - x stays 0, y is 0 - height
    // if center - x is 0 - (1/2 width), y is 0 - (1/2) height
    // set start-x and start-y values accordingly
    // let start_x = if zero_xy == "center" {
    //   0.000 - (0.5 * material_width)
    // } else {
    //   0.000
    // };

    // let start_y = if zero_xy == "top-left" {
    //   0.0 - material_height
    // } else if zero_xy == "center" || zero_xy == "center-left" {
    //   0.0 - (0.5 * material_height)
    // } else {
    //   0.000
    // };
    
    
    // calculate bit starting point before plunging into material
    // if top z = 0 + retract height
    // if bottom z = height of material + retract height
    // let z_with_retract = if zero_z == "bottom" {
    //   material_thickness + retract_height
    // } else {
    //   0.000 + retract_height
    // };
    
    // calculate z start point
    // if top z = 0 - bit depth of cut
    // if bottom z = height of material - bit depth of cut
    // let z_height = if zero_z == "bottom" {
    //   material_thickness - bit_pass_depth
    // } else {
    //   0.000 - bit_pass_depth
    // };
    
    // write start codes
    let start_point = format!("G0X{}Y{}\nZ{}\nG1Z{}F{}\n",get_start_x(zero_xy, material_height).to_string(),get_start_y(zero_xy, material_height).to_string(), get_z_with_retract(zero_z,material_thickness,retract_height).to_string(), get_z_cutting_height(zero_z,material_thickness, bit_pass_depth).to_string(), bit_plunge.to_string());
    gcode.push_str(&start_point);



    

    
    // let unit_code= get_unit_code(units);
    // println!("{}", unit_code);
    
    
    
    
    // // Extract and print the school name
    // let prefs = data["name"].as_str().unwrap_or("Unknown");
    // println!("{}", prefs);
    
    println!("{}", gcode);
    Ok(())
}

fn get_unit_code(units: &str) -> &str {
     let code: &str = if units == "mm" {
        "G21"
      } else {
        "G20"
      };
      code
    }


// fn build_bit()