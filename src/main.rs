use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone)]
/// Represents a region of memory.
///
/// This struct is used to track a region of memory.
struct MemoryRegion {
    kind: String,
    
    /// The start address of the memory region.
    ///
    /// This is a u64 integer representing the start address of the memory region
    start_addr: u64,
    
    /// The end address of the memory region.
    ///
    /// This is a u64 integer representing the end address of the memory region.
    end_addr: u64,
    
    /// The size of the memory region in bytes.
    ///
    /// This is a u64 integer representing the size of the memory region in bytes.
    size: u64,
    
    /// The type of the memory.
    ///
    /// This is a string that describes the type of the memory. Example values are "MEM_IMAGE", "MEM_MAPPED", "MEM_PRIVATE", etc.
    mem_type: String,

    /// The state of the memory region.
    ///
    /// This is a string that describes the state of the memory region. Example values are "MEM_COMMIT", "MEM_FREE", "MEM_RESERVE", etc.
    state: String,

    /// The protection level of the memory region.
    ///
    /// This is a string that describes the protection level of the memory region. Example values are "PAGE_EXECUTE", "PAGE_READWRITE", "PAGE_NOACCESS", etc.
    protection: String,

    /// The usage of the memory region.
    ///
    /// This is a string that describes the usage of the memory region. For example it may contain the name of the image that is mapped into the memory region.
    usage: String,
}

/// Parses the output of !sos maddress into a HashMap.
///
/// This function takes a string that represents multiple memory regions, each on a new line,
/// and parses each line into a `MemoryRegion` struct. The function then inserts each `MemoryRegion`
/// into a HashMap, using the start address of the memory region as the key.
///
/// The function expects the input string to have a specific format. Each line should represent a memory region,
/// with fields separated by the "|" character. The fields should be in the following order: kind, start address,
/// end address, size, memory type, state, protection, and usage. The start address and end address should be
/// hexadecimal strings, and the size should be a human-readable size string.
///
/// The function skips the the table header (first three lines) of the input string.
///
/// # Arguments
///
/// * `input` - A string that holds the memory regions.
///
/// # Returns
///
/// * A HashMap where the keys are u64 integers representing the start addresses of the memory regions,
/// and the values are `MemoryRegion` structs representing the memory regions.
///
/// # Example
///
/// ```
/// let input = "kind|start_addr|end_addr|size|mem_type|state|protection|usage\n\
///              ----|----------|-------|----|-------|-----|---------|-----\n\
///              Stack|0x7ffeefbff000|0x7ffffffff000|132 kB|Private|RW|-|-";
/// let regions = parse_maddress_memory_regions(input);
/// assert_eq!(regions.len(), 1);
/// ```
fn parse_maddress_memory_regions(input: &str) -> HashMap<u64, MemoryRegion> {
    let mut regions = HashMap::<u64, MemoryRegion>::new();

    for line in input.lines().skip(3) {
        let fields: Vec<String> = line
            .split_terminator("|")
            .map(|part| part.trim().to_string())
            .collect();
        if fields.len() >= 9 {
            let region = MemoryRegion {
                kind: fields[1].to_string(),
                start_addr: parse_hex(&fields[2]),
                end_addr: parse_hex(&fields[3]),
                size: parse_human_readable_size(&fields[4]),
                mem_type: fields[5].to_string(),
                state: fields[6].to_string(),
                protection: fields[7].to_string(),
                usage: fields[8].to_string(),
            };
            regions.insert(region.start_addr, region);
        }
    }

    regions
}

