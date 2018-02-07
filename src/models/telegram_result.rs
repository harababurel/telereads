#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramResult<T> {
    ok: bool,
    result: T,
}

impl<T> TelegramResult<T> {
    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn unwrap(self) -> T {
        self.result
    }

    pub fn get_ref(&self) -> &T {
        &self.result
    }
}

