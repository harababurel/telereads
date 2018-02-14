#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl User {
    pub fn full_name(&self) -> String {
        format!("{first_name}{last_name}",
                first_name = self.first_name,
                last_name = match self.last_name {
                    Some(ref name) => format!(" {}", name),
                    None => String::new(),
                })
    }
}
