use hmac::{Hmac, Mac};
use icloud_auth::{anisette::AnisetteData, AppleAccount};
use omnisette::AnisetteProvider;
use sha2::{Digest, Sha256};

pub struct XcodeSession {
    pub dsid: String,
    pub auth_token: String,
    pub anisette: AnisetteData,
}

impl XcodeSession {
    pub fn with<T: AnisetteProvider>(account: &AppleAccount<T>) -> XcodeSession {
        let _ = account;
        // let spd = account.spd.as_ref().unwrap();
        // // println!("spd: {:#?}", spd);
        // let dsid = spd.get("adsid").unwrap().as_string().unwrap();
        // let auth_token = spd.get("GsIdmsToken").unwrap().as_string().unwrap();

        // let sk = spd.get("sk").unwrap().as_data().unwrap();
        // let c = spd.get("c").unwrap().as_data().unwrap();
        // println!("adsid: {}", dsid);
        // println!("GsIdmsToken: {}", auth_token);
        // // println!("spd: {:#?}", spd);
        // println!("sk: {:#?}", sk);
        // println!("c: {:#?}", c);

        //TODO: use apptoken func from account
        todo!()
    }
}
