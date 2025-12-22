pub enum Choice {
    Exit,
    GetEmployees,
    AddEmployees,
    InValidInput
}

impl Choice {
    pub fn new(value: u8) -> Choice {
        match value {
            0 => Choice::Exit,
            1 => Choice::AddEmployees,
            2 => Choice::GetEmployees,
            _ => Choice::InValidInput
        }
    }
}