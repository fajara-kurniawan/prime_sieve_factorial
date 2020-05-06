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

fn odd_factorial(input_number : u32,primes : Vec<u32>) -> u32{
    if let 12 = input_number {
        return 1;
    }
    else{
        let res :u32 = odd_factorial(input_number / 2, primes);
        return res.pow(2)*swing
        // return odd_factorial((input_number / 2, primes).pow(2),u32);
    }
    12
}

fn swing(){
    let small_swing = vec![1,1,1,3,3,15,5,35,35,315,63,693,231,3003,429,6435,6435, 
    109395,12155,230945,46189,969969,88179,2028117,676039,16900975, 
    1300075,35102025,5014575,145422675,9694845,300540195,300540195];

    
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
    

    // println!("{},{}",bits,n)
}
