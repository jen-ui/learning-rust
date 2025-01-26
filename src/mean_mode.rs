use std::io;
use std::collections::HashMap;

pub fn mean_mode(){
    
    let mut v:  Vec<i32> = Vec::new();
    println!("enter the list length");
    
    let mut vector_size= String::new();
    
    io::stdin()
        .read_line(&mut vector_size)
        .expect("failed to read line");
    
    let vector_size: u32 = vector_size.trim().parse().expect("Pleae enter a number");
    
    for _ in 0..vector_size{
        println!("enter a number");
        let mut number = String::new();
        
        io::stdin()
            .read_line(&mut number)
            .expect("Enter a number");
        
        let number: i32= number.trim().parse().expect("please enter the element");
        
        v.push(number)
    }
    
    println!("{v:?}");
    

    calculate_median(&mut v);
    calculate_mode(&v);

    
    
    println!("Hello, world!");
}

fn calculate_median(list: &mut Vec<i32>)-> i32{
    list.sort_by(|a, b| a.cmp(b));
    let med_pos=list.len()/2;
    println!("median postion:{}",med_pos);
    let median: &i32=&list[med_pos] ;
    println!("Median: {}",*median);
    *median
}

fn calculate_mode(list: &Vec<i32 >)-> i32{
    let mut map: HashMap<i32,i32> = HashMap::new();
    for number in list{
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    } 
    let mut greatest=0;
    let mut mode=0;
    for (number,count) in &map{
        if *count>greatest{
            greatest=*count;
            mode = *number;
        }
    }
    println!("{map:?}");
    println!("Mode: {}", mode);
    mode
}

