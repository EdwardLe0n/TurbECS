use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]

pub enum ActiveStates {
    Active,
    Inactive,
    Destroyed,
    NtbAwake,
    NtbStart
}

impl ActiveStates {
    
    pub fn get_string(&self) -> String{
        match self {
            ActiveStates::Active => {return "Active".to_owned();},
            ActiveStates::Inactive => {return "Inactive".to_owned();},
            ActiveStates::Destroyed => {return "Destroyed".to_owned();},
            ActiveStates::NtbAwake => {return "NTB Awake".to_owned();},
            ActiveStates::NtbStart => {return "NTB Start".to_owned();},
        }
    }

}