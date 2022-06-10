use std::io;
fn main() {


    // matrix template  
    


    #[derive(Debug)]
    struct Matrix {
    vector: Vec<i32>,
    rows: u32,
    colloms: u32,

    }


   //  controls
   

    let action = '*';



// matrix storage 

    let m1 = Matrix {

        vector: [0, 1, 2, 3].to_vec(),
        rows: 2,
        colloms: 2,

    };


    let m2 = Matrix {

        vector: [0, 1, 2, 3].to_vec(),
        rows: 2,
        colloms: 2,

    };

    let mut m3 = Matrix {

        vector: [0].to_vec(),
        rows: 0,
        colloms: 0,
    };

    let mut m4 = Matrix {
        vector: [0].to_vec(),
        rows: 0,
        colloms: 0,
    };

    
    


    // var init

    let mut pm1 = 0;

    let mut add = false;

    let mut min = false;

    let mut addmin = false;

    let mut multi = false;

    let mut vecm = 0;



    // error check

    if m1.vector.len() == (m1.rows * m1.colloms).try_into().unwrap() {

        if m2.vector.len() == (m2.rows * m2.colloms).try_into().unwrap() {

            if m1.vector.len() == m2.vector.len() {

                if action == '+' {
                    add = true;
                    addmin = true


                }else if action == '-' {
                    min = true;
                    addmin = true;
            }

            if m1.colloms == m2.rows {
                if action == '*' {

                multi = true;
                }
            }
        }
    }

    }



    

    println!("\n{}\n{}\n{}", add, min,  multi);
  

    // addition


    if addmin == true  {

        m3.rows = m1.rows;
        m3.colloms = m1.colloms;

        for x in 0..m1.vector.len() -1  {

            m3.vector.push(0);
 
        }; 



        if add == true {
        for x in 0..m1.vector.len() {


            m3.vector[x] = m1.vector[x] +  m2.vector[x];
        }

        }else { 
            for x in 0..m1.vector.len() {

                m3.vector[x] = m1.vector[x] - m2.vector[x];
            };

        
        }


    }else if action == '*' {


        if multi == true {



        m3.rows = m1.colloms;
        m3.colloms = m2.rows;

        m4.rows = m3.rows;
        m4.colloms = m3.colloms;

        for x in 0..m3.colloms * m3.rows -1 {

            m3.vector.push(0);
            m4.vector.push(0);

        }

//        println!("{:#?}", m3);


        // multiplication var init

       let mut a = 0;
       let mut b = 0;
       let mut c = 0;
       let mut d = 0;
       let mut e = 0;
       let rowin = m3.rows as usize;
       let colin = m3.colloms as usize;



       for z in 0..m3.rows {

        for y in 0..m3.colloms {


            for x in 0..m3.colloms {


        m4.vector[e] = m1.vector[b] * m2.vector[c];



        m3.vector[d] = m3.vector[d] + m4.vector[e];

        a = a + 1;
        b = b + 1;
        c = c + rowin;
        e = e + 1;
        
            }

            e = 0;
            
            d = d + 1;

            a = 0;

           b = b - colin; 
            c = c - (rowin * colin - 1);


            
        }

        e = 0;


        b = b + colin;
        c = c - rowin;

       }

        }

        

    }else {
        println!("invalid operation");

    }

    // print section

   

    for x in 0..m3.rows {

        print!("( ");

        for y in 0..m3.colloms {
            print!("{} ", m3.vector[pm1]);

        pm1 = pm1 + 1;

        };


        print!(")\n");
    };



}
