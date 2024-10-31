use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/openOrders`
///
/// Get all open orders on a symbol. Careful when accessing this with no symbol.
///
/// Weight(IP):
/// * `6` for a single symbol;
/// * `80` when the symbol parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade;
///
/// let request = trade::open_orders().symbol("BNBUSDT");
/// ```
pub struct OpenOrders {
    symbol: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl OpenOrders {
    pub fn new() -> Self {
        Self {
            symbol: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl Default for OpenOrders {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpenOrders> for Request {
    fn from(request: OpenOrders) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/openOrders".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OpenOrders;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_open_orders_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = OpenOrders::new()
            .symbol("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/openOrders".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
