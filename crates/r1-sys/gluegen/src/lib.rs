use std::{
    collections::{HashMap, HashSet},
    env,
    io::{BufWriter, Read, Write},
};

use convert_case::Casing;
use msvc_demangler::{
    CallingConv, DemangleFlags, FuncClass, Name, Operator, Params, StorageClass, VarStorageKind,
};

#[derive(Default, Debug)]
struct Type {
    methods: Vec<Function>,
    static_vars: Vec<Var>,
    subtypes: HashMap<String, Type>,
    vtable: VTable,
    vbtables: Vec<(String, String)>,
    /// Set of all allowed classes to be used as generics parameters, expressed as a Vec so multiple parameters can be mapped
    possible_generics: HashSet<Vec<String>>,
}

#[derive(Default, Debug)]
struct Globals {
    functions: Vec<Function>,
    static_vars: Vec<Var>,
}

#[derive(Debug, Default)]
enum VTable {
    #[default]
    None,
    Mono(String),
    Multiple(Vec<(String, String)>),
}

#[derive(Debug)]
struct Function {
    name: String,
    call_conv: msvc_demangler::CallingConv,
    is_static: bool,
    is_virtual: bool,
    params: String,
    rettype: String,
    orig_name: String,
    symbol: String,
}

#[derive(Debug)]
struct Var {
    name: String,
    ty: String,
    is_const: bool,

    symbol: String,
}

pub fn generate_glue(
    mut def_file: std::fs::File,
    lib_name: &str,
    manual_types: &[&str],
    additional_opaque_types: &[&str],
) {
    let mut definitions = String::new();
    def_file.read_to_string(&mut definitions).unwrap();

    let mut types = HashMap::new();
    let mut globals = Globals::default();

    for symbol in definitions.lines().skip(1) {
        if symbol.as_bytes()[0] == b'?' {
            demangle(symbol, &mut types, &mut globals);
        } else {
            println!("{symbol}")
        }
    }

    let mut outfile = BufWriter::new(
        std::fs::File::create(format!("{}/symbols.rs", env::var("OUT_DIR").unwrap())).unwrap(),
    );

    writeln!(outfile, "pub use i8 as char;").unwrap();
    writeln!(outfile, "pub use i32 as long;").unwrap();
    writeln!(outfile, "pub use u32 as ulong;").unwrap();
    writeln!(outfile, "#[link(name = \"{lib_name}\")]").unwrap();
    writeln!(outfile, "#[allow(non_camel_case_types, unused_imports)]").unwrap();
    writeln!(outfile, "unsafe extern \"C\" {{").unwrap();
    globals.write(&mut outfile);
    writeln!(outfile, "}}").unwrap();
    for (name, ty) in types {
        ty.write_links(&name, &mut outfile);
        if !manual_types.contains(&name.as_str()) {
            ty.write_type(&name, &mut outfile);
        }
    }

    for name in additional_opaque_types {
        writeln!(outfile, "#[repr(C)]").unwrap();
        writeln!(outfile, "pub struct {name} {{").unwrap();
        writeln!(outfile, "  _opaque: [u8; 0],").unwrap();
        writeln!(outfile, "}}").unwrap();
    }
}

fn flatten_name(input: &str) -> String {
    input
        .replace(" ", "_")
        .replace("*", "p")
        .replace(",", "")
        .replace(":", "")
}

#[derive(Debug, Clone, Copy)]
enum GenericsHandling {
    AsGeneric,
    Flatten,
    Skip,
}

