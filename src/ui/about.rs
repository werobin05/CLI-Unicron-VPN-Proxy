use crate::service::{path_key::get_connection_path, wait_enter::wait_enter};

pub fn about() {
    let connection_path = get_connection_path();
    let about_us = format!(
        r#"
        [------------<Unicron CLI VPN/Proxy>------------]

        Unicron CLI is a secure VPN/Proxy client running directly in the console or terminal. 
        It provides the user with privacy and data protection without unnecessary interfaces. 
        The program does not collect any information about users, 
        allowing you to fully control your connection and remain anonymous on the network.

        Main Features:
          ✔︎ A fully console-based interface for easy PC operation.
          ✔︎ Protection of privacy and anonymity on the Internet.
          ✔︎ Lack of user data collection.
          ✔︎ Quick and easy setup.
          ✔︎ Connection list is stored in a hidden system file.
          ✔︎ All connection data is encrypted for maximum security.

        Your connection list is stored:
            {}

        Thank you for your trust😊!
        "#, connection_path.display()
    );

    println!("{about_us}");
    wait_enter();
}
