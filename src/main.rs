fn main() {
    let check : bool = false;
    let check1 : bool = false;

    let mut add : Vec<i32> = Vec::new();
    let mut xs  = vec![1, 2, 0];
    //check if xs contains int 0
    for a in 0..xs.len(){
        if xs[a] != 0 && check1 == true{
            xs.push(0);
        } 
    }
    //sorting of the vector
    xs.sort();
    //lt count be equal to the first nnumber in the vector
    let mut count: i32 = xs[0];
    let mut count1: i32 = 1;
    // run a loop to check the vector
    for i in 0..xs.len(){
        loop{
            //check if the vector contains number 1
            if xs[i] == 1 && check == true{
                
                add.push(count1);
                break;
            }else{
                //check for the smallest positive number excluding number 0
            if xs[i] == count.try_into().unwrap(){
                count+= 1;
                //append the count
                add.push(count);
            }else{
                break;
            }
        }
    }
    }


    println!("{}", add[add.len() - 1]);
}