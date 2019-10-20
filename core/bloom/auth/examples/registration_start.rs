fn main() {
    let input = bloom_messages::auth::RegistrationStart {
        display_name: "some display name".to_string(),
        email: "some.email@protonmail.com".to_string(),
    };

    let res = bloom_auth::registration_start(input);
    println!("res = {:?}", res);
}
