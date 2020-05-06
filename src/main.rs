fn eval(input_number : u32) -> (u32,u32) {
    let mut n = input_number.clone();
    let mut bits = input_number.clone();
    while n != 0 {
        bits = bits - (n & 1);
        n = n >> 1;
    }
    (n,bits)
}

fn primes(input_number : u32) -> Vec<u32>{
    let mut primes = vec![2,3];
    let mut tog : bool = false;
    let lim = input_number / 3;
    // println!("{}",lim);
    // let ci =  constant::from(lim)
    let mut composite = std::vec::from_elem(false,lim as usize);
    let mut d1 :u32 = 8; 
    let mut d2 :u32 = 8; 
    let mut p1 :u32 = 3; 
    let mut p2 :u32 = 7; 
    let mut s :u32 = 7; 
    let mut s2 :u32 = 3; 
    let mut m :u32 = 0;
    // println!("{}", lim);
    while s < lim {
        // m = m + 1;
        // println!("{:?}",composite.get(m));
        if let Some(false) = composite.get(m as usize) {
            let inc = p1 + p2;
            for k in (s..lim).step_by(inc as usize) {
                // println!("{}", k);
                std::mem::replace(&mut composite[k as usize], true);
            }
            // println!("{:?}",composite);
            for k in (s+s2..lim).step_by(inc as usize) {
                // println!("{}", k);
                std::mem::replace(&mut composite[k as usize], true);
            }
            tog = !tog;
            // println!("{}",tog);
            // println!("{:?}",composite)
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
    // println!("Start");
    let mut k : u32  = 0;
    let mut p : u32  = 5;
    tog = false;
    while p <= input_number {
        // println!("{}",p);
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
    // println!("oke");
    
    return primes
    // println!("{:?}",composite);
    // 32
}

// fn odd_factorial(input_number : u32,primes : Vec<u32>) -> u32{
//     if let 12 = input_number {
//         return 1;
//     }
//     else{
//         let res :u32 = odd_factorial(input_number / 2, primes);
//         return res.pow(2)*swing
//         // return odd_factorial((input_number / 2, primes).pow(2),u32);
//     }
//     12
// }

fn swing(input_number : u32, primes : Vec<u32>) -> u32{
    let small_swing = vec![1,1,1,3,3,15,5,35,35,315,63,693,231,3003,429,6435,6435, 
    109395,12155,230945,46189,969969,88179,2028117,676039,16900975, 
    1300075,35102025,5014575,145422675,9694845,300540195,300540195];
    
    let output : u32;
    if input_number < 33 {
        output = small_swing[input_number as usize] as u32
    }
    
    let s = bisect_left(primes.clone(), (input_number as f64).sqrt() as u32 + 1) as usize;
    let d = bisect_left(primes.clone(), (input_number/3) + 1) as usize;
    let e = bisect_left(primes.clone(), (input_number/2) + 1) as usize;
    let g = bisect_left(primes.clone(), input_number + 1) as usize;

    let mut factors = vec![0;d-s];
    factors.copy_from_slice(&primes[s..d]);
    // : u32 = clone_from_slice(&y);
    // let temp_factor = &primes[s..d];
    for i in &primes[s..d]{
        let r = (input_number / i) & 1;
        if r == 1{
            factors.push(i.clone())
        }
    }
    output

}
fn bisect_left(input_vector : Vec<u32>, input_number : u32) -> u32{
    let mut index : u32 = 0;
    if input_vector[input_vector.len()-1 as usize] < input_number {
        index = (input_vector.len()-1) as u32
    }
    for i in 0..input_vector.len(){
        // println!("{}",i);
        if input_vector[i as usize] >= input_number{
            index = i as u32;
        } 
        // else if input_vector[i as usize] > input_number{
        //     index = (i-1) as u32
        // }
        
    } 
    index
}

fn main() {
    let (n,bits) = eval(31);
    // let c = vec![0;3];
    // // println!("{:?}",c.get(4));
    // if c.get(4).is_none(){
    //     println!("oke")
    // }
    // // if c.get(4).i
    println!("{:?}",primes(31));
    println!("{},{}",n,bits);
    // println!("{:?}",c);
    // println!("*");
    // let c = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    // let d = &c[3..5];
    // for i in d{
    //     let r = (38 / i) & 1;
    //     if r == 1 {
    //     println!("{}",i)}
    // }
    // let d = bisect_left(c, 3);
    // println!("{}",d) 
    // if c[1] <= 5 {
    //     println!("okee")
    // }
    // println!("{:?}",c[1]);
    // for i in 0..c.len(){
    //     // println!("{}",i);
    //     if c[i as usize] >= 3{
    //         println!("{}",i);
    //         println!("oke");
    //     }
    // }

    // println!("{},{}",bits,n)
}
