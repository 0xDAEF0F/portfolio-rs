pub enum Coin {
    Btc,
    Eth,
    Sol,
    Hype,
    Wif,
    Fart,
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Coin::Btc => write!(f, "BTC"),
            Coin::Eth => write!(f, "ETH"),
            Coin::Sol => write!(f, "SOL"),
            Coin::Hype => write!(f, "HYPE"),
            Coin::Wif => write!(f, "WIF"),
            Coin::Fart => write!(f, "FART"),
        }
    }
}
