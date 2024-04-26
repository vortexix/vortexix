use std::io;


fn main () 
{
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();
    
    loop 
    {
    println!("Write a:");
    match io::stdin().read_line(&mut a_str)
    {
    Ok(_) => {},
    Err(e) => println!("Erorr - {}", e)
    }

    println!("Write b:");
    match io::stdin().read_line(&mut b_str)
    {
    Ok(_) => {},
    Err(e) => println!("Erorr -  {}", e)
    }

    println!("Write c:");
    match io::stdin().read_line(&mut c_str)
    {
    Ok(_) => {},
    Err(e) => println!("Erorr - {}", e)
    }
    

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    let d: f64 = (b*b) - 4.0 * (a * c);

    if  d > 0.0 {
        let x1 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2 = ((-b) - d.sqrt()) / (2.0 * a);

        println!("Yeah! Answer:  d= {}  1.radix:{}  2.radix: {}", d, x1, x2);
  
     if d == 0.0 
     {
        let x = (-b) / (2.0 *a);
        println!("Yeah! Answer:  d= 0  radix:{} ", x);
     }
     
     if d < 0.0 {
        println!("Radix not have! D = {}", d);
     }
    }
}
}