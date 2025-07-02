mod config;
mod manage_bot;
mod query;

use crate::{all::*, message_reg};

command_reg![manage_bot::Restart];
message_reg![query::Query, query::Execute];
