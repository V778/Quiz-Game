use std::io::*;
fn main() {
    let mut i = String::new();
    let number = 1;
    let num="y";
    let mut s = String::new();
    let mut sa = String::new();
    let mut sb = String::new();
    let mut sc = String::new();
    let mut sd = String::new();
    let mut se = String::new();
    let mut h = String::new();
    let mut j = String::new();
    let mut k = String::new();
    let mut d = String::new();
    let mut a = String::new();
    let mut x = String::new();
    let mut y = String::new();
    let mut name=String::new();
    let mut age=String::new();
    let mut tru=0;
    let mut fal=0;
    let mut inv=0;
    let res:i32;
    println!("quiz");
    println!("press 1. start the game");
    stdin().read_line(&mut i).expect("Did not enter a correct string");
   
   match number {
     1 => println!("enter ur name"),
   
     //2 => println!("press 1"),
     _ => println!("wrong")
   }
     stdin().read_line(&mut name).expect("name");
     println!("enter ur age");
     stdin().read_line(&mut age).expect("age");
     println!("what is the capital of Maharashtra");
     println!("a.Mumbai  b.Delhi");

     stdin().read_line(&mut a).expect("correct option");
     if a.trim() == "a"
     {
         println!("correct answer!");
         tru+=1;
     }
    else if a.trim() == "b"
     {
         println!("wrong answer!");
         fal+=1;
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("press y for Second quiz");
     stdin().read_line(&mut s).expect("Did not enter a correct string");
     if s.trim() == "y"
     {
         println!("enter");
     }
     else
     {
         println!("invalid");
     }

     match num {
        "y" => println!("Second quiz start "),
      
        //2 => println!("press 1"),
        _ => println!("")
      }
     println!("Who is the founder of polkadot");
     println!("a.Vitalik Buterin  b.Galvin Wood");

     stdin().read_line(&mut x).expect("correct option");
     if x.trim() == "a"
     {
         println!("Wrong answer!");
         fal+=1;
     }
    else if x.trim() == "b"
     {
         println!("Correct answer!");
         tru+=1;
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("press y for Third quiz");
     stdin().read_line(&mut sa).expect("Did not enter a correct string");
     if sa.trim() == "y"
     {
         println!("enter");
     }
     else{
         println!("invalid");
     }

     match num {
        "y" => println!("Third quiz start "),
      
        //2 => println!("press 1"),
        _ => println!("")
      }
     println!("What is cryptocurrency of Etherium");
     println!("a.Ether(ETH)  b.Bitcoin");

     stdin().read_line(&mut y).expect("correct option");
     if y.trim() == "a"
     {
         println!("Correct answer!");
         tru+=1;
         
         
     }
    else if y.trim() == "b"
     {
         println!("Wrong answer!");
         fal+=1;
         
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("press y for Fourth quiz");
     stdin().read_line(&mut sb).expect("Did not enter a correct string");
     if sb.trim()=="y"
     {
         println!("enter");
     }
     else{
         println!("invalid");
     }

     match num {
        "y" => println!("Fourth quiz start "),
      
        //2 => println!("press 1"),
        _ => println!("")
      }
     println!("Who is the PM of india");
     println!("a.Rahul Gandhi  b.Narendra Modi");

     stdin().read_line(&mut d).expect("correct option");
     if d.trim() == "a"
     {
         println!("Wrong answer!");
         fal+=1;
     }
    else if d.trim() == "b"
     {
         println!("Right answer!");
         tru+=1;
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("press y for Fifth quiz");
     stdin().read_line(&mut sc).expect("Did not enter a correct string");
     if sc.trim() == "y"
     {
         println!("enter");
     }
     else{
         println!("invalid");
     }

     match num {
        "y" => println!("Fifth quiz start "),
      
        //2 => println!("press 1"),
        _ => println!("")
      }
     println!("Who is Indian Actor ");
     println!("a.TomCruise(ETH)  b.Shahrukh Khan");

     stdin().read_line(&mut h).expect("correct option");
     if h.trim() == "a"
     {
         println!("Wrong answer!");
         fal+=1;
         
         
     }
    else if h.trim() == "b"
     {
         println!("Correct answer!");
         tru+=1;
         
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("press y for Sixth quiz");
     stdin().read_line(&mut sd).expect("Did not enter a correct string");
     if sd.trim() == "y"
     {
         println!("enter");
     }
     else{
         println!("invalid");
     }

     match num {
        "y" => println!("Sixth quiz start "),
      
        //2 => println!("press 1"),
        _ => println!("")
      }
     println!("Who is a cricketer in indian team");
     println!("a.Ricky pointing  b.Sachin Tendulkar");

     stdin().read_line(&mut j).expect("correct option");
     if j.trim() == "a"
     {
         println!("Wrong answer!");
         fal+=1;
         
         
     }
    else if j.trim() == "b"
     {
         println!("Correct answer!");
         tru+=1;
         
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("press y for Seventh quiz");
     stdin().read_line(&mut se).expect("Did not enter a correct string");
     if se.trim() == "y"
     {
         println!("enter");
     }
     else{
         println!("invalid");
     }

     match num {
        "y" => println!("Seventh quiz start "),
      
        //2 => println!("press 1"),
        _ => println!("")
      }
     println!("What is national game of india");
     println!("a.cricket  b.Hockey");

     stdin().read_line(&mut k).expect("correct option");
     if k.trim() == "a"
     {
         println!("Wrong answer!");
         fal+=1;
         
         
     }
    else if k.trim() == "b"
     {
         println!("Correct answer!");
         tru+=1;
         
     }
     else {
         println!("Invalid input");
         inv+=1;
     }
     println!("You have solved correct answer are {}",tru);
     println!("You have wrong answers are {}",fal);
     println!("You have invalid answers are {}",inv);
     res=7-fal-inv;
     println!("Your score is {} {}",res,"out of 7");
     if(res>5)
     {
         println!("Congratulation! ..You are qualified!!!!");
     }
     else {
         println!("Sorry! you are not qualified");
     }
}
