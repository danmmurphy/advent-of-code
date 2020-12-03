use std::fs::File;
use std::io::{Result, ErrorKind, BufRead, BufReader};

fn parse_line(line: String) -> Result<Vec<u32>> {
  let mut vec = Vec::new();
  for (i, pixel) in line.chars().enumerate() {
    if pixel == '#' {
      vec.push(i as u32)
    }
  }
  Ok(vec)
}

fn read_file(file_name: &str) -> Result<(u32, Vec<Vec<u32>>)>{
  let file = File::open(file_name)?;
  let tree_array: Vec<Vec<u32>> = BufReader::new(file)
    .lines()
    .map(|line| parse_line(line?).map_err(
      |e| std::io::Error::new(ErrorKind::InvalidData, e)
    ))
    .collect::<Result<Vec<Vec<u32>>>>()?;
  let file = File::open(file_name)?;
  let mut buf = String::new();
  BufReader::new(file).read_line(&mut buf)?;
  let line_len = buf.trim().len() as u32;

  Ok((line_len, tree_array))
}

fn main() -> Result<()> {
  let (line_len, tree_array) = read_file("input_3.txt")?;
  let pairs_to_try = [[1u32,1u32],[3u32,1u32],[5u32,1u32],[7u32,1u32],[1u32,2u32]];
  let mut counter = 1u32;
  for [x,y] in pairs_to_try.iter() {
      counter = counter * tree_array
      .iter()
      .enumerate()
      .filter(|(i,t)| t.iter().any(|&tree_pos| ((*i as u32)%y==0)&&(tree_pos==(*i as u32)*x % line_len)))
      .count() as u32
  }
  println!("{}",
    counter
    );
  Ok(())
}
