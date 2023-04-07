





pub mod MUDStrings{
    pub const Beginning:&str="Welcome to the world of rust!";
    pub const WouldYouLikeToContinue:&str="Would you like to continue? [y/n]";
    pub const AskForName:&str="What is your name?";
    pub const AreYouForSure:&str="Are you sure your name is ";
    pub const YesOrNo:&str="[y/n]";
    pub const Welcome:&str="Welcome to the world of rust, ";
}

mod Rooms{

    use super::*;
    pub mod TextInputRooms{
        use super::*;
        pub fn Welcome(UserName:String){

            println!("{} {}!",MUDStrings::Welcome,UserName);
        }
        pub fn AskForName() ->String{
            let mut UserName:String;
            loop{
                println!("{}",MUDStrings::AskForName);
                UserName=GeneralFunctions::RequireInput();
                println!("{}{} {}?",MUDStrings::AreYouForSure,UserName,MUDStrings::YesOrNo);
                if GeneralFunctions::RequireSingleCharInput().to_lowercase().next().unwrap()=='y'{
                    break;
                }
    
            }
            return UserName;
        

        }
        pub fn Beginning() ->bool{

            println!("{}",MUDStrings::Beginning);
            println!("{}",MUDStrings::WouldYouLikeToContinue);
            loop{
                let StartOrNot:char=GeneralFunctions::RequireSingleCharInput().to_lowercase().next().unwrap();
                if StartOrNot=='y'{
                    break;

                }
                else if StartOrNot=='n'{
                    return false;

                }
                else{
        
                    println!("Invalid input, try again!");
                    return false;
                }
            }
            return true;
        }

        
    }

}
mod GeneralFunctions{
    pub fn RequireSingleCharInput() -> char{
        let mut input:String=("").to_string(); 
        std::io::stdin().read_line(&mut input).unwrap();   
        return input.chars().next().unwrap(); 

    }
    pub fn RequireInput() -> String{
        let mut input:String=("").to_string(); 
        std::io::stdin().read_line(&mut input).unwrap();
        if input.chars().last().unwrap()=='\n'{
            input.pop().unwrap();
        }
        return input;
    }
}

fn main(){

    if Rooms::TextInputRooms::Beginning() == false{
        return;
    }
    let UserName=Rooms::TextInputRooms::AskForName();
    Rooms::TextInputRooms::Welcome(UserName);
}