fn name_cpp_to_rust(name: &Name<'_>, gen_han: GenericsHandling) -> Option<String> {
    Some(match name {
        Name::NonTemplate(symbol_name) => {
            let symbol_name = std::str::from_utf8(symbol_name).unwrap();
            symbol_name.to_string()
        }
        Name::Template(name, params) => {
            let base = name_cpp_to_rust(name, gen_han).unwrap();
            let generics = params_to_rust(None, params, false);
            match gen_han {
                GenericsHandling::Flatten => format!("{base}_{}", flatten_name(&generics)),
                GenericsHandling::AsGeneric => format!("{base}<{generics}>"),
                GenericsHandling::Skip => format!("{base}"),
            }
        }
        Name::Operator(Operator::Ctor) => "op_ctor".to_string(),
        Name::Operator(Operator::New) => "op_new".to_string(),
        Name::Operator(Operator::Call) => "op_call".to_string(),
        Name::Operator(Operator::Equal) => "op_assign".to_string(),
        Name::Operator(Operator::EqualEqual) => "op_equals".to_string(),
        Name::Operator(Operator::Less) => "op_less".to_string(),
        Name::Operator(Operator::LessEqual) => "op_less_eq".to_string(),
        Name::Operator(Operator::Greater) => "op_greater".to_string(),
        Name::Operator(Operator::GreaterEqual) => "op_greater_eq".to_string(),
        Name::Operator(Operator::BangEqual) => "op_not_equals".to_string(),
        Name::Operator(Operator::Dtor) => "op_dtor".to_string(),
        Name::Operator(Operator::Delete) => "op_delete".to_string(),
        Name::Operator(Operator::Subscript) => "op_index".to_string(),
        Name::Operator(Operator::Star) => "op_star".to_string(),
        Name::Operator(Operator::StarEqual) => "op_mul_into".to_string(),
        Name::Operator(Operator::Plus) => "op_add".to_string(),
        Name::Operator(Operator::PlusEqual) => "op_add_into".to_string(),
        Name::Operator(Operator::PlusPlus) => "op_increment".to_string(),
        Name::Operator(Operator::Minus) => "op_sub".to_string(),
        Name::Operator(Operator::MinusEqual) => "op_sub_into".to_string(),
        Name::Operator(Operator::MinusMinus) => "op_decrement".to_string(),
        Name::Operator(Operator::Slash) => "op_div".to_string(),
        Name::Operator(Operator::SlashEqual) => "op_div_into".to_string(),
        Name::Operator(Operator::Amp) => "op_bin_and".to_string(),
        Name::Operator(Operator::AmpEqual) => "op_bin_and_into".to_string(),
        Name::Operator(Operator::Pipe) => "op_bin_or".to_string(),
        Name::Operator(Operator::PipeEqual) => "op_bin_or_into".to_string(),
        Name::Operator(Operator::Caret) => "op_bin_xor".to_string(),
        Name::Operator(Operator::CaretEqual) => "op_bin_xor_into".to_string(),
        Name::Operator(Operator::Tilde) => "op_invert".to_string(),
        Name::Operator(Operator::RShift) => "op_rshift".to_string(),
        Name::Operator(Operator::GreaterGreaterEqual) => "op_rshift_into".to_string(),
        Name::Operator(Operator::LShift) => "op_lshift".to_string(),
        Name::Operator(Operator::LessLessEqual) => "op_lshift_into".to_string(),
        Name::Operator(Operator::DefaultCtorClosure) => "op_ctor_closure".to_string(),
        Name::Operator(Operator::Conversion) => "op_conversion".to_string(),
        Name::Operator(Operator::Arrow) => "op_deref".to_string(),
        Name::Operator(Operator::VFTable) | Name::Operator(Operator::VBTable) => String::new(),
        Name::Discriminator(_) | Name::ParsedName(_) => {
            return None;
        }
        unknown => {
            panic!("Unknown symbol name type: {unknown:?}");
        }
    })
}

fn demangle(symbol: &str, types: &mut HashMap<String, Type>, globals: &mut Globals) {
    let demangled = msvc_demangler::parse(symbol).unwrap();

    println!("{demangled:?}");
    println!(
        "{}",
        msvc_demangler::serialize(&demangled, DemangleFlags::llvm()).unwrap()
    );

    let mut scope_iter = demangled.symbol.scope.names.iter().rev();
    if let Some(class_name) = scope_iter.next() {
        demangle_class_member(symbol, types, &demangled, scope_iter, class_name);
    } else {
        demangle_global(symbol, globals, &demangled);
    }
}

fn demangle_global(
    symbol: &str,
    globals: &mut Globals,
    demangled: &msvc_demangler::ParseResult<'_>,
) {
    let mut symbol_name =
        name_cpp_to_rust(&demangled.symbol.name, GenericsHandling::Flatten).unwrap();

    match &demangled.symbol_type {
        msvc_demangler::Type::NonMemberFunction(conv, params, storage, rettype) => {
            if symbol_name.starts_with("op_") {
                for param in &params.types {
                    symbol_name += "_";
                    symbol_name += &flatten_name(&msvc_type_to_rust_type_name(param));
                }
            }
            globals.functions.push(Function::new(
                &"".to_string(),
                symbol_name,
                &FuncClass::empty(),
                conv,
                params,
                storage,
                symbol,
                rettype,
            ))
        }
        msvc_demangler::Type::Var(ty, var_storage_kind, storage_class) => {
            assert_eq!(var_storage_kind, &VarStorageKind::Global);
            globals.static_vars.push(Var::new(
                symbol_name,
                ty.as_ref(),
                var_storage_kind,
                storage_class,
                symbol.to_string(),
            ));
        }
        unknown => {
            panic!("Unknown symbol type: {unknown:?}");
        }
    }
}

