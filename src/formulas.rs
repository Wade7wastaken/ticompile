use serde::Deserialize;

struct LabelGenerator(usize);

const LBL_ORDER: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl LabelGenerator {
    fn new() -> Self {
        LabelGenerator(0)
    }
    fn next(&mut self) -> String {
        let mut output = String::new();
        output.push(LBL_ORDER[self.0 / LBL_ORDER.len()]);
        output.push(LBL_ORDER[self.0 % LBL_ORDER.len()]);
        self.0 += 1;
        output
    }
}

struct StringBuilder {
    strs: Vec<String>
}

impl StringBuilder {
    fn new() -> Self {
        StringBuilder { strs: vec![] }
    }

    fn add(&mut self, s: String) {
        self.strs.push(s);
    }

    fn combine(self, s: &str) -> String {
        self.strs.join(s)
    }
}

fn build_item(item_label: Option<String>, item: FormulaItem, gen: &mut LabelGenerator) -> String {
    let mut builder = StringBuilder::new();
    if let Some(lbl) = item_label {
        builder.add(format!("Lbl {lbl}"));
    }
    match item {
        FormulaItem::Group { name, contents } => {
            let mut menu = StringBuilder::new();
            let mut body = StringBuilder::new();
            menu.add(format!("Menu \"{name}\""));
            for next in contents {
                let next_lbl = gen.next();
                menu.add(next.get_name().clone());
                menu.add(next_lbl.clone());

                body.add(build_item(Some(next_lbl), next, gen));
            }
            builder.add(menu.combine(","));
            builder.add(body.combine("\n"));
        }
        FormulaItem::Text { name, lines } => {
            builder.add(format!("Disp \"{name}"));

            for line in lines {
                builder.add(format!("Disp \"{line}"));
            }

            builder.add("Stop".to_string());
        }
    }
    builder.combine("\n")
}

pub fn build_formulas() -> String {
    let json = include_str!("../output.json");
    let data: FormulaItem = serde_json::from_str(json).unwrap();

    let mut gen = LabelGenerator::new();

    build_item(None, data, &mut gen)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FormulaItem {
    Text {
        name: String,
        lines: Vec<String>,
    },
    Group {
        name: String,
        contents: Vec<FormulaItem>,
    },
}

impl FormulaItem {
    fn get_name(&self) -> &String {
        match self {
            Self::Group { name, .. } => name,
            Self::Text { name, .. } => name,
        }
    }
}

// #[derive(Debug, Deserialize)]
// struct GroupItem {
//     name: String,
//     contents: Vec<FormulaItem>
// }

// #[derive(Debug, Deserialize)]
// struct TextItem {
//     name: String,
//     lines: Vec<String>
// }
