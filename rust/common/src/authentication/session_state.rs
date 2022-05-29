use std::ops::Deref;

use actix_session::Session;
use send_wrapper::SendWrapper;
use uuid::Uuid;
use wither::bson::oid::ObjectId;
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
        &*self.0.as_deref().unwrap()
    }
}

type SessionShared = Shared<actix_session::Session>;

#[derive(Debug, thiserror::Error)]
pub enum TypedSessionError {
    #[error("Failed to parse data")]
    ParsingFailure(#[from] serde_json::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

type TypedSessionResult<T> = Result<T, TypedSessionError>;

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

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_object_id(&self, user_id: &ObjectId) -> TypedSessionResult<()> {
        Ok(self.0.insert(Self::USER_ID_KEY, user_id)?)
    }

    pub fn get_user_object_id(&self) -> TypedSessionResult<Option<ObjectId>> {
        let user_id = self.0.get::<ObjectId>(Self::USER_ID_KEY)?;
        Ok(user_id)
    }

    pub fn insert_user_uuid(&self, user_id: Uuid) -> TypedSessionResult<()> {
        self.0.insert(Self::USER_ID_KEY, user_id)?;
        Ok(())
    }

    pub fn get_user_uuid(&self) -> TypedSessionResult<Option<Uuid>> {
        Ok(self.0.get::<Uuid>(Self::USER_ID_KEY)?)
    }

    pub fn clear(&self) {
        self.0.clear()
    }
}
