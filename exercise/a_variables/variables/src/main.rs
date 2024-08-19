fn main() {

    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    
    //let mut missiles: i32 = STARTING_MISSILES;
    //let ready       : i32 = READY_AMOUNT;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
    
    //let missiles_left = missiles - ready; //COMPILER-warning: variable does not need to be mutable
    //println!("{} missiles left", missiles_left);
    
    //READY_AMOUNT = 1; //COMPILER-error: invalid left-hand side of assignment
 
    //let unused_variable = 3; //COMPILER-warning: unused variable

}
