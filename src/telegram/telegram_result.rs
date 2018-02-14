use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TelegramResult<T: Debug> {
    ok: bool,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    result: Option<T>,
}

impl<T> TelegramResult<T>
    where T: Debug {
    pub fn ok(&self) -> bool {
        if self.ok && !self.result.is_some() {
            error!("TelegramResult is ok but the content is missing!");
            error!("{:?}", self);
        }

        self.ok && self.result.is_some()
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    pub fn unwrap(self) -> T {
        self.result.unwrap()
    }

    pub fn get_ref(&self) -> &T {
        self.result.as_ref().unwrap()
    }
}

