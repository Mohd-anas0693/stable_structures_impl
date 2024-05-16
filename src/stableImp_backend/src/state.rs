use crate::types::PostData;
use crate::Memory;
use candid::Principal;
use ic_stable_structures::StableBTreeMap;

pub struct State {
    pub postdata: StableBTreeMap<Principal, PostData, Memory>,
    // Generates aliases for file requests.
}
impl State {
    pub fn new() -> Self {
        Self {
            postdata: init_file_contents(),
        }
    }
    pub fn set_post_data(
        &mut self,
        user_principal: Principal,
        data: PostData,
    ) -> Result<(), String> {
        let response = self.postdata.insert(user_principal, data);
        if let Some(_res) = response {
            Err("Not Able to upload data".to_string())
        } else {
            Ok(())
        }
    }
    pub fn get_post_data(&self, user_principal: Principal) -> Result<PostData, String> {
        let post_data = self.postdata.get(&user_principal);
        if let Some(post) = post_data {
            Ok(post)
        } else {
            Err("No Post Exist!".to_string())
        }
    }

    pub fn remove_user_post(&mut self, user_principal: Principal) -> Result<PostData, String> {
        let remove_success = self.postdata.remove(&user_principal);
        if let Some(remove) = remove_success {
            Ok(remove)
        } else {
            Err("no User found!".to_string())
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

fn init_file_contents() -> StableBTreeMap<Principal, PostData, Memory> {
    StableBTreeMap::init(crate::memory::get_postdata_memory())
}
#[cfg(test)]
mod tests {
    use super::*;

    fn get_principal() -> Principal {
        Principal::from_text("bxquz-fu76r-igixs-bw537-mgkyg-h4goq-ldrwe-q4zg2-zxtqy-zupgm-nqe")
            .unwrap()
    }
    fn generate_user_data() -> PostData {
        let user_data = PostData {
            title: "No".to_string(),
            data: "New".to_string(),
            created_by: "32334".to_string(),
        };
        return user_data;
    }
    #[test]
    fn test_post_creation() {
        let mut state = State::default();
        let data = generate_user_data();
        let response = state.set_post_data(get_principal(), data.clone());
        match response {
            Ok(res) => assert_eq!(res, ()),
            Err(e) => assert_eq!(e, "Already uploaed the post of the User".to_string()),
        }
    }
    #[test]
    fn test_post_exist() {
        let state = State::default();
        let data = generate_user_data();
        let user_data = state.get_post_data(get_principal());
        match user_data {
            Ok(res) => assert_eq!(res, data),
            Err(e) => assert_eq!(e, "No data found".to_string()),
        };
    }
    #[test]
    fn test_post_exist_after_remove() {
        let mut state = State::default();
        let data = generate_user_data();
        let user_data = state.remove_user_post(get_principal());
        match user_data {
            Ok(res) => assert_eq!(res, data),
            Err(e) => assert_eq!(e, "no User found!".to_string()),
        };
    }
}
