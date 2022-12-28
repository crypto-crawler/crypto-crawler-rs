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
        ("BUSD", 6),
        ("XRP", 7),
        ("DOGE", 8),
        ("ADA", 9),
        ("MATIC", 10),
        ("DOT", 11),
        ("DAI", 12),
        ("SHIB", 13),
        ("SOL", 14),
        ("TRX", 15),
        ("UNI", 16),
        ("LTC", 17),
        ("WBTC", 18),
        ("AVAX", 19),
        ("LEO", 20),
        ("ATOM", 21),
        ("LINK", 22),
        ("ETC", 23),
        ("XMR", 24),
        ("XLM", 25),
        ("CRO", 26),
        ("BCH", 27),
        ("ALGO", 28),
        ("TON", 29),
        ("NEAR", 30),
        ("VET", 31),
        ("FIL", 32),
        ("QNT", 33),
        ("FLOW", 34),
        ("CHZ", 35),
        ("LUNC", 36),
        ("OKB", 37),
        ("HBAR", 38),
        ("EGLD", 39),
        ("ICP", 40),
        ("XCN", 41),
        ("EOS", 42),
        ("XTZ", 43),
        ("USDP", 44),
        ("APE", 45),
        ("SAND", 46),
        ("THETA", 47),
        ("MANA", 48),
        ("TUSD", 49),
        ("AAVE", 50),
        ("GUSD", 51),
        ("BSV", 52),
        ("HT", 53),
        ("USDD", 54),
        ("MKR", 55),
        ("KCS", 56),
        ("BTT", 57),
        ("USDN", 58),
        ("AXS", 59),
        ("BIT", 60),
        ("TWT", 61),
        ("ZEC", 62),
        ("MIOTA", 63),
        ("CAKE", 64),
        ("APT", 65),
        ("XEC", 66),
        ("PAXG", 67),
        ("KLAY", 68),
        ("SNX", 69),
        ("FTM", 70),
        ("NEO", 71),
        ("ETHW", 72),
        ("GRT", 73),
        ("FEI", 74),
        ("NEXO", 75),
        ("MINA", 76),
        ("BNX", 77),
        ("GT", 78),
        ("DASH", 79),
        ("RUNE", 80),
        ("BAT", 81),
        ("CSPR", 82),
        ("LDO", 83),
        ("HNT", 84),
        ("LRC", 85),
        ("OSMO", 86),
        ("ENJ", 87),
        ("CRV", 88),
        ("XDC", 89),
        ("1INCH", 90),
        ("STX", 91),
        ("KAVA", 92),
        ("GMX", 93),
        ("AR", 94),
        ("ZIL", 95),
        ("DCR", 96),
        ("XEM", 97),
        ("IMX", 98),
        ("COMP", 99),
        ("HOT", 100),
        ("CVX", 101),
        ("RVN", 102),
        ("WAVES", 103),
        ("BTG", 104),
        ("ENS", 105),
        ("FTT", 106),
        ("BAL", 107),
        ("CHSB", 108),
        ("ROSE", 109),
        ("GNO", 110),
        ("TFUEL", 111),
        ("IOTX", 112),
        ("GMT", 113),
        ("USTC", 114),
        ("OP", 115),
        ("QTUM", 116),
        ("LUNA", 117),
        ("CELO", 118),
        ("GALA", 119),
        ("YFI", 120),
        ("KSM", 121),
        ("GLM", 122),
        ("JST", 123),
        ("ANKR", 124),
        ("KDA", 125),
        ("POLY", 126),
        ("ONE", 127),
        ("XYM", 128),
        ("ABBC", 129),
        ("ELON", 130),
        ("LPT", 131),
        ("RSR", 132),
        ("XCH", 133),
        ("GLMR", 134),
        ("OMG", 135),
        ("HIVE", 136),
        ("IOST", 137),
        ("AMP", 138),
        ("VIDT", 139),
        ("SUSHI", 140),
        ("ZRX", 141),
        ("ONT", 142),
        ("BORA", 143),
        ("WOO", 144),
        ("NFT", 145),
        ("DYDX", 146),
        ("CEL", 147),
        ("ICX", 148),
        ("T", 149),
        ("AUDIO", 150),
        ("ASTR", 151),
        ("ZEN", 152),
        ("INJ", 153),
        ("RNDR", 154),
        ("BTRST", 155),
        ("WAXP", 156),
        ("SC", 157),
        ("STORJ", 158),
        ("UMA", 159),
        ("FLUX", 160),
        ("SXP", 161),
        ("RBN", 162),
        ("PEOPLE", 163),
        ("SCRT", 164),
        ("DGB", 165),
        ("SLP", 166),
        ("SKL", 167),
        ("LSK", 168),
        ("TRIBE", 169),
        ("KNC", 170),
        ("MASK", 171),
        ("PLA", 172),
        ("PUNDIX", 173),
        ("EWT", 174),
        ("MXC", 175),
        ("DAO", 176),
        ("WIN", 177),
        ("CVC", 178),
        ("CKB", 179),
        ("MED", 180),
        ("SYN", 181),
        ("MX", 182),
        ("COTI", 183),
        ("METIS", 184),
        ("REQ", 185),
        ("API3", 186),
        ("CEEK", 187),
        ("XNO", 188),
        ("OCEAN", 189),
        ("VGX", 190),
        ("XPRT", 191),
        ("CELR", 192),
        ("SRM", 193),
        ("BAND", 194),
        ("SSV", 195),
        ("ONG", 196),
        ("REN", 197),
        ("SYS", 198),
        ("CFG", 199),
        ("ANT", 200),
        ("HEX", 201),
        ("WTRX", 202),
        ("stETH", 203),
        ("BTCB", 204),
        ("FRAX", 205),
        ("WBNB", 206),
        ("BTTOLD", 207),
        ("HBTC", 208),
        ("WEMIX", 209),
        ("XAUT", 210),
        ("CCXX", 211),
        ("MV", 212),
        ("NXM", 213),
        ("FRTS", 214),
        ("DFI", 215),
        ("LUSD", 216),
        ("LN", 217),
        ("RPL", 218),
        ("BRISE", 219),
        ("VERI", 220),
        ("BabyDoge", 221),
        ("VVS", 222),
        ("EURS", 223),
        ("TEL", 224),
        ("ZEON", 225),
        ("SNN", 226),
        ("vUSDC", 227),
        ("SAFE", 228),
        ("USDX", 229),
        ("HFT", 230),
        ("ERG", 231),
        ("LYXe", 232),
        ("SOLO", 233),
        ("DEXE", 234),
        ("AVINOC", 235),
        ("DESO", 236),
        ("EUROC", 237),
        ("CTC", 238),
        ("BFC", 239),
        ("MBX", 240),
        ("FUN", 241),
        ("SNT", 242),
        ("WEVER", 243),
        ("EVER", 244),
        ("MVL", 245),
        ("YFII", 246),
        ("PYR", 247),
        ("SPELL", 248),
        ("BNT", 249),
        ("ILV", 250),
        ("RKN", 251),
        ("NU", 252),
        ("BTCST", 253),
        ("FX", 254),
        ("FXS", 255),
        ("KEEP", 256),
    ]
    .into_iter()
    .map(|x| (x.0.to_string(), x.1))
    .collect();
    let online = get_cmc_ranks(1024);

    if online.is_empty() { offline } else { online }
});

fn http_get(url: &str) -> Result<String, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

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
    let url = format!(
        "https://api.coinmarketcap.com/data-api/v3/cryptocurrency/listing?start=1&limit={limit}&sortBy=market_cap&sortType=desc&convert=USD&cryptoType=all&tagType=all&audited=false"
    );
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
            println!("(\"{coin}\", {rank}),");
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
