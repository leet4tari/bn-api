use bigneon_db::prelude::*;
use config::Config;
use db::Connection;
use domain_events::errors::DomainActionError;
use domain_events::executor_future::ExecutorFuture;
use domain_events::send_communication::SendCommunicationExecutor;
use std::borrow::Borrow;
use std::collections::HashMap;

pub trait DomainActionExecutor {
    fn execute(&self, action: DomainAction, conn: Connection) -> ExecutorFuture;
}

pub struct DomainActionRouter {
    routes: HashMap<DomainActionTypes, Box<DomainActionExecutor>>,
}
impl DomainActionRouter {
    pub fn new() -> DomainActionRouter {
        DomainActionRouter {
            routes: HashMap::new(),
        }
    }

    pub fn add_executor(
        &mut self,
        action_type: DomainActionTypes,
        executor: Box<DomainActionExecutor>,
    ) -> Result<(), DomainActionError> {
        match self.routes.insert(action_type, executor) {
            Some(_) => Err(DomainActionError::Simple(
                "Action type already has an executor".to_string(),
            )),
            _ => Ok(()),
        }
    }

    pub fn get_executor_for(
        &self,
        action_type: DomainActionTypes,
    ) -> Option<&dyn DomainActionExecutor> {
        self.routes.get(&action_type).map(|o| (*o).borrow())
    }

    pub fn set_up_executors(&mut self, conf: &Config) {
        // This method is not necessary, but creates a compile time error
        // by using the `match` to identify DomainActionTypes that have not been catered for.
        // If you disagree with this approach or find a better way, feel free to unroll it.
        let find_executor = |action_type| match action_type {
            DomainActionTypes::Communication => {
                Box::new(SendCommunicationExecutor::new(conf.clone()))
            } // DO NOT add
              // _ => ()
        };

        self.add_executor(
            DomainActionTypes::Communication,
            find_executor(DomainActionTypes::Communication),
        ).expect("Configuration error");
    }
}