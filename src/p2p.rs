

#[cfg(test)]
mod tests {
    use crust::Service;
    use crust::read_config_file;
    use std::sync::mpsc::{ channel };

    #[test]
    fn test_conn() {
        let ( tx, rx ) = channel();
        let service = Service::new(tx, "hello");
    }
}

