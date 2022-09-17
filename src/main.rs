//use std::io;

#[derive(Debug, Clone)]
    pub struct Matrix {
    vector: Vec<i32>,
    rows: u32,
    colloms: u32,

    }



fn main() {



    // optional IO input
   
/*
    println!("operator?");

    let mut action_1 = String::new();
    io::stdin().read_line(&mut action_1).ok();

    let action = action_1.trim().chars().next().expect("string is empty");


    */

    let action = '*';



// matrix storage 

    let m1 = Matrix {

        vector: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        rows: 3,
        colloms: 3,

    };


    let m2 = Matrix {

        vector: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        rows: 3,
        colloms: 3,

    };


    let mut m4 = Matrix {

        vector: vec![],
        rows: 0,
        colloms: 0,

    };

   
    

    // error check

    if m1.vector.len() == (m1.rows * m1.colloms).try_into().unwrap() {

        if m2.vector.len() == (m2.rows * m2.colloms).try_into().unwrap() {

            if m1.vector.len() == m2.vector.len() {

                if action == '+' {
                    println!("adding");
                    m4 = add_m(m1.clone(), m2.clone());

                }else if action == '-' {

                    println!("subtracting");
                    m4 = min_m(m1.clone(), m2.clone());

                }else if m1.colloms == m2.rows {
                    if action == '*' {

                    println!("multiplying");
                    m4 = multi_m(m1.clone(), m2.clone());


        
                    }else {
                    println!("invalid operator");
                    return

                    }
                }
            }
        }
    }

    println!("{}", pr_m(m4));


}

pub fn min_m(m1: Matrix, m2: Matrix) -> Matrix {

    let rows = m1.rows;
    let colloms = m1.colloms;

    let mut m3 = Matrix {
        vector: vec![],
        rows: rows,
        colloms: colloms
    };

    for x in 0..m1.vector.len() {

        m3.vector.push( m1.vector[x] - m2.vector[x])
    };


    return m3

}


pub fn add_m(m1: Matrix, m2: Matrix) -> Matrix {

    let rows = m1.rows;
    let colloms = m1.colloms;
    
    let mut m3 = Matrix {
        vector: vec![],
        rows: rows,
        colloms: colloms
    };
    
    for x in 0..m1.vector.len() {

        m3.vector.push( m1.vector[x] + m2.vector[x]);
        };

    return m3
}


pub fn multi_m(m1: Matrix, m2: Matrix) -> Matrix {


    // rows & colloms

    let rows = m1.rows;
    let colloms = m2.colloms;
    
    let mut m4 = Matrix {
        vector: vec![],
        rows: rows,
        colloms: colloms,
    };

    for _x in 0..rows * colloms {


        m4.vector.push(0)

    }


    println!("{rows} x {colloms}");

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    
    for _x in 0..m4.rows {
        for _y in 0..m4.colloms {
            for _z in 0..m1.colloms {

                m4.vector[z] += m1.vector[x] * m2.vector[y];

                x += 1;
                y += rows as usize;
            };

            y -= (rows * m1.colloms) as usize -1;
            z += 1;
            x -= m1.colloms as usize;
       
        };

        y -= rows as usize;
        x += m1.colloms as usize;
    };

       return m4
}

        


pub fn pr_m(m: Matrix) -> String {

    if m.vector.len() == 0 {
        return "invalid matrix".to_string()
    };
    let mut stdout = String::new();
    let mut pm1 = 0;

     for _x in 0..m.rows {

        stdout += &format!("( ").to_string();

        for _y in 0..m.colloms {
            stdout += &format!("{} ", m.vector[pm1]).to_string();

        pm1 += 1;

        };


        stdout += &format!(")\n").to_string();
    };


     return stdout



}
