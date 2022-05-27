// use a loop to iterate through the input parameters and print the contents to the console.
fn main() {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        println!("{}", arg);
    }

    // Create a constant string that contains the help text.
    const HELP_TEXT: &str = "This is a help text.";
    
    // Create a constant string that contains the version text.
    const VERSION_TEXT: &str = "This is a version text.";

    // Create a constant string that containes the license text.
    const LICENSE_TEXT: &str = "This is a license text.";

    // Create a constant string that contains the liveofftheland text.
    const LIVE_OFF_THE_LAND_TEXT: &str = "This is a liveofftheland text.";

    // Create a constant string that contains the batch deploy text.
    const BATCH_DEPLOY_TEXT: &str = "This is a batch deploy text.";

    // Check if the input has a value equal to "--help".
    // If it does, print the help text and exit the program.
    if args.any(|x| x == "--help") {
        println!("{}", HELP_TEXT);
        return;
    }

    // Check if the input has a value equal to "--version".
    // If it does, print the version text and exit the program.
    if args.any(|x| x == "--version") {
        println!("{}", VERSION_TEXT);
        return;
    }

    // Check if the input has a value equal to "--license".
    // If it does, print the license text and exit the program.
    if args.any(|x| x == "--license") {
        println!("{}", LICENSE_TEXT);
        return;
    }

    // Check if the input has a value equal to "--liveofftheland".
    // If it does, print the liveoftheland text and exit the program.
    if args.any(|x| x == "--liveoftheland") {
        println!("{}", LIVE_OFF_THE_LAND_TEXT); 

        // Run sys_info to get the system information every 24 hours.
        // This is a blocking call.
        sys_info::run_sys_info();

        return;
    }

    // Check if the input has a value equal to "--batch-deploy".
    // If it does, print the batch deploy text and exit the program.
    if args.any(|x| x == "--batch-deploy") {
        println!("{}", BATCH_DEPLOY_TEXT);
        return;
    }

    // If none of the above conditions are met, print the help text and exit the program.
    println!("{}", HELP_TEXT);


}

// This is a blocking call.
fn run_sys_info() {
    // Create a thread that calls system_info every 24 hours.
    thread::Builder::new()
        .name("sys_info".to_string())
        .spawn(|| {
            loop {
                // Call system_info.
                system_info::system_info();

                // Sleep for 24 hours.
                thread::sleep(time::Duration::from_secs(86400));
            }
        });
}

// This function should print the following information to the console:
// - System name and version.
// - System architecture.
// - Number of cores.
// - Number of logical processors.
// - Number of physical processors.
// - System uptime.
// - System load.
// - System memory.
// - System swap.
// - System free disk space.
// - System total disk space.
// - System free inodes.
// - System total inodes.
fn system_info() {
    // Create a constant string that contains the system name and version.
    const SYSTEM_NAME_AND_VERSION: &str = "This is a system name and version.";
    // Create a constant string that contains the system architecture.
    const SYSTEM_ARCHITECTURE: &str = "This is a system architecture.";
    // Create a constant string that contains the number of cores.
    const NUMBER_OF_CORES: &str = "This is a number of cores.";
    // Create a constant string that contains the number of logical processors.
    const NUMBER_OF_LOGICAL_PROCESSORS: &str = "This is a number of logical processors.";
    // Create a constant string that contains the number of physical processors.
    const NUMBER_OF_PHYSICAL_PROCESSORS: &str = "This is a number of physical processors.";
    // Create a constant string that contains the system uptime.
    const SYSTEM_UPTIME: &str = "This is a system uptime.";
    // Create a constant string that contains the system load.
    const SYSTEM_LOAD: &str = "This is a system load.";
    // Create a constant string that contains the system memory.
    const SYSTEM_MEMORY: &str = "This is a system memory.";
    // Create a constant string that contains the system swap.
    const SYSTEM_SWAP: &str = "This is a system swap.";
    // Create a constant string that contains the system free disk space.
    const SYSTEM_FREE_DISK_SPACE: &str = "This is a system free disk space.";
    // Create a constant string that contains the system total disk space.
    const SYSTEM_TOTAL_DISK_SPACE: &str = "This is a system total disk space.";
    // Create a constant string that contains the system free inodes.
    const SYSTEM_FREE_INODES: &str = "This is a system free inodes.";
    // Create a constant string that contains the system total inodes.
    const SYSTEM_TOTAL_INODES: &str = "This is a system total inodes.";

    // Print the system name and version.
    println!("{}", SYSTEM_NAME_AND_VERSION);
    // Print the system architecture.
    println!("{}", SYSTEM_ARCHITECTURE);
    // Print the number of cores.
    println!("{}", NUMBER_OF_CORES);
    // Print the number of logical processors.
    println!("{}", NUMBER_OF_LOGICAL_PROCESSORS);
    // Print the number of physical processors.
    println!("{}", NUMBER_OF_PHYSICAL_PROCESSORS);
    // Print the system uptime.
    println!("{}", SYSTEM_UPTIME);
    // Print the system load.
    println!("{}", SYSTEM_LOAD);
    // Print the system memory.
    println!("{}", SYSTEM_MEMORY);
    // Print the system swap.
    println!("{}", SYSTEM_SWAP);
    // Print the system free disk space.
    println!("{}", SYSTEM_FREE_DISK_SPACE);
    // Print the system total disk space.
    println!("{}", SYSTEM_TOTAL_DISK_SPACE);
    // Print the system free inodes.
    println!("{}", SYSTEM_FREE_INODES);
    // Print the system total inodes.
    println!("{}", SYSTEM_TOTAL_INODES);
}
