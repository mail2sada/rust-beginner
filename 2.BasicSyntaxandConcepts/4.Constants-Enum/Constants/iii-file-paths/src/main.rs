/// Demonstraits constants
const CONFIG_FILE_PATH: &str = "/etc/myapp/config.json";
const DATA_DIR_PATH: &str = "/var/lib/myapp/data/";

fn read_config_file() {
    println!("Reading configuration from: {}", CONFIG_FILE_PATH);
    // Logic to read configuration file from the specified path
}

fn process_data_files() {
    println!("Processing data files from: {}", DATA_DIR_PATH);
    // Logic to process data files from the specified directory path
}

fn main() {
    println!("Demo:Usage of constants");
    read_config_file();
    process_data_files();
}
