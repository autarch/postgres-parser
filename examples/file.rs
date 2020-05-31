use postgres_parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("no filename");
    let contents =
        std::fs::read_to_string(filename).expect(&format!("failed to read file: {}", filename));

    let scanner = SqlStatementScanner::new(&contents);
    for (i, stmt) in scanner.into_iter().enumerate() {
        println!("#{}\n{}", i + 1, stmt.sql.trim_end());

        match stmt.parsed {
            Ok(parse_list) => {
                let as_json =
                    serde_json::to_string(&parse_list).expect("failed to convert to json");
                println!("-- {}", as_json);
                if stmt.payload.is_some() {
                    println!("/* DATA:\n{}\n*/", stmt.payload.unwrap());
                }
            }
            Err(e) => {
                println!("-- ERROR:  {:?}", e);
            }
        };
        println!();
    }
}
