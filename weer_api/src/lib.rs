//! A wrapper library for using [Weather API](https://www.weatherapi.com/)
//! 
//! *This is an unofficial library*

//! ## Usage
//! Put this in your Cargo.toml:
//! ```
//! [dependencies]
//! weer_api = "0.1.0"
//! ```

//! ### Examples
//! 
//! #### Get forecast
//! ```
//! use weer_api::{*, chrono::{Utc, TimeZone}};
//! 
//! # fn main() {
//! let client = Client::new("api_key", true);
//! let result = client.forecast()
//!     .query(Query::City("London".to_string()))
//!     .dt(Utc.ymd(2022, 08, 21).and_hms(0, 0, 0))
//!     .lang(Language::Spanish)
//!     .call();
//! 
//! assert!(result.is_ok())
//! # }
//! ```
//! 
//! #### Get future 
//! ```
//! use weer_api::{*, chrono::{Utc, TimeZone}};
//! 
//! # fn main() {
//! let client = Client::new("api_key", true);
//! let result = client.future()
//!     .query(Query::Coords(48.8567, 2.3508))
//!     .dt(Utc.ymd(2022, 09, 21).and_hms(0, 0, 0))
//!     .lang(Language::Spanish)
//!     .call();
//! 
//! # }
//! assert!(result.is_ok())
//! ```
//! 
//! #### Get history
//! ```
//! use weer_api::{*, chrono::{Utc, TimeZone}};
//! 
//! # fn main() {
//! let client = Client::new("api_key", true);
//! let result = client.history()
//!     .query(Query::Ip(None))
//!     .dt(Utc.ymd(2022, 07, 21).and_hms(0, 0, 0))
//!     .hour()
//!     .call();
//! 
//! # }
//! assert!(result.is_ok())
//! ```


mod api;
pub use api::*;

mod models;
pub use models::*;

mod lang;
pub use lang::Language;

mod query;
pub use query::Query;

pub use chrono;

use ureq::{Agent, AgentBuilder};
use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) api_key: String,
    pub(crate) agent: Agent,
    pub(crate) https: bool
}

impl Client {
    /// Creates a new client
    /// 
    /// The `https` parameter if set to true it will cause the client to make an https request instead of an http request.
    /// 
    /// # Example: 
    /// ```
    /// use weer_api::Client;
    /// 
    /// let client = Client::new("api_key", true);
    /// ```
    pub fn new(api_key: &str, https: bool) -> Self {
        let user_agent = format!(
            "{name} ({repo} {version})", 
            name = env!("CARGO_PKG_NAME"), 
            repo = env!("CARGO_PKG_REPOSITORY"),
            version = env!("CARGO_PKG_VERSION"),
        );

        let agent = AgentBuilder::new()
            .user_agent(&user_agent)
            .https_only(https)
            .build();

        Self { 
            api_key: api_key.to_string(),
            agent,
            https
        }
    }

    /// Function to get forecast
    /// 
    /// This returns upto next 14 day weather forecast and weather alert. The data is returned as a Forecast Object. 
    /// 
    /// ## Example:
    /// ```
    /// use weer_api::{*, chrono::{Utc, TimeZone}};
    /// 
    /// let client = Client::new("api_key", true);
    /// let result = client.forecast()
    ///     .query(Query::City("London".to_string()))
    ///     .dt(Utc.ymd(2022, 08, 21).and_hms(0, 0, 0))
    ///     .lang(Language::Spanish)
    ///     .call();
    /// 
    /// assert!(result.is_ok())
    /// ```
    pub fn forecast<Tz: chrono::TimeZone>(&self) -> ForecastApi<Tz> 
    where
        Tz::Offset: Display
    {
        ForecastApi::<Tz>::new(&self)
    }

