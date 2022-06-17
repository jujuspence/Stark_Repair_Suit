#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        if let Command::Power(b,n) = self{
            if *b==true{
                let s:String = format!("Power increased by {}%",n);
                return s;
            }else{
                return format!("Power decreased by {}%",n);
            }
        }else if let Command::Missiles(b,n)=self{
            if b==&true {
                return format!("Missiles increased by {}",n);
                
            }else {
                return format!("Missiles decreased by {}",n);
            }
            
        }else if let Command::Shield(b)=self{
            if b == &true {
                return format!("Shield turned on");
            }else{
                return format!("Shield turned off");
            }
        }else if let Command::Try=self{
            return format!("Call attempt failed");
        }else{
            return format!("Not a command");
        }

    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
//	let pow_re = Regex::new("^power (inc|dec) [0-9]+$").unwrap();
//	let mis_re = Regex::new("^(fire|add) [0-9]+ missiles$").unwrap();
//	let sh_re = Regex::new("^shield (on|off)$").unwrap();
//	let try_re = Regex::new("try calling Miss Potts").unwrap();

   	let v:Vec<&str> = s.split(' ').collect(); 
     if v.len()==3{   
	    if v[0].eq("power"){
            if v[1].eq("inc"){
                if v[2].parse::<f64>().is_ok(){
                    let num:i32 = v[2].parse().unwrap();
			        Command::Power(true,num)
                }else{
                     Command::Invalid
                }
            }else if v[1].eq("dec"){
                 if v[2].parse::<f64>().is_ok(){
		            let num:i32 = v[2].parse().unwrap();
                    Command::Power(false,num)
                }else{
                    Command::Invalid
                }
            }else{
                Command::Invalid
            }
        }else if v[2].eq("missiles"){
            if v[0].eq("fire"){
                if v[1].parse::<f64>().is_ok(){
		            let num:i32 = v[1].parse().unwrap(); 
                    Command::Missiles(false,num)
                }else{
                    Command::Invalid
                }
            }else if v[0].eq("add"){
                if v[1].parse::<f64>().is_ok(){
  		            let num:i32 = v[1].parse().unwrap();
                    Command::Missiles(true,num)
                }else{
                Command::Invalid
                }
            }else{
                Command::Invalid
            }
        }else{
            Command::Invalid
        }
    }else if v.len()==2{
        if v[0].eq("shield"){
            if v[1].eq("on"){

                Command::Shield(true)
 
            }else if v[1].eq("off"){
 
                Command::Shield(false)
  
            }else{
                Command::Invalid
            }
        }else{
            Command::Invalid
        }
    }else if v.len()==4{
        if v[0].eq("try"){
            if v[1].eq("calling") && v[2].eq("Miss") && v[3].eq("Potts"){
                Command::Try
            }else{
                Command::Invalid
            }
        }else{
            Command::Invalid
        }
    }else{
		return Command::Invalid
	}
}


