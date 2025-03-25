use std::fs::File;
use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::error::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;

fn main() {

    // &str is string slice instead of string 
    // &str is immutable and more mem efficient 
    // unused values as z
    let xtf_file_headers: Vec<(&str, &str, usize)> = vec![
        ("FileFormat", "b", 0), // bytes as num
        ("SystemType", "b", 1), // byte as num
        ("RecordingProgramName", "8s", 2),
        ("RecordingProgramVersion", "8s", 10),
        ("SonarName", "16s", 18),
        ("SensorsType", "H", 34),
        ("NoteString", "64s", 36),
        ("ThisFileName", "64s", 100),
        ("NavUnits", "H", 164),
        ("NumberOfSonarChannels", "H", 166),
        ("NumberOfBathymetryChannels", "H", 168),
        ("NumberOfSnippetChannels", "b", 170),
        ("NumberOfForwardLookArrays", "b", 171),
        ("NumberOfEchoStrengthChannels", "H", 172),
        ("NumberOfInterferometryChannels", "b", 174),
        ("Reserved1", "b", 175),
        ("Reserved2", "b", 176),
        ("ReferencePointHeight", "b", 178),
        ("ProjectionType", "12z", 182),  // Not currently used set to zero
        ("SpheriodType", "10z", 194),   // Not currently used set to zero
        ("NavigationLatency", "H", 204), //was 2H
        ("OriginY", "f", 208),
        ("OriginX", "f", 212),
        ("NavOffsetY", "f", 216),
        ("NavOffsetX", "f", 220),
        ("NavOffsetZ", "f", 224),
        ("NavOffsetYaw", "f", 228),
        ("MRUOffsetY", "f", 232),
        ("MRUOffsetX", "f", 236),
        ("MRUOffsetZ", "f", 240),
        ("MRUOffsetYaw", "f", 244),
        ("MRUOffsetPitch", "f", 248),
        ("MRUOffsetRoll", "f", 252),
    ];

    let xtf_chan_info: Vec<(&str, &str, usize)> = vec![
        ("TypeOfChannel", "b", 0),
        ("SubChannelNumber", "b", 1),
        ("CorrectionFlags", "H", 2),
        ("UniPolar", "H", 4),
        ("BytesPerSample", "H", 6),
        ("Reserved", "H", 8), // was i
        ("ChannelName", "16s", 12),
        ("VoltScale", "f", 28),
        ("Frequency", "f", 32),
        ("HorizBeamAngle", "f", 36),
        ("TiltAngle", "f", 40),
        ("BeamWidth", "f", 44),
        ("OffsetX", "f", 48),
        ("OffsetY", "f", 52),
        ("OffsetZ", "f", 56),
        ("OffsetYaw", "f", 60),
        ("OffsetPitch", "f", 64),
        ("OffsetRoll", "f", 68),
        ("BeamsPerArray", "H", 72),
        ("SampleFormat", "b", 74),
        ("ReservedArea2", "53z", 75), // Not currently used set to zero
    ];

    let xtf_ping_header: Vec<(&str, &str, usize)> = vec![
        ("MagicNumber", "H", 0),
        ("HeaderType", "b", 2),
        ("SubChannelNumber", "b", 3),
        ("NumChansToFollow", "H", 4), // determines the number of XTFPINGCHANHEADERs to follow
        ("Reserved1", "2H", 6), // should be H
        ("NumBytesThisRecord", "H", 10),
        ("Year", "H", 14), 
        ("Month", "b", 16),
        ("Day", "b", 17),
        ("Hour", "b", 18),
        ("Minute", "b", 19),
        ("Second", "b", 20),
        ("HSeconds", "b", 21),
        ("JulianDay", "H", 22),
        ("EventNumber", "H", 24), 
        ("PingNumber", "H", 28),
        ("SoundVelocity", "f", 32),
        ("OceanTide", "f", 36),
        ("Reserved2", "2H", 40),
        ("ConductivityFreq", "f", 44),
        ("TemperatureFreq", "f", 48),
        ("PressureFreq", "f", 52),
        ("PressureTemp", "f", 56),
        ("Conductivity", "f", 60),
        ("WaterTemperature", "f", 64),
        ("Pressure", "f", 68),
        ("ComputedSoundVelocity", "f", 72),
        ("MagX", "f", 76),
        ("MagY", "f", 80),
        ("MagZ", "f", 84),
        ("AuxVal1", "f", 88),
        ("AuxVal2", "f", 92),
        ("AuxVal3", "f", 96),
        ("Reserved3", "f", 100),
        ("Reserved4", "f", 104),
        ("Reserved5", "f", 108),
        ("SpeedLog", "f", 112),
        ("Turbidity", "f", 116),
        ("ShipSpeed", "f", 120),
        ("ShipGyro", "f", 124),
        ("ShipYcoordinate", "d", 128),
        ("ShipXcoordinate", "d", 136),
        ("ShipAltitude", "H", 144),
        ("ShipDepth", "H", 146),
        ("FixTimeHour", "b", 148),
        ("FixTimeMinute", "b", 149),
        ("FixTimeSecond", "b", 150),
        ("FixTimeHsecond", "b", 151),
        ("SensorSpeed", "f", 152),
        ("KP", "f", 156),
        ("SensorYcoordinate", "d", 160),
        ("SensorXcoordinate", "d", 168),
        ("SonarStatus", "H", 176),
        ("RangeToFish", "H", 178),
        ("BearingToFish", "H", 180),
        ("CableOut", "H", 182),
        ("Layback", "f", 184),
        ("CableTension", "f", 188),
        ("SensorDepth", "f", 192),
        ("SensorPrimaryAltitude", "f", 196),
        ("SensorAuxAltitude", "f", 200),
        ("SensorPitch", "f", 204),
        ("SensorRoll", "f", 208),
        ("SensorHeading", "f", 212),
        ("Heave", "f", 216),
        ("Yaw", "f", 220),
        ("AttitudeTimeTag", "2H", 224),
        ("DOT", "f", 228),
        ("NavFixMilliseconds", "2H", 232),
        ("ComputerClockHour", "b", 236),
        ("ComputerClockMinute", "b", 237),
        ("ComputerClockSecond", "b", 238),
        ("ComputerClockHsec", "b", 239),
        ("FishPositionDeltaX", "h", 240),
        ("FishPositionDeltaY", "h", 242),
        ("FishPositionErrorCode", "b", 244),
        ("OptionalOffset", "2H", 245),
        ("CableOutHundredths", "b", 249),
        ("ReservedSpace2", "6b", 250),
    ];

    let xtf_ping_chan_header: Vec<(&str, &str, usize)> = vec![
        ("ChannelNumber", "H", 0),
        ("DownsampleMethod", "H", 2),
        ("SlantRange", "f", 4),
        ("GroundRange", "f", 8),
        ("TimeDelay", "f", 12),
        ("TimeDuration", "f", 16),
        ("SecondsPerPing", "f", 20),
        ("ProcessingFlags", "H", 24),
        ("Frequency", "H", 26),
        ("InitialGainCode", "H", 28),
        ("GainCode", "H", 30),
        ("BandWidth", "H", 32),
        ("ContactNumber", "2H", 34),
        ("ContactClassification", "H", 38),
        ("ContactSubNumber", "b", 40),
        ("ContactType", "b", 41),
        ("NumSamples", "H", 42),  // Number of samples in the data
        ("MillivoltScale", "H", 46),
        ("ContactTimeOffTrack", "f", 48),
        ("ContactCloseNumber", "b", 52),
        ("Reserved2", "b", 53),
        ("FixedVSOP", "f", 54),
        ("Weight", "h", 58),
        ("ReservedSpace", "4b", 60),
    ];

    // Read Binary Data
    let filename = "/Users/dev/Documents/sss_data/processed_raw_pair/GV_M_ECC_S0_GP_003H.xtf";
    //let filename = "/home/samuel/git/rovco/mbes_processing_tools/local_test/input/GP22_152_NLP_GS_GEOP_0011.001H.xtf";
    let mut data: Vec<u8> = Vec::new(); // initialise here so do not get possibly-uninitialised error


    match read_binary_data(filename) {
        Ok(d) => {
            data = d; // pass d out so remains in scope after match block
            println!("Read {} bytes from the file", data.len());
            // Optionally, process the binary data here
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    // header starts and lengths
    // let file_header_length = 254;
    // let chan_header_length = 256; 
    // let file_chan_header_length = 1024;

    // Iterate over FileHeaders
    let (file_headers_map, final_byte) = read_headers(xtf_file_headers, &data, 0);

    for (key, value) in &file_headers_map {
        match value {
            Some(val) => println!("Key: {}, Value: {:?}", key, val),
            None => println!("Key: {}, Value: None", key),
        }
    }
    println!("\nFinal byte file headers {} \n", final_byte);

    //Iterate over Chan Headers
    // let mut number_of_channels = file_headers_map.get("NumberOfSonarChannels").unwrap().as_ref();
    // let mut number_of_channels = match number_of_channels {
    //     Some(val) => val,
    //     None => {
    //         eprintln!("Error: Number of channels not found");
    //         return;
    //     }
    // };
    // Check the value and downcast it to an integer
    // let mut number_of_channels = match number_of_channels {
    //     Some(val) => {
    //         // Try to downcast to a concrete type, e.g., i32
    //         match val.downcast_ref::<i32>() {
    //             Some(&val) => val, // Successfully downcasted
    //             None => {
    //                 eprintln!("Error: Expected an integer value for NumberOfSonarChannels");
    //                 return; // Return or handle error
    //             }
    //         }
    //     },
    //     None => {
    //         eprintln!("Error: Number of channels not found");
    //         return; // Return or handle error
    //     }
    // };
    // println!("Number of channels: {:?}", number_of_channels);

    // // set up var to hold channel headers
    // let mut chan_headers_vec: Vec<HashMap<String, Option<Box<dyn std::fmt::Debug>>>> = Vec::new();

    // for i in 0..number_of_channels {
    //     // Here, `i` will range from 0 to number_of_channels - 1
    //     println!("Reading channel {}", i);

    //     let (channel_headers_map, final_byte) = read_headers(xtf_chan_info, &data, final_byte);

    //     for (key, value) in &channel_headers_map {
    //         match value {
    //             Some(val) => println!("Key: {}, Value: {:?}", key, val),
    //             None => println!("Key: {}, Value: None", key),
    //         }
    //     }

    //     chan_headers_vec.push(channel_headers_map);

    }

    

    
}


#[derive(Debug)]
enum HeaderValue {
    Byte(u8),
    Float(f32),
    String(String),
    Short(u16),
    Int(i32), // You can add more types as needed
}


fn read_headers(
    file_header: Vec<(&str, &str, usize)>, 
    data: &Vec<u8>, 
    base_offset: usize
) -> (HashMap<String, Option<Box<dyn std::fmt::Debug>>>, usize) {

    let mut final_byte = base_offset;
    let mut result_map: HashMap<String, Option<Box<dyn std::fmt::Debug>>> = HashMap::new();
    let mut latest_offset_plus_base = 0;
    let mut last_in_loop_fmt = String::new();
    let mut last_number = 0;

    for (name, fmt, offset) in &file_header {
       // println!("Name: {}, Fmt: {}, Offset: {}", name, fmt, offset);

        let mut in_loop_fmt = fmt.to_string();
        let mut offset_plus_base = base_offset + offset;
        latest_offset_plus_base = offset_plus_base;
    
        //println!{"Offset plus base: {}", offset_plus_base} //I think it is adding it all every it

        // if fmt is char or multi byte, split into number and type 
        let mut number = 0;

        if contains_number_and_z_or_s(fmt) {
            let (parsed_number, char_type) = match parse_size_and_type(fmt) {
                Ok((number, char_type)) => {
                    (number, char_type)
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return (result_map, final_byte);
                }
            };

            number = parsed_number;
            in_loop_fmt = char_type.to_string();
        }

        last_number = number;
        last_in_loop_fmt = in_loop_fmt.clone();

        let result = match in_loop_fmt.as_str() {
            "b" => {
                let byte_value = match read_and_decode_byte_as_number_u8(&data, offset_plus_base) {
                    Ok(byte_value) => {
                        //println!("Byte value: {}", byte_value); // Print the decoded byte value
                        Some(byte_value)
                    }
                    Err(e) => {
                        //eprintln!("Error: {}", e); // Print the error if the function returns an Err
                        None
                    }
                };
                byte_value.map(|v| Box::new(v) as Box<dyn std::fmt::Debug>)
            },

            "f" => {
                let float_value = match read_float_from_binary_at_offset(&data, offset_plus_base) {
                    Ok(float_value) => {
                        //println!("Float value: {}", float_value); // Print the decoded byte value
                        Some(float_value)
                    }
                    Err(e) => {
                        //eprintln!("Error: {}", e); // Print the error if the function returns an Err
                        None
                    }
                };
                float_value.map(|v| Box::new(v) as Box<dyn std::fmt::Debug>)
            },

            "s" => {
                let string_value = match read_and_decode_bytes_as_string(&data, offset_plus_base, number) {
                    Ok(string_value) => {
                        //println!("String value: {}", string_value); // Print the decoded string value
                        Some(string_value)
                    }
                    Err(e) => {
                        //eprintln!("Error: {}", e); // Print the error if the function returns an Err
                        None
                    }
                };
                string_value.map(|v| Box::new(v) as Box<dyn std::fmt::Debug>)
            },

            "H" => {
                let short_value = match read_unsigned_short(&data, offset_plus_base) {
                    Ok(short_value) => {
                        //println!("Short value: {}", short_value); // Print the decoded byte value
                        Some(short_value)
                    }
                    Err(e) => {
                        //eprintln!("Error: {}", e); // Print the error if the function returns an Err
                        None
                    }
                };
                short_value.map(|v| Box::new(v) as Box<dyn std::fmt::Debug>)
            },

            "z" => {
                let x = 0;

                // Convert the integer into Option<Box<dyn Debug>>
                let boxed_value: Option<Box<dyn Debug>> = Some(Box::new(x) as Box<dyn Debug>);
                boxed_value //return

            }

            _ => {
                println!("Unknown value type: {}", fmt);
                None
            },
        };

        // Insert the result into the map
        result_map.insert(name.to_string(), result);

        // Update the final_byte after processing each header
        
    }
    // final byte = base offset + last offset + size of last 

    let last_format_size = match last_in_loop_fmt.as_str() {
        "b" => 1,
        "f" => 4,
        "s" => last_number,
        "H" => 2,
        "z" => last_number,
        _ => {
            println!("Unknown value type: {}", last_in_loop_fmt.as_str());
            0  // You should return a valid number in the default case, or consider handling the error differently.
        }
    };

    let final_byte = latest_offset_plus_base + last_format_size;

    (result_map, final_byte) // Return the map
}




// fn read_headers(
//     file_header: Vec<(&str, &str, usize)>, 
//     data: &Vec<u8>, 
//     base_offset: usize
// ) -> (HashMap<String, Option<HeaderValue>>, usize) {

//     let mut final_byte = base_offset;
//     let mut result_map: HashMap<String, Option<HeaderValue>> = HashMap::new();
//     let mut latest_offset_plus_base = 0;
//     let mut last_in_loop_fmt = String::new();
//     let mut last_number = 0;

//     // Helper function to read and box values
//     fn read_and_box<F, T>(
//         fmt: &str,
//         offset: usize,
//         read_fn: F,
//     ) -> Option<HeaderValue>
//     where
//         F: Fn(usize) -> Result<T, String>,
//         T: 'static + std::fmt::Debug,
//     {
//         match read_fn(offset) {
//             Ok(value) => {
//                 // Determine type based on the format
//                 match fmt {
//                     "b" => Some(HeaderValue::Byte(value as u8)),
//                     "f" => Some(HeaderValue::Float(value as f32)),
//                     "s" => Some(HeaderValue::String(value.to_string())),
//                     "H" => Some(HeaderValue::Short(value as u16)),
//                     "z" => Some(HeaderValue::Int(value as i32)),
//                     _ => None,
//                 }
//             },
//             Err(e) => {
//                 eprintln!("Error reading '{}': {}", fmt, e);
//                 None
//             }
//         }
//     }

//     // Iterate over each header and process the data
//     for (name, fmt, offset) in &file_header {
//         let mut in_loop_fmt = fmt.to_string();
//         let mut offset_plus_base = base_offset + offset;
//         latest_offset_plus_base = offset_plus_base;

//         // If format contains a number and 'z' or 's', split into number and type
//         let mut number = 0;
//         if contains_number_and_z_or_s(fmt) {
//             let (parsed_number, char_type) = match parse_size_and_type(fmt) {
//                 Ok((number, char_type)) => (number, char_type),
//                 Err(e) => {
//                     eprintln!("Error: {}", e);
//                     return (result_map, final_byte);
//                 }
//             };
//             number = parsed_number;
//             in_loop_fmt = char_type.to_string();
//         }

//         last_number = number;
//         last_in_loop_fmt = in_loop_fmt.clone();

//         // Use a match for format types and call appropriate read functions
//         let result = match in_loop_fmt.as_str() {
//             "b" => read_and_box("b", offset_plus_base, read_and_decode_byte_as_number_u8),
//             "f" => read_and_box("f", offset_plus_base, read_float_from_binary_at_offset),
//             "s" => read_and_box("s", offset_plus_base, |offset| {
//                 read_and_decode_bytes_as_string(data, offset, number)
//             }),
//             "H" => read_and_box("H", offset_plus_base, read_unsigned_short),
//             "z" => {
//                 let x = 0;
//                 Some(HeaderValue::Int(x)) // Special case for 'z'
//             }
//             _ => {
//                 println!("Unknown value type: {}", fmt);
//                 None
//             }
//         };

//         // Insert the result into the map
//         result_map.insert(name.to_string(), result);
//     }

//     // Calculate the final byte position based on the last format type
//     let last_format_size = match last_in_loop_fmt.as_str() {
//         "b" => 1,
//         "f" => 4,
//         "s" => last_number,
//         "H" => 2,
//         "z" => last_number,
//         _ => {
//             println!("Unknown value type: {}", last_in_loop_fmt);
//             0
//         }
//     };

//     final_byte = latest_offset_plus_base + last_format_size;

//     (result_map, final_byte)
// }



fn contains_number_and_z_or_s(s: &str) -> bool {
    Regex::new(r"^\d{1,2}[zs]$").unwrap().is_match(s)
}


fn read_binary_data(filename: &str) -> io::Result<Vec<u8>> {
    // Open the file in read-only mode
    let mut file = File::open(filename)?;

    // Create a buffer to store the data
    let mut buffer = Vec::new();

    // Read the file content into the buffer
    file.read_to_end(&mut buffer)?;

    // Return the buffer containing the binary data
    Ok(buffer)
}


fn read_float_from_binary_at_offset(data: &[u8], offset: usize) -> Result<f32, Box<dyn Error>> {
    // Check if the offset is within bounds of the data
    if offset >= data.len() {
        return Err("Offset exceeds data length".into());
    }

    // Create a cursor to read from the binary data, starting at the given offset
    let mut cursor = Cursor::new(&data[offset..]);

    // Read the first 4 bytes from the offset and interpret them as a little-endian f32
    let value = cursor.read_f32::<LittleEndian>()?;

    // Return the float value
    Ok(value)
}


fn read_and_decode_byte_as_number_u8(data: &[u8], offset:usize) -> Result<u8, Box<dyn std::error::Error>> {
    // u8
    // Ensure the offset is within bounds
    if offset >= data.len() {
        return Err("Offset exceeds data length".into());
    }

    // Create a cursor from the data, starting from the specified offset
    let mut cursor = Cursor::new(&data[offset..]);

    // Read a single byte from the cursor
    let byte = cursor.read_u8()?;  // unsigned 8bit int?

    // Return the byte value (this is already a numeric value)
    Ok(byte)
}


fn read_and_decode_bytes_as_string(data: &[u8], offset: usize, num_bytes: usize) -> Result<String, Box<dyn std::error::Error>> {
    // Ensure the offset and the number of bytes to read are within bounds
    if offset + num_bytes > data.len() {
        return Err("Offset and number of bytes exceed data length".into());
    }

    // Create a cursor from the data, starting at the specified offset
    let mut cursor = Cursor::new(&data[offset..]);

    // Create a buffer to hold the bytes we want to read
    let mut buffer = vec![0; num_bytes];

    // Read the specified number of bytes into the buffer
    cursor.read_exact(&mut buffer)?;

    // Strip the padding bytes (assuming 0x00 is the padding byte)
    buffer.retain(|&byte| byte != 0x00);


    // Attempt to decode the bytes as a UTF-8 string
    match String::from_utf8(buffer) {
        Ok(decoded_string) => Ok(decoded_string),
        Err(_) => Err("Failed to decode bytes as UTF-8".into()), // Handle invalid UTF-8
    }
}


fn read_unsigned_short(data: &[u8], offset: usize) -> Result<u16, Box<dyn std::error::Error>> {
    // u16
    // Ensure the offset is within bounds (at least 2 bytes for u16)
    if offset + 1 >= data.len() {
        return Err("Offset exceeds data length or insufficient bytes for u16".into());
    }

    // Create a cursor from the data, starting at the specified offset
    let mut cursor = Cursor::new(&data[offset..]);

    // Read a 16-bit unsigned short (u16) using LittleEndian byte order
    let value = cursor.read_u16::<LittleEndian>()?;

    // Return the decoded u16 value
    Ok(value)
}


// // Generic function to read bytes from the given offset
// fn read_bytes_from_binary(data: &[u8], offset: usize, num_bytes: &i32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     // Ensure the offset and number of bytes are within bounds
//     if offset + num_bytes > data.len() {
//         return Err("Offset and number of bytes exceed data length".into());
//     }

//     // Create a cursor from the data, starting at the specified offset
//     let mut cursor = Cursor::new(&data[offset..]);

//     // Create a buffer to hold the bytes we want to read
//     let mut buffer = vec![0; num_bytes];

//     // Read the specified number of bytes into the buffer
//     cursor.read_exact(&mut buffer)?;

//     // Return the decoded bytes
//     Ok(buffer)
// }


fn parse_size_and_type(input: &str) -> Result<(usize, char), Box<dyn Error>> {
    // Create a regular expression to match a number followed by a character
    let re = Regex::new(r"^(\d+)([a-zA-Z])$")?;

    // Try to capture the number and the character
    if let Some(captures) = re.captures(input) {
        // Get the number part and parse it to usize
        let number: usize = captures[1].parse()?;

        // Get the character part directly from captures[2]
        let char_type: char = captures[2].chars().next().unwrap();

        Ok((number, char_type))
    } else {
        Err("Invalid format".into())
    }
}

// Make it so can choose Endian-ness but defaults to littler
// Instead of returning debug trait thing make an enum with different types and use that