extern crate num;
use bitarray;

use num::pow;
//use self::bit_vec::BitVec;
//use std::mem::size_of;
use std::cmp;

//summation from N to M, where N is less than M
pub fn summation(n: i64, m:i64)-> i64{
    if m<n {
        panic!("{} is greater than {}", n, m);
    }
    let x = n*(n+1)/2;
    let y = m*(m+1)/2;

    return y-x;    

}

pub fn gcd(a: usize, b: usize) ->usize {
    let remainder = a%b;
    if remainder == 0 {
        return b;
    }
    else {
        return gcd(b, remainder);
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    return (a*b)/gcd(a,b);
}

pub fn get_magnitude(val: usize) -> usize{
	let mut magnitude =1;
	while val > num::pow(10,magnitude) {
		magnitude+=1;
	}
	////println!("{}, {}",val, magnitude);
	return magnitude-1;
}


pub fn is_palindrome(x:usize, magnitude:usize) -> bool {
	////println!("{},{}",x,magnitude);
	let first_digit=x/num::pow(10, magnitude);
	let last_digit=x%10;
	////println!("{}/{}={}, {}%10={}",x,num::pow(10, magnitude),first_digit, x, last_digit);
	if first_digit == last_digit {
		if magnitude <= 1 {
			return true;	
		}
		////println!("now checking {}", (x-first_digit*num::pow(10, magnitude))/10);
		return is_palindrome((x-first_digit*num::pow(10, magnitude))/10, magnitude-2);
	}
	return false;
}

pub fn bitarray_eratosthenes(n: usize)->Vec<usize> {
    let mut range = bitarray::BitArray::new();
    let mut primes = vec![2,3,5];
    let mut index = 7;
    let gaps = [4,2,4,2,4,6,2,6];
    let mut increment = gaps.iter().cycle();

    while index < n {
//        println!("{}: {}", index, range.get(index));
        if range.get(index){
            primes.push(index);
            if index*index<n{
                let mut j =index;
                
                while j*index<n{
                //for j in index..n/index {
                    range.set(j*index, false);
                    j+=2;
                }
            }
        }
    let next_increment = increment.next().unwrap();
    index+=*next_increment;
    }
    primes.shrink_to_fit();
    return primes;
}
 
pub fn naive_eratosthenes(n: usize)->Vec<usize> {
    let mut range = vec![true; n];
    let mut primes = vec![2,3,5];
    let mut index = 7;
    let gaps = [4,2,4,2,4,6,2,6];
    let mut increment = gaps.iter().cycle();

    while index < n {
        if range[index]==true{
            primes.push(index);
            if index*index<n{
                let mut j =index;

                while j*index<n{
                //for j in index..n/index {
                    range[j*index]=false;
                    j+=2;
                }
            }
        }
    let next_increment = increment.next().unwrap();
    index+=*next_increment;
    }
    primes.shrink_to_fit();
    return primes;
}
 

pub fn single_sieve (n: usize)->Vec<usize>{
    const LIMIT: usize = 64000*8;
    let sqrt  =  (n as f64).sqrt() as usize+1; 
    let mut primes = bitarray_eratosthenes(sqrt);

    let mut output = primes.to_owned();

    let primes = primes.split_off(3);
    let mut low: usize = sqrt;
    let mut high: usize = cmp::min(sqrt+LIMIT, n);

    let gaps = [6,4,2,4,2,4,6,2];
    let mut gaps = gaps.iter().cycle();
    let mut spin =1;
    while spin < low {
        spin+=*gaps.next().unwrap();
    }
 
    while low <= n {
        let mut mark=bitarray::BitArray::new();

        for prime in primes.iter() {
            let mut seive_start =  (low/prime) as usize * prime;
            if seive_start < low {
                seive_start +=*prime;
            }
            if seive_start%2 == 0 {
                seive_start +=*prime;
            }
            let mut j = seive_start;
            while j < high {
                mark.set(j-low, false);
                j+=*prime*2;
            }
        }
        while spin < high  {
            if mark.get(spin-low) {
                output.push(spin);
            }    
            spin+=*gaps.next().unwrap();
        }
        low = low + LIMIT;
        high = high + LIMIT;
        if high >= n {high = n;}
    }
    return output;
}
/*
pub fn segmented_eratosthenes(n: usize)->Vec<usize>{
  //  println!("{}", size_of::<bool>());
    let sqrt_n = (n as f64).sqrt() as usize+1; 
    let mut primes = naive_eratosthenes(sqrt_n);
    let mut output = primes.to_owned();
    let primes = primes.split_off(3);
    let gaps = [6,4,2,4,2,4,6,2];
    let mut gaps = gaps.iter().cycle();
    const SEGMENT_SIZE: usize = 3000; 
    let mut segment_start = sqrt_n;

    let mut seive = primes.iter().map(|x| x*x).collect::<Vec<_>>();
    let mut i = 0;

    //define and set the value of the first spoke of the wheel in the search range
    let mut sweep = (sqrt_n/30)*30+1;

   //  println!("n:{}, sqrt_n:{}, first searched prime:{}, sweep_start:{}, segment size:{}, intial start:{}",n, sqrt_n, primes[0], sweep, SEGMENT_SIZE, segment_start);


    while sweep < sqrt_n {
        sweep += *gaps.next().unwrap();
    // println!("incremented sweep: {}",sweep);

    }

    //find multiple of seiving primes at start of search range
    for prime in primes.iter(){
        while seive[i] < segment_start {
            seive[i]+=*prime;
        }
       i+=1; 
    }
  //  println!("{:?}",primes);
  //  println!("{:?}",seive);
    /* segment loop */
    while segment_start < n {
        let mut is_prime: [bool; SEGMENT_SIZE] = [true; SEGMENT_SIZE];
        i = 0;
        let segment_end = cmp::min(segment_start + SEGMENT_SIZE, n);
        
        //remove odd multiples of each prime until the multiple of the prime is over the range
        //LIMIT
        while i < primes.len(){
            let mut j = seive[i]-segment_start; //initial array offset

          //  println!("prime:{}, start:{} end:{}", primes[i], segment_start, segment_end);
            
            while seive[i] < segment_end {
            //    println!("i:{}, seive[i]:{} j:{}", i, seive[i], j);
                is_prime[j] = false;
          //      println!("is_prime[{}]={}",j, is_prime[j]);
                j+=2*primes[i];
                seive[i]+=2*primes[i];
            }
        //    println!("Next segment start for prime {}={}: {}",i, primes[i], seive[i]);
            i+=1;

        }

      //  println!("{}",is_prime[289]);
        //collect all unmarked numbers and store as prime
        while  sweep  < SEGMENT_SIZE {
   //         println!("{}", sweep-segment_start);
            if is_prime[sweep-segment_start] {
     //           println!("{} is prime", sweep);
                output.push(sweep);
            }
            sweep += *gaps.next().unwrap();

        }        

        segment_start+=SEGMENT_SIZE; 
    }
   // println!("{:?}",output);
    return output;
}

