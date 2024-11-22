use std::fmt::Display;

use symbol_table::GlobalSymbol;

pub type Id = u32;

#[derive(Debug, Clone)]
pub struct ENode {
    pub symbol: GlobalSymbol,
    pub children: Vec<Id>,
}

impl ENode {
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct RExpr {
    pub root: Id,
    pub enodes: Vec<ENode>,
}

impl Display for RExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_sexpr(
            f: &mut std::fmt::Formatter<'_>,
            enodes: &Vec<ENode>,
            id: Id,
        ) -> std::fmt::Result {
            let enode = &enodes[id as usize];
            if enode.is_leaf() {
                return write!(f, "{}", enode.symbol);
            }
            write!(f, "({}", enode.symbol)?;
            for child in &enode.children {
                write!(f, " ")?;
                fmt_sexpr(f, enodes, *child)?;
            }
            write!(f, ")")
        }
        fmt_sexpr(f, &self.enodes, self.root)
    }
}

pub struct RExprBuilder {
    enodes: Vec<ENode>,
}

impl RExprBuilder {
    pub fn new() -> Self {
        RExprBuilder { enodes: Vec::new() }
    }

    pub fn insert(&mut self, symbol: GlobalSymbol, children: Vec<Id>) -> Id {
        let id = self.enodes.len() as Id;
        self.enodes.push(ENode { symbol, children });
        id
    }

    pub fn build(self, root: Id) -> RExpr {
        RExpr {
            root,
            enodes: self.enodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rexpr() {
        let mut builder = RExprBuilder::new();
        let a = builder.insert(GlobalSymbol::new("a"), Vec::new());
        let b = builder.insert(GlobalSymbol::new("b"), vec![a]);
        let c = builder.insert(GlobalSymbol::new("c"), vec![a, b]);
        let d = builder.insert(GlobalSymbol::new("d"), vec![c]);

        let r = builder.build(d);
        assert_eq!(format!("{}", r), "(d (c a (b a)))");
    }
}
