use base64::{engine::GeneralPurpose, Engine};

pub(crate) fn base64_encode(input: &[u8]) -> String {
    use base64::{alphabet::Alphabet, engine::GeneralPurposeConfig};
    let alphabet = "LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA";
    let alphabet = Alphabet::new(alphabet);
    let engine = GeneralPurpose::new(&alphabet.unwrap(), GeneralPurposeConfig::new()); // TODO: make const engine
    Engine::encode(&engine, input)
}
