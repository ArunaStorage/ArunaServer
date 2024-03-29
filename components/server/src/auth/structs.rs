use crate::database::{
    dsls::user_dsl::User,
    enums::{DbPermissionLevel, ObjectMapping},
};
use anyhow::{bail, Result};
use diesel_ulid::DieselUlid;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ContextVariant {
    NotActivated,
    Resource((DieselUlid, DbPermissionLevel)),
    User((DieselUlid, DbPermissionLevel)),
    SelfUser, // True: Registered, False: Unregistered
    Registered,
    GlobalProxy,
    GlobalAdmin,
}
#[derive(Debug, Serialize, Clone)]
pub struct Context {
    pub variant: ContextVariant,
    pub allow_service_account: bool,
    pub is_self: bool,
}

impl Context {
    pub fn res_ctx(id: DieselUlid, level: DbPermissionLevel, allow_sa: bool) -> Self {
        Self {
            variant: ContextVariant::Resource((id, level)),
            allow_service_account: allow_sa,
            is_self: false,
        }
    }

    pub fn user_ctx(id: DieselUlid, level: DbPermissionLevel) -> Self {
        Self {
            variant: ContextVariant::User((id, level)),
            allow_service_account: false,
            is_self: false,
        }
    }

    pub fn admin() -> Self {
        Self {
            variant: ContextVariant::GlobalAdmin,
            allow_service_account: false,
            is_self: false,
        }
    }

    pub fn self_ctx() -> Self {
        Self {
            variant: ContextVariant::SelfUser,
            allow_service_account: false,
            is_self: true,
        }
    }

    pub fn proxy() -> Self {
        Self {
            variant: ContextVariant::GlobalProxy,
            allow_service_account: false,
            is_self: false,
        }
    }

    pub fn registered() -> Self {
        Self {
            variant: ContextVariant::Registered,
            allow_service_account: true,
            is_self: false,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            variant: ContextVariant::SelfUser,
            allow_service_account: true,
            is_self: false,
        }
    }
}

impl User {
    pub fn get_permissions(
        &self,
        token: Option<DieselUlid>,
    ) -> Result<(Vec<(DieselUlid, DbPermissionLevel)>, bool)> {
        if let Some(token) = token {
            if let Some(token) = self.attributes.0.tokens.get(&token) {
                // Check if token is mapped to an object
                let object_id = if let Some(mapping) = token.object_id {
                    match mapping {
                        ObjectMapping::PROJECT(id)
                        | ObjectMapping::COLLECTION(id)
                        | ObjectMapping::DATASET(id)
                        | ObjectMapping::OBJECT(id) => id,
                    }
                } else {
                    let user_perms = &self.attributes.0.permissions;

                    return Ok((
                        user_perms
                            .iter()
                            .map(|entry| (*entry.key(), entry.value().into_inner()))
                            .collect_vec(),
                        true,
                    ));
                };

                Ok((vec![(object_id, token.user_rights)], false))
            } else {
                bail!("Token not found")
            }
        } else {
            Ok((
                self.attributes
                    .0
                    .permissions
                    .iter()
                    .map(|item| {
                        (
                            *item.key(),
                            match *item.value() {
                                ObjectMapping::PROJECT(perm)
                                | ObjectMapping::COLLECTION(perm)
                                | ObjectMapping::DATASET(perm)
                                | ObjectMapping::OBJECT(perm) => perm,
                            },
                        )
                    })
                    .collect(),
                true,
            ))
        }
    }
}