/// Parses the output of !address into a HashMap.
///
/// This function takes a string that represents multiple memory regions, each on a new line,
/// and parses each line into a `MemoryRegion` struct. The function then inserts each `MemoryRegion`
/// into a HashMap, using the start address of the memory region as the key.
///
/// The function expects the input string to have a specific format. Each line should represent a memory region,
/// with fields separated by whitespace. The fields should be in the following order: start address,
/// end address, size, memory type, state, protection, kind, and usage. The start address and end address should be
/// hexadecimal strings, and the size should be a hexadecimal representation of the size in bytes.
///
/// The function skips the table header (first two lines) of the input string.
///
/// If a line has less than 9 fields, the function assumes that the missing fields are empty strings.
///
/// # Arguments
///
/// * `input` - A string that holds the memory regions.
///
/// # Returns
///
/// * A HashMap where the keys are u64 integers representing the start addresses of the memory regions,
/// and the values are `MemoryRegion` structs representing the memory regions.
///
/// # Example
///
/// ```
/// let input = "        BaseAddress      EndAddress+1        RegionSize     Type       State                 Protect             Usage \
/// -------------------------------------------------------------------------------------------------------------------------- \
/// +        0`00000000     569d`3bd6e000     569d`3bd6e000                                                            <unknown>  ";
/// let regions = parse_address_memory_regions(input);
/// assert_eq!(regions.len(), 1);
/// ```
fn parse_address_memory_regions(input: &str) -> HashMap<u64, MemoryRegion> {
    let mut regions = HashMap::<u64, MemoryRegion>::new();

    for line in input.lines().skip(2) {
        let fields: Vec<String> = line
            .split_whitespace()
            .map(|part| part.trim().to_string())
            .collect();
        if fields.len() >= 9 {
            let region = MemoryRegion {
                start_addr: parse_hex(&fields[1]),
                end_addr: parse_hex(&fields[2]),
                size: parse_hex(&fields[3].replace("`", "")),
                mem_type: fields[4].to_string(),
                state: fields[5].to_string(),
                protection: fields[6].to_string(),
                kind: fields[7].to_string(),
                usage: fields[8..].join(" ").to_string(),
            };
            regions.insert(region.start_addr, region);
        } else if fields.len() >= 4 {
            let region = MemoryRegion {
                start_addr: parse_hex(&fields[1]),
                end_addr: parse_hex(&fields[2]),
                size: parse_hex(&fields[3].replace("`", "")),
                mem_type: "".to_string(),
                state: "".to_string(),
                protection: "".to_string(),
                kind: "".to_string(),
                usage: fields[3..].join(" ").to_string(),
            };
            regions.insert(region.start_addr, region);
        }
    }

    regions
}

/// Parses a hexadecimal string into a u64 integer.
///
/// This function takes a string that represents a hexadecimal number,
/// removes any backtick characters from the string, and converts the result
/// into a u64 integer. If the string cannot be parsed into a u64, the function
/// returns 0.
///
/// # Arguments
///
/// * `hex_str` - A string that holds the hexadecimal number.
///
/// # Returns
///
/// * A u64 integer representing the parsed hexadecimal number. If the string
/// cannot be parsed into a u64, the function returns 0.
///
/// # Example
///
/// ```
/// let num = parse_hex("10");
/// assert_eq!(num, 16);
/// ```
fn parse_hex(hex_str: &String) -> u64 {
    u64::from_str_radix(hex_str.replace("`", "").as_str(), 16).unwrap_or(0)
}

/// Converts a size from a human-readable format to bytes.
///
/// # Arguments
///
/// * `size` - A string that holds the size in a human-readable format.
///
/// # Returns
///
/// * An u64 integer representing the size in bytes.
///
/// # Example
///
/// ```
/// let size = parse_human_readable_size("4,00kb");
/// assert_eq!(size, 4096);
/// ```
///
/// # Note
///
/// This function assumes that the input string is in the format `numberunit` without any space in between, 
/// and the number uses a comma as the decimal separator. If your input strings have a different format, 
/// you may need to adjust the function accordingly.
fn parse_human_readable_size(size: &String) -> u64 {
    
    let size = size.replace(",", ".").trim().to_lowercase();
    let (num, unit) = size.split_at(size.len() - 2);

    let num: f64 = num.parse().unwrap();
    let factor = match unit {
        "kb" => 1024.0,
        "mb" => 1024.0 * 1024.0,
        "gb" => 1024.0 * 1024.0 * 1024.0,
        "tb" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => {
            eprintln!("Unknown unit for size of memory page '{}'", unit);
            std::process::exit(1);
        }
    };

    (num * factor) as u64
}

