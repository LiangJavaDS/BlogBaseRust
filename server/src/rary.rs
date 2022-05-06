pub fn public_function() {
    println!("public_fun!!!!!!!!!!!");
}

fn private_function() {
    println!("private_function")
}

pub fn indirect_access() {
    println!("indirect_access");
    private_function();
}
