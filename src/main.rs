use std::io;

fn eval(input_number : u128) -> u128 {
    if input_number == 0 {
        return 1
    }
    else if input_number < 20{
        let range_input : Vec<u128> = (2..input_number+1).collect::<Vec<u128>>();
        return products(&range_input, 0, input_number-2) 
    }
    let mut n = input_number.clone();
    let mut bits : u128 = input_number.clone();
    while n != 0 {
        bits = bits - (n & 1);
        n = n >> 1;
    }
    let primes = primes(input_number);
    odd_factorial(input_number, &primes) * 2u128.pow(bits as u32) 
}

fn primes(input_number : u128) -> Vec<u128>{
    let mut primes = vec![2,3];
    let mut tog : bool = false;
    let lim = input_number / 3;
    let mut composite = std::vec::from_elem(false,lim as usize);
    let mut d1 :u128 = 8; 
    let mut d2 :u128 = 8; 
    let mut p1 :u128 = 3; 
    let mut p2 :u128 = 7; 
    let mut s :u128 = 7; 
    let mut s2 :u128 = 3; 
    let mut m :u128 = 0;
    while s < lim {
        if let Some(false) = composite.get(m as usize) {
            let inc = p1 + p2;
            for k in (s..lim).step_by(inc as usize) {

                std::mem::replace(&mut composite[k as usize], true);
            }
            for k in (s+s2..lim).step_by(inc as usize) {
                std::mem::replace(&mut composite[k as usize], true);
            }
            tog = !tog;
            if let true = tog{
                s = s + d2;
                d1 = d1 + 16;
                p1 = p1 + 2;
                p2 = p2 + 2;
                s2 = p2.clone();
            } else {
                s = s + d1;
                d2 = d2 + 8;
                p1 = p1 + 2;
                p2 = p2 + 6;
                s2 = p1.clone();
            }
        }
        m = m+1;
    }
    let mut k : u128  = 0;
    let mut p : u128  = 5;
    tog = false;
    while p <= input_number {
        if let Some(false) = composite.get(m as usize) {
            primes.push(p);
        }
        k = k + 1;
        tog = !tog;
        if let true = tog {
            p = p + 2
        } else {
            p = p + 4
        }
        
    }
    return primes
}

fn odd_factorial(input_number : u128, primes : &[u128]) -> u128{
    if input_number < 2 {
        return 1;
    }
    else{
        let res :u128 = odd_factorial(input_number / 2, primes);
        return res.pow(2)*swing(input_number,primes) as u128
    }
}

fn swing(input_number : u128, primes : &[u128]) -> u128{
    let small_swing = vec![1,1,1,3,3,15,5,35,35,315,63,693,231,3003,429,6435,6435, 
    109395,12155,230945,46189,969969,88179,2028117,676039,16900975, 
    1300075,35102025,5014575,145422675,9694845,300540195,300540195];
    
    if input_number < 33 {
        return small_swing[input_number as usize] as u128
    }
    
    let s = bisect_left(primes.to_vec(), (input_number as f64).sqrt() as u128 + 1) as usize;
    let d = bisect_left(primes.to_vec(), (input_number/3) + 1) as usize;
    let e = bisect_left(primes.to_vec(), (input_number/2) + 1) as usize;
    let g = bisect_left(primes.to_vec(), input_number + 1) as usize;

    let mut factors = vec![0;e-g];
    factors.copy_from_slice(&primes[s..d]);
    for i in &primes[s..d]{
        let r = (input_number / i) & 1;
        if r == 1{
            factors.push(i.clone())
        }
    }
    for prime in &primes[1..s]{
        let mut p = 1;
        let mut q = input_number.clone();
        loop {
            q = q / prime;
            if q == 0{
                break
            }
            let q_check = q & 1;
            if q_check == 1 {
                p = p * prime;
            }
        }
        if p > 1{
            factors.push(p);
        }
    }
    let len_factors = factors.len() - 1;
    products(&factors, 0, len_factors as u128)
}
fn bisect_left(input_vector : Vec<u128>, input_number : u128) -> u128{
    let mut index : u128 = 0;
    if input_vector[input_vector.len()-1 as usize] < input_number {
        index = (input_vector.len()-1) as u128
    }
    for i in 0..input_vector.len(){
        if input_vector[i as usize] >= input_number{
            index = i as u128;
        } 
    } 
    index
}

fn  products(s : &Vec<u128>, n: u128, m: u128) -> u128{
    if n > m{
        return 1 
    }
    else if n == m{
        return s[n as usize] 
    }
    let k = (n + m) / 2;
    products(&s, n, k) * products(&s,k+1,m) as u128
}

fn main() {

    loop {
        println!("Please input number");
    
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let num = input.trim().parse::<u128>();
        if let Err(_) = num {
            println!("{} is not a number",input.trim());
            continue
        } else {
            println!("{}",eval(num.unwrap()))
            }
        }

    
}
