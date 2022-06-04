use std::env;
use std::io;
use std::io::{Write, BufWriter};
use std::fs::File;

fn no_memory_leak(file : File) -> io::Result<()> {
  let mut writer = BufWriter::new(&file);
  for i in 0..1000000{
      let line = format!("{}\n", i);
      writer.write(&line.into_bytes())?;
  }
  writer.flush().unwrap();
  Ok(())
}


fn memory_leak(file : File) -> io::Result<()> {
  for i in 0..1000000{
      let line = format!("{} {} {}\n", i, i, i);
      // create scope so writer is dropped at the end
      // which flushes the buffer
      {
          let mut writer = BufWriter::new(&file);
          writer.write(&line.into_bytes())?;
          // even if flushing manually it doesn't work
          writer.flush().unwrap();
      }
  }
  Ok(())
}

fn main() -> (){
    let filename = "test.txt";
    let file     = File::create(filename).expect("Could not create file");

    let mut args = env::args();
    args.next(); // Ignore program name

    let _result =
    match args.next().unwrap_or(String::new()).as_str() {
        "good" => no_memory_leak(file),
         "bad" =>    memory_leak(file),
        _      => panic!("Invalid argument"),
    };

}