/// Converts a size in bytes to a human-readable format.
///
/// # Arguments
///
/// * `bytes` - A u64 integer representing the size in bytes.
///
/// # Returns
///
/// * A string representing the size in a human-readable format. The size is rounded down to the nearest whole number of the appropriate unit.
///
/// # Example
///
/// ```
/// let size = bytes_to_human_readable(1500);
/// assert_eq!(size, "1KB");
/// ```
fn bytes_to_human_readable(mut bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut unit = units[0];

    for u in &units[1..] {
        if bytes < 1024 {
            break;
        }
        bytes /= 1024;
        unit = u;
    }

    format!("{}{}", bytes, unit)
}

fn print_memory_regions<I>(iterable: I) 
where
    I: IntoIterator<Item = (u64, MemoryRegion)>
{
    for (_, region) in iterable.into_iter() {
        println!(
            "{:12x} - {:12x} Size: {:8} Type: {:12} State: {:12} Protection: {:24} Kind: {:20} Usage: {}",
            region.start_addr,
            region.end_addr,
            bytes_to_human_readable(region.size),
            region.mem_type,
            region.state,
            region.protection,
            region.kind,
            region.usage
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the filename) is provided
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <sos.maddress.output> <windbg.address.output>",
            args[0]
        );
        std::process::exit(1);
    }

    // Get the filename from the arguments
    let sos_maddress_filename = &args[1];
    let windbg_address_filename = &args[2];

    let sos_maddress_memory_regions: HashMap<u64, MemoryRegion>;
    let windbg_address_memory_regions: HashMap<u64, MemoryRegion>; 
                                                                
    // Parse the SOS maddress memory table
    match fs::read_to_string(sos_maddress_filename) {
        Ok(contents) => {
            sos_maddress_memory_regions = parse_maddress_memory_regions(contents.as_str());
        }
        Err(err) => {
            eprintln!("Error reading file '{}': {}", sos_maddress_filename, err);
            std::process::exit(1);
        }
    }
    println!(
        "Memory regions found by sos maddress command: {}",
        sos_maddress_memory_regions.len()
    );

    // Parse the WinDBG address memory table
    match fs::read_to_string(windbg_address_filename) {
        Ok(contents) => {
            windbg_address_memory_regions = parse_address_memory_regions(contents.as_str());
        }
        Err(err) => {
            eprintln!("Error reading file '{}': {}", sos_maddress_filename, err);
            std::process::exit(1);
        }
    }
    println!(
        "Memory regions found by windbg address command: {}",
        windbg_address_memory_regions.len()
    );

    // Remove memory regions from the windbg_address_memory_regions that are known by sos maddress
    let mut all_memory_regions_not_in_maddress = windbg_address_memory_regions.clone();
    all_memory_regions_not_in_maddress.retain(|&k, _| !sos_maddress_memory_regions.contains_key(&k));
    println!("After distinct: {}", all_memory_regions_not_in_maddress.len());

    // Print the memory regions with known memory type (!address prints all memory regions, including free ones)
    let unaccounted_memory_regions:Vec<_> = all_memory_regions_not_in_maddress.into_iter().filter(|kvp| !String::is_empty(&kvp.1.mem_type) ).collect();
    println!("Memory regions found by windbg address but not by sos maddress: {}", unaccounted_memory_regions.len());
    print_memory_regions(unaccounted_memory_regions.clone());

    let unaccounted_size = &unaccounted_memory_regions.iter().map(|(_, region)| region.size).sum::<u64>();
    println!("Total unaccounted size: {}", bytes_to_human_readable(*unaccounted_size));

    // Print the memory regions found by sos maddress but not by windbg address (should be internal managed sub pages for GcHeap, LowFrequencyHeap, HighFrequencyHeap etc.)
    let mut all_memory_regions_not_in_address = sos_maddress_memory_regions.clone();
    all_memory_regions_not_in_address.retain(|&k, _| !windbg_address_memory_regions.contains_key(&k));
    println!("Memory regions found by sos maddress but not by windbg address: {}", all_memory_regions_not_in_address.len());
    print_memory_regions(all_memory_regions_not_in_address);

}