    /// Function to get future
    /// 
    /// This returns weather in a 3 hourly interval in future for a date between 14 days and 300 days from today in the future.
    /// 
    /// ## Example:
    /// ```
    /// use weer_api::{*, chrono::{Utc, TimeZone}};
    /// 
    /// let client = Client::new("api_key", true);
    /// let result = client.future()
    ///     .query(Query::Coords(48.8567, 2.3508))
    ///     .dt(Utc.ymd(2022, 09, 21).and_hms(0, 0, 0))
    ///     .lang(Language::Spanish)
    ///     .call();
    /// 
    /// assert!(result.is_ok())
    /// ```
    pub fn future<Tz: chrono::TimeZone>(&self) -> FutureApi<Tz> 
    where
        Tz::Offset: Display
    {
        FutureApi::new(&self)
    }

    /// Function to get history
    /// 
    /// This returns historical weather for a date on or after 1st Jan, 2010. The data is returned as a Forecast Object. 
    /// 
    /// ## Example:
    /// ```
    /// use weer_api::{*, chrono::{Utc, TimeZone}};
    /// 
    /// let client = Client::new("api_key", true);
    /// let result = client.history()
    ///     .query(Query::Ip(None))
    ///     .dt(Utc.ymd(2022, 07, 21).and_hms(0, 0, 0))
    ///     .hour()
    ///     .call();
    /// 
    /// assert!(result.is_ok())
    /// ```
    pub fn history<Tz: chrono::TimeZone>(&self) -> HistoryApi<Tz>
    where
        Tz::Offset: Display
    {
        HistoryApi::<Tz>::new(&self)
    }

    /// Function to get realtime
    /// 
    /// Use to get up to date current weather information. The data is returned as a Current Object. 
    /// 
    /// ## Example:
    /// ```
    /// use weer_api::*;
    /// 
    /// let client = Client::new("api_key", true);
    /// let result = client.realtime()
    ///     .query(Query::Ip(None))
    ///     .lang(Language::Spanish)
    ///     .call();
    /// 
    /// assert!(result.is_ok())
    /// ```
    pub fn realtime(&self) -> RealtimeApi {
        RealtimeApi::new(&self)
    }

    /// Function to get realtime
    /// 
    /// This returns matching cities and towns as an array of Location object
    /// 
    /// ## Example:
    /// ```
    /// use weer_api::*; 
    /// 
    /// let client = Client::new("api_key", true);
    /// let result = client.search()
    ///     .query(Query::Ip(None))
    ///     .call();
    /// 
    /// assert!(result.is_ok())
    /// ```
    pub fn search(&self) -> SearchApi {
        SearchApi::new(&self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{Utc, TimeZone};

    fn get_client() -> Client {        
        let api_key = option_env!("API_KEY").unwrap();
        Client::new(api_key, true)
    }

    #[test]
    fn forecast() {
        let client = get_client();
        let result = client.forecast()
            .query(Query::Ip(None))
            .dt(Utc.ymd(2022, 08, 21).and_hms(0, 0, 0))
            .lang(Language::Spanish)
            .call();

        assert!(result.is_ok())
    }

    #[test]
    fn future() {
        let client = get_client();
        let result = client.future()
            .query(Query::Ip(None))
            .dt(Utc.ymd(2022, 09, 21).and_hms(0, 0, 0))
            .lang(Language::Spanish)
            .call();

        assert!(result.is_ok())
    }

    #[test]
    fn history() {
        let client = get_client();
        let result = client.history()
            .query(Query::Ip(None))
            .dt(Utc.ymd(2022, 07, 21).and_hms(0, 0, 0))
            .hour()
            .call();
        
        assert!(result.is_ok())
    }

    #[test]
    fn realtime() {
        let client = get_client();
        let result = client.realtime()
            .query(Query::Ip(None))
            .lang(Language::Spanish)
            .call();

        assert!(result.is_ok())
    }

    #[test]
    fn search() {
        let client = get_client();
        let result = client.search().query(Query::Ip(None)).call();

        assert!(result.is_ok())
    }

    #[test]
    fn lang() {
        let l = Language::new("bg").unwrap();
        assert_eq!(Language::Bulgarian, l)
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
