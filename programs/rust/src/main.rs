use std::ops::Add;
use std::ops::Mul;
use num::BigUint;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let Lim:i32 = args[1].parse::<i32>().unwrap();
    use std::time::Instant;
    let now = Instant::now();
    
    // for n in 0 .. 30 {
    //     println!("{}",fib2(n));
    // }
    // fib3(Lim);
    println!("Fibonacci {}: {}", Lim, fib4(Lim));
    // println!("Integer log base 2 of 35: {}",35_i32.ilog(2));
    // let n = 52;
    // for i in 0..i32::BITS{
    //     dbg!(n>>i &1);
    // }
    let elapsed = now.elapsed();
    println!("Run time: {:.2?}",elapsed);

}



fn fib(n: i32) -> i32{
    for i in 0 .. n {
        println!("{}", fib2(i));
    }
    return 0;
}

fn fib1(n:i32) -> i32{
    if n <= 1 {
        return n;
    }
    let num = fib1(n-1) + fib1(n-2);
    return num
}

fn fib2(mut n: i32) -> i32{
    let mut a = 0;
    let mut b = 1;
    let mut next = 0;

    while n > 0 {
        next = a + b;
        a = b;
        b = next;
        n = n - 1;
    }
    return a;
}

fn fib3(n: i32){
    let oone: [BigUint;2]=[BigUint::new(vec![0u32]),BigUint::new(vec!(1u32))];
    let oneo: [BigUint;2]=[BigUint::new(vec!(1u32)),BigUint::new(vec!(0u32))];
    let oneone: [BigUint;2]=[BigUint::new(vec!(1u32)),BigUint::new(vec!(1u32))];
    let m:u32 = n.ilog(2);
    let mut powers = Vec::new();
    let mut A = [oone.clone(),oneone.clone()];
    powers.push(A.clone());
    for _i in 1 .. m+1 {
        let B = big_int_mult(&A,&A);
        powers.push(B.clone());
        A = B;
    }
    let mut C = [oneo.clone(),oone.clone()];
    
        let mut j = 0;
        C = [oneo.clone(),oone.clone()];
        for B in powers.iter(){
            if n>>j & 1 ==1{
                C = big_int_mult(&C,&B);
            }
            j+=1;
        
    }
    println!("Fibonacci {}: {}",n,C[1][1]);
}

fn big_int_mult(A: &[[BigUint;2];2], B: &[[BigUint;2];2])->[[BigUint;2];2]{
    let mut a: [BigUint;2] = [BigUint::new(vec![0]),BigUint::new(vec![0])];
    let mut b: [BigUint;2] = [BigUint::new(vec![0]),BigUint::new(vec![0])];
    
    a[0] = A[0][0].clone().mul(&B[0][0]).add(&A[0][1].clone().mul(&B[1][0]));
    a[1] = A[0][0].clone().mul(&B[0][1]).add(&A[0][1].clone().mul(&B[1][1]));
    b[0]= a[1].clone();
    b[1] = A[1][0].clone().mul(&B[0][1]).add(&A[1][1].clone().mul(&B[1][1]));
    let C = [a,b];  

    return C;
}

fn nbu(n: u32)->BigUint{
    let res = BigUint::new(vec![n]);
    return res;
}

#[derive(Clone)]
struct FibMat {
    a00: BigUint,
    aad: BigUint,
    a11: BigUint,
}

impl FibMat {
    fn mul(&self, B: &FibMat) -> FibMat{
        let mut res = FibMat::new(0,0,0);
        res.a00 = self.a00.clone().mul(&B.a00).add(self.aad.clone().mul(&B.aad));
        res.aad = self.a00.clone().mul(&B.aad).add(self.aad.clone().mul(&B.a11));
        res.a11 = self.aad.clone().mul(&B.aad).add(self.a11.clone().mul(&B.a11));
        return res;
    }
    fn new(a: u32, b: u32, c: u32)->FibMat{
        let res = FibMat{
            a00: nbu(a),
            aad: nbu(b),
            a11: nbu(c)
        };
        return res;
    }
}

fn mat_powers(n: &i32)->Vec<FibMat>{
    let mut A = FibMat::new(1,1,0);
    let m = n.ilog(2);
    let mut powers = vec![A.clone()];
    for i in 1 .. m+1 {
        let B = A.mul(&A);
        powers.push(B.clone());
        A = B;
        println!("Computed A^{}",i+1);
    }
    return powers;
}

fn fib4(n: i32)->BigUint{
    if n == 0 {
        return nbu(0);
    }
    if n==1 || n==2{
        return nbu(1);
    }
    let powers = mat_powers(&n);
    let mut j = 0;
    let mut C = FibMat::new(1,0,1);
    for B in powers.iter(){
        if n+1>>j & 1 ==1{
            C = C.mul(&B);
        }
        j+=1;
    }
    return C.a00;
}