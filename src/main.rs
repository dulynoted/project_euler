mod common;
mod bitarray;
extern crate num;
extern crate time;
extern crate primal;

use time::PreciseTime;

fn main() {
    println!("pe1: {}", pe_1());
    println!("pe2: {}", pe_2());
    println!("pe5: {}", pe_5(20));
    println!("pe4: {}", pe_4());
    println!("pe6: {}", pe_6());
    println!("pe7: {}", common::naive_eratosthenes(150000)[10000]);
    println!("pe8: {}", pe_8());

    let mut start = PreciseTime::now();
    let mut seive = common::naive_eratosthenes(100000000);
    let mut end = PreciseTime::now();
    println!("Erasnothenes: {}, {} seconds.",seive[10000], start.to(end));

    start = PreciseTime::now();
    seive = common::bitarray_eratosthenes(1000000);
    end = PreciseTime::now();
    println!("simple bitarray Erasnothenes: {}, {} seconds.",seive[10000], start.to(end));
    
//    println!("{:?} seconds.",seive);

    start = PreciseTime::now();
    let count = primal::StreamingSieve::prime_pi(100000000);
    end = PreciseTime::now();
    println!("Primal: {} primes below 3000000 in {} seconds.",count, start.to(end));

    start = PreciseTime::now();
    seive = common::single_sieve(100000000);
    end = PreciseTime::now();
    println!("stolen Segmented Erasnothenes: {}, {} seconds.",seive[10000], start.to(end));
 

    println!("there are {} primes below 30 million", count);

    }

fn pe_1()-> i64{
    let x = 3 * common::summation(0,333);
    let y = 5 * common::summation(0,199);
    let z = 15 * common::summation(0,1000/15);
   // println!("x={}, y={}, x={}",x,y,z);
    return x + y - z;
}


fn pe_2()-> i64{
    let mut fib =1;
    let mut lastfib=1;
    let mut even_fib_sum=0;

    while fib<4000000 {
      //  println!("{}",fib);
        let temp_fib = fib;
        fib=fib+lastfib;
        lastfib=temp_fib;
        if fib%2==0 {
            even_fib_sum+=fib;
        }

    }

    return even_fib_sum

}

fn pe_4() -> usize{
	let mut max_palindrome=0;
	for i in (500..999).rev() {
		for j in (i..999).rev() {
			let  val = i*j;
			
			let magnitude  = common::get_magnitude(val);

			if common::is_palindrome(val, magnitude){
			//	println!("{}*{}={} ,{}",i,j,val, magnitude);
				if max_palindrome<val {
					max_palindrome = val;
				}
			}
			
		}
	} 
	return max_palindrome;
}




fn pe_5(x: usize) -> usize {
    //println!("X is {}",x);
    if x == 2 {
        return 1;
    }
    return common::lcm(x,pe_5(x-1));
}

  
fn pe_6() -> i64{
    let square_of_sum = num::pow(common::summation(0,100),2);
    let mut sum_of_squares = 0;
    for i in 1..101 {
        sum_of_squares+=i*i;
    }

//    println!("square of sum:{}\n sum of squares: {}", square_of_sum, sum_of_squares);
    return square_of_sum-sum_of_squares;
}


fn pe_8() -> usize {
    let sequence = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    let mut index =0;
    let mut product =1;
    let mut max_product =1;
    let chars= sequence.chars();
    let mut vec = vec![];
    let mut lastzero = 13;
    for c in chars {
        let digit = c.to_digit(10).unwrap();
        vec.push(digit as usize);
        
        
        if index>12 {
            if digit==0 {
                lastzero = -1;
                product = 1;
                //println!("Digit is zero product is {}",product);
            }
            else{
                product *= digit as usize;
               // println!("Digit: {}, lastzero: {}, product: {}",digit, lastzero, product);
                
            }
            //println!("{:?}\n",vec);
            if lastzero>12 {
                match vec.first() {
                   Some(x) =>{if x>&0 {product = product/x}},
                   None => {},
                }
                if  product > max_product {
                    max_product=product;
                }
            }
           vec.remove(0);
        }
                            
        lastzero+=1;
        index+=1;
        
    }

    return max_product;
}