fn demangle_class_member<'a, I: Iterator<Item = &'a Name<'a>>>(
    symbol: &str,
    types: &mut HashMap<String, Type>,
    demangled: &msvc_demangler::ParseResult<'_>,
    scope_iter: I,
    cpp_class_name: &Name<'_>,
) {
    if let Some(class_name) = name_cpp_to_rust(cpp_class_name, GenericsHandling::Skip) {
        let mut class = types
            .entry(class_name.clone())
            .or_insert_with(Default::default);

        let generics_content = match cpp_class_name {
            Name::Template(_, params) => params
                .types
                .iter()
                .map(|gen_param| msvc_type_to_rust_type_name(gen_param))
                .collect(),
            _ => vec![],
        };
        class.possible_generics.insert(generics_content);

        for subclass in scope_iter {
            if let Some(subclass) = name_cpp_to_rust(subclass, GenericsHandling::Skip) {
                class = class
                    .subtypes
                    .entry(subclass)
                    .or_insert_with(Default::default)
            } else {
                println!("Unknown scope entry: {subclass:?}");
                return;
            }
        }

        let symbol_name =
            name_cpp_to_rust(&demangled.symbol.name, GenericsHandling::Flatten).unwrap();

        match &demangled.symbol_type {
            msvc_demangler::Type::MemberFunction(cl, conv, params, storage, rettype) => {
                Function::new(
                    &class_name,
                    symbol_name,
                    cl,
                    conv,
                    params,
                    storage,
                    symbol,
                    rettype,
                );
            }
            msvc_demangler::Type::CXXVFTable(ns, _) => {
                if ns.names.is_empty() && !matches!(&class.vtable, VTable::Multiple(_)) {
                    class.vtable = VTable::Mono(symbol.to_string())
                } else {
                    class.vtable.to_multi().push((
                        ns.names
                            .first()
                            .and_then(|n| name_cpp_to_rust(n, GenericsHandling::AsGeneric))
                            .unwrap_or_default(),
                        symbol.to_string(),
                    ));
                }
            }
            msvc_demangler::Type::CXXVBTable(ns, _) => {
                class.vbtables.push((
                    ns.names
                        .first()
                        .and_then(|n| name_cpp_to_rust(n, GenericsHandling::AsGeneric))
                        .unwrap_or_default(),
                    symbol.to_string(),
                ));
            }
            msvc_demangler::Type::Var(ty, var_storage_kind, storage_class) => {
                assert!(
                    var_storage_kind == &VarStorageKind::PublicStatic
                        || var_storage_kind == &VarStorageKind::ProtectedStatic
                        || var_storage_kind == &VarStorageKind::PrivateStatic
                );
                class.static_vars.push(Var::new(
                    symbol_name,
                    ty,
                    var_storage_kind,
                    storage_class,
                    symbol.to_string(),
                ));
            }
            unknown => {
                panic!("Unknown symbol type: {unknown:?}");
            }
        }
    } else {
        println!("Unknown scope entry: {cpp_class_name:?}");
    }
}

impl Function {
    fn new(
        this: &String,
        name: String,
        cl: &FuncClass,
        conv: &CallingConv,
        params: &Params<'_>,
        storage: &StorageClass,
        symbol: &str,
        rettype: &msvc_demangler::Type<'_>,
    ) -> Self {
        assert!(
            storage
                .clone()
                .difference(StorageClass::PTR64 | StorageClass::CONST)
                .is_empty(),
            "Storage: {storage:?}"
        );
        assert!(
            cl.clone()
                .difference(
                    FuncClass::STATIC
                        | FuncClass::PUBLIC
                        | FuncClass::PROTECTED
                        | FuncClass::PRIVATE
                        | FuncClass::VIRTUAL
                )
                .is_empty(),
            "Class: {cl:?}"
        );
        let params = if storage.contains(StorageClass::PTR64) {
            params_to_rust(
                Some((this, storage.contains(StorageClass::CONST))),
                params,
                true,
            )
        } else {
            params_to_rust(None, params, true)
        };
        Self {
            name: name.clone(),
            call_conv: *conv,
            is_static: cl.contains(FuncClass::STATIC),
            is_virtual: cl.contains(FuncClass::VIRTUAL),
            params,
            rettype: msvc_type_to_rust_type_name(rettype),
            symbol: symbol.to_string(),
            orig_name: name,
        }
    }
    fn write<W: Write>(&self, target: &mut W) {
        writeln!(target, "    /// {}", self.orig_name).unwrap();
        writeln!(target, "    #[link_name = \"\\x01{}\"]", self.symbol).unwrap();
        write!(target, "    pub unsafe fn ").unwrap();
        write!(target, "{}", self.name).unwrap();
        write!(target, "({})", self.params).unwrap();
        writeln!(target, "-> {};", self.rettype).unwrap();
    }
}

