use std::ops::Deref;

use actix_session::Session;
use actix_web::Error;
use send_wrapper::SendWrapper;

#[derive(Clone, Debug)]
struct Shared<T>(pub Option<SendWrapper<T>>);

impl<T> Shared<T> {
    pub fn new(v: T) -> Self {
        Self(Some(SendWrapper::new(v)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0.as_deref().clone().unwrap()
    }
}

type SessionShared = Shared<actix_session::Session>;

/*
TODO:
This is somewhat like a hack: https://github.com/async-graphql/async-graphql/issues/426
Session is  Session(Rc<RefCell<SessionInner>>) and probably okay to use
SendWrapper for now but having it implement Send would allow
using Arc/Mutex
*/
pub struct TypedSession(SessionShared);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn new(session: Session) -> Self {
        Self(Shared::new(session))
    }

    pub fn renew(&self) -> anyhow::Result<()> {
        self.0.renew();
        Ok(())
    }

    pub fn insert_user_id(&self, user_id: impl Into<String>) -> Result<(), Error> {
        self.0.insert(Self::USER_ID_KEY, user_id.into())
    }

    pub fn get_user_id(&self) -> Result<Option<String>, Error> {
        self.0.get::<String>(Self::USER_ID_KEY)
    }
}