pub fn naive_segmented_eratosthenes(n: usize)->Vec<usize>{
    let sqrt_n = (n as f64).sqrt() as usize+1; 
    let mut primes = naive_eratosthenes(sqrt_n);
    let mut output = primes.to_owned();
    let primes = primes.split_off(3);
    let gaps = [6,4,2,4,2,4,6,2];
    const  SEGMENT_SIZE : usize = 8000; 
    

    let mut seg_indexes = primes.iter().map(|x| x*x).collect::<Vec<_>>();  
    let mut segment_start = sqrt_n; 

//    println!("{:?}", primes);
//    println!("n:{}, sqrt_n:{}, first searched prime:{}, primes.len:{}, segment size:{}, intial start:{}",n, sqrt_n, primes[0], primes.len(), segment_size, segment_start);


    /* segment loop */
    while segment_start < n {


        let mut seg : [bool; SEGMENT_SIZE] = [true; SEGMENT_SIZE];

//        println!("segment start: {}", segment_start);
        let segment_max=SEGMENT_SIZE+segment_start;

        /* eliminate candidates  */
           for i in 1..primes.len(){
          // println!("prime: {}, start {}", primes[i], seg_indexes[i]);
        
           let mut counter = seg_indexes[i];

           while counter<segment_start{
               counter+=primes[i];
          //     println!("counter: {}, segment_start: {}", counter, segment_start);
           }
           while counter < segment_max {
//              println!("{} is not prime", counter-segment_start+segment_start);
                              seg[counter-segment_start]=false;
                counter+=primes[i];
           }
           seg_indexes[i]=counter;
        }


        /* collect primes  */
        let mut wheel_position = (segment_start/30)*30+1;
        let mut increment = gaps.iter().cycle();
//        println!("wheel start:{}",wheel_position);
        while wheel_position<segment_start {
            let wheel_increment = *increment.next().unwrap();

//           println!("wheel position: {}, wheel increment:{}",wheel_position, wheel_increment );
             wheel_position+=wheel_increment;
        } 
//        println!("wheel position {}",wheel_position);

        let mut index =wheel_position-segment_start;
//        println!("prime collection start index: {}", index);
        while index < SEGMENT_SIZE && index+segment_start<n{
            if seg[index] == true{ 
//                println!("index: {}, {} is prime",index, segment_start+index);
                output.push(segment_start+index);
            }
            else {
//                println!("index: {}, {} is composite",index,  segment_start+index);
            }
           index+=*increment.next().unwrap(); 
        }
                
        segment_start+=SEGMENT_SIZE;
    }
//    println!("{:?}",output);
    return output;

}
*/






