fn main() {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./sequences/sequence-5-2.csv")
        .unwrap();

    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());

    for result in csv_reader.deserialize() {
        const ALPHABET_SIZE: isize = 5;

        let mut record: Vec<isize> = result.unwrap();
        record.pop(); // Ignore leading zero

        let mut delta_record: Vec<isize> = Vec::new();
        for window in record.windows(2) {
            let delta = (window[1] - window[0]).rem_euclid(ALPHABET_SIZE);
            delta_record.push(delta);
        }

        csv_writer.serialize(delta_record).unwrap();
    }
    csv_writer.flush().unwrap();
}
