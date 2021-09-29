use lazy_static::lazy_static;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref CMC_RANKS: HashMap<String, u64> = {
        // offline data, in case the network is down
        let offline: HashMap<String, u64> = vec![
            ("BTC", 1),
            ("ETH", 2),
            ("USDT", 3),
            ("ADA", 4),
            ("BNB", 5),
            ("XRP", 6),
            ("SOL", 7),
            ("USDC", 8),
            ("DOT", 9),
            ("DOGE", 10),
            ("UNI", 11),
            ("AVAX", 12),
            ("LUNA", 13),
            ("BUSD", 14),
            ("LINK", 15),
            ("ALGO", 16),
            ("LTC", 17),
            ("BCH", 18),
            ("WBTC", 19),
            ("ATOM", 20),
            ("MATIC", 21),
            ("ICP", 22),
            ("DAI", 23),
            ("FIL", 24),
            ("XLM", 25),
            ("TRX", 26),
            ("FTT", 27),
            ("ETC", 28),
            ("VET", 29),
            ("XTZ", 30),
            ("XEC", 31),
            ("THETA", 32),
            ("BTCB", 33),
            ("XMR", 34),
            ("AXS", 35),
            ("CAKE", 36),
            ("EGLD", 37),
            ("CRO", 38),
            ("EOS", 39),
            ("AAVE", 40),
            ("QNT", 41),
            ("HBAR", 42),
            ("NEAR", 43),
            ("FTM", 44),
            ("GRT", 45),
            ("IOTA", 46),
            ("SHIB", 47),
            ("KSM", 48),
            ("UST", 49),
            ("KLAY", 50),
            ("NEO", 51),
            ("LEO", 52),
            ("WAVES", 53),
            ("BSV", 54),
            ("MKR", 55),
            ("BTT", 56),
            ("AMP", 57),
            ("COMP", 58),
            ("CELO", 59),
            ("HNT", 60),
            ("XDC", 61),
            ("ONE", 62),
            ("DASH", 63),
            ("RUNE", 64),
            ("CHZ", 65),
            ("TUSD", 66),
            ("STX", 67),
            ("TFUEL", 68),
            ("AR", 69),
            ("HOT", 70),
            ("ZEC", 71),
            ("DYDX", 72),
            ("DCR", 73),
            ("REV", 74),
            ("OMG", 75),
            ("HT", 76),
            ("SUSHI", 77),
            ("XEM", 78),
            ("CEL", 79),
            ("MANA", 80),
            ("SNX", 81),
            ("YFI", 82),
            ("PERP", 83),
            ("ENJ", 84),
            ("MINA", 85),
            ("CRV", 86),
            ("FLOW", 87),
            ("OKB", 88),
            ("IOST", 89),
            ("ICX", 90),
            ("SRM", 91),
            ("USDP", 92),
            ("REN", 93),
            ("CELR", 94),
            ("RVN", 95),
            ("ZIL", 96),
            ("BTG", 97),
            ("BAT", 98),
            ("QTUM", 99),
            ("TEL", 100),
            ("AUDIO", 101),
            ("KCS", 102),
            ("NEXO", 103),
            ("BNT", 104),
            ("MDX", 105),
            ("ZEN", 106),
            ("RENBTC", 107),
            ("ZRX", 108),
            ("SC", 109),
            ("VGX", 110),
            ("RAY", 111),
            ("ONT", 112),
            ("DGB", 113),
            ("CHSB", 114),
            ("ANKR", 115),
            ("NANO", 116),
            ("IOTX", 117),
            ("UMA", 118),
            ("SAND", 119),
            ("USDN", 120),
            ("FET", 121),
            ("COTI", 122),
            ("HUSD", 123),
            ("LRC", 124),
            ("KAVA", 125),
            ("1INCH", 126),
            ("DENT", 127),
            ("GLM", 128),
            ("FEI", 129),
            ("WOO", 130),
            ("UBT", 131),
            ("NMR", 132),
            ("SXP", 133),
            ("XDB", 134),
            ("OCEAN", 135),
            ("RSR", 136),
            ("GNO", 137),
            ("LPT", 138),
            ("LSK", 139),
            ("WAXP", 140),
            ("ERG", 141),
            ("DAG", 142),
            ("XYO", 143),
            ("SKL", 144),
            ("ALPHA", 145),
            ("BAKE", 146),
            ("CFX", 147),
            ("CKB", 148),
            ("POLY", 149),
            ("WRX", 150),
            ("STORJ", 151),
            ("VTHO", 152),
            ("AGIX", 153),
            ("PAXG", 154),
            ("INJ", 155),
            ("XVG", 156),
            ("ELF", 157),
            ("WIN", 158),
            ("MED", 159),
            ("BCD", 160),
            ("GT", 161),
            ("VLX", 162),
            ("CVC", 163),
            ("FX", 164),
            ("ONG", 165),
            ("RLC", 166),
            ("MLN", 167),
            ("PROM", 168),
            ("ASD", 169),
            ("ARDR", 170),
            ("EWT", 171),
            ("OGN", 172),
            ("REEF", 173),
            ("ALICE", 174),
            ("BAND", 175),
            ("STMX", 176),
            ("SNT", 177),
            ("XVS", 178),
            ("ROSE", 179),
            ("STRAX", 180),
            ("CTSI", 181),
            ("MAID", 182),
            ("ORBS", 183),
            ("HIVE", 184),
            ("OXT", 185),
            ("REP", 186),
            ("DERO", 187),
            ("FUN", 188),
            ("NKN", 189),
            ("CSPR", 190),
            ("ARK", 191),
            ("NU", 192),
            ("MTL", 193),
            ("REQ", 194),
            ("TOMO", 195),
            ("SYS", 196),
            ("STEEM", 197),
            ("PHA", 198),
            ("ANT", 199),
            ("BTCST", 200),
            ("HEX", 201),
            ("BCHA", 202),
            ("STETH", 203),
            ("WBNB", 204),
            ("CCXX", 205),
            ("HBTC", 206),
            ("CTC", 207),
            ("TTT", 208),
            ("EGR", 209),
            ("YOUC", 210),
            ("OMI", 211),
            ("ALT", 212),
            ("NXM", 213),
            ("LUSD", 214),
            ("XWC", 215),
            ("SAFEMOON", 216),
            ("DFI", 217),
            ("XYM", 218),
            ("INO", 219),
            ("vBNB", 220),
            ("GALA", 221),
            ("LN", 222),
            ("C98", 223),
            ("MOVR", 224),
            ("XPRT", 225),
            ("ARRR", 226),
            ("ETN", 227),
            ("YGG", 228),
            ("AKT", 229),
            ("BIT", 230),
            ("FRAX", 231),
            ("ANC", 232),
            ("LYXe", 233),
            ("BEST", 234),
            ("TWT", 235),
            ("KOK", 236),
            ("RPL", 237),
            ("MNGO", 238),
            ("ILV", 239),
            ("ORC", 240),
            ("PUNDIX", 241),
            ("SDN", 242),
            ("MBOX", 243),
            ("TRIBE", 244),
            ("SUSD", 245),
            ("HEDG", 246),
            ("FIDA", 247),
            ("MASK", 248),
            ("TITAN", 249),
            ("KNC", 250),
            ("XCH", 251),
            ("PEAK", 252),
            ("ORN", 253),
            ("RGT", 254),
            ("MIR", 255),
            ("KDA", 256),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();
        let online = get_cmc_ranks(1024);

        let mut result = HashMap::<String, u64>::new();
        result.extend(offline);
        result.extend(online);
        result
    };
}

