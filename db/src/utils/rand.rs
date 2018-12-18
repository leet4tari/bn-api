use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_alpha_string(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}
