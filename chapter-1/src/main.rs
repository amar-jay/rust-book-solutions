use std::collections::HashMap;

/**
 *
 * Find the mean, mode and median of a given list of intergers 
 * */ 
fn question_one() {
    let mut arr = vec![1,3,4,6,7,4,8,9];
    let arr_len = arr.len();

    {
        // calculating the mean
    let mut sum = 0;
    for num in &arr {
        *&mut sum += num 
    }

    println!("Array average is {}", sum/arr_len);
    }

    {
        // calculating the mode
    let mut val:(i32, i32) = (0,0);
    let mut stack = HashMap::new();  
        for num in &arr {
            let cnt = stack.entry(*num).or_insert(0);
            *cnt += 1;
        }
        //val = (*stack.iter().nth(0).expect("fatal").0 as i32,*stack.iter().nth(0).expect("fatal").1 as i32);

       // TODO: Why didn't 👇 work? How is possibly uninitialized variables handled?
       let option = stack.iter().nth(0);
           match option {
               Some(usize) => {
               val = (*usize.0 as i32, 0);
                  val.1 = *usize.1 as i32;
              },
               None => {},
          }
 
        for x in stack.iter() {
          if *x.1 > val.1 {
              val = (*x.0 as i32, *x.1);
          }
      }
       
           println!("Array mode is {} ", val.0 );
    }

    {
        // calculating the median
        let mid_pt = (arr_len ) / 2;
        arr.sort();
        let median = arr[mid_pt];

        println!("Array median is {}", median);

    }

}


/** 
 * convert strings to pig_latin.
 * First constant of each word is moved to the end of a line
*/
fn question_two(text: String) {
    let map_func = |word: &str| -> String {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' => String::from(word) + "-hay",
            _ => {
                let mut xform = String::new();
                for ch in chars {
                    xform.push(ch);
                }
                xform.push('-');
                xform.push(first_char);
                xform.push_str("ay");
                xform
            }
        }
    };

    let folder = |current: String, next: String| -> String {
        let mut this_str = String::new();
        this_str.push_str(&current);
        if !current.is_empty() {
            this_str.push(' ');
        }
        this_str.push_str(&next);
        this_str
    };

    let ans = text.split_whitespace()
        .map(map_func)
        .fold(String::new(), folder);

//    let vowels = String::from("aeiou");
//    for v in vowels.chars() {
//        if v == string.chars().get(0){ 
//        println!("Array average is {}", v);
//        }
//    }
    println!("Array average is {}", ans);
}

fn question_three(){
    let mut stack = HashMap::new();  
    let data = vec![
        ("Taifun", "Enginneering"),
        ("Sally", "Enginneering"),
        ("Manan", "Coding"),
        ("Jay", "Enginneering"),

        ("Fisher", "Sales"),
        ("Amir", "Sales"),
    ];
        for each in &data {
            let pt = stack.entry(String::from(each.1)).or_insert(vec![]);
            pt.push(each.0);
        }
            // TODO: try to append to value without replacing the entire vec. 
            for (_,arr) in stack.iter_mut() {
                    arr.sort();
            }
            println!("data is {:#?}", stack);
}
fn main(){
    let str1 =  String::from("First");
    question_one();
    question_two(str1);
    question_three();
}
