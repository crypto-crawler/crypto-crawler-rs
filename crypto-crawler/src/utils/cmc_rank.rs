use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use once_cell::sync::Lazy;

pub(crate) static CMC_RANKS: Lazy<HashMap<String, u64>> = Lazy::new(|| {
    // offline data, in case the network is down
    let offline: HashMap<String, u64> = vec![
        ("BTC", 1),
        ("ETH", 2),
        ("USDT", 3),
        ("BNB", 4),
        ("USDC", 5),
        ("XRP", 6),
        ("SOL", 7),
        ("LUNA", 8),
        ("ADA", 9),
        ("UST", 10),
        ("BUSD", 11),
        ("DOGE", 12),
        ("AVAX", 13),
        ("DOT", 14),
        ("SHIB", 15),
        ("WBTC", 16),
        ("DAI", 17),
        ("MATIC", 18),
        ("NEAR", 19),
        ("TRX", 20),
        ("CRO", 21),
        ("LTC", 22),
        ("BCH", 23),
        ("LEO", 24),
        ("FTT", 25),
        ("ATOM", 26),
        ("LINK", 27),
        ("UNI", 28),
        ("ALGO", 29),
        ("XLM", 30),
        ("APE", 31),
        ("XMR", 32),
        ("ETC", 33),
        ("VET", 34),
        ("HBAR", 35),
        ("ICP", 36),
        ("FIL", 37),
        ("MANA", 38),
        ("EGLD", 39),
        ("SAND", 40),
        ("THETA", 41),
        ("XTZ", 42),
        ("CAKE", 43),
        ("RUNE", 44),
        ("EOS", 45),
        ("FTM", 46),
        ("AAVE", 47),
        ("GMT", 48),
        ("KLAY", 49),
        ("AXS", 50),
        ("HNT", 51),
        ("KCS", 52),
        ("ZEC", 53),
        ("FLOW", 54),
        ("BTT", 55),
        ("GRT", 56),
        ("MIOTA", 57),
        ("HT", 58),
        ("MKR", 59),
        ("BSV", 60),
        ("XEC", 61),
        ("CVX", 62),
        ("TUSD", 63),
        ("WAVES", 64),
        ("NEO", 65),
        ("STX", 66),
        ("NEXO", 67),
        ("QNT", 68),
        ("OKB", 69),
        ("KSM", 70),
        ("CHZ", 71),
        ("CELO", 72),
        ("CRV", 73),
        ("LRC", 74),
        ("ONE", 75),
        ("GALA", 76),
        ("ENJ", 77),
        ("DASH", 78),
        ("USDP", 79),
        ("USDN", 80),
        ("BAT", 81),
        ("ZIL", 82),
        ("MINA", 83),
        ("AR", 84),
        ("XEM", 85),
        ("DCR", 86),
        ("AMP", 87),
        ("XDC", 88),
        ("KAVA", 89),
        ("HOT", 90),
        ("TFUEL", 91),
        ("COMP", 92),
        ("KNC", 93),
        ("KDA", 94),
        ("YFI", 95),
        ("SCRT", 96),
        ("GLMR", 97),
        ("PAXG", 98),
        ("GNO", 99),
        ("ROSE", 100),
        ("ICX", 101),
        ("AUDIO", 102),
        ("ZRX", 103),
        ("ANC", 104),
        ("QTUM", 105),
        ("IOTX", 106),
        ("SNX", 107),
        ("BORA", 108),
        ("OMG", 109),
        ("BNT", 110),
        ("SKL", 111),
        ("XYM", 112),
        ("ANKR", 113),
        ("ENS", 114),
        ("CEL", 115),
        ("BTG", 116),
        ("SXP", 117),
        ("1INCH", 118),
        ("GT", 119),
        ("ELON", 120),
        ("SRM", 121),
        ("LPT", 122),
        ("RVN", 123),
        ("JST", 124),
        ("WAXP", 125),
        ("SC", 126),
        ("FEI", 127),
        ("IOST", 128),
        ("VLX", 129),
        ("ONT", 130),
        ("FXS", 131),
        ("NFT", 132),
        ("RNDR", 133),
        ("RLY", 134),
        ("CHSB", 135),
        ("ZEN", 136),
        ("RENBTC", 137),
        ("IMX", 138),
        ("FLUX", 139),
        ("UMA", 140),
        ("WOO", 141),
        ("ACA", 142),
        ("BTRST", 143),
        ("GLM", 144),
        ("STORJ", 145),
        ("HIVE", 146),
        ("POLY", 147),
        ("ILV", 148),
        ("VGX", 149),
        ("SUSHI", 150),
        ("KEEP", 151),
        ("DGB", 152),
        ("REN", 153),
        ("CSPR", 154),
        ("TWT", 155),
        ("MXC", 156),
        ("CKB", 157),
        ("DAR", 158),
        ("PLA", 159),
        ("SPELL", 160),
        ("SYS", 161),
        ("CEEK", 162),
        ("OCEAN", 163),
        ("TEL", 164),
        ("PERP", 165),
        ("WIN", 166),
        ("CELR", 167),
        ("XPRT", 168),
        ("DYDX", 169),
        ("REV", 170),
        ("MX", 171),
        ("XNO", 172),
        ("CFX", 173),
        ("LSK", 174),
        ("NU", 175),
        ("XDB", 176),
        ("RAY", 177),
        ("DAO", 178),
        ("GUSD", 179),
        ("PYR", 180),
        ("C98", 181),
        ("DENT", 182),
        ("MED", 183),
        ("XCH", 184),
        ("POWR", 185),
        ("UOS", 186),
        ("COTI", 187),
        ("INJ", 188),
        ("JOE", 189),
        ("DAG", 190),
        ("FET", 191),
        ("PUNDIX", 192),
        ("CHR", 193),
        ("WRX", 194),
        ("ONG", 195),
        ("ORBS", 196),
        ("SNT", 197),
        ("XYO", 198),
        ("PEOPLE", 199),
        ("FX", 200),
        ("HEX", 201),
        ("stETH", 202),
        ("WTRX", 203),
        ("YOUC", 204),
        ("BTCB", 205),
        ("FRAX", 206),
        ("TON", 207),
        ("DFI", 208),
        ("WBNB", 209),
        ("BTTOLD", 210),
        ("HBTC", 211),
        ("XCN", 212),
        ("OSMO", 213),
        ("LDO", 214),
        ("SAFE", 215),
        ("LUSD", 216),
        ("BIT", 217),
        ("T", 218),
        ("LN", 219),
        ("NXM", 220),
        ("CCXX", 221),
        ("ASTR", 222),
        ("XAUT", 223),
        ("TTT", 224),
        ("EVER", 225),
        ("WVLX", 226),
        ("HUSD", 227),
        ("KOK", 228),
        ("SAPP", 229),
        ("RPL", 230),
        ("RACA", 231),
        ("WEMIX", 232),
        ("XWC", 233),
        ("SAFEMOON", 234),
        ("BEST", 235),
        ("FRTS", 236),
        ("MOB", 237),
        ("VVS", 238),
        ("BSW", 239),
        ("DESO", 240),
        ("ARRR", 241),
        ("HUM", 242),
        ("CBG", 243),
        ("BNX", 244),
        ("MPL", 245),
        ("MVL", 246),
        ("REQ", 247),
        ("TLOS", 248),
        ("erowan", 249),
        ("DIVI", 250),
        ("ARDR", 251),
        ("CVC", 252),
        ("ANY", 253),
        ("TRIBE", 254),
        ("USDX", 255),
        ("OGN", 256),
    ]
    .into_iter()
    .map(|x| (x.0.to_string(), x.1))
    .collect();
    let online = get_cmc_ranks(1024);

    let mut result = HashMap::<String, u64>::new();
    result.extend(offline);
    result.extend(online);
    result
});

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
    use crypto_market_type::MarketType;

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
