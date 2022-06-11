# Komio_Auto_Did_Macro
## How to use

    use Komio_Macro;
    use Komio_Macro::{Did_Gen,Did_Gen_End,Did_Gen_Start};
    
    Did_Gen_Start!();
    #[Komio_Macro::Did_Gen(query)]
    pub fn demo_function(    a:u8 , b:u8 , c :Vec<u8> )->u8
    {
        let mut a :[u8;3]=[0;3];
        return 0;
    }

    #[Komio_Macro::Did_Gen(query)]
    pub fn demo_function2(    a:u8 , b:u8 , c :Vec<u8> )->String
    {

        let mut a :[u8;3]=[0;3];
        return String::from("");
    }

    Did_Gen_End!("./komio_demo.did");

## Gen Result

    service : {
    "demo_function":(Nat,Nat,[Nat])->(Nat) query;
    "demo_function2":(Nat,Nat,[Nat])->(Text) query;
    }
