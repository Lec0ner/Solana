fn main() {
    let mut num = 0;
    let mut num2: i8 = 20;
    let mut num2: f32 = 30.20;
    let mut num = "Ebal";
    let mut name = String::from("Ebal");


    println!("{}",num.len());
    println!("{}",name.len());
       while num2 > 10
       {
           if num > 3
           {
               println!("Мы хуесосы");
           }else
           {
               println!("Мы не хуесосы");
           }
           num += 1;
           num2 -= 1;
       }
       let mut counter = 0;
       loop
       {
           println!("Мы хуесосы");
           counter += 1;
           if counter == 15{
               break;
           }

       }
       println!("Ура");
       let mut i = 60;
       for i in 1..6{
           let mut i = 60;
           print!("{}\n", i);
       }
       println!("{}", i);
    let mut i = 1;
    let mut j = 1;
    while i < 10{
        while j < 10{
            print!("{}\t", i * j);
            j += 1;
        }
        println!();
        i += 1;
        j = 1;
    }

    let mut rut2= 10;
    let mut rut1 = match rut2 {
        1..=10 => 1,
        20 => 2,
        _ => 3
    };
    println!("{}", rut1);

    let mut boo: bool = true;

    let mut string: String = String::new();

    match boo {
        true =>{
            string = String::from("Пидорас");
        }
        false =>{
            string = String::from("Пидорас1");
        }
    }
    println!("{}",string);


    let mut just = 10.21;
    let mut doit = if just == 10 { 4 } else { 5 };
    println!("{}", doit);
    println!("{:p}", &just);
    let mut just = 20;
    println!("{:p}", &just);


}

