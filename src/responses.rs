use std::fmt;

#[derive(Debug)]
pub struct UserResponse {
   pub account: String,
   pub arn: String,
   pub user_id: String
}

#[derive(Debug)]
pub struct InstanceData {
    pub name: String,
    pub instance_id: String,
    pub state: String,
    pub private_ip: String,
}

#[derive(Debug)]
pub struct InstanceResponse {
    pub instances: Vec<InstanceData>,
}

impl InstanceResponse {
    pub fn new() -> Self {
        InstanceResponse {
            instances: Vec::new(),
        }
    }

    pub fn push(&mut self, instance: InstanceData) {
        self.instances.push(instance);
    }
}

impl FromIterator<InstanceData> for InstanceResponse {
    fn from_iter<I: IntoIterator<Item = InstanceData>>(iter: I) -> Self {
        let instances: Vec<InstanceData> = iter.into_iter().collect();
        InstanceResponse { instances }
    }

}

#[derive(Debug)]
pub struct ParameterData {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ParameterResponse {
    pub parameters: Vec<ParameterData>,
}

impl ParameterResponse {
    pub fn new() -> Self {
        ParameterResponse {
            parameters: Vec::new(),
        }
    }

    pub fn push(&mut self, parameter: ParameterData) {
        self.parameters.push(parameter);
    }
}

impl FromIterator<ParameterData> for ParameterResponse {
    fn from_iter<I: IntoIterator<Item = ParameterData>>(iter: I) -> Self {
        let parameters: Vec<ParameterData> = iter.into_iter().collect();
        ParameterResponse { parameters }
    }
}

//ANCHOR Container Registry responses

pub trait CregAllowedResponse {}

#[derive(Debug)]
pub struct CregResponse<T> where T: CregAllowedResponse {
    pub response: Vec<T>,
}

#[derive(Debug)]
pub struct CregRepoResponse {
    pub path: String
}

impl CregAllowedResponse for CregRepoResponse {}

impl fmt::Display for CregRepoResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[derive(Debug)]
pub struct CregImageResponse {
    pub tag: String,
}

impl CregAllowedResponse for CregImageResponse {}

impl fmt::Display for CregImageResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}
