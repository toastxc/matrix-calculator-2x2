use std::io;
fn main() {
println!(r"

 __  __       _        _         _____      _            _       _                _____       _____  
|  \/  |     | |      (_)       / ____|    | |          | |     | |              / /__ \     |__ \ \ 
| \  / | __ _| |_ _ __ ___  __ | |     __ _| | ___ _   _| | __ _| |_ ___  _ __  | |   ) |_  __  ) | |
| |\/| |/ _` | __| '__| \ \/ / | |    / _` | |/ __| | | | |/ _` | __/ _ \| '__| | |  / /\ \/ / / /| |
| |  | | (_| | |_| |  | |>  <  | |___| (_| | | (__| |_| | | (_| | || (_) | |    | | / /_ >  < / /_| |
|_|  |_|\__,_|\__|_|  |_/_/\_\  \_____\__,_|_|\___|\__,_|_|\__,_|\__\___/|_|    | ||____/_/\_\____| |
                                                                                 \_\             /_/ 
                                                                                                     
");


                                                println!("operation: ");

    struct Config {
        rowcol: String,
        action: String,


    }

    let mut success = true;

    let mut action_input = String::new();
    io::stdin().read_line(&mut action_input)
        .ok()
        .expect("could not create string");
    let action_input = action_input.trim();


    let test = Config {
        rowcol: "2x2".to_string(),
        action: action_input.to_string(),
    };

    println!("first matrix:");

    let mut matrix1 = String::new();
    io::stdin().read_line(&mut matrix1)
        .ok()
        .expect("could not create var");

    let matrix1 = matrix1.trim();

    let matrix1: Vec<u32> = matrix1.split(" ")
    .map(|x| x.parse().expect("Not an integer!"))
    .collect();

    println!("second matrix:");
    let mut matrix2 = String::new();
    io::stdin().read_line(&mut matrix2)
        .ok()
        .expect("could not create var");

    let matrix2 = matrix2.trim();

    let matrix2: Vec<u32> = matrix2.split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();


    if test.rowcol == "2x2".to_string() { 
        let matrix1: [u32; 4] = [matrix1[0], matrix1[1], matrix1[2], matrix1[3]];
        let matrix2: [u32; 4] = [matrix2[0], matrix2[1], matrix2[2], matrix2[3]];
        let mut matrix3: [u32; 4] = [0, 0, 0, 0];

        if test.action == "*" {
        matrix3[0] = matrix1[0] * matrix2[0] + matrix1[1] * matrix2[2];
        matrix3[1] = matrix1[0] * matrix2[1] + matrix1[1] * matrix2[3];
        matrix3[2] = matrix1[2] * matrix2[0] + matrix1[3] * matrix2[2];
        matrix3[3] = matrix1[2] * matrix2[1] + matrix1[3] * matrix2[3];
        } 
        
     else if test.action == "/" {
        println!("cannot divide matrices");
        success = false;

    } else if test.action == "+"{
        for x in 0..4 {
            matrix3[x] = matrix1[x] + matrix2[x];
        }

    }else if test.action == "-"{
        for x in 0..4 {
            matrix3[x] = matrix1[x] - matrix2[x];
        }
    
    }
     
     if success == false {
         println!(" ")
     }else {
     println!("matrix3 = \n({}, {})\n({}, {})", matrix3[0], matrix3[1], matrix3[2], matrix3[3]);
    }
    }
}