impl VTable {
    fn to_multi(&mut self) -> &mut Vec<(String, String)> {
        match self {
            Self::Multiple(v) => v,
            Self::Mono(vt) => {
                *self = Self::Multiple(vec![(String::new(), vt.clone())]);
                self.to_multi()
            }
            Self::None => {
                *self = Self::Multiple(Vec::new());
                self.to_multi()
            }
        }
    }
}

impl Var {
    fn new(
        name: String,
        ty: &msvc_demangler::Type<'_>,
        _var_storage_kind: &VarStorageKind,
        storage_class: &StorageClass,
        symbol: String,
    ) -> Self {
        //assert!(storage_class.difference(StorageClass::CONST).is_empty());

        let is_const = storage_class.contains(StorageClass::CONST);

        Self {
            name,
            is_const,
            ty: msvc_type_to_rust_type_name(ty),
            symbol,
        }
    }
}

fn msvc_type_to_rust_type_name(ty: &msvc_demangler::Type<'_>) -> String {
    match ty {
        msvc_demangler::Type::Ptr(ty, ptrstor)
        | msvc_demangler::Type::Ref(ty, ptrstor)
        | msvc_demangler::Type::RValueRef(ty, ptrstor) => {
            if ptrstor.contains(StorageClass::CONST) {
                format!("*const {}", msvc_type_to_rust_type_name(ty))
            } else {
                format!("*mut {}", msvc_type_to_rust_type_name(ty))
            }
        }
        msvc_demangler::Type::Class(sym, _) | msvc_demangler::Type::Struct(sym, _) => {
            let mut scope: Vec<_> = sym
                .scope
                .names
                .iter()
                .flat_map(|n| name_cpp_to_rust(n, GenericsHandling::AsGeneric))
                .collect();
            scope.push(name_cpp_to_rust(&sym.name, GenericsHandling::AsGeneric).unwrap());
            scope.join("_")
        }
        msvc_demangler::Type::Union(sym, _) | msvc_demangler::Type::Enum(sym, _) => {
            let mut scope: Vec<_> = sym
                .scope
                .names
                .iter()
                .flat_map(|n| name_cpp_to_rust(n, GenericsHandling::AsGeneric))
                .collect();
            scope.push(name_cpp_to_rust(&sym.name, GenericsHandling::AsGeneric).unwrap());
            scope.join("_")
        }
        msvc_demangler::Type::Uchar(_) => "u8".to_string(),
        msvc_demangler::Type::Char(_) => "char".to_string(), // is mapped to i8, but kept separate to avoid collisions
        msvc_demangler::Type::Schar(_) => "i8".to_string(),
        msvc_demangler::Type::Bool(_) => "bool".to_string(),
        msvc_demangler::Type::Short(_) => "i16".to_string(),
        msvc_demangler::Type::Ushort(_) | msvc_demangler::Type::Wchar(_) => "u16".to_string(),
        msvc_demangler::Type::Int(_) => "i32".to_string(),
        msvc_demangler::Type::Long(_) => "long".to_string(), // is mapped to i32, but kept separate to avoid collisions
        msvc_demangler::Type::Uint(_) => "u32".to_string(),
        msvc_demangler::Type::Ulong(_) => "ulong".to_string(), // is mapped to u32, but kept separate to avoid collisions
        msvc_demangler::Type::Float(_) => "f32".to_string(),
        msvc_demangler::Type::Int64(_) => "i64".to_string(),
        msvc_demangler::Type::Uint64(_) => "u64".to_string(),
        msvc_demangler::Type::Double(_) => "f64".to_string(),
        msvc_demangler::Type::Void(_) | msvc_demangler::Type::None => "()".to_string(),
        msvc_demangler::Type::VarArgs => "...".to_string(),
        msvc_demangler::Type::NonMemberFunction(cc, params, storage, rettype) => {
            msvc_function_decl(cc, params, storage, rettype)
        }
        msvc_demangler::Type::Array(len, ty, stor) => {
            let mutability = if stor.contains(StorageClass::CONST) {
                "const"
            } else {
                "mut"
            };
            format!("*{mutability} [{};{len}]", msvc_type_to_rust_type_name(ty))
        }
        _ => panic!("Unknown type name source: {ty:?}"),
    }
}

