use std::collections::{HashMap, HashSet};

use anyhow::{ensure, Context, Result};

/// Global asertion
#[derive(Clone, PartialEq, Debug)]
pub struct GA {
    a: A,
    vars: HashMap<String, String>, // name -> set
    disjs: HashMap<String, HashSet<String>>,
}

impl Default for GA {
    fn default() -> Self {
        GA {
            a: A {
                set: String::new(),
                f: F::Complex {
                    constant: String::new(),
                    args: vec![],
                },
            },
            vars: HashMap::new(),
            disjs: HashMap::new(),
        }
    }
}

/// Assretion
#[derive(Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub struct A {
    set: String,
    f: F,
}

/// Formula
#[derive(Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum F {
    Complex { constant: String, args: Vec<F> },
    Var(String),
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
}

/// Rule
#[derive(Clone)]
pub struct R {
    pub hyps: Vec<A>,
    pub head: A,
    vars: HashMap<String, String>, // name -> set
    disjs: HashMap<String, HashSet<String>>,
}

pub fn process(
    context: (&[GA], &[R]),
    vars_map: &HashMap<String, String>,
    steps: &[Step],
) -> Result<Vec<A>> {
    let mut proof_stack = vec![];
    let mut disjs: HashMap<String, HashSet<String>> = HashMap::new();
    for step in steps {
        match step {
            Step::A { i, substs } => {
                let mut ga = context.0[*i].clone();
                scroll(
                    &mut disj_check(vars_map, &ga.vars, &ga.disjs, substs, &mut disjs)?,
                    &mut proof_stack,
                )?;
                subst(&mut ga.a.f, substs)?;
                proof_stack.push(ga.a);
            }
            Step::R { i, substs } => {
                let mut r = context.1[*i].clone();
                scroll(
                    &mut disj_check(vars_map, &r.vars, &r.disjs, substs, &mut disjs)?,
                    &mut proof_stack,
                )?;
                println!("substs: {substs:?}");
                for hyp in &mut r.hyps {
                    subst(&mut hyp.f, substs)?;
                }
                scroll(&mut r.hyps, &mut proof_stack)?;
                subst(&mut r.head.f, substs)?;
                proof_stack.push(r.head);
            }
        }
    }
    Ok(proof_stack)
}

// Order of result
fn disj_check(
    g_vars_map: &HashMap<String, String>,
    vars_map: &HashMap<String, String>,
    from_disjs: &HashMap<String, HashSet<String>>,
    substs: &HashMap<String, F>,
    to_disjs: &mut HashMap<String, HashSet<String>>,
) -> Result<Vec<A>> {
    let mut hyps = vec![];
    for (name, f) in substs {
        let set = vars_map.get(name).unwrap().to_string();
        let proven = if let F::Var(name) = f {
            *g_vars_map.get(name).unwrap() == set
        } else {
            false
        };
        if !proven {
            hyps.push(A { set, f: f.clone() });
        }
        if let Some(disjs) = from_disjs.get(name) {
            for disj in disjs {
                let disj_f = substs.get(disj).unwrap();
                for f_var in vars(f) {
                    for disj_f_var in vars(disj_f) {
                        match f_var.cmp(&disj_f_var) {
                            std::cmp::Ordering::Less => {
                                to_disjs.get_mut(&f_var).unwrap().insert(disj_f_var);
                            }
                            std::cmp::Ordering::Equal => anyhow::bail!(""),
                            std::cmp::Ordering::Greater => {
                                to_disjs.get_mut(&disj_f_var).unwrap().insert(f_var.clone());
                            }
                        };
                    }
                }
            }
        }
    }
    hyps.sort_unstable();
    Ok(hyps)
}

fn vars(f: &F) -> HashSet<String> {
    match f {
        F::Complex { args, .. } => {
            let mut acc = HashSet::new();
            for arg in args {
                acc.extend(vars(arg));
            }
            acc
        }
        F::Var(name) => HashSet::from([name.clone()]),
    }
}

fn scroll(hyps: &mut Vec<A>, proof_stack: &mut Vec<A>) -> Result<()> {
    println!("hyps: {hyps:?}");
    println!("proof_stack: {proof_stack:?}");
    while let Some(hyp) = hyps.pop() {
        ensure!(hyp == proof_stack.pop().context("")?);
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
            let subst_f = substs.get(name).unwrap();
            *f = subst_f.clone();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
