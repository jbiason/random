use std::slice::Iter;

use reqwest::blocking::Client;
use serde::{de::DeserializeOwned, Deserialize};

static LIB_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// This is the connection handler.
/// API documentation is here:
/// https://punkapi.com/documentation/v2
struct Punk {
    url: String,
    client: Client,
}

impl Punk {
    pub fn new(/* connection information here */) -> Self {
        Self {
            url: "https://api.punkapi.com/v2/".into(),
            client: Client::builder()
                .user_agent(LIB_USER_AGENT)
                .build()
                .unwrap(),
        }
    }

    /// Information about beers.
    /// (This is done like this 'cause we want a function call/method/attribute for each different
    /// information in the server; it is just that this specific server have only one data type).
    pub fn beers(&self) -> Result<BeerIter, reqwest::Error> {
        BeerIter::new(self)
    }

    pub fn get<T: DeserializeOwned>(&self, page: u16) -> Result<T, reqwest::Error> {
        Ok(self.client.get(&self.url).send()?.json::<T>()?)
    }
}

/// This is a definition of data from the API
#[derive(Deserialize)]
struct Beer {
    id: u16,
    name: String,
    tagline: String,
    // ... and let's ignore everything else 'cause that's enough.
}

/// This is the magical iterator that gets stuff from the internet
struct BeerIter<'a> {
    connection: &'a Punk,
    data: Vec<Beer>,
    iter: Option<Iter<'a, Beer>>,
    page: u16,
}

impl<'a> BeerIter<'a> {
    pub fn new(connection: &'a Punk) -> Result<Self, reqwest::Error> {
        let mut result = Self {
            connection,
            data: Vec::new(),
            iter: None,
            page: 1
        };
        result.next_page()?;
        Ok(result)
    }

    fn next_page(&mut self) -> Result<(), reqwest::Error> {
        let page = self.connection.get::<Vec<Beer>>(self.page)?;
        self.data = page;
        self.iter = Some(self.data.iter());
        self.page += 1;
        Ok(())
    }
}

impl<'a> Iterator for BeerIter<'a> {
    type Item = &'a Beer;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(record) = self.iter?.next() {
            Some(record)
        } else {
            self.next_page();
            Some(self.iter?.next().unwrap())
        }
    }
}

fn main() {
    let connection = Punk::new();
    let beers = connection.beers().expect("Failed to get the initial data");
    // for beer in beers {
    //     println!("     ID: {}", beer.id);
    //     println!("   Name: {}", beer.name);
    //     println!("Tagline: {}\n", beer.tagline);
    // }
    println!("Hello, world!");
}
