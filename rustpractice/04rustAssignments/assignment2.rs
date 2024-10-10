fn most_frequent_word(text: &str) -> (String, usize) {
    let text_vector: Vec<&str> = text.split_whitespace().collect();
    
    let mut max_count = 0;
    let mut count = 0; //to compare to max_count
    let mut max_word = String::new(); 
    let mut word = max_word.clone(); //to compare to max_word
    for _i in 0..text_vector.len() {
      max_word = text_vector[_i].to_string();
      for _j in 0..text_vector.len(){
        if text_vector[_j] == max_word {
          count += 1;
        }
        word = max_word.clone();
        if count > max_count {
          max_count = count;
          max_word = word;
        }
      }
      
    }
     return (max_word, max_count); // return tuple
    
  }

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

