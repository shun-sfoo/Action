//因为文件支持seek(),即拥有向前或者向后移动到不同的位置上的能力，
//要让Vec<T>能够模拟文件，必须要额外做一些事情。而io::Cursor就是
//做这个的，它使得位于内存中的Vec<T>在行为上类似于文件.
use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

fn write_numbers_to_file() -> (u32, i8, f64) {
  let mut w = vec![];

  let one: u32 = 1;
  let two: i8 = 2;
  let three: f64 = 3.0;

  w.write_u32::<LittleEndian>(one).unwrap();
  println!("{:?}", &w);
  w.write_i8(two).unwrap();
  println!("{:?}", &w);
  w.write_f64::<LittleEndian>(three).unwrap();
  println!("{:?}", &w);

  (one, two, three)
}

fn read_number_from_file() -> (u32, i8, f64) {
  let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
  let one_ = r.read_u32::<LittleEndian>().unwrap();
  let two_ = r.read_i8().unwrap();
  let three_ = r.read_f64::<LittleEndian>().unwrap();
  (one_, two_, three_)
}

fn main() {
  let (one, two, three) = write_numbers_to_file();
  let (one_, two_, three_) = read_number_from_file();

  assert_eq!(one, one_);
  assert_eq!(two, two_);
  assert_eq!(three, three_);
}
