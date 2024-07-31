use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct UserData {
    nickname: String,
    avater_url: Option<String>,
}

impl UserData {
    pub fn new(nickname: String) -> Self {
        Self {
            nickname,
            avater_url: None,
        }
    }
}