fn msvc_function_decl(
    cc: &CallingConv,
    params: &Params<'_>,
    _storage: &StorageClass,
    rettype: &msvc_demangler::Type<'_>,
) -> String {
    format!(
        "unsafe {} fn({}) -> {}",
        calling_conv_to_rust(cc),
        params_to_rust(None, params, false),
        msvc_type_to_rust_type_name(rettype)
    )
}

fn params_to_rust(this: Option<(&String, bool)>, params: &Params<'_>, param_names: bool) -> String {
    let mut results = Vec::with_capacity(params.types.len() + this.is_some() as usize);
    if let Some((this, is_const)) = this {
        if is_const {
            results.push(format!("this: *const {this}"));
        } else {
            results.push(format!("this: *mut {this}"));
        }
    }
    if params.types.len() == 1 && matches!(&params.types[0], &msvc_demangler::Type::Void(_)) {
        return results.join(", ");
    }
    for (idx, para) in params.types.iter().enumerate() {
        if para == &msvc_demangler::Type::None {
            break;
        }
        if param_names {
            results.push(format!(
                "{} : {}",
                "abcdefghijklmnopqrstuvwxyz".chars().nth(idx).unwrap(),
                msvc_type_to_rust_type_name(para)
            ));
        } else {
            results.push(msvc_type_to_rust_type_name(para));
        }
    }
    results.join(", ")
}

fn calling_conv_to_rust(cc: &CallingConv) -> String {
    match cc {
        CallingConv::Cdecl => r#"extern "C""#.to_string(),
        CallingConv::Thiscall => r#"extern "thiscall""#.to_string(),
        CallingConv::Stdcall => r#"extern "stdcall""#.to_string(),
        CallingConv::Fastcall => r#"extern "fastcall""#.to_string(),
        CallingConv::Pascal => todo!(),
        CallingConv::_Regcall => todo!(),
    }
}

impl Globals {
    fn write<W: Write>(&self, target: &mut W) {
        for static_var in &self.static_vars {
            writeln!(target, "    #[link_name = \"\\x01{}\"]", static_var.symbol).unwrap();
            write!(target, "    pub static ").unwrap();
            if !static_var.is_const {
                write!(target, "mut ").unwrap();
            }
            write!(
                target,
                "{} : ",
                static_var.name.to_case(convert_case::Case::Constant)
            )
            .unwrap();
            writeln!(target, "{};", static_var.ty).unwrap();
        }
        writeln!(target, "").unwrap();
        for function in &self.functions {
            function.write(target);
        }
    }
}

impl Type {
    fn write_links<W: Write>(&self, name: &str, target: &mut W) {
        let escaped_name = name.replace(['<', '>'], "_");
        writeln!(target, "pub mod {escaped_name}_link {{").unwrap();
        writeln!(target, "    #[allow(unused_imports)]").unwrap();
        writeln!(target, "    use super::*;").unwrap();

        writeln!(target, "    unsafe extern \"C\" {{").unwrap();
        for static_var in &self.static_vars {
            writeln!(
                target,
                "        #[link_name = \"\\x01{}\"]",
                static_var.symbol
            )
            .unwrap();
            writeln!(
                target,
                "        pub unsafe static {} : {};",
                static_var.name, static_var.ty
            )
            .unwrap();
        }
        let mut method_names = HashMap::new();
        for method in &self.methods {
            writeln!(target, "        #[link_name = \"\\x01{}\"]", method.symbol).unwrap();
            if method.is_static {
                writeln!(target, "        /// static").unwrap();
            }
            if method.is_virtual {
                writeln!(target, "        /// virtual").unwrap();
            }
            let callconv = if method.call_conv == CallingConv::Cdecl {
                "".to_string()
            } else {
                format!("{} ", calling_conv_to_rust(&method.call_conv))
            };

            let methodname = match method_names.entry(method.name.clone()) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    *occupied_entry.get_mut() += 1;
                    format!("{}_{}", method.name, *occupied_entry.get())
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                    method.name.clone()
                }
            };
            writeln!(
                target,
                "        pub unsafe {callconv}fn {methodname}({}) -> {};",
                method.params, method.rettype,
            )
            .unwrap();
        }
        writeln!(target, "    }}").unwrap();
        writeln!(target, "}}").unwrap();
    }

    fn write_type<W: Write>(&self, name: &str, target: &mut W) {
        writeln!(target, "#[repr(C)]").unwrap();
        writeln!(target, "pub struct {name} {{").unwrap();
        writeln!(target, "  _opaque: [u8; 0],").unwrap();
        writeln!(target, "}}").unwrap();

        for (subtype, _) in &self.subtypes {
            writeln!(target, "pub struct {name}_{subtype} {{ }}").unwrap();
        }
    }
}
