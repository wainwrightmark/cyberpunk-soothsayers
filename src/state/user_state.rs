use std::rc::Rc;
use yewdux::store::Reducer;
use yewdux::store::Store;

use crate::web::js::get_referrer;
use crate::web::js::get_user_agent;

use super::logging::EventLog;
use super::logging::LoggableEvent;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserState {
    pub user_id: Option<uuid::Uuid>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct CreateUserIfNewMessage {
    pub ref_param: String,
}

impl Reducer<UserState> for CreateUserIfNewMessage {
    fn apply(self, state: Rc<UserState>) -> Rc<UserState> {
        if state.user_id.is_some() {
            state
        } else {
            let user_id = uuid::Uuid::new_v4();
            let user_agent = get_user_agent();
            let referrer = get_referrer();

            let message = EventLog {
                user_id,

                event: LoggableEvent::NewUser {
                    user_agent,
                    ref_param: self.ref_param,
                    referrer,
                },
                resent: false,
            };
            message.send_log();

            Rc::new(UserState {
                user_id: Some(user_id),
            })
        }
    }
}
