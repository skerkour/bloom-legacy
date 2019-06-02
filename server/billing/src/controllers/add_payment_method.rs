use actix::{Message, Handler};
use kernel::{
    db::DbActor,
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::payement_method,
    domain::profile,
};


#[derive(Clone)]
pub struct AddPaymentMethod {
    pub account_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for AddPaymentMethod {
    type Result = Result<(), KernelError>;
}

impl Handler<AddPaymentMethod> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: AddPaymentMethod, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            billing_profile,
            billing_profile_events,
        };
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            // if profile.stripe.stripe_customer_id == null => create stripe customer with this card as default
            //    profile.UpdateStripeCustomer
            // Add payment method, as default
            // if payment methods.count != 1, set last one

            return Ok(());
        })?);
    }
}
