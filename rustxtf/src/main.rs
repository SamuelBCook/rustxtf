use std::fs::File;
use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::error::Error;
use regex::Regex;

fn main() {

    // &str is string slice instead of string 
    // &str is immutable and more mem efficient 
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
        ("ProjectionType", "12b", 182),  // Not currently used set to zero
        ("SpheriodType", "10b", 194),   // Not currently used set to zero
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

    let xtf_chan_info: Vec<(&str, &str, i32)> = vec![
        ("TypeOfChannel", "B", 0),
        ("SubChannelNumber", "b", 1),
        ("CorrectionFlags", "H", 2),
        ("UniPolar", "H", 4),
        ("BytesPerSample", "H", 6),
        ("Reserved", "i", 8),
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
        ("ReservedArea2", "53s", 75),
    ];

    let xtf_ping_header: Vec<(&str, &str, i32)> = vec![
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

    let xtf_ping_chan_header: Vec<(&str, &str, i32)> = vec![
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
    let mut data: Vec<u8> = Vec::new(); // initialise here so do not get possibly-uninitialised error


    match read_binary_data(filename) {
        Ok(d) => {
            data = d; // pass d out so remains in scope after match block
            println!("Read {} bytes from the file", data.len());
            // Optionally, process the binary data here
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    // Iterate over FileHeaders

    for (name, fmt, offset) in &xtf_file_headers {
        println!("Name: {}, Fmt: {}, Offset: {}", name, fmt, offset);
        let mut in_loop_fmt = fmt.to_string();

        // if is char split into number and type 
        let mut number = 0;
        if fmt.contains("s") {
            let (parsed_number, char_type) = match parse_size_and_type(fmt) {
                Ok((number, char_type)) => {
                    println!("Captured: Number = {}, Char = {}", number, char_type);
                    (number, char_type) // Return the values as a tuple
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return; // Early return to exit if there's an error
                }
            };
        
            // Update the outer number only if parsed_number is valid
            number = parsed_number; // Only update number here, it's already valid
            in_loop_fmt = char_type.to_string();
            println!("\nParsed number: {}, Parsed string: {}", number, in_loop_fmt);
        }
        println!("FMT AFTER IF STATEMENT {}",in_loop_fmt);

        let result = match in_loop_fmt.as_str() {
            "b" => {
                let byte_value = match read_and_decode_byte_as_number_u8(&data, *offset) {
                    Ok(byte_value) => {
                        println!("Byte value: {}", byte_value); // Print the decoded byte value
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e); // Print the error if the function returns an Err
                    }
                };
                Some(byte_value)
                },

            "f" => {
                let float_value = match read_float_from_binary_at_offset(&data, *offset) {
                    Ok(float_value) => {
                        println!("Float value: {}", float_value); // Print the decoded byte value
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e); // Print the error if the function returns an Err
                    }
                };
                Some(float_value)
            },
            "s" => {
                let string_value = match read_and_decode_bytes_as_string(&data, *offset, number) {
                    Ok(string_value) => {
                        println!("String value: {}", string_value); // Print the decoded string value
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e); // Print the error if the function returns an Err
                    }
                };
                Some(string_value) // Wrap the string_value in Some and return
            },
            "H" => {
                let short_value = match read_unsigned_short(&data, *offset) {
                    Ok(short_value) => {
                        println!("Short value: {}", short_value); // Print the decoded byte value
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e); // Print the error if the function returns an Err
                    }
                };
                Some(short_value) // Convert to f32 for consistency in return type
            },

            _ => {
                println!("Unknown value type: {}", fmt);
                None
            },
        };

        // Check and print the result
        // match result {
        //     Some(value) => println!("Decoded header {} value: {}", name, value),
        //     None => eprintln!("Failed to decode the value"),
        // }

        //Ok(())

    }

    // Match first value
    // let mut offset = 0;
    // match read_and_decode_byte_as_number_u8(&data, offset) {
    //     Ok(decoded) => println!("Decoded byte at offset {}: {}", offset, decoded),
    //     Err(e) => eprintln!("Error reading binary data: {}", e),
    // }

    // // Match first float
    // let offset = 208;
    // match read_float_from_binary_at_offset(&data, offset) {
    //     Ok(value) => println!("Float at offset {}: {}", offset, value),
    //     Err(e) => eprintln!("Error reading binary data: {}", e),
    // }

    // // Match first string
    // let offset = 2;
    // let mut num_bytes = 8;
    // match read_and_decode_bytes_as_string(&data, offset, num_bytes) {
    //     Ok(decoded) => println!("Decoded string: {}", decoded),
    //     Err(e) => eprintln!("Error reading and decoding binary data: {}", e),
    // }

    // // Match first unsigned short (H)
    // let offset = 34;
    // match read_unsigned_short(&data, offset) {
    //     Ok(decoded) => println!("Decoded unsigned short: {}", decoded),
    //     Err(e) => eprintln!("Error reading binary data: {}", e),
    // }


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

// TODO: pass values out of match statement 
// TODO: edit dtypes so that bytes that decode to nums and bytes which decode to chars are diff
// PLAN: Create func to read each dtype, then create funcs to read each type 
// of header (iterate through vect with match to decide which type of decode), then create
// function to read all of it at once - do not need to call func in match as short code
// TODO: add buffer to each to read exact amount?:
// Make it so can choose Endian-ness but defaults to littler