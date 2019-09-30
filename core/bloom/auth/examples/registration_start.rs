fn main() {
    let display_name = "some display name";
    let email = "some.email@protonmail.com";
    let password = "some password";

    let res = bloom_auth::registration_start(display_name, email, password);
    println!("res = {:?}", res);
}
