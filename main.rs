
use std::thread;
use std::time::Instant;
use std::sync::mpsc;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;

fn main() {
    //notes before problem

    //lower bound number is 1 and upper bound is 100000000
    //lower = 1
    //upper = 100000000
    //upper divided by 8 is 12500000 so it is 1/8

    //make a receiver and sender to keep  track of data so it can do work parallel
    let (tx, rx) = mpsc::channel();
    
    //start the timer before threads are made
    let now = Instant::now();
    //this for loop decides how many threads for example 0..8 gives us 8 threads
    for t in 0..8{
        //we have to clone the sender to use it in the threads that we will spawn
        let send = tx.clone();
        //spawn our threads
        thread::spawn(move||{
            //tried to divide the work evenly for each thread by giving it 1/8 of 100000000
            //this divides the work of numbers in a fair manner
            //however the execution time will be longer the higher the number therefore, the work
            //is actually not distributed evenly by execution time but it will work in a parallel manner
            //since all threads will work at the same time
            let begin: i64 = 0+(t*12500000+1);
            let end = 12500000*(t+1);

            //pass beginning of the chunk and end with sender also
            check_prime_sqrt_2(begin, end, send);
            
        });
    }
    // we have to drop the sender here
    drop(tx);

    //make a list to store all numbers
    let mut lista = vec![];
    //initialize our sum value 0 and i64 so it doesn't overflow i32 will be overflow
    let mut sum:i64 = 0;
    //rx will have 8 list so we need another for loop inside to actually get the elements of each list and
    //push them into our new list
    for received in rx {
        for i in received{
            //push each number onto list
            lista.push(i);
            //sum each number
            sum = sum + i;
        }
    }
    //sort the list
    lista.sort();
    //get the top 10 numbers in list
    let top_10 = lista.as_slice()[lista.len()-10..].to_vec();
    //count how many elements in the list
    let count = lista.len();
    //since we have completed our task stop the execution time
    let elapsed = now.elapsed().as_secs();
    let mut f = BufWriter::new(File::create("primes.txt").expect("Unable to create file"));
    write!(f, "Elapsed: {}, Count: {}, Sum of count: {}, top10: {:?}\n", elapsed, count,sum,top_10);
    //file.write_all("}")

    //print the execution time, top 10 primes, count of primes and their sum
    //println!("Elapsed: {:?}", elapsed);
    //println!("Sum of count: {}", sum);
    //println!("top10: {:?}", top_10);
    //println!("Count is {}", count);

}

//function to make algorithm to check for primes i use the sqrt method and just coded the mathematical formula by making it return a boolean
//sieve would be faster but its confusing me so i decided to go with an approach i could understand and explain
//also was learning rust :)
fn check_prime(n: i64)->bool{
    if n == 2 || n == 3{
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0{
        return false;
    }

    let mut i = 5;
    while i*i <= n
    {
        if (n % i == 0) || (n % (i + 2) == 0){
            return false;
        }
        i = i + 6;
    }

    return true;
}
//function to check every number in decided range
fn check_prime_sqrt_2(i:i64,max: i64, send: mpsc::Sender<Vec<i64>>){
    //creating empty list
    let mut list: Vec<i64> = vec![];
    //for loop for the chunk given to the thread
    for i in i..max
    {
        //if number is prime push to list
        if check_prime(i) == true 
        {
            //pushing prime numbers into list
            list.push(i);
        }
    }
    //send the list to receiver
    send.send(list).unwrap();
}


