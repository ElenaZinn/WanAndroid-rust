use crate::repository::{AuthRepository, AuthenticatedUser, RepositoryError};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthState {
    pub user: Option<AuthenticatedUser>,
    pub is_loading: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthAction {
    Restore,
    Login { username: String, password: String },
    Logout,
    LoginFinished(Result<AuthenticatedUser, RepositoryError>),
    LogoutFinished(Result<(), RepositoryError>),
    SessionRestored(Option<AuthenticatedUser>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthEffect {
    RestoreSession,
    Login { username: String, password: String },
    Logout,
}

pub struct AuthInteractor<R> {
    repository: R,
    pub state: AuthState,
}

impl<R: AuthRepository> AuthInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            state: AuthState::default(),
        }
    }

    pub fn dispatch(&mut self, action: AuthAction) -> Vec<AuthEffect> {
        match action {
            AuthAction::Restore => {
                self.state.is_loading = true;
                self.state.error_message = None;
                vec![AuthEffect::RestoreSession]
            }
            AuthAction::Login { username, password } => {
                self.state.is_loading = true;
                self.state.error_message = None;
                vec![AuthEffect::Login { username, password }]
            }
            AuthAction::Logout => {
                self.state.is_loading = true;
                self.state.error_message = None;
                vec![AuthEffect::Logout]
            }
            AuthAction::LoginFinished(result) => {
                self.state.is_loading = false;
                match result {
                    Ok(user) => {
                        self.state.user = Some(user);
                        self.state.error_message = None;
                    }
                    Err(error) => self.state.error_message = Some(error.to_string()),
                }
                vec![]
            }
            AuthAction::LogoutFinished(result) => {
                self.state.is_loading = false;
                match result {
                    Ok(()) => {
                        self.state.user = None;
                        self.state.error_message = None;
                    }
                    Err(error) => self.state.error_message = Some(error.to_string()),
                }
                vec![]
            }
            AuthAction::SessionRestored(user) => {
                self.state.is_loading = false;
                self.state.user = user;
                vec![]
            }
        }
    }

    pub fn run(&self, effect: AuthEffect) -> AuthAction {
        match effect {
            AuthEffect::RestoreSession => {
                AuthAction::SessionRestored(self.repository.restore_session())
            }
            AuthEffect::Login { username, password } => {
                AuthAction::LoginFinished(self.repository.login(&username, &password))
            }
            AuthEffect::Logout => AuthAction::LogoutFinished(self.repository.logout()),
        }
    }
}
