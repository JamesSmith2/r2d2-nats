pub extern crate nats;
pub extern crate r2d2;

use std::io::Error;

#[derive(Debug)]
pub struct NatsConnectionManager {
    params: String,
    nkey: String,
    seed: String,
}

impl NatsConnectionManager {
    pub fn new(
        connection_string: String,
        nkey: String,
        seed: String,
    ) -> Result<NatsConnectionManager, Error> {
        Ok(NatsConnectionManager {
            params: connection_string,
            nkey,
            seed,
        })
    }
}

impl r2d2::ManageConnection for NatsConnectionManager {
    type Connection = nats::Connection;
    type Error = Error;

    fn connect(&self) -> Result<nats::Connection, Error> {
        let kp = nkeys::KeyPair::from_seed(&self.seed).unwrap();

        nats::Options::with_nkey(&self.nkey, move |nonce| kp.sign(nonce).unwrap())
            .connect(&self.params.to_owned())
        //nats::Options::with_credentials(&self.path.to_owned()).connect(&self.params.to_owned())

        //nats::connect(&self.params.to_owned())
        /*match Client::new(self.params.to_owned()) {
            Ok(client) => Ok(client),
            Err(err) => Err(Error::Other(err)),
        }*/
    }

    fn is_valid(&self, conn: &mut nats::Connection) -> Result<(), Error> {
        match conn.publish("r2d2_nats", "PING".as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn has_broken(&self, _conn: &mut nats::Connection) -> bool {
        false
    }
}
