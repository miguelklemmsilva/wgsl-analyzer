use std::fmt::Write;

use super::ImportValue;
use crate::{
    db::DefDatabase,
    module_data::{ModuleInfo, ModuleItem},
};

pub fn pretty_print_module(
    db: &dyn DefDatabase,
    module: &ModuleInfo,
) -> String {
    let mut f = String::new();

    for item in module.items() {
        write_pretty_module_item(item, module, &mut f, db);
        f.push_str(";\n");
    }

    f
}

pub fn pretty_module_item(
    item: &ModuleItem,
    module: &ModuleInfo,
    db: &dyn DefDatabase,
) -> String {
    let mut f = String::new();
    write_pretty_module_item(item, module, &mut f, db);
    f
}

fn write_pretty_module_item(
    item: &ModuleItem,
    module: &ModuleInfo,
    f: &mut String,
    db: &dyn DefDatabase,
) {
    match *item {
        ModuleItem::Function(id) => {
            let function = &module.data[id.index];

            let _ = write!(f, "fn {}(", function.name.0);
            for parameter in function.parameters.clone().map(|index| &module.data[index]) {
                let r#type = db.lookup_intern_type_ref(parameter.r#type);
                let _ = write!(f, "{}, ", &r#type);
            }
            trim_in_place(f, ", ");
            let _ = write!(f, ")");
        },
        ModuleItem::Struct(id) => {
            let r#struct = &module.data[id.index];
            let _ = writeln!(f, "struct {} {{", r#struct.name.0);
            for field in r#struct.fields.clone() {
                let field = &module.data[field];
                let r#type = db.lookup_intern_type_ref(field.r#type);
                let _ = writeln!(f, "    {}: {};", field.name.0, r#type);
            }
            let _ = write!(f, "}}");
        },
        ModuleItem::GlobalVariable(var) => {
            let var = &module.data[var.index];
            let r#type = var.r#type.map(|r#type| db.lookup_intern_type_ref(r#type));
            let _ = write!(f, "var {}", &var.name.0);
            if let Some(r#type) = r#type {
                let _ = write!(f, ": {type}");
            }
        },
        ModuleItem::GlobalConstant(var) => {
            let constant = &module.data[var.index];
            let r#type = constant
                .r#type
                .map(|r#type| db.lookup_intern_type_ref(r#type));
            let _ = write!(f, "let {}", &constant.name.0);
            if let Some(r#type) = r#type {
                let _ = write!(f, ": {type}");
            }
        },
        ModuleItem::Override(var) => {
            let override_decl = &module.data[var.index];
            let r#type = override_decl
                .r#type
                .map(|r#type| db.lookup_intern_type_ref(r#type));
            let _ = write!(f, "override {}", &override_decl.name.0);
            if let Some(r#type) = r#type {
                let _ = write!(f, ": {type}");
            }
        },
        ModuleItem::Import(import) => {
            let import = &module.data[import.index];
            let _ = match &import.value {
                ImportValue::Path(path) => write!(f, "#import \"{path}\""),
                ImportValue::Custom(key) => write!(f, "#import {key}"),
            };
        },
        ModuleItem::TypeAlias(type_alias) => {
            let type_alias = &module.data[type_alias.index];
            let name = &type_alias.name.0;
            let r#type = db.lookup_intern_type_ref(type_alias.r#type);
            let _ = write!(f, "type {name} = {type};");
        },
    }
}

fn trim_in_place(
    s: &mut String,
    pat: &str,
) {
    let new_length = s.trim_end_matches(pat).len();
    s.truncate(new_length);
}
