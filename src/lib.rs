use std::collections::HashMap;

use anyhow::ensure;

/// Assertion
#[derive(Clone, PartialEq, Debug)]
pub struct A {
    set: String,
    f: F,
}

/// Formula
#[derive(Clone, PartialEq, Debug)]
pub enum F {
    Complex { constant: String, args: Vec<F> },
    Var { name: String, set: String },
}

pub enum Step {
    A(usize),
    R(usize),
}

/// Rule
#[derive(Clone)]
pub struct R {
    pub hyps: Vec<A>,
    pub head: A,
}

#[must_use]
pub fn is_proof(context: (&[A], &[R]), a: &A, steps: &[Step]) -> bool {
    let proof_stack = process(context, steps);
    proof_stack.len() == 1 && proof_stack[0] == *a
}

// TODO: subst step (add substs to A and R steps)

#[must_use]
pub fn process(context: (&[A], &[R]), steps: &[Step]) -> Vec<A> {
    let mut proof_stack = vec![];
    for step in steps {
        match step {
            Step::A(i) => proof_stack.push(context.0[*i].clone()),
            Step::R(i) => {
                let mut r = context.1[*i].clone();
                let mut substs = HashMap::new();
                while let Some(mut hyp) = r.hyps.pop() {
                    println!("subst hyp {:?} with {substs:?}", hyp.f);
                    subst(&substs, &mut hyp.f);
                    println!("the hyp {:?} after subst", hyp.f);
                    let a = proof_stack.pop().unwrap();
                    println!("take {a:?} from proof stack and unify");
                    let mut u = unify(hyp.f, a.f, &mut substs).unwrap();
                    r.hyps.append(&mut u);
                    println!("post hyps: {:?}", r.hyps);
                    println!("post substs: {substs:?}");
                }
                subst(&substs, &mut r.head.f);
                proof_stack.push(r.head);
            }
        }
    }
    proof_stack
}

fn subst(substs: &HashMap<String, F>, f: &mut F) {
    match f {
        F::Complex { args, .. } => {
            for arg in args {
                subst(substs, arg);
            }
        }
        F::Var { name, .. } => {
            if let Some(subst_f) = substs.get(name) {
                *f = subst_f.clone();
            }
        }
    }
}

fn unify(hyp: F, a: F, substs: &mut HashMap<String, F>) -> anyhow::Result<Vec<A>> {
    Ok(match hyp {
        F::Complex {
            constant: x_constant,
            args: x_args,
        } => match a {
            F::Complex {
                constant: y_constant,
                args: y_args,
            } => {
                ensure!(x_constant == y_constant);
                ensure!(x_args.len() == y_args.len());
                let mut hyps = vec![];
                x_args.into_iter().zip(y_args).for_each(|(x, y)| {
                    let mut u = unify(x, y, substs).unwrap();
                    hyps.append(&mut u);
                });
                hyps
            }
            F::Var { set: y_set, .. } => vec![A {
                set: y_set,
                f: F::Complex {
                    constant: x_constant,
                    args: x_args,
                },
            }],
        },
        F::Var {
            name: x_name,
            set: x_set,
        } => match a {
            F::Complex {
                constant: y_constant,
                args: y_args,
            } => {
                substs.insert(
                    x_name,
                    F::Complex {
                        constant: y_constant.clone(),
                        args: y_args.clone(),
                    },
                );
                vec![A {
                    set: x_set,
                    f: F::Complex {
                        constant: y_constant,
                        args: y_args,
                    },
                }]
            }
            F::Var {
                name: y_name,
                set: y_set,
            } => {
                ensure!(x_set == y_set);
                substs.insert(
                    x_name,
                    F::Var {
                        name: y_name,
                        set: y_set,
                    },
                );
                vec![]
            }
        },
    })
}

#[cfg(test)]
mod tests;
