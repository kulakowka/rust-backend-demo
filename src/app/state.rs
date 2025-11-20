use crate::features::user_management::domain::UserService;
use crate::features::ai_integration::domain::AIService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub ai_service: AIService,
}

impl AppState {
    pub fn new(user_service: UserService, ai_service: AIService) -> Self {
        Self {
            user_service,
            ai_service,
        }
    }
}
