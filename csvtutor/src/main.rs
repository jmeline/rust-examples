// This makes the csv crate accessible to your program.
extern crate csv;

// The `main` function is where your program starts executing.
fn main() {
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_path("Rate_PUF.csv");
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        if &record[2] == "38344" {
            println!("{:?}", record);
        }
    }
    println!("Done!");
}
