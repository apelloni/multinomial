extern crate mpolynomial;
use mpolynomial::MPolynomial;

fn main() {
    let mut mpoly1 = MPolynomial::new(2);
    let mut mpoly2 = MPolynomial::new(2);
    mpoly1.add(&vec![0, 1], 1.0);
    mpoly1.add(&vec![1, 0], 1.0);
    mpoly2.add(&vec![0, 1], 1.0);
    mpoly2.add(&vec![1, 0], 1.0);
    println!("mploy1: {}", mpoly1);
    println!("mploy2: {}", mpoly2);
    mpoly1.mult(&mpoly2);
    mpoly1.mult(&mpoly2);
    println!("mpoly1*mpoly2: {}", mpoly1);
    let start = std::time::Instant::now();
    let mpoly = MPolynomial::linear_pown(&[1.0, 1.0], &[1, 2], 2, 2);
    println!("{}", mpoly);
    let mpoly = MPolynomial::linear_pown(&[11.0, 12.0, 99.0, 21.0], &[1, 2, 3, 4], 4, 30);
    println!("#{} in {:?}", mpoly.coeffs.len(), start.elapsed());
    //for (pows, c) in mpoly.powers.iter().zip(mpoly.coeffs.iter()) {
    //    print!("{:+}", c);
    //    for (n, p) in pows.iter().enumerate() {
    //        if *p > 0 {
    //            print!("*x{}^{}", n+1, p);
    //        }
    //    }
    //}
    //println!("");
    // test the time
    let n_var = 3;
    let n = 10;
    // Create two copies of (3x1 +5x2+ x3)
    let mut mpoly1 = mpolynomial::MPolynomial::new(n_var);
    mpoly1.add(&vec![1, 0, 0], 3.0);
    mpoly1.add(&vec![0, 1, 0], 5.0);
    mpoly1.add(&vec![0, 0, 1], 1.0);
    
    mpoly1.add(&vec![1, 1, 0], 3.0);
    mpoly1.add(&vec![0, 1, 1], 5.0);
    mpoly1.add(&vec![1, 0, 1], 1.0);
    mpoly1.add(&vec![1, 0, 1], 3.0);
    mpoly1.add(&vec![1, 1, 0], 5.0);
    mpoly1.add(&vec![0, 1, 1], 1.0);
    mpoly1.add(&vec![1, 1, 1], 3.0);
    mpoly1.add(&vec![0, 2, 0], 5.0);
    mpoly1.add(&vec![0, 0, 2], 1.0);
    mpoly1.add(&vec![2, 0, 0], 3.0);
    mpoly1.add(&vec![1, 2, 0], 5.0);
    mpoly1.add(&vec![1, 0, 2], 1.0);
    mpoly1.add(&vec![2, 1, 0], 3.0);
    mpoly1.add(&vec![0, 2, 1], 5.0);
    mpoly1.add(&vec![0, 1, 2], 1.0);
    mpoly1.add(&vec![2, 0, 1], 3.0);
    mpoly1.add(&vec![1, 2, 1], 5.0);
    mpoly1.add(&vec![1, 1, 2], 1.0);
    
    let mut mpoly2 = mpoly1.clone(); 
    let mut mpoly3 = mpoly1.clone(); 
    println!("General: {} pow {}", mpoly1, n);

    // multiply
    let start = std::time::Instant::now();
    for _ in 1..n {
        mpoly1.mult(&mpoly2);
    }
    println!("\t[mult]  in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    mpoly2.pown(n);
    println!("\t[pown]  in {:?}", start.elapsed());
    assert_eq!(mpoly1, mpoly2);
    let start = std::time::Instant::now();
    mpoly3.pown2(n);
    println!("\t[pown2] in {:?}", start.elapsed());
    assert_eq!(mpoly2, mpoly3);

    // power of linear expression 
    println!("Linear:");
    let mut mpoly1 = mpolynomial::MPolynomial::new(n_var);
    mpoly1.add(&vec![1, 0, 0], 3.0);
    mpoly1.add(&vec![0, 1, 0], 5.0);
    mpoly1.add(&vec![0, 0, 1], 1.0);
    let mut mpoly2 = mpoly1.clone(); 

    let start = std::time::Instant::now();
    for _ in 1..n {
        mpoly1.mult(&mpoly2);
    }
    println!("\t[mult]        in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    mpoly2.pown2(n);
    println!("\t[pown2]       in {:?}", start.elapsed());
    assert_eq!(mpoly1, mpoly2);
    let start = std::time::Instant::now();
    mpoly2 = mpolynomial::MPolynomial::linear_pown(&[1.0, 3.0, 5.0], &[3, 1, 2], n_var, n);
    println!("\t[linear_pown] in {:?}", start.elapsed());
    println!("#{}", mpoly1.coeffs.len());
    assert_eq!(mpoly1, mpoly2);

    let mut mpoly: MPolynomial<f64> = MPolynomial::new(3);
    mpoly.coeffs.clear();
    mpoly.powers.clear();
    mpoly.add(&vec![1, 0, 1], 1.0);
    mpoly.add(&vec![2, 1, 0], 3.0);
    mpoly.add(&vec![1, 3, 0], 5.0);
    println!("{}", mpoly);
    mpoly.replace(1, &[7.0], &[0]);
    println!("{}", mpoly);

    println!("Hello, world!");
}
