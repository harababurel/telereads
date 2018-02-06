#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramResult<T> {
    ok: bool,
    result: T,
}

impl<T> TelegramResult<T> {
    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn get(self) -> T {
        self.result
    }
}

