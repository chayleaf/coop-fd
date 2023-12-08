use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use rslint_parser::{
    ast::{Expr, ExprOrSpread, LiteralKind, ObjectExpr, ObjectProp, Stmt},
    AstNode,
};

use crate::{digits, Config, Item, Receipt};

#[derive(Clone, Debug)]
enum ZulValue {
    Null,
    Bool(bool),
    Num(f64),
    String(String),
    Nodes(Vec<ZulNode>),
    Values(Vec<ZulValue>),
}

impl ZulValue {
    fn pretty(&self, fmt: &mut std::fmt::Formatter<'_>, ident: usize) -> std::fmt::Result {
        match self {
            Self::Null => fmt.write_str("null"),
            Self::Bool(true) => fmt.write_str("true"),
            Self::Bool(false) => fmt.write_str("false"),
            Self::Num(x) => write!(fmt, "{x}"),
            Self::String(x) => {
                fmt.write_str(&serde_json::to_string(&x).unwrap_or_else(|_| x.clone()))
            }
            Self::Nodes(x) => {
                if x.is_empty() {
                    fmt.write_str("[]")
                } else {
                    fmt.write_str("[\n")?;
                    for expr in x {
                        for _ in 0..ident {
                            fmt.write_str("  ")?;
                        }
                        expr.pretty(fmt, ident)?;
                        fmt.write_str("\n")?;
                    }
                    for _ in 0..ident - 1 {
                        fmt.write_str("  ")?;
                    }
                    fmt.write_str("]")
                }
            }
            Self::Values(x) => {
                if x.is_empty() {
                    fmt.write_str("[]")
                } else {
                    fmt.write_str("[")?;
                    for expr in x {
                        expr.pretty(fmt, ident)?;
                        fmt.write_str(", ")?;
                    }
                    fmt.write_str("]")
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ZulNode {
    tag: Option<String>,
    id: String,
    attrs: HashMap<String, ZulValue>,
    #[allow(unused)]
    idk: HashMap<String, ZulValue>,
    children: Vec<ZulNode>,
}

impl ZulNode {
    fn pretty(&self, fmt: &mut std::fmt::Formatter<'_>, ident: usize) -> std::fmt::Result {
        write!(
            fmt,
            "{} id={}",
            self.tag.as_deref().unwrap_or("root"),
            serde_json::to_string(&self.id).unwrap_or_else(|_| self.id.clone())
        )?;
        for (k, v) in &self.attrs {
            fmt.write_str(" ")?;
            fmt.write_str(k)?;
            fmt.write_str("=")?;
            v.pretty(fmt, ident + 1)?;
        }
        if self.children.is_empty() {
            fmt.write_str(" { }")
        } else {
            fmt.write_str(" {\n")?;
            for child in &self.children {
                for _ in 0..=ident {
                    fmt.write_str("  ")?;
                }
                child.pretty(fmt, ident + 1)?;
                fmt.write_str("\n")?;
            }
            for _ in 0..ident {
                fmt.write_str("  ")?;
            }
            fmt.write_str("}")
        }
    }

    fn get_elements_by_tag_name(&self, name: &str) -> Vec<&Self> {
        let mut ret = vec![];
        if self.tag.as_deref() == Some(name) {
            ret.push(self);
        }
        for attr in self.attrs.values() {
            #[allow(clippy::single_match)]
            match attr {
                ZulValue::Nodes(x) => {
                    for x in x {
                        ret.extend_from_slice(&x.get_elements_by_tag_name(name));
                    }
                }
                _ => {}
            }
        }
        for child in &self.children {
            ret.extend_from_slice(&child.get_elements_by_tag_name(name));
        }
        ret
    }

    fn get_element_by_id(&self, id: &str) -> Option<&Self> {
        if let Some(ZulValue::String(id2)) = self.attrs.get("id") {
            if id == id2 {
                return Some(self);
            }
        }
        for attr in self.attrs.values() {
            #[allow(clippy::single_match)]
            match attr {
                ZulValue::Nodes(x) => {
                    for x in x {
                        if let Some(ret) = x.get_element_by_id(id) {
                            return Some(ret);
                        }
                    }
                }
                _ => {}
            }
        }
        for child in &self.children {
            if let Some(ret) = child.get_element_by_id(id) {
                return Some(ret);
            }
        }
        None
    }
}

impl Display for ZulNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty(f, 0)
    }
}

fn into_val(expr: Expr) -> ZulValue {
    #[allow(clippy::single_match)]
    match &expr {
        Expr::Literal(x) => match x.kind() {
            LiteralKind::Bool(x) => return ZulValue::Bool(x),
            LiteralKind::Number(x) => return ZulValue::Num(x),
            LiteralKind::Null => return ZulValue::Null,
            LiteralKind::String => {
                if let Ok(x) = json5::from_str(&x.text()) {
                    return ZulValue::String(x);
                }
            }
            _ => {}
        },
        _ => {}
    }
    let ret = visit_expr(&expr);
    if ret.is_empty() {
        ZulValue::Values(visit_expr2(expr))
    } else {
        ZulValue::Nodes(ret)
    }
}

fn parse_obj(expr: &ObjectExpr) -> HashMap<String, ZulValue> {
    let mut ret = HashMap::new();
    for prop in expr.props() {
        #[allow(clippy::single_match)]
        match prop {
            ObjectProp::LiteralProp(prop) => {
                let Some(k) = prop.key().and_then(|x| x.as_string()) else {
                    continue;
                };
                let Some(v) = prop.value() else {
                    continue;
                };
                ret.insert(k, into_val(v));
            }
            _ => {}
        }
    }
    ret
}

fn visit_expr2(expr: Expr) -> Vec<ZulValue> {
    match expr {
        Expr::CallExpr(expr) => {
            let mut ret = vec![];
            if let Some(args) = expr.arguments() {
                for x in args.args() {
                    ret.extend_from_slice(&visit_expr2(x));
                }
            }
            ret
        }
        x @ Expr::Literal(_) => vec![into_val(x)],
        _ => vec![],
    }
}

fn visit_expr(expr: &Expr) -> Vec<ZulNode> {
    match expr {
        Expr::ArrayExpr(expr) => {
            let mut arr = expr.elements();
            let Some(ExprOrSpread::Expr(Expr::Literal(tag))) = arr.next() else {
                return vec![];
            };
            let tag = match into_val(Expr::Literal(tag)) {
                ZulValue::String(s) => Some(s),
                _ => None,
            };
            let Some(ExprOrSpread::Expr(Expr::Literal(id))) = arr.next() else {
                return vec![];
            };
            let ZulValue::String(id) = into_val(Expr::Literal(id)) else {
                return vec![];
            };
            let Some(ExprOrSpread::Expr(Expr::ObjectExpr(attrs))) = arr.next() else {
                return vec![];
            };
            let attrs = parse_obj(&attrs);
            let Some(ExprOrSpread::Expr(Expr::ObjectExpr(idk))) = arr.next() else {
                return vec![];
            };
            let idk = parse_obj(&idk);
            let Some(ExprOrSpread::Expr(Expr::ArrayExpr(ch))) = arr.next() else {
                return vec![];
            };
            let mut children = vec![];
            for child in ch.elements() {
                if let ExprOrSpread::Expr(child) = child {
                    children.extend(visit_expr(&child));
                }
            }
            vec![ZulNode {
                tag,
                id,
                attrs,
                idk,
                children,
            }]
        }
        Expr::CallExpr(expr) => expr.arguments().map_or_else(Vec::new, |args| {
            args.args().flat_map(|arg| visit_expr(&arg)).collect()
        }),
        Expr::FnExpr(expr) => expr
            .body()
            .map_or_else(Vec::new, |stmt| visit_stmt(&Stmt::BlockStmt(stmt))),
        _ => vec![],
    }
}

fn visit_stmt(stmt: &Stmt) -> Vec<ZulNode> {
    let mut ret = vec![];
    match stmt {
        Stmt::BlockStmt(stmt) => {
            for stmt in stmt.stmts() {
                ret.extend_from_slice(&visit_stmt(&stmt));
            }
        }
        Stmt::ExprStmt(stmt) => {
            if let Some(expr) = stmt.expr() {
                ret.extend_from_slice(&visit_expr(&expr));
            }
        }
        _ => {}
    }
    ret
}

async fn fetch2(config: &Config, rec: &mut Receipt) -> reqwest::Result<bool> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
        .cookie_store(true)
        .build()?;
    let url = format!(
        "https://check.ofd-magnit.ru/CheckWebApp/fds.zul?fn={}&fs={}&fd={}",
        rec.r#fn, rec.fp, rec.i
    );
    let res = client
        .execute(client.get(url).build()?)
        .await?
        .text()
        .await?;
    let mut path = config.data_path.clone();
    path.push("raw");
    path.push("magnit");
    path.push(rec.fnifp().unwrap_or_default() + ".html");
    if let Err(err) = tokio::fs::write(path, &res).await {
        log::error!("failed to write raw receipt: {err:?}");
    }
    let data = res
        .split("<script class=\"z-runonce\" type=\"text/javascript\">")
        .last()
        .unwrap()
        .split("</script>")
        .next()
        .unwrap();
    let data = rslint_parser::parse_text(data, 0);
    if let Some(tree) = data.try_tree() {
        for item in tree.items() {
            for x in visit_stmt(&item) {
                if let Some(receipt) = x.get_element_by_id("receiptDiv") {
                    if let Some(details) = receipt.get_element_by_id("detailsGrid") {
                        let mut labels = details
                            .get_elements_by_tag_name("zul.wgt.Label")
                            .into_iter()
                            .filter_map(|x| x.attrs.get("value"))
                            .filter_map(|x| {
                                if let ZulValue::String(x) = x {
                                    Some(x.trim().to_owned())
                                } else {
                                    None
                                }
                            })
                            .peekable();
                        while let Some(label) = labels.next() {
                            if matches!(labels.peek(), Some(x) if x.starts_with("ИНН ")) {
                                rec.company.name = label;
                            } else if label.starts_with("ИНН ") {
                                rec.company.inn =
                                    label.split_whitespace().last().unwrap().to_owned();
                            }
                        }
                    }
                    if let Some(items) = receipt.get_element_by_id("recieptItemsGrid") {
                        let items = items
                            .get_elements_by_tag_name("zul.grid.Row")
                            .into_iter()
                            // skip table header
                            .skip(1)
                            .map(|x| {
                                x.get_elements_by_tag_name("zul.wgt.Label")
                                    .into_iter()
                                    .filter_map(|x| x.attrs.get("value"))
                                    .filter_map(|x| {
                                        if let ZulValue::String(x) = x {
                                            Some(x.trim().to_owned())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            });
                        let mut item = None::<Item>;
                        for info in items {
                            if info.len() == 4 {
                                if let Some(item) = item.replace(Item::default()) {
                                    rec.items.push(item);
                                }
                                let item = item.as_mut().unwrap();
                                let mut info = info.into_iter();
                                let Some(name) = info.next() else {
                                    continue;
                                };
                                item.name = name;
                                let Some(count) = info.next() else {
                                    continue;
                                };
                                item.count = count.parse().unwrap_or_default();
                                let Some(per_item) = info.next() else {
                                    continue;
                                };
                                item.per_item = digits(&per_item).parse().unwrap_or_default();
                                let Some(total) = info.next() else {
                                    continue;
                                };
                                item.total = digits(&total).parse().unwrap_or_default();
                            } else if let Some(item) = item.as_mut() {
                                let mut info = info.into_iter();
                                let Some(key) = info.next() else {
                                    continue;
                                };
                                if let Some(tax_percentage) = key
                                    .strip_suffix('%')
                                    .and_then(|key| key.strip_prefix("НДС "))
                                    .and_then(|x| x.parse::<u64>().ok())
                                {
                                    // "total" is already tax added
                                    let total_percentage = 100 + tax_percentage;
                                    item.tax = (item.total * tax_percentage + total_percentage - 1)
                                        / total_percentage;
                                    continue;
                                }
                                let Some(val) = info.next() else {
                                    log::warn!("warning: magnit: no val for item key {key}");
                                    continue;
                                };
                                match key.as_str() {
                                    "РЕЗ. ПРОВ. СВЕД. О ТОВАРЕ"
                                    | "ПРИЗНАК ПР. РАСЧЕТА"
                                    | "СПОСОБ РАСЧЕТА" => {}
                                    "МЕРА КОЛИЧЕСТВА ПР. РАСЧЕТА" => {
                                        item.unit = val;
                                    }
                                    "КОД ТОВАРА" => item.id = val,
                                    key => {
                                        log::warn!("warning: unknown magnit item key: {key}");
                                    }
                                }
                            }
                        }
                        rec.items.extend(item);
                    }
                    if let Some(bottom) = receipt.get_element_by_id("bottomGrid") {
                        for row in bottom.get_elements_by_tag_name("zul.grid.Row") {
                            let mut labels = row
                                .get_elements_by_tag_name("zul.wgt.Label")
                                .into_iter()
                                .filter_map(|x| x.attrs.get("value"))
                                .filter_map(|x| {
                                    if let ZulValue::String(x) = x {
                                        Some(x.trim().to_owned())
                                    } else {
                                        None
                                    }
                                });
                            let Some(key) = labels.next() else {
                                continue;
                            };
                            let Some(val) = labels.next() else {
                                continue;
                            };
                            match key.as_str() {
                                "Итого:" => {
                                    rec.total = digits(&val).parse().unwrap_or_default();
                                }
                                "Наличными:" => {
                                    rec.total_cash = digits(&val).parse().unwrap_or_default();
                                }
                                "Безналичными:" => {
                                    rec.total_card = digits(&val).parse().unwrap_or_default();
                                }
                                x if x.starts_with("НДС ") => {
                                    if let Ok(val) = digits(&val).parse::<u64>() {
                                        rec.total_tax += val;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

pub(crate) async fn fetch(config: &'static Config, mut rec: Receipt) -> Option<Receipt> {
    fetch2(config, &mut rec).await.ok()?.then_some(rec)
}
