fn main() {
    let mut v = vec![1, 2, 3];
    //Correct way to modify the first element 
    v[0] = 10;
    println!("The first element is: {}", v[0]);
    //Another safer alternative using iter_mut
    for x in v.iter_mut(){
        *x = *x * 2;
    }
    println!("The vector is {:?}",v);
} 