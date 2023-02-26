use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use once_cell::sync::Lazy;

pub(super) static CMC_RANKS: Lazy<HashMap<String, u64>> = Lazy::new(|| {
    // offline data, in case the network is down
    let offline: HashMap<String, u64> = vec![
        ("BTC", 1),
        ("ETH", 2),
        ("USDT", 3),
        ("BNB", 4),
        ("USDC", 5),
        ("XRP", 6),
        ("ADA", 7),
        ("BUSD", 8),
        ("MATIC", 9),
        ("DOGE", 10),
        ("SOL", 11),
        ("DOT", 12),
        ("SHIB", 13),
        ("LTC", 14),
        ("TRX", 15),
        ("AVAX", 16),
        ("DAI", 17),
        ("UNI", 18),
        ("WBTC", 19),
        ("LINK", 20),
        ("ATOM", 21),
        ("LEO", 22),
        ("OKB", 23),
        ("ETC", 24),
        ("TON", 25),
        ("XMR", 26),
        ("FIL", 27),
        ("BCH", 28),
        ("LDO", 29),
        ("XLM", 30),
        ("APT", 31),
        ("CRO", 32),
        ("NEAR", 33),
        ("VET", 34),
        ("HBAR", 35),
        ("APE", 36),
        ("ALGO", 37),
        ("ICP", 38),
        ("QNT", 39),
        ("GRT", 40),
        ("FTM", 41),
        ("FLOW", 42),
        ("EGLD", 43),
        ("MANA", 44),
        ("THETA", 45),
        ("EOS", 46),
        ("BIT", 47),
        ("AAVE", 48),
        ("XTZ", 49),
        ("AXS", 50),
        ("SAND", 51),
        ("STX", 52),
        ("TUSD", 53),
        ("LUNC", 54),
        ("RPL", 55),
        ("KLAY", 56),
        ("CHZ", 57),
        ("USDP", 58),
        ("NEO", 59),
        ("HT", 60),
        ("KCS", 61),
        ("BSV", 62),
        ("IMX", 63),
        ("MINA", 64),
        ("DASH", 65),
        ("CAKE", 66),
        ("FXS", 67),
        ("MKR", 68),
        ("CRV", 69),
        ("ZEC", 70),
        ("USDD", 71),
        ("MIOTA", 72),
        ("OP", 73),
        ("XEC", 74),
        ("BTT", 75),
        ("SNX", 76),
        ("GMX", 77),
        ("GUSD", 78),
        ("CFX", 79),
        ("GT", 80),
        ("TWT", 81),
        ("RUNE", 82),
        ("ZIL", 83),
        ("PAXG", 84),
        ("AGIX", 85),
        ("LRC", 86),
        ("ENJ", 87),
        ("OSMO", 88),
        ("1INCH", 89),
        ("FLR", 90),
        ("DYDX", 91),
        ("BAT", 92),
        ("SSV", 94),
        ("BONE", 95),
        ("CVX", 96),
        ("FEI", 97),
        ("ANKR", 98),
        ("CSPR", 99),
        ("ETHW", 100),
        ("BNX", 101),
        ("NEXO", 102),
        ("ROSE", 103),
        ("RVN", 104),
        ("LUNA", 105),
        ("CELO", 106),
        ("HNT", 107),
        ("COMP", 108),
        ("TFUEL", 109),
        ("XEM", 110),
        ("XDC", 111),
        ("KAVA", 112),
        ("RNDR", 113),
        ("HOT", 114),
        ("WOO", 115),
        ("YFI", 116),
        ("FET", 117),
        ("QTUM", 118),
        ("MOB", 119),
        ("DCR", 120),
        ("MAGIC", 121),
        ("T", 122),
        ("AR", 123),
        ("BLUR", 124),
        ("AUDIO", 125),
        ("KSM", 126),
        ("BAL", 127),
        ("ASTR", 128),
        ("ENS", 129),
        ("BTG", 130),
        ("SUSHI", 131),
        ("JASMY", 132),
        ("ONE", 133),
        ("GALA", 134),
        ("WAVES", 135),
        ("GNO", 136),
        ("USTC", 137),
        ("GLM", 138),
        ("IOTX", 139),
        ("INJ", 140),
        ("JST", 141),
        ("GLMR", 142),
        ("XCH", 143),
        ("MASK", 144),
        ("BAND", 145),
        ("AMP", 146),
        ("KDA", 147),
        ("OCEAN", 148),
        ("ICX", 149),
        ("OMG", 150),
        ("RSR", 151),
        ("ELON", 152),
        ("SC", 153),
        ("FLUX", 154),
        ("GMT", 155),
        ("ZRX", 156),
        ("CHSB", 157),
        ("ONT", 158),
        ("BICO", 159),
        ("XCN", 160),
        ("IOST", 161),
        ("XYM", 162),
        ("HIVE", 163),
        ("DAO", 164),
        ("LPT", 165),
        ("SKL", 166),
        ("ACH", 167),
        ("CKB", 168),
        ("SYN", 169),
        ("BORA", 170),
        ("WAXP", 171),
        ("SFP", 172),
        ("DGB", 173),
        ("STORJ", 174),
        ("SXP", 175),
        ("POLY", 176),
        ("EVER", 177),
        ("STG", 178),
        ("ZEN", 179),
        ("ILV", 180),
        ("KEEP", 181),
        ("ELF", 182),
        ("RLC", 183),
        ("LSK", 184),
        ("UMA", 185),
        ("KNC", 186),
        ("METIS", 187),
        ("CELR", 188),
        ("MC", 189),
        ("SLP", 190),
        ("PUNDIX", 191),
        ("BTRST", 192),
        ("RIF", 193),
        ("TRAC", 194),
        ("PLA", 195),
        ("EWT", 196),
        ("MED", 197),
        ("SYS", 198),
        ("SCRT", 199),
        ("NFT", 200),
        ("HEX", 201),
        ("WTRX", 202),
        ("stETH", 203),
        ("BTCB", 204),
        ("TMG", 205),
        ("WBNB", 206),
        ("FRAX", 207),
        ("HBTC", 208),
        ("BTTOLD", 209),
        ("TNC", 210),
        ("WEMIX", 211),
        ("BGB", 212),
        ("FTT", 213),
        ("XRD", 214),
        ("XAUT", 215),
        ("FLOKI", 216),
        ("NXM", 217),
        ("BabyDoge", 218),
        ("USDJ", 219),
        ("ASTRAFER", 220),
        ("LN", 221),
        ("DFI", 222),
        ("BRISE", 223),
        ("MV", 224),
        ("LUSD", 225),
        ("EDGT", 226),
        ("ANY", 227),
        ("ALI", 228),
        ("WEVER", 229),
        ("COCOS", 230),
        ("TEL", 231),
        ("LYXe", 232),
        ("MULTI", 233),
        ("KAS", 234),
        ("BDX", 235),
        ("CORE", 236),
        ("RON", 237),
        ("VVS", 238),
        ("PEOPLE", 239),
        ("HFT", 240),
        ("EURS", 241),
        ("MX", 242),
        ("API3", 243),
        ("AXL", 244),
        ("VGX", 245),
        ("GTC", 246),
        ("RBN", 247),
        ("CHR", 248),
        ("XNO", 249),
        ("DENT", 250),
        ("CVC", 251),
        ("LQTY", 252),
        ("CEL", 253),
        ("POLYX", 254),
        ("HOOK", 255),
        ("XTN", 256),
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
