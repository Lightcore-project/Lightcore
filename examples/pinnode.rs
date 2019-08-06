use crust::read_config_file;

fn main() {
    let config = read_config_file().unwrap();

    println!("{:?}",config);
}

