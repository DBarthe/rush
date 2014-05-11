
use std::io;


/// A buffered reader which wrap the standard input or a file.
pub enum CmdReader {
	FileReader(io::BufferedReader<io::fs::File>),
	StdinReader(io::BufferedReader<io::stdio::StdReader>),
}

impl CmdReader {

	/// Instantiate a stdin CmdReader.
	pub fn new() -> CmdReader {
		StdinReader(io::stdin())
	}

	/// Open the file in read-only mode and wrap it.
	pub fn set_to_file(&mut self, filename: &str) -> io::IoResult<()> {
		 let reader = try!(io::fs::File::open(&Path::new(filename)));
		*self = FileReader(io::BufferedReader::new(reader));
		Ok(()) 
	}

	/// Like stdio::io::buffer::read_line().
	pub fn read_line(&mut self) -> io::IoResult<~str> {
		match self {
			&FileReader(ref mut br) => br.read_line(),
			&StdinReader(ref mut br) => br.read_line(),
		}
	} 

}