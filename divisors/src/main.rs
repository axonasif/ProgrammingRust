use std::str::FromStr; 
use std::env; 

fn main() { 
    let mut numbers = Vec::new(); 
    for arg in env:: args().skip( 1) { 
        numbers.push( u64::from_str(& arg) 
           .expect(" error parsing argument")); 
    } 

    if numbers.len() == 0 { 
        eprintln!(" Usage: dividors NUMBER ..."); 
        std::process::exit( 1); 
    } 
    let mut d; 
    for m in &numbers[0..] { 
        d = dividors(*m); 

        println!("The divisors of {:?} is {:?}", m, d); 
    } 



}



fn dividors(m: u64) -> Vec<u64> {
    let mut dividorslist = Vec:: new(); 
     assert!(m != 0); 
        for n in 1..=m {
            if m % n == 0 {
                dividorslist.push(n);
            }

         }
         dividorslist
            

}

#[ test] 
fn test_divisors() { 
    assert_eq!( dividors(10), {1,2,5,10}); 
    assert_eq!( dividors(10, 9), {1,2,5,10}, {3,3}); 
}