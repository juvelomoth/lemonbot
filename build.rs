use inline_python::python;
// use unicode_normalization::UnicodeNormalization;
use unic::ucd::category::GeneralCategory;
use unic::normal::StrNormalForm;

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
    "λεμόνι",
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
    "mabbs"
    ].iter().map(|l| l.nfkd().collect::<String>().chars().filter(|c| !GeneralCategory::of(*c).is_mark()).collect::<String>()).collect::<Vec<String>>();

    python! {
        def fixcompat(r):
            return r.replace("\\ "," ")
        import confusables 
        with open("lemon_regex.txt", "w") as fh: 
            fh.write("|".join(fixcompat(confusables.confusable_regex(l, include_character_padding=True)) for l in 'lemons))
    }

}
