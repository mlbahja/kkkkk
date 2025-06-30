
#[derive(Debug, PartialEq, Eq)]

pub struct OfficeWorker {
 pub   name: String,
 pub   age: u32,
  pub  role: WorkerRole,
}
#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole{
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    /*fn from(s: &str) -> Self {
        match s {
            "Louise,25,admin" => OfficeWorker {
                name: "Louise".to_string(),
                age: 25,
                role: WorkerRole::Admin,
            },
            "Rob,11,guest" => OfficeWorker {
                name: "Rob".to_string(),
                age: 11,
                role: WorkerRole::Guest,
            },
            "Maria Agata,44,user" => OfficeWorker {
                name: "Maria Agata".to_string(),
                age: 44,
                role: WorkerRole::User,
            },
            _ => panic!("Hardcoded input not recognized"),
        }
    }*/
    fn from(s: &str) -> Self{
        let info = info.split(",").collect()::<Vec<_>>()
    }

}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> Self {
        match s {
            "guest" => WorkerRole::Guest,
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            _ => panic!("Unknown role"),
        }
    }
}