use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Output<T> {
    pub success: bool,
    pub message: String,
    pub code: i32,
    pub data:  Option<T>,
}

impl<T> Output<T> {
    pub fn successful(data: T) -> Self {
        Output::<T> {
            success: true,
            message: "".to_owned(),
            code: 0,
            data: Some(data),
        }
    }

    pub fn failed(code: i32, message: String) -> Self {
        Output::<T> {
            success: false,
            message,
            code,
            data: None,
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct CreateEntryInput {
    pub title: String,
    pub body: String,
    pub publish: bool,
    pub tags: Vec<String>,
}


#[derive(Serialize, Deserialize)]
pub struct CreateTagInput {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilterTag {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilterEntry {
    id: Option<String>,
    title: Option<String>,
}
