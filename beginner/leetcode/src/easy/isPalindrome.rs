pub fn is_palindrome(s: String) -> bool {
       let lower :String = s.to_lowercase().chars().filter(|c|c.is_alphanumeric()).collect();
       if lower.is_empty(){
              return true;
       }
       let(mut l , mut r)  = (0 , lower.len()-1);
       while l<r {
              if lower.chars().nth(l) != lower.chars().nth(r){
                     return false;
              }
              l+=1;
              r-=1;
       }
       true
}
