use inline_python::python;
// use unicode_normalization::UnicodeNormalization;
use dedup_iter::DedupAdapter;
use unic::normal::StrNormalForm;
use unic::ucd::category::GeneralCategory;

fn makereg(terms: &[&str], fname: &str) {
    let mut terms = terms
        .iter()
        .map(|l| {
            l.nfkd()
                .collect::<String>()
                .chars()
                .filter(|c| !GeneralCategory::of(*c).is_mark())
                .dedup()
                .collect::<String>()
                .to_lowercase()
        })
        .collect::<Vec<String>>();
    terms.sort_unstable();
    terms.dedup_by(|a, b| a.contains(b.as_str()));

    python! {
        def fixcompat(r):
            return r.replace("\\ "," ")
        import confusables
        with open('fname, "w") as fh:
            fh.write("|".join(fixcompat(confusables.confusable_regex(l, include_character_padding=True)) for l in 'terms))
    }
}

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=build.rs");

    let lemons = [
        "citrea",
        "Chanh",
        // "Lima",
        "Limon",
        "Limone",
        "Sítrónu",
        "Zitrone",
        "Zitroun",
        "citrina",
        "citroen",
        "citrom",
        "citron",
        "citrono",
        "citrons",
        "citrón",
        "cytrynowy",
        "ilamuna",
        "lemon",
        "lemun",
        // "liin",
        "limau",
        "limoi",
        "limon",
        "limona",
        "limun",
        "limão",
        "limón",
        "linglang",
        "llimona",
        // "lumi",
        "líomóid",
        "lămâie",
        "lẹmọnu",
        "mandimu",
        "oromankịrịsị",
        "rēmana",
        "sidrun",
        "sirilamunu",
        "sitron",
        "sitruuna",
        "sitwon",
        "suurlemoen",
        "txiv qaub",
        "voasarymakirana",
        "λεμόν",
        "лимон",
        "лимун",
        // "лимӯ",
        "цытрына",
        "կիտրոն",
        "לִימוֹן",
        "לימענע",
        "ليمون",
        "لیموشیرین",
        "لیموں",
        "कागति",
        "नींबू",
        "लिंबू",
        "লেবু",
        "લીંબુ",
        "எலுமிச்சை",
        "నిమ్మకాయ",
        "ನಿಂಬೆ",
        "ചെറുനാരങ്ങ",
        "දෙහි",
        "มะนาว",
        "ຫມາກນາວ",
        "သံပယိုသီး",
        "ក្រូចឆ្មា",
        "レモン",
        "れもん",
        "柠檬",
        "檸檬",
        "레몬",
        "🍋",
        "citrvm",
        "citrum",
        "lemom",
        "nomel",
    ];

    makereg(&lemons, "regex_lemon.txt");
    makereg(&["mabbs"], "regex_mabbs.txt");
    makereg(
        &["asher", "stink", "stonk", "stimk", "stomk"],
        "regex_asher.txt",
    );
    makereg(&["lucretius"], "regex_lucretius.txt");
    makereg(&["moth"], "regex_moth.txt");
    makereg(&["lime"], "regex_lime.txt");
}
