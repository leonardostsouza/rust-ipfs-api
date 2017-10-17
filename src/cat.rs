use IpfsApi;

use std::io::Read;

use reqwest;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParseError(reqwest::UrlError);
    }
}

impl IpfsApi {
    /// Retrieves the contents of a file from the IPFS network. Takes a
    /// hash and returns an iterator of bytes. The result can be streamed, if
    /// the file is large.
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// let hello = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")?;
    /// let hello_string = String::from_utf8(hello.collect())?;
    /// println!("{}", hello_string);
    /// ```
    pub fn cat(&self, hash: &str) -> Result<impl Iterator<Item=u8>> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/cat");
        url.query_pairs_mut()
            .append_pair("arg", hash);
        let resp = reqwest::get(url)?;
        Ok(resp.bytes().filter(|x|x.is_ok()).map(|x|x.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use IpfsApi;
    #[test]
    fn test_cat() {
        let api = IpfsApi::new("127.0.0.1", 5001);
        // Hello world object
        let bytes = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u").unwrap();
        let data = String::from_utf8(bytes.collect()).unwrap();

        assert_eq!("Hello World\n", &data);
    }
}
