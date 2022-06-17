/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
	if n<0{
		return -1;
	}else if n == 0{
		return 0;
	}else{
		let mut sum = 0;
		for i in 1..=n{
			sum +=i;
		}
		return sum;
	} 
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
	let mut count = 0;
	for elem in ls.iter(){
		if elem >= &s{
			if elem <= &e{
				count+=1;
			}
		} 
	}
	return count;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
	if target == []{
		return true;
	}else{
		let mut updated = false;
		let mut idx = 0;
		for elem in target.iter(){
			idx = 0;
			updated = false; 
			while updated==false{
				if elem == &set[idx]{
					updated = true; 
				} 
			
				if updated != true{
					if idx < set.len()-1{
						idx += 1;
					}else{
						return false;
					}
				}
			}
		}
		return true;
	}
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
	if ls.len()==0{
		return None;
	}else{
		let mut mean:f64 = 0.0;
		for i in ls.iter(){
			mean += i;
		}
		let l = ls.len() as f64;
		mean /= l;
		return Some(mean);
	}

}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
	let mut num:i32 = 0;
	for i in 0..ls.len(){
		if ls[i] == 1{
			let y = (ls.len()-1-i) as u32;
			num += 2_i32.pow(y);
		}
	}
	return num;
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
	let mut factors:Vec<u32> = Vec::new();
	let mut m = n.clone();
	while m%2 == 0{
		factors.push(2);
		m /= 2;
	}
	let j = (m as f64).sqrt() as u32;
	for i in (3..j).step_by(2){
		while m%i == 0{
			factors.push(i);
			m /= i;
		}
	}
	if m>2 {
		factors.push(m);
	}
	return factors;
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
 ers/jujuspence/cmsc330spring22/project5/src/" && rustc basics.rs && "/Users/jujuspence/cmsc330spring22/project5/src/"basics
cd "/Users/jujuspence/cmsc330spring22/project5/src/" && rustc basics.rs && "/Users/jujuspence/cmsc330spring22/project5/src/"basics
  so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
	if lst.len()==0{
		return Vec::<i32>::new();
	}
	let mut l:Vec<i32> = Vec::with_capacity(lst.len());
	for i in 1..lst.len(){
		l.push(lst[i]);
	}
	l.push(lst[0]);
	return l;
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
	let mut s_idx = 0;
	
	if target.len()==0{
		return true;
	}
	if target.len()>s.len(){
		return false;
	}

	for c in s.chars(){
		if c==target.chars().nth(0).unwrap(){
			if target.len()==1{
				return true;
			}
			let mut t_char = 1;
			let mut idx = s_idx+1;
			while target.chars().nth(t_char).unwrap() == s.chars().nth(idx).unwrap(){
				if t_char == target.len()-1{
					return true;
				}else{
					t_char +=1;
					idx +=1;
				}
			} 
		}
		s_idx += 1;
	}
	return false;
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
	//let record_str:String = String::from("");
	if s == "" {
		return None;
	}
	else{
		let mut streak = 1;
		let mut start = 0;
		let mut count = 0;
		let mut idx = 0;
		//let mut record = s.chars().nth(0).unwrap().clone();
		let mut focus: char = s.chars().nth(0).unwrap();
		for c in s.chars(){
			if focus != c {
				if count > streak{
					streak = count;
					//record = focus;	
					start = idx - count;		
				}
					focus = c;	
					count = 0;	
			}
			count += 1;
			idx += 1;
		}
		let record_str = &s[start..start+streak];
		Some(&record_str)
	}
}
