const ALPHABET_SIZE: isize = 7;

fn main() {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./sequences/sequence-7-3.csv")
        .unwrap();

    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());

    let mut smallest_delta_record: Vec<isize> = Vec::new();
    let mut smallest_record: Vec<isize> = Vec::new();
    let mut smallest_sum: isize = isize::MAX;

    for result in csv_reader.deserialize() {
        let record: Vec<isize> = result.unwrap();

        let mut delta_record: Vec<isize> = Vec::new();
        for window in record.windows(3) {
            let delta = delta_3(window[2], window[1], window[0], ALPHABET_SIZE);
            delta_record.push(delta);
        }

        let sum = delta_record.iter().sum();
        if sum < smallest_sum {
            smallest_sum = sum;
            smallest_delta_record = delta_record;
            smallest_record = record;
        }
    }
    println!("Sequence:\t{:?}", smallest_record);
    println!("Delta Encoding:\t{:?}", smallest_delta_record);
    println!("Sum of Deltas:\t{}", smallest_sum);
    // csv_writer.serialize(smallest_record).unwrap();
    // csv_writer.serialize(smallest_delta_record).unwrap();
    csv_writer.flush().unwrap();
}

const fn delta_3(a_3: isize, a_2: isize, a_1: isize, modulo: isize) -> isize {
    (a_3 - a_2 + a_1).rem_euclid(modulo)
}