fn http_get(url: &str) -> Result<String, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::blocking::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.get(url).send()?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text()?),
        Err(error) => Err(error),
    }
}

// Returns a map of coin to cmcRank.
fn get_cmc_ranks(limit: i64) -> HashMap<String, u64> {
    let mut mapping: HashMap<String, u64> = HashMap::new();
    let url = format!("https://api.coinmarketcap.com/data-api/v3/cryptocurrency/listing?start=1&limit={}&sortBy=market_cap&sortType=desc&convert=USD&cryptoType=all&tagType=all&audited=false", limit);
    if let Ok(txt) = http_get(&url) {
        if let Ok(json_obj) = serde_json::from_str::<HashMap<String, Value>>(&txt) {
            if let Some(data) = json_obj.get("data") {
                #[derive(Serialize, Deserialize)]
                #[allow(non_snake_case)]
                struct Currency {
                    id: i64,
                    name: String,
                    symbol: String,
                    cmcRank: u64,
                }
                let arr = data["cryptoCurrencyList"].as_array().unwrap();
                for currency in arr {
                    let currency: Currency = serde_json::from_value(currency.clone()).unwrap();
                    mapping.insert(currency.symbol, currency.cmcRank);
                }
            }
        }
    }
    mapping
}

pub(crate) fn sort_by_cmc_rank(exchange: &str, symbols: &mut [String]) {
    symbols.sort_by_key(|symbol| {
        if let Some(pair) = crypto_pair::normalize_pair(symbol, exchange) {
            let base = pair.split('/').next().unwrap();
            *CMC_RANKS.get(base).unwrap_or(&u64::max_value())
        } else {
            u64::max_value()
        }
    });
}

#[cfg(test)]
mod tests {
    use crypto_markets::MarketType;

    #[test]
    fn test_get_cmc_ranks() {
        let mapping = super::get_cmc_ranks(256);
        let mut v = Vec::from_iter(mapping);
        v.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        for (coin, rank) in v {
            println!("(\"{}\", {}),", coin, rank);
        }
    }

    #[test]
    fn test_sort_by_cmc_rank() {
        let mut binance_linear_swap =
            crypto_markets::fetch_symbols("binance", MarketType::LinearSwap).unwrap();
        super::sort_by_cmc_rank("binance", &mut binance_linear_swap);
        assert_eq!("BTCUSDT", binance_linear_swap[0]);
        assert_eq!("BTCBUSD", binance_linear_swap[1]);
        assert_eq!("ETHUSDT", binance_linear_swap[2]);
        assert_eq!("ETHBUSD", binance_linear_swap[3]);
    }
}
