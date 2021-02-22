use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct LinkBudget {
    pub name: String,
    pub xmit_eirp: f64,
    pub receive_gain: f64,
}
