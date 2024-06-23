use crate::PlRExpr;
use polars::lazy::dsl;
use savvy::{savvy, Result};

#[savvy]
pub fn when(condition: PlRExpr) -> Result<PlRWhen> {
    Ok(PlRWhen {
        inner: dsl::when(condition.inner),
    })
}

#[savvy]
#[derive(Clone)]
pub struct PlRWhen {
    inner: dsl::When,
}

#[savvy]
#[derive(Clone)]
pub struct PlRThen {
    inner: dsl::Then,
}

#[savvy]
#[derive(Clone)]
pub struct PlRChainedWhen {
    inner: dsl::ChainedWhen,
}

#[savvy]
#[derive(Clone)]
pub struct PlRChainedThen {
    inner: dsl::ChainedThen,
}

#[savvy]
impl PlRWhen {
    fn then(&self, statement: PlRExpr) -> Result<PlRThen> {
        Ok(PlRThen {
            inner: self.inner.clone().then(statement.inner),
        })
    }
}

#[savvy]
impl PlRThen {
    fn when(&self, condition: PlRExpr) -> Result<PlRChainedWhen> {
        Ok(PlRChainedWhen {
            inner: self.inner.clone().when(condition.inner),
        })
    }

    fn otherwise(&self, statement: PlRExpr) -> Result<PlRExpr> {
        Ok(self.inner.clone().otherwise(statement.inner).into())
    }
}

#[savvy]
impl PlRChainedWhen {
    fn then(&self, statement: PlRExpr) -> Result<PlRChainedThen> {
        Ok(PlRChainedThen {
            inner: self.inner.clone().then(statement.inner),
        })
    }
}

#[savvy]
impl PlRChainedThen {
    fn when(&self, condition: PlRExpr) -> Result<PlRChainedWhen> {
        Ok(PlRChainedWhen {
            inner: self.inner.clone().when(condition.inner),
        })
    }

    fn otherwise(&self, statement: PlRExpr) -> Result<PlRExpr> {
        Ok(self.inner.clone().otherwise(statement.inner).into())
    }
}
