fn main() {
   let mystr = String::from("are we there yet?");
   let fw = first_word(&mystr);
   println!("The first word of the string: {}\nis: {}", mystr, fw);
}

fn first_word(s: &String) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate(){
       if item == b' ' {
       	  return &s[0..i];
       }
   }

   &s[..]
}