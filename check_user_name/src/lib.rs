#[derive(PartialEq)]
pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    pub name: String,
    pub accessLevel: AccessLevel,
}

impl User {
  pub fn new(name: String, level: AccessLevel) -> Self {
    Self {
        name,
        accessLevel: level,
    }
  }
  pub fn send_name(&self) -> Option<&str> {
    if self.accessLevel == AccessLevel::Guest  {
       return None;
    }
    return Some(&self.name)
  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
   if let Some(name) = user.send_name(){
    return (true, name);
   }else{
   return (false, "ERROR: User is guest");
   }
}