#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn kachua(ctx: &Ctx) -> Script {
    let mut l_gamble1 = 0;
    let mut l_gamble2 = 0;
    let mut l_item = 0;
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        return ctx.close();
    }
    ctx.fx().cutin("katsua01.bmp", 2)?;
    ctx.lines_as(
        "Kachua",
        args![
            "Diamonds...!",
            "I simply can't get my mind off",
            "them! Ever since that man",
            "showed me that diamond,",
            "it's been all I think about!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Would you like to have mine?", "Ah, what a shame..."])? == 1 {
        ctx.lines_as(
            "Kachua",
            args![
                "Yes, I know...",
                "Even among everything",
                "in my collections, nothing",
                "compares to diamonds..."
            ],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("katsua01.bmp", 255)?;
        return ctx.end();
    }
    if ctx.call(Function::CountItem, args![732])? == 0 {
        ctx.fx().cutin("katsua01.bmp", 255)?;
        ctx.fx().cutin("katsua03.bmp", 2)?;
        ctx.lines_as(
            "Kachua",
            args!["*piff*", "You don't have any diamonds!", "Don't even try to fool me!"],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("katsua03.bmp", 255)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Kachua",
        args![
            "Are you sure you don't mind",
            "giving this to me? Thank you",
            "so much! I don't have much in",
            "the way of money, but i can give",
            "you something from one of my",
            "collections~"
        ],
    )?;
    ctx.next()?;
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 5500 {
        ctx.fx().cutin("katsua01.bmp", 255)?;
        ctx.fx().cutin("katsua03.bmp", 2)?;
        ctx.lines_as(
            "Kachua",
            args![
                "Errr...",
                "You brought too many things.",
                "You can't receive this item now.",
                "You better reorganize your inventory and try again."
            ],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("katsua03.bmp", 255)?;
        return ctx.end();
    }
    if ctx.call(Function::CountItem, args![732])? == 0 {
        ctx.fx().cutin("katsua01.bmp", 255)?;
        return ctx.close();
    }
    ctx.items().take(732, 1)?;
    ctx.lines_as("Kachua", args!["So what would", "you like to have?"])?;
    ctx.next()?;
    match ctx.menu(&["Weapon", "Armor", "Garment", "Helmet", "Shoes", "Shield"])? {
        0 => {
            l_gamble1 = ctx.rand_range(1, 1000)?;
            if l_gamble1 > 920 && l_gamble1 < 931 {
                l_gamble2 = ctx.rand_range(1, 85)?;
                if l_gamble2 == 1 {
                    l_item = 1128;
                } else if l_gamble2 == 2 {
                    l_item = 1120;
                } else if l_gamble2 == 3 {
                    l_item = 1127;
                } else if l_gamble2 == 4 {
                    l_item = 1158;
                } else if l_gamble2 == 5 {
                    l_item = 1155;
                } else if l_gamble2 == 6 {
                    l_item = 1220;
                } else if l_gamble2 == 7 {
                    l_item = 1222;
                } else if l_gamble2 == 8 {
                    l_item = 1253;
                } else if l_gamble2 == 9 {
                    l_item = 1529;
                } else if l_gamble2 == 10 {
                    l_item = 1251;
                } else if l_gamble2 == 11 {
                    l_item = 1361;
                } else if l_gamble2 == 12 {
                    l_item = 1258;
                } else if l_gamble2 == 13 {
                    l_item = 1257;
                } else if l_gamble2 == 14 {
                    l_item = 1256;
                } else if l_gamble2 == 15 {
                    l_item = 1259;
                } else if l_gamble2 == 16 {
                    l_item = 1260;
                } else if l_gamble2 == 17 {
                    l_item = 1716;
                } else if l_gamble2 == 18 {
                    l_item = 1715;
                } else if l_gamble2 == 19 {
                    l_item = 1711;
                } else if l_gamble2 == 20 {
                    l_item = 1702;
                } else if l_gamble2 == 21 {
                    l_item = 1520;
                } else if l_gamble2 == 22 {
                    l_item = 1610;
                } else if l_gamble2 == 23 {
                    l_item = 1615;
                } else if l_gamble2 == 24 {
                    l_item = 1602;
                } else if l_gamble2 == 25 {
                    l_item = 1461;
                } else if l_gamble2 == 26 {
                    l_item = 1402;
                } else if l_gamble2 == 27 {
                    l_item = 1961;
                } else if l_gamble2 == 28 {
                    l_item = 1957;
                } else if l_gamble2 == 29 {
                    l_item = 1552;
                } else if l_gamble2 == 30 {
                    l_item = 1551;
                } else if l_gamble2 == 31 {
                    l_item = 1553;
                } else if l_gamble2 == 32 {
                    l_item = 1554;
                } else if l_gamble2 == 33 {
                    l_item = 1555;
                } else if l_gamble2 == 34 {
                    l_item = 1556;
                } else if l_gamble2 == 35 {
                    l_item = 1951;
                } else if l_gamble2 == 36 {
                    l_item = 1959;
                } else if l_gamble2 == 37 {
                    l_item = 1953;
                } else if l_gamble2 == 38 {
                    l_item = 1955;
                } else if l_gamble2 == 39 {
                    l_item = 1810;
                } else if l_gamble2 == 40 {
                    l_item = 1910;
                } else if l_gamble2 == 41 {
                    l_item = 1906;
                } else if l_gamble2 == 42 {
                    l_item = 1902;
                } else if l_gamble2 == 43 {
                    l_item = 1904;
                } else if l_gamble2 == 44 {
                    l_item = 1912;
                } else if l_gamble2 == 45 {
                    l_item = 1908;
                } else if l_gamble2 == 46 {
                    l_item = 1808;
                } else if l_gamble2 == 47 {
                    l_item = 1802;
                } else if l_gamble2 == 48 {
                    l_item = 1812;
                } else if l_gamble2 == 49 {
                    l_item = 1806;
                } else if l_gamble2 == 50 {
                    l_item = 1804;
                } else if l_gamble2 == 51 {
                    l_item = 1550;
                } else if l_gamble2 == 52 {
                    l_item = 1246;
                } else if l_gamble2 == 53 {
                    l_item = 1147;
                } else if l_gamble2 < 56 {
                    l_item = 1264;
                } else if l_gamble2 < 58 {
                    l_item = 1262;
                } else if l_gamble2 < 60 {
                    l_item = 1622;
                } else if l_gamble2 == 60 {
                    l_item = 1723;
                } else if l_gamble2 < 63 {
                    l_item = 1965;
                } else if l_gamble2 < 65 {
                    l_item = 1966;
                } else if l_gamble2 < 67 {
                    l_item = 1967;
                } else if l_gamble2 < 69 {
                    l_item = 1968;
                } else if l_gamble2 < 71 {
                    l_item = 1914;
                } else if l_gamble2 < 73 {
                    l_item = 1915;
                } else if l_gamble2 < 75 {
                    l_item = 1916;
                } else if l_gamble2 < 77 {
                    l_item = 1917;
                } else if l_gamble2 < 79 {
                    l_item = 13004;
                } else if l_gamble2 < 81 {
                    l_item = 1307;
                } else if l_gamble2 == 81 {
                    l_item = 1560;
                } else if l_gamble2 == 82 {
                    l_item = 1618;
                } else if l_gamble2 == 83 {
                    l_item = 1620;
                } else if l_gamble2 < 86 {
                    l_item = 1971;
                }
            } else if l_gamble1 < 201 {
                l_item = 1201;
            } else if l_gamble1 < 301 {
                l_item = 1101;
            } else if l_gamble1 < 401 {
                l_item = 1601;
            } else if l_gamble1 < 501 {
                l_item = 1116;
            } else if l_gamble1 < 601 {
                l_item = 1250;
            } else if l_gamble1 < 701 {
                l_item = 1301;
            } else if l_gamble1 < 801 {
                l_item = 1701;
            } else if l_gamble1 < 851 {
                l_item = 1504;
            } else if l_gamble1 < 901 {
                l_item = 1604;
            } else if l_gamble1 < 911 {
                l_item = 1108;
            } else if l_gamble1 < 921 {
                l_item = 1163;
            } else if l_gamble1 < 961 {
                l_item = 1522;
            } else if l_gamble1 < 971 {
                l_item = 1608;
            } else if l_gamble1 < 981 {
                l_item = 1408;
            } else if l_gamble1 < 991 {
                l_item = 1452;
            } else if l_gamble1 < 1001 {
                l_item = 1208;
            }
        }
        1 => {
            l_gamble1 = ctx.rand_range(1, 500)?;
            if l_gamble1 > 299 && l_gamble1 < 303 {
                l_gamble2 = ctx.rand_range(1, 30)?;
                if l_gamble2 < 3 {
                    l_item = 2315;
                } else if l_gamble2 < 5 {
                    l_item = 2336;
                } else if l_gamble2 < 7 {
                    l_item = 2318;
                } else if l_gamble2 < 9 {
                    l_item = 2326;
                } else if l_gamble2 < 11 {
                    l_item = 2327;
                } else if l_gamble2 < 13 {
                    l_item = 2342;
                } else if l_gamble2 < 15 {
                    l_item = 2331;
                } else if l_gamble2 < 17 {
                    l_item = 2342;
                } else if l_gamble2 < 19 {
                    l_item = 2311;
                } else if l_gamble2 < 21 {
                    l_item = 2320;
                } else if l_gamble2 < 23 {
                    l_item = 2319;
                } else if l_gamble2 < 25 {
                    l_item = 2344;
                } else if l_gamble2 < 27 {
                    l_item = 2346;
                } else if l_gamble2 < 29 {
                    l_item = 2348;
                } else if l_gamble2 < 31 {
                    l_item = 2350;
                }
            } else if l_gamble1 < 51 {
                l_item = 2301;
            } else if l_gamble1 < 101 {
                l_item = 2302;
            } else if l_gamble1 < 151 {
                l_item = 2303;
            } else if l_gamble1 < 201 {
                l_item = 2304;
            } else if l_gamble1 < 251 {
                l_item = 2305;
            } else if l_gamble1 < 300 {
                l_item = 2301;
            } else if l_gamble1 < 351 {
                l_item = 2307;
            } else if l_gamble1 < 401 {
                l_item = 2309;
            } else if l_gamble1 < 402 {
                l_item = 2322;
            } else if l_gamble1 < 403 {
                l_item = 2310;
            } else if l_gamble1 < 411 {
                l_item = 2306;
            } else if l_gamble1 < 416 {
                l_item = 2308;
            } else if l_gamble1 < 421 {
                l_item = 2313;
            } else if l_gamble1 < 426 {
                l_item = 2337;
            } else if l_gamble1 < 431 {
                l_item = 2341;
            } else if l_gamble1 < 436 {
                l_item = 2325;
            } else if l_gamble1 < 441 {
                l_item = 2317;
            } else if l_gamble1 < 446 {
                l_item = 2330;
            } else if l_gamble1 < 451 {
                l_item = 2314;
            } else if l_gamble1 < 456 {
                l_item = 2335;
            } else if l_gamble1 < 461 {
                l_item = 2324;
            } else if l_gamble1 < 466 {
                l_item = 2329;
            } else if l_gamble1 < 471 {
                l_item = 2340;
            } else if l_gamble1 < 476 {
                l_item = 2312;
            } else if l_gamble1 < 481 {
                l_item = 2339;
            } else if l_gamble1 < 486 {
                l_item = 2328;
            } else if l_gamble1 < 491 {
                l_item = 2321;
            } else if l_gamble1 < 501 {
                l_item = 2323;
            }
        }
        2 => {
            l_gamble1 = ctx.rand_range(1, 500)?;
            if l_gamble1 > 200 && l_gamble1 < 204 {
                l_gamble2 = ctx.rand_range(1, 16)?;
                if l_gamble2 < 3 {
                    l_item = 2506;
                } else if l_gamble2 < 5 {
                    l_item = 2504;
                } else if l_gamble2 < 8 {
                    l_item = 2508;
                } else if l_gamble2 < 11 {
                    l_item = 2507;
                } else if l_gamble2 == 11 {
                    l_item = 2513;
                } else if l_gamble2 == 12 {
                    l_item = 2514;
                } else if l_gamble2 == 13 {
                    l_item = 2523;
                } else if l_gamble2 == 14 {
                    l_item = 2530;
                } else if l_gamble2 == 15 {
                    l_item = 2509;
                } else if l_gamble2 == 16 {
                    l_item = 2515;
                }
            } else if l_gamble1 < 101 {
                l_item = 2503;
            } else if l_gamble1 < 201 {
                l_item = 2505;
            } else if l_gamble1 < 451 {
                l_item = 2501;
            } else if l_gamble1 < 501 {
                l_item = 2502;
            }
        }
        3 => {
            l_gamble1 = ctx.rand_range(1, 1000)?;
            if l_gamble1 > 299 && l_gamble1 < 304 {
                l_gamble2 = ctx.rand_range(1, 93)?;
                if l_gamble2 < 3 {
                    l_item = 2251;
                } else if l_gamble2 < 5 {
                    l_item = 2285;
                } else if l_gamble2 < 7 {
                    l_item = 2255;
                } else if l_gamble2 < 9 {
                    l_item = 5045;
                } else if l_gamble2 < 11 {
                    l_item = 2233;
                } else if l_gamble2 < 13 {
                    l_item = 2231;
                } else if l_gamble2 < 15 {
                    l_item = 2217;
                } else if l_gamble2 < 17 {
                    l_item = 2206;
                } else if l_gamble2 < 19 {
                    l_item = 2246;
                } else if l_gamble2 < 21 {
                    l_item = 2261;
                } else if l_gamble2 < 23 {
                    l_item = 2287;
                } else if l_gamble2 < 25 {
                    l_item = 5012;
                } else if l_gamble2 < 27 {
                    l_item = 2244;
                } else if l_gamble2 < 29 {
                    l_item = 2213;
                } else if l_gamble2 < 31 {
                    l_item = 2248;
                } else if l_gamble2 < 33 {
                    l_item = 2223;
                } else if l_gamble2 < 35 {
                    l_item = 2247;
                } else if l_gamble2 < 37 {
                    l_item = 2245;
                } else if l_gamble2 < 39 {
                    l_item = 5003;
                } else if l_gamble2 < 41 {
                    l_item = 2225;
                } else if l_gamble2 < 43 {
                    l_item = 5017;
                } else if l_gamble2 < 45 {
                    l_item = 5030;
                } else if l_gamble2 < 47 {
                    l_item = 5035;
                } else if l_gamble2 < 49 {
                    l_item = 2250;
                } else if l_gamble2 < 51 {
                    l_item = 2277;
                } else if l_gamble2 < 53 {
                    l_item = 5011;
                } else if l_gamble2 < 55 {
                    l_item = 2290;
                } else if l_gamble2 < 57 {
                    l_item = 5010;
                } else if l_gamble2 < 60 {
                    l_item = 2259;
                } else if l_gamble2 < 62 {
                    l_item = 5008;
                } else if l_gamble2 < 63 {
                    l_item = 2249;
                } else if l_gamble2 < 65 {
                    l_item = 2229;
                } else if l_gamble2 == 65 {
                    l_item = 2258;
                } else if l_gamble2 == 66 {
                    l_item = 2274;
                } else if l_gamble2 == 67 {
                    l_item = 5019;
                } else if l_gamble2 == 68 {
                    l_item = 2254;
                } else if l_gamble2 == 69 {
                    l_item = 5007;
                } else if l_gamble2 == 70 {
                    l_item = 5066;
                } else if l_gamble2 == 71 {
                    l_item = 2235;
                } else if l_gamble2 == 72 {
                    l_item = 2234;
                } else if l_gamble2 == 73 {
                    l_item = 2256;
                } else if l_gamble2 == 74 {
                    l_item = 5093;
                } else if l_gamble2 == 75 {
                    l_item = 5072;
                } else if l_gamble2 == 76 {
                    l_item = 5002;
                } else if l_gamble2 < 80 {
                    l_item = 5118;
                } else if l_gamble2 < 83 {
                    l_item = 5120;
                } else if l_gamble2 < 86 {
                    l_item = 5111;
                } else if l_gamble2 < 89 {
                    l_item = 5116;
                } else if l_gamble2 < 92 {
                    l_item = 5119;
                } else if l_gamble2 < 94 {
                    l_item = 5141;
                }
            } else if l_gamble1 < 101 {
                l_item = 2226;
            } else if l_gamble1 < 201 {
                l_item = 2211;
            } else if l_gamble1 < 300 {
                l_item = 2209;
            } else if l_gamble1 < 401 {
                l_item = 2220;
            } else if l_gamble1 < 501 {
                l_item = 2232;
            } else if l_gamble1 < 601 {
                l_item = 2216;
            } else if l_gamble1 < 701 {
                l_item = 2230;
            } else if l_gamble1 < 801 {
                l_item = 2224;
            } else if l_gamble1 < 901 {
                l_item = 2222;
            } else if l_gamble1 < 906 {
                l_item = 2228;
            } else if l_gamble1 < 911 {
                l_item = 2252;
            } else if l_gamble1 < 916 {
                l_item = 2227;
            } else if l_gamble1 < 921 {
                l_item = 2221;
            } else if l_gamble1 < 926 {
                l_item = 2299;
            } else if l_gamble1 < 931 {
                l_item = 2236;
            } else if l_gamble1 < 936 {
                l_item = 2275;
            } else if l_gamble1 < 941 {
                l_item = 5015;
            } else if l_gamble1 < 946 {
                l_item = 2215;
            } else if l_gamble1 < 951 {
                l_item = 5092;
            } else if l_gamble1 < 1001 {
                l_item = 2226;
            }
        }
        4 => {
            l_gamble1 = ctx.rand_range(1, 500)?;
            if l_gamble1 > 299 && l_gamble1 < 303 {
                l_gamble2 = ctx.rand_range(1, 10)?;
                if l_gamble2 < 3 {
                    l_item = 2406;
                } else if l_gamble2 < 5 {
                    l_item = 2412;
                } else if l_gamble2 < 8 {
                    l_item = 2404;
                } else if l_gamble2 < 11 {
                    l_item = 2407;
                }
            } else if l_gamble1 < 201 {
                l_item = 2401;
            } else if l_gamble1 < 300 {
                l_item = 2408;
            } else if l_gamble1 < 351 {
                l_item = 2411;
            } else if l_gamble1 < 401 {
                l_item = 2403;
            } else if l_gamble1 < 451 {
                l_item = 2405;
            } else if l_gamble1 < 476 {
                l_item = 2409;
            } else if l_gamble1 < 501 {
                l_item = 2402;
            }
        }
        5 => {
            l_gamble1 = ctx.rand_range(1, 500)?;
            if l_gamble1 > 200 && l_gamble1 < 205 {
                l_gamble2 = ctx.rand_range(1, 10)?;
                if l_gamble2 < 3 {
                    l_item = 2104;
                } else if l_gamble2 < 5 {
                    l_item = 2106;
                } else if l_gamble2 < 7 {
                    l_item = 2102;
                } else if l_gamble2 < 9 {
                    l_item = 2111;
                } else if l_gamble2 < 11 {
                    l_item = 2109;
                }
            } else if l_gamble1 < 201 {
                l_item = 2101;
            } else if l_gamble1 < 301 {
                l_item = 2103;
            } else if l_gamble1 < 401 {
                l_item = 2107;
            } else if l_gamble1 < 481 {
                l_item = 2105;
            } else if l_gamble1 < 501 {
                l_item = 2108;
            }
        }
        _ => {
            ctx.fx().cutin("katsua01.bmp", 255)?;
            return ctx.close();
        }
    }
    ctx.call(Function::GetItem, args![l_item, 1])?;
    ctx.fx().cutin("katsua01.bmp", 255)?;
    ctx.fx().cutin("katsua02.bmp", 2)?;
    ctx.lines_as(
        "Kachua",
        args!["Ah~ that Diamond is so beautiful.", "I wish I could repay you better."],
    )?;
    ctx.close_window()?;
    ctx.fx().cutin("katsua02.bmp", 255)?;
    return ctx.end();
}

pub fn devellin(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Devellin",
        args![
            "It seems some traveller showed",
            "a huge diamond to Kachua a while ago. Ever since then, all she's been talking about is diamonds and how much she wants them."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Devellin", args!["She's been getting pretty obsessive about it, which scares me. It seems she's more than willing to sacrifice anything she owns for a diamond."])?;
    ctx.next()?;
    ctx.lines_as("Devellin", args!["She's the type of person who'll do anything to get what she wants. I'm worried that she might give away something far more valuable than a diamond in exchange..."])?;
    return ctx.close();
}

pub fn suspicious_guy_cmd(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_THIEF {
        ctx.lines_as(
            "Cain",
            args![
                "Heeeey...",
                "It seems we share the same line of work, you and me. Heh heh, lemme give you a hot tip."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cain", args!["I've been charging other people for this information, but since, shall we say, we work in the same professional field, I don't have the heart to take your zeny."])?;
        ctx.next()?;
        ctx.lines_as(
            "Cain",
            args![
                "You see that lady over there?",
                "She's just totally in love with diamonds. And from what my sources tell me, she's loaded. Tons and tons of valuables."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cain", args!["I'm talkin' rare items.", "I took the liberty of sneaking a peek at what she owns, and saw she's got a helmet with goat horns and even a crown! She's so rich it's ridiculous!"])?;
        ctx.next()?;
        ctx.lines_as("Cain", args!["The buzz that's been going around is that she's got a cache of rare equipment and weapons too! So whaddya say? Wanna be partners in crime and rob her house?"])?;
        ctx.next()?;
        match ctx.menu(&["No, thanks.", "Sweet, I'm in!"])? {
            0 => {
                ctx.lines_as(
                    "Cain",
                    args![
                        "Wha...?",
                        "C'mon! I thought pilfering was something you do! Aw well, I'm gonna do it, but stay hushed on this, got it?"
                    ],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Cain",
                    args!["Ha ha ha!", "...^660000Dork^000000!", "I'm just jivin'", "sp fuggedabout it!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Cain", args!["Ah right. Supposedly, there's a mountain where tons of diamonds are buried. A pal o' mine says there's a mine near the mountain too, so I guess if you went to the mine, you'd find Diamonds."])?;
                ctx.next()?;
                ctx.lines_as("Cain", args!["I guess it can't hurt to gather some Diamonds there and try to exchange them for whatever the old lady's got. Take care, pal~"])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Cain",
        args![
            "Hey...",
            "I got a hot tip for you.",
            "It'll just cost you 500 zeny",
            "and trust me, it's worth it.",
            "So whaddya say...?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Alright.", "No, thanks."])? {
        0 => {
            if ctx.player().zeny()? > 499 {
                ctx.lines_as("Cain", args!["You see that lady over there? She's just totally in love with diamonds. And from what my sources tell me, she's loaded. Tons and tons of valuables."])?;
                ctx.next()?;
                ctx.lines_as("Cain", args!["I'm talkin' rare items.", "I took the liberty of sneaking a peek at what she owns, and saw she's got a helmet with goat horns and even a crown! She's so rich it's ridiculous!"])?;
                ctx.next()?;
                ctx.lines_as("Cain", args!["There's a chance that rare equipment and weapons might be yours! She'll give anything for a 3 carat diamond. So if you have any of those, you might as well see her."])?;
                ctx.next()?;
                ctx.lines_as("Cain", args!["Ah right. Supposedly, there's a mountain where tons of diamonds", "are buried. A pal o' mine says there's a mine near the mountain too, so I guess if you went to the mine, you'd find Diamonds."])?;
                ctx.next()?;
                ctx.lines_as("Cain", args!["I guess it can't hurt to gather some Diamonds there and try to exchange them for whatever the old lady's got. Take care, pal~"])?;
                ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Cain",
                args![
                    "What the hell?!",
                    "Don't you have any money? Didn't I say 500 zeny? Hey man, info like this doesn't come cheap!"
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("Cain", args!["Hey hey!", "What are you, a cheapskate? You understand that everything has its price and this information is so worth it. C'mon, you can't pass this up, can you?"])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn blacksmith_miner(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        return ctx.close();
    }
    ctx.lines_as("Dwayne", args!["Wahahahaha~", "I've dug up a fortune!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Dwayne",
        args!["Diamonds! Hundreds and", "thousands of Diamonds,", "all of them mine!", "I'm rich!"],
    )?;
    ctx.npc().emotion(constants::ET_SMILE)?;
    ctx.next()?;
    match ctx.menu(&["I want to buy some.", "Congratulations."])? {
        0 => {
            ctx.lines_as(
                "Dwayne",
                args![
                    "Ah, you have an",
                    "eye for valuables!",
                    "Sure, sure why not!",
                    "I'll give you a discount, too!",
                    "55,000 Zeny for a diamond,",
                    "how does that sound?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dwayne",
                args![
                    "How many",
                    "diamonds do you need?",
                    "If you change your mind,",
                    "please enter '0' to cancel."
                ],
            )?;
            ctx.next()?;
            let (input, _) = runtime::input_number(ctx, None, None)?;
            let amount = input.number()?;
            if amount == 0 {
                ctx.lines_as("Dwayne", args!["Alright, you've", "canceled the trade.", "Take care!"])?;
                return ctx.close();
            } else if amount < 1 || amount > 500 {
                ctx.lines_as("Dwayne", args!["The maximum", "amount is 500.", "Please enter 500 or less."])?;
                return ctx.close();
            }
            let cost = amount * 55000;
            let weight = amount * 100;
            if ctx.player().zeny()? < cost {
                ctx.lines_as(
                    "Dwayne",
                    args![
                        "Errr...",
                        "I'm sorry, but you",
                        "do not have enough money.",
                        "I'll be losing money if",
                        "I sell them at that price."
                    ],
                )?;
                return ctx.close();
            }
            if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < weight {
                ctx.lines_as("Dwayne", args!["Errr...", "You're carrying too many items.", "I don't think give you anything if there's no room in your inventory. Why don't you put some of your stuff into Kafra Storage?"])?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - cost)?;
            ctx.call(Function::GetItem, args![732, amount])?;
            ctx.lines_as(
                "Dwayne",
                args!["Thank you for", "buying my diamonds!", "You're welcome to", "come back anytime."],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Dwayne",
                args![
                    "Haha, thank you~",
                    "If by any chance",
                    "you need a diamond,",
                    "please drop by.",
                    "I'll sell them to",
                    "you at a cheap price."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
