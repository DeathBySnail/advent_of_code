use std::fs::File;
use std::io;
use std::path::Path;

pub fn buffer_read<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file_result = File::open(filename);

    match file_result {
        Ok(file) => Ok(io::BufReader::new(file)),
        Err(e) => Err(e),
    }
}
