use serde::{Deserialize, Serialize};

use crate::models::company::Company;
use crate::models::job::Job;
use crate::models::map_resume_job::MapResumeJob;
use crate::models::resume::Resume;
use crate::models::role::Role;
use crate::models::user::UserInfo;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Data {
    UserInfo(UserInfo),
    ListUserInfo(Vec<UserInfo>),

    Company(Company),
    Role(Role),
    Job(Job),
    Resume(Resume),
    MapJobResume(MapResumeJob),

    ListCompany(Vec<Company>),
    ListRole(Vec<Role>),
    ListJob(Vec<Job>),
    ListResume(Vec<Resume>),
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PayloadWithData {
    pub message: String,
    pub data: Data,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PayloadNoData {
    pub message: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PayloadForLogin {
    pub message: String,
    pub access_token: String,
    pub data: Data,
}
