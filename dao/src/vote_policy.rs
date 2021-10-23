use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::proposal::ProposalKindIdent;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum VVoteConfig {
    //Prev(VoteConfigOld)
    Curr(VoteConfig)
}

impl VVoteConfig {
    pub fn migrate(self) -> Self {
        //TODO: implement on migration
        self
    }
}

impl From<VVoteConfig> for VoteConfig {
    fn from(config: VVoteConfig) -> Self {
        match config {
            VVoteConfig::Curr(c) => c,
            _ => unimplemented!(),
        }
    }
}


#[derive(Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq, Serialize))]
#[serde(crate = "near_sdk::serde")]
/// User provided struct
pub struct VoteConfigInput {
    pub proposal_kind: ProposalKindIdent,
    pub duration: u64,
    pub waiting_open_duration: u64,
    pub quorum: u8,
    pub approve_threshold: u8,
    pub vote_only_once: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
/// For DAO settings or Proposal subject
pub struct VoteConfig {
    pub duration: u64,
    pub quorum: u8,
    pub vote_only_once: bool,
    pub waiting_open_duration: u64,
    pub approve_threshold: u8,
}

impl TryFrom<VoteConfigInput> for VoteConfig {
    type Error = &'static str;

    fn try_from(input: VoteConfigInput) -> Result<Self, Self::Error> {
        if input.quorum > 100 || input.approve_threshold > 100 {
            return Err("Quorum/Approve out of bounds");
        }

        Ok(VoteConfig {
            duration: input.duration,
            quorum: input.quorum,
            vote_only_once: input.vote_only_once,
            waiting_open_duration: input.waiting_open_duration,
            approve_threshold: input.approve_threshold,
        })
    }
}
