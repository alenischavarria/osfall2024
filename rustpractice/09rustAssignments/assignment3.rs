fn check_guess(guess: i32, answer: i32) -> i32{
    if guess == answer{
      return 0;
    }else if guess > answer{
      return 1;
    }else{
      return -1;
    }
  }
  
  
fn main() {
    let user_inputs: [i32;10] = [4, 7, 11, 14, 16, 10, 20, 32, 45, 19];
    let answer = 10;
    let mut amt_of_guesses = 0;
    println!("SECRET NUMBER [{}]", answer);
    for _i in 0..user_inputs.len(){
    let guess = user_inputs[_i];
    println!("Guess is {}", guess);
    let result = check_guess(guess, answer);
    amt_of_guesses += 1;
        if result == 0{
            println!("You win after {} guesses", amt_of_guesses);
            break;
        }else if result == 1{
            println!("Too high");
        }else{
            println!("Too low");
        }
    }
}