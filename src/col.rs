use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::io::Write;

fn mk_iterf<P: AsRef<Path>>(filename: P) -> Box<dyn Iterator<Item = String>> {
    let file = File::open(filename).expect("Unable to open file");
    Box::new(
        BufReader::new(file)
            .lines()
            .map(|line| line.expect("Unable to read line").to_string()),
    )
}

pub fn concat<W: Write>(files: Vec<String>, mut writer: W) -> Result<(), Error> {
    // Create iterators over the files
    let mut iters: Vec<_> = files.iter().map(|filename| mk_iterf(filename)).collect();

    loop {
        let mut lines = Vec::new();
        let mut exhausted_iterators = 0;

        // Collect the next line from each iterator
        for iter in &mut iters {
            match iter.next() {
                Some(line) => lines.push(line),
                None => exhausted_iterators += 1,
            }
        }

        if exhausted_iterators == iters.len() {
            // All iterators are exhausted
            break;
        }

        if exhausted_iterators > 0 {
            // Some iterators are exhausted before others
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Files have different number of lines",
            ));
        }

        // Parse each line into timestamp and data
        let mut timestamps = Vec::new();
        let mut data_parts = Vec::new();
        for line in &lines {
            let mut parts = line.split_whitespace();
            if let Some(timestamp) = parts.next() {
                timestamps.push(timestamp.to_string());
                let data = parts.collect::<Vec<_>>().join(" ");
                data_parts.push(data);
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Line is empty or malformed: {:?}", line),
                ));
            }
        }

        // Verify that all timestamps match
        let first_timestamp = &timestamps[0];
        if !timestamps.iter().all(|t| t == first_timestamp) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Timestamps do not match across files",
            ));
        }

        // Write the combined data to the writer
        write!(writer, "{}", first_timestamp)?;
        for data in &data_parts {
            write!(writer, " {}", data)?;
        }
        writeln!(writer)?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_concat() {
        // Create temporary files with test data
        let mut file1 = NamedTempFile::new().unwrap();
        let mut file2 = NamedTempFile::new().unwrap();

        use std::io::Write;

        writeln!(file1, "1 data1_file1").unwrap();
        writeln!(file1, "2 data2_file1").unwrap();

        writeln!(file2, "1 data1_file2").unwrap();
        writeln!(file2, "2 data2_file2").unwrap();

        // Collect file paths
        let files = vec![
            file1.path().to_str().unwrap().to_string(),
            file2.path().to_str().unwrap().to_string(),
        ];

        // Create a buffer to capture output
        let mut buffer = Vec::new();

        // Call concat with the buffer as the writer
        concat(files, &mut buffer).unwrap();

        // Convert buffer to string
        let output_str = String::from_utf8(buffer).unwrap();

        // Define expected output
        let expected_output = "1 data1_file1 data1_file2\n2 data2_file1 data2_file2\n";

        // Assert equality
        assert_eq!(output_str, expected_output);
    }
}