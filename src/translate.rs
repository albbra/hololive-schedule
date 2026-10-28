pub fn translate(name: &str) -> String {
    let out = match name {
        // ── Hololive JP ──────────────────────────────────────────────
        "ときのそら"                     => "Tokino Sora",
        "ロボ子さん"                     => "Roboko-san",
        "さくらみこ"                     => "Sakura Miko",
        "星街すいせい"                   => "Hoshimachi Suisei",
        "夜空メル"                       => "Yozora Mel",
        "白上フブキ"                     => "Shirakami Fubuki",
        "夏色まつり"                     => "Natsuiro Matsuri",
        "赤井 はあと"                    => "Akai Haato",
        "アキ・ローゼンタール" | "アキロゼ" => "Aki Rosenthal",
        "湊あくあ"                       => "Minato Aqua",
        "紫咲シオン"                     => "Murasaki Shion",
        "百鬼あやめ"                     => "Nakiri Ayame",
        "癒月ちょこ"                     => "Yuzuki Choco",
        "大空スバル"                     => "Oozora Subaru",
        "大神ミオ"                       => "Ookami Mio",
        "猫又おかゆ"                     => "Nekomata Okayu",
        "戌神ころね"                     => "Inugami Korone",
        "兎田ぺこら"                     => "Usada Pekora",
        "不知火フレア"                   => "Shiranui Flare",
        "白銀ノエル"                     => "Shirogane Noel",
        "宝鐘マリン"                     => "Houshou Marine",
        "天音かなた"                     => "Amane Kanata",
        "角巻わため"                     => "Tsunomaki Watame",
        "常闇トワ"                       => "Tokoyami Towa",
        "姫森ルーナ"                     => "Himemori Luna",
        "雪花ラミィ"                     => "Yukihana Lamy",
        "桃鈴ねね"                       => "Momosuzu Nene",
        "獅白ぼたん"                     => "Shishiro Botan",
        "尾丸ポルカ"                     => "Omaru Polka",
        "ラプラス・ダークネス"           => "La+ Darknesss",
        "鷹嶺ルイ"                       => "Takane Lui",
        "博衣こより"                     => "Hakui Koyori",
        "沙花叉クロヱ"                   => "Sakamata Chloe",
        "風真いろは"                     => "Kazama Iroha",
        "ホロライブ"                     => "Hololive",

        // ── Hololive DEV_IS — ReGLOSS ────────────────────────────────
        "火威青"                         => "Hiodoshi Ao",
        "音乃瀬奏"                       => "Otonose Kanade",
        "一条莉々華"                     => "Ichijou Ririka",
        "儒烏風亭らでん"                 => "Juufuutei Raden",
        "轟はじめ"                       => "Todoroki Hajime",

        // ── Hololive DEV_IS — FLOW GLOW ──────────────────────────────
        "響咲リオナ"                     => "Isaki Riona",
        "虎金妃笑虎"                     => "Koganei Niko",
        "水宮枢"                         => "Mizumiya Su",
        "輪堂千速"                       => "Rindo Chihaya",
        "綺々羅々ヴィヴィ"               => "Kikirara Vivi",

        // ── Hololive EN — Myth ───────────────────────────────────────
        "Mori Calliope" | "Calliope" | "Calli"
                                         => "Mori Calliope",
        "Takanashi Kiara" | "Kiara"      => "Takanashi Kiara",
        "Ninomae Ina'nis" | "Ina'nis" | "Ina"
                                         => "Ninomae Ina'nis",
        "Gawr Gura" | "Gura"             => "Gawr Gura",
        "Watson Amelia" | "Amelia" | "Ame"
                                         => "Watson Amelia",

        // ── Hololive EN — Promise ────────────────────────────────────
        "IRyS"                           => "IRyS",
        "Ceres Fauna" | "Fauna"          => "Ceres Fauna",
        "Ouro Kronii" | "Kronii"         => "Ouro Kronii",
        "Nanashi Mumei" | "Mumei"        => "Nanashi Mumei",
        "Hakos Baelz" | "Baelz" | "Bae"  => "Hakos Baelz",

        // ── Hololive EN — Advent ─────────────────────────────────────
        "Shiori Novella" | "Shiori"      => "Shiori Novella",
        "Koseki Bijou" | "Bijou"         => "Koseki Bijou",
        "Nerissa Ravencroft" | "Nerissa" => "Nerissa Ravencroft",
        "Fuwawa Abyssgard" | "Fuwawa"    => "Fuwawa Abyssgard",
        "Mococo Abyssgard" | "Mococo"    => "Mococo Abyssgard",

        // ── Hololive EN — Justice ────────────────────────────────────
        "Elizabeth Rose Bloodflame" | "Elizabeth" | "Liz"
                                         => "Elizabeth Rose Bloodflame",
        "Gigi Murin" | "Gigi"            => "Gigi Murin",
        "Cecilia Immergreen" | "Cecilia" => "Cecilia Immergreen",
        "Raora Panthera" | "Raora"       => "Raora Panthera",

        // ── Hololive ID — Area 15 ────────────────────────────────────
        "Ayunda Risu" | "Risu"           => "Ayunda Risu",
        "Moona Hoshinova" | "Moona"      => "Moona Hoshinova",
        "Airani Iofifteen" | "Iofifteen" | "Iofi"
                                         => "Airani Iofifteen",

        // ── Hololive ID — HoloH3ro ───────────────────────────────────
        "Kureiji Ollie" | "Ollie"        => "Kureiji Ollie",
        "Anya Melfissa" | "Anya"         => "Anya Melfissa",
        "Pavolia Reine" | "Reine"        => "Pavolia Reine",

        // ── Hololive ID — HoloH3ro (gen 2) ───────────────────────────
        "Kobo Kanaeru" | "Kobo"          => "Kobo Kanaeru",
        "Kaela Kovalskia" | "Kaela"      => "Kaela Kovalskia",
        "Vestia Zeta" | "Zeta"           => "Vestia Zeta",

        // ── HOLOSTARS JP ─────────────────────────────────────────────
        "花咲みやび"                     => "Hanasaki Miyabi",
        "奏手イヅル"                     => "Kanade Izuru",
        "アルランディス"                 => "Arurandeisu",
        "律可"                           => "Rikka",
        "アステル・レダ"                 => "Astel Leda",
        "岸堂天真"                       => "Kishido Temma",
        "夕刻ロベル"                     => "Yukoku Roberu",
        "影山シエン"                     => "Kageyama Shien",
        "荒咬オウガ"                     => "Aragami Oga",
        "夜十神封魔"                     => "Yatogami Fuma",
        "羽継烏有"                       => "Utsugi Uyu",
        "緋崎ガンマ"                     => "Hizaki Gamma",
        "水無世燐央"                     => "Minase Rio",

        // ── HOLOSTARS EN — Tempus HQ ─────────────────────────────────
        "Regis Altare" | "Altare"        => "Regis Altare",
        "Magni Dezmond" | "Magni"        => "Magni Dezmond",
        "Axel Syrios" | "Axel"           => "Axel Syrios",
        "Noir Vesper" | "Vesper"         => "Noir Vesper",

        // ── HOLOSTARS EN — Tempus Vanguard ───────────────────────────
        "Gavis Bettel" | "Bettel"        => "Gavis Bettel",
        "Machina X Flayon" | "Flayon"    => "Machina X Flayon",
        "Banzoin Hakka" | "Hakka"        => "Banzoin Hakka",
        "Josuiji Shinri" | "Shinri"      => "Josuiji Shinri",

        // ── HOLOSTARS EN — Armis ─────────────────────────────────────
        "Jurard T Rexford" | "Jurard"    => "Jurard T Rexford",
        "Goldbullet"                     => "Goldbullet",
        "Octavio"                        => "Octavio",
        "Crimzon Ruze" | "Ruze"          => "Crimzon Ruze",

        _ => return name.to_string(),
    };
    out.to_string()
}