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
    strs: Vec<String>,
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

macro_rules! quote {
    ($s:expr) => {
        format!(r#""{}""#, $s)
    };
}

fn build_group(
    item_label: String,
    parent_label: Option<String>,
    name: String,
    contents: Vec<FormulaItem>,
    page_number: usize,
    gen: &mut LabelGenerator,
    builder: &mut StringBuilder,
) {
    let mut menu = StringBuilder::new();
    let mut body = StringBuilder::new();

    let menu_name = if page_number == 1 {
        &name
    } else {
        &format!("{name} - page {page_number}")
    };

    menu.add(format!("Menu({}", quote!(menu_name)));
    for next in contents.iter().take(7).cloned() {
        let next_lbl = gen.next();
        menu.add(quote!(next.get_name()).to_string());
        menu.add(next_lbl.clone());

        body.add(build_item(next_lbl, Some(item_label.clone()), next, 1, gen));
    }

    if contents.len() > 8 {
        let rest_lbl = gen.next();
        menu.add(quote!("Next"));
        menu.add(rest_lbl.clone());

        let next_item = FormulaItem::Group {
            name,
            contents: contents[7..].to_vec(),
        };

        body.add(build_item(
            rest_lbl,
            Some(item_label),
            next_item,
            page_number + 1,
            gen,
        ));
    }

    if let Some(par_lbl) = parent_label {
        menu.add(quote!("Back"));
        menu.add(par_lbl.clone());
    }

    builder.add(menu.combine(","));
    builder.add(body.combine("\n"));
}

fn build_item(
    item_label: String,
    parent_label: Option<String>,
    item: FormulaItem,
    page_number: usize,
    gen: &mut LabelGenerator,
) -> String {
    let mut builder = StringBuilder::new();
    builder.add(format!("Lbl {item_label}"));
    match item {
        FormulaItem::Group { name, contents } => {
            build_group(
                item_label,
                parent_label,
                name,
                contents,
                page_number,
                gen,
                &mut builder,
            );
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
    let json = include_str!("./data.json");
    let data: FormulaItem = serde_json::from_str(json).unwrap();

    let mut gen = LabelGenerator::new();

    let first_lbl = gen.next();

    build_item(first_lbl, None, data, 1, &mut gen)
}

#[derive(Debug, Clone, Deserialize)]
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
