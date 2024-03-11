use anyhow::{Context, Result};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

/// Global assertion
#[derive(Clone, Default)]
pub struct GA {
    a: A,
    var_to_set: HashMap<String, String>,
    disjs: HashMap<String, HashSet<String>>,
}

/// Assertion
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct A {
    set: String,
    f: F,
}

impl Default for A {
    fn default() -> Self {
        A {
            set: String::new(),
            f: F::Complex {
                constant: String::new(),
                args: vec![],
            },
        }
    }
}

/// Formula
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum F {
    Complex { constant: String, args: Vec<F> },
    Var(String),
}

/// Rule
#[derive(Clone)]
pub struct R {
    ga: GA,
    hyps: Vec<A>,
}

/// Definition
pub struct D {
    constant: String,
    t: T,
}

/// Template
#[derive(Debug, Clone)]
pub enum T {
    Complex { constant: String, args: Vec<T> },
    Var(usize),
}

pub enum Step {
    A {
        i: usize,
        substs: HashMap<String, F>,
    },
    R {
        i: usize,
        substs: HashMap<String, F>,
    },
    D {
        i: usize,
    },
}

#[allow(clippy::implicit_hasher)]
pub fn process(
    context: (&[GA], &[R], &[D]),
    var_to_set: &HashMap<String, String>,
    steps: &[Step],
) -> Result<Vec<A>> {
    let mut proof_stack = vec![];
    let mut disjs = HashMap::new();
    for step in steps {
        match step {
            Step::A { i, substs } => {
                let mut ga = context
                    .0
                    .get(*i)
                    .with_context(|| format!("There isn't {i} in ga context."))?
                    .clone();
                scroll(
                    &mut substs_check(substs, var_to_set, &ga.var_to_set, &mut disjs, &ga.disjs)?,
                    &mut proof_stack,
                )?;
                subst(&mut ga.a.f, substs)?;
                proof_stack.push(ga.a);
            }
            Step::R { i, substs } => {
                let mut r = context
                    .1
                    .get(*i)
                    .with_context(|| format!("There isn't {i} in r context."))?
                    .clone();
                scroll(
                    &mut substs_check(
                        substs,
                        var_to_set,
                        &r.ga.var_to_set,
                        &mut disjs,
                        &r.ga.disjs,
                    )?,
                    &mut proof_stack,
                )?;
                for hyp in &mut r.hyps {
                    subst(&mut hyp.f, substs)?;
                }
                scroll(&mut r.hyps, &mut proof_stack)?;
                subst(&mut r.ga.a.f, substs)?;
                proof_stack.push(r.ga.a);
            }
            Step::D { i } => unfold(
                &mut proof_stack.last_mut().context("Empty proof context.")?.f,
                context
                    .2
                    .get(*i)
                    .with_context(|| format!("There isn't {i} in d context."))?,
            )?,
        }
    }
    Ok(proof_stack)
}

fn unfold(f: &mut F, d: &D) -> Result<()> {
    if let F::Complex { constant, args } = f {
        if constant == &d.constant {
            *f = complete(d.t.clone(), args)?;
        } else {
            for arg in args {
                unfold(arg, d)?;
            }
        }
    }
    Ok(())
}

/// Like ``subst()``, but for ``T``
fn complete(t: T, f_args: &[F]) -> Result<F> {
    Ok(match t {
        T::Complex { constant, args } => F::Complex {
            constant,
            args: args
                .into_iter()
                .map(|x| complete(x, f_args))
                .collect::<Result<Vec<F>>>()?,
        },
        T::Var(i) => f_args
            .get(i)
            .with_context(|| format!("There isn't enough args in {f_args:?} for {t:?}."))?
            .clone(),
    })
}

fn substs_check(
    substs: &HashMap<String, F>,
    g_var_to_set: &HashMap<String, String>,
    var_to_set: &HashMap<String, String>,
    g_disjs: &mut HashMap<String, HashSet<String>>,
    disjs: &HashMap<String, HashSet<String>>,
) -> Result<Vec<A>> {
    let mut hyps = vec![];
    for (var, f) in substs {
        let set = var_to_set
            .get(var)
            .with_context(|| format!("{var} isn't declared."))?
            .to_string();
        let proven = if let F::Var(name) = f {
            g_var_to_set
                .get(name)
                .with_context(|| format!("{name} isn't declared."))?
                == &set
        } else {
            false
        };
        if !proven {
            hyps.push(A { set, f: f.clone() });
        }
        if let Some(var_disjs) = disjs.get(var) {
            for var_disj in var_disjs {
                let mut f_vars = HashSet::new();
                vars(f, &mut f_vars);
                let mut disj_f_vars = HashSet::new();
                vars(
                    substs
                        .get(var_disj)
                        .with_context(|| format!("There isn't {var_disj} in substs."))?,
                    &mut disj_f_vars,
                );
                for f_var in f_vars {
                    for disj_f_var in &disj_f_vars {
                        match f_var.cmp(disj_f_var) {
                            Ordering::Less => match g_disjs.get_mut(&f_var) {
                                Some(r) => {
                                    r.insert(disj_f_var.clone());
                                }
                                None => {
                                    g_disjs
                                        .insert(f_var.clone(), HashSet::from([disj_f_var.clone()]));
                                }
                            },
                            Ordering::Equal => {
                                anyhow::bail!("Disj check failed with {f_var} and {disj_f_var}")
                            }
                            Ordering::Greater => match g_disjs.get_mut(disj_f_var) {
                                Some(r) => {
                                    r.insert(f_var.clone());
                                }
                                None => {
                                    g_disjs
                                        .insert(disj_f_var.clone(), HashSet::from([f_var.clone()]));
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    hyps.sort_unstable();
    Ok(hyps)
}

fn vars(f: &F, acc: &mut HashSet<String>) {
    match f {
        F::Complex { args, .. } => {
            for arg in args {
                vars(arg, acc);
            }
        }
        F::Var(name) => {
            acc.insert(name.clone());
        }
    }
}

fn scroll(hyps: &mut Vec<A>, proof_stack: &mut Vec<A>) -> Result<()> {
    while let Some(hyp) = hyps.pop() {
        let top = proof_stack
            .pop()
            .with_context(|| format!("Proof stack is empty, but not hyps: {hyps:?}."))?;
        anyhow::ensure!(
            hyp == top,
            "Not matching hyp: {hyp:?} and proof stack top: {top:?}."
        );
    }
    Ok(())
}

fn subst(f: &mut F, substs: &HashMap<String, F>) -> Result<()> {
    match f {
        F::Complex { args, .. } => {
            for arg in args {
                subst(arg, substs)?;
            }
        }
        F::Var(name) => {
            *f = substs
                .get(name)
                .with_context(|| format!("There isn't {name} in substs."))?
                .clone();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
