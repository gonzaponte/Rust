fn fibo(n : usize) -> i32 {
    if n <=1 {
        return 1;
    }

    fibo(n-1) + fibo(n-2)
}

fn main() {
    for i in 0..10 {
        println!("{}th number: {}", i, fibo(i));
    }
}
