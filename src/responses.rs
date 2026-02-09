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

