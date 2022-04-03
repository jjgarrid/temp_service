mod sensor_temp;

use std::process::Command;
use sensor_temp::sensor_temp::SensorTemp;


fn main() {
    let output = Command::new("sensors")
        .arg("-j")
        .output()
        .expect("failed to execute process");  
        
    
    let json = String::from_utf8(output.stdout);
    let output_sensor_temp;


    match json {
        Err(err) => println!("{}",err),
        Ok(sensor_temp) =>  { 
            output_sensor_temp = serde_json::from_str::<SensorTemp>(&sensor_temp);           
            match output_sensor_temp {
                Err(e) => println!("{}", e),
                Ok(sensor) => println!("The temperature for the core O is {} C", sensor.coretemp_isa_0000.core_0.temp2_input),
                    
        }
    }

    } 

    println!("This is the end");
}
