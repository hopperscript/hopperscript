#[allow(dead_code)]

pub struct Fncall {
    pub fnname: String,
}

#[derive(Debug)]
pub struct Project {
    pub variables: Vec<Variable>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub typ: i32,
    pub object_id_string: String,
}
