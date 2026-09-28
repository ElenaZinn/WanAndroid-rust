use wanandroid_core::{
    AuthAction, AuthEffect, AuthInteractor, AuthRepository, AuthenticatedUser, RepositoryError,
};

struct FakeAuth {
    user: Option<AuthenticatedUser>,
    login_result: Result<AuthenticatedUser, RepositoryError>,
    logout_result: Result<(), RepositoryError>,
}

impl AuthRepository for FakeAuth {
    fn login(
        &self,
        _username: &str,
        _password: &str,
    ) -> Result<AuthenticatedUser, RepositoryError> {
        self.login_result.clone()
    }

    fn logout(&self) -> Result<(), RepositoryError> {
        self.logout_result.clone()
    }

    fn restore_session(&self) -> Option<AuthenticatedUser> {
        self.user.clone()
    }
}

fn user() -> AuthenticatedUser {
    AuthenticatedUser {
        id: 7,
        username: "Elena".into(),
    }
}

#[test]
fn login_is_an_effect_and_success_updates_authenticated_state() {
    let mut interactor = AuthInteractor::new(FakeAuth {
        user: None,
        login_result: Ok(user()),
        logout_result: Ok(()),
    });

    let effects = interactor.dispatch(AuthAction::Login {
        username: "Elena".into(),
        password: "secret".into(),
    });
    assert_eq!(
        effects,
        vec![AuthEffect::Login {
            username: "Elena".into(),
            password: "secret".into(),
        }]
    );
    assert!(interactor.state.is_loading);

    let action = interactor.run(effects[0].clone());
    interactor.dispatch(action);

    assert_eq!(interactor.state.user, Some(user()));
    assert!(!interactor.state.is_loading);
    assert!(interactor.state.error_message.is_none());
}

#[test]
fn restore_reads_existing_session_and_logout_clears_state() {
    let mut interactor = AuthInteractor::new(FakeAuth {
        user: Some(user()),
        login_result: Ok(user()),
        logout_result: Ok(()),
    });

    let restore = interactor.dispatch(AuthAction::Restore);
    assert_eq!(restore, vec![AuthEffect::RestoreSession]);
    let restored = interactor.run(restore[0].clone());
    interactor.dispatch(restored);
    assert_eq!(interactor.state.user, Some(user()));

    let logout = interactor.dispatch(AuthAction::Logout);
    assert_eq!(logout, vec![AuthEffect::Logout]);
    let logged_out = interactor.run(logout[0].clone());
    interactor.dispatch(logged_out);
    assert!(interactor.state.user.is_none());
}

#[test]
fn failed_login_is_renderable_error_state() {
    let mut interactor = AuthInteractor::new(FakeAuth {
        user: None,
        login_result: Err(RepositoryError::Api {
            code: -1,
            message: "invalid credentials".into(),
        }),
        logout_result: Ok(()),
    });

    let effects = interactor.dispatch(AuthAction::Login {
        username: "Elena".into(),
        password: "wrong".into(),
    });
    let finished = interactor.run(effects[0].clone());
    interactor.dispatch(finished);

    assert_eq!(
        interactor.state.error_message.as_deref(),
        Some("api error -1: invalid credentials")
    );
    assert!(!interactor.state.is_loading);
}
