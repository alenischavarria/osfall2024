fn is_even(num: i32) -> bool{
    if num%2 == 0{
      return true;
    }else{
      return false;
    }
}

fn main(){
    let arr: [i32;10] = [4, 7, 11, 14, 16, 19, 20, 32, 45, 57];

    for i in 0..arr.len(){
        if arr[i] % 3 == 0 && arr[i] % 5 !=0{
        println!("Fizz");
        }else if arr[i] % 5 == 0 && arr[i] % 3 !=0{
        println!("Buzz");
        }else if arr[i] % 5 == 0 && arr[i] % 3 ==0{
        println!("FizzBuzz");
        }else{
            if is_even(arr[i]) == true{
                println!("{} is even", arr[i]);
            }else{
                println!("{} is odd", arr[i]);
            }
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < arr.len(){
        sum = sum + arr[i];
        i = i +1;
    }
    println!("Sum of array is {}", sum);

    let mut largest_num = arr[0];
    for i in 0..arr.len(){
        if arr[i] > largest_num{
        largest_num = arr[i];
        }
    }
    println!("Largest number of array is {}", largest_num);
}