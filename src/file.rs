use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct File {
    name: String,
    extention: Type,
    path: String,
}

impl File {
    pub fn new(name: String, extention: String, path: String) -> Self {
        let extention = match extention.as_str() {
            "txt" => Type::TXT,
            "md" => Type::MD,
            "jpg" => Type::JPG,
            _ => panic!("Can't create file with extention: {extention}"),
        };
        Self {name, extention, path}
    }
}

#[derive(Debug)]
pub enum Type {
    TXT,
    MD,
    JPG,
} 

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Self::TXT => ".txt",
            Self::MD => ".md",
            Self::JPG => ".jpg", 
        }.to_string()
    }
}