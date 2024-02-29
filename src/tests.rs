use super::*;

// a: (x : wff) \in p
// F \in p > T \in p

#[test]
fn unify_t_1() {
    let mut substs = HashMap::new();
    assert_eq!(
        unify(
            F::Complex {
                constant: "T".to_string(),
                args: vec![],
            },
            F::Complex {
                constant: "T".to_string(),
                args: vec![],
            },
            &mut substs,
        )
        .unwrap(),
        vec![]
    );
    assert_eq!(substs, HashMap::new());
}

#[test]
fn unify_t_2() {
    let mut substs = HashMap::new();
    assert_eq!(
        unify(
            F::Var {
                name: "x".to_string(),
                set: "wff".to_string(),
            },
            F::Complex {
                constant: "T".to_string(),
                args: vec![],
            },
            &mut substs,
        )
        .unwrap(),
        vec![A {
            set: "wff".to_string(),
            f: F::Complex {
                constant: "T".to_string(),
                args: vec![]
            }
        }]
    );
    assert_eq!(
        substs,
        HashMap::from([(
            "x".to_string(),
            F::Complex {
                constant: "T".to_string(),
                args: vec![]
            }
        )])
    );
}

#[test]
fn unify_t_3() {
    let mut substs = HashMap::new();
    assert_eq!(
        unify(
            F::Var {
                name: "x".to_string(),
                set: "wff".to_string(),
            },
            F::Var {
                name: "y".to_string(),
                set: "wff".to_string()
            },
            &mut substs,
        )
        .unwrap(),
        vec![]
    );
    assert_eq!(
        substs,
        HashMap::from([(
            "x".to_string(),
            F::Var {
                name: "y".to_string(),
                set: "wff".to_string()
            }
        )])
    );
}

#[test]
fn subst_t_1() {
    let mut f = F::Var {
        name: "x".to_string(),
        set: "wff".to_string(),
    };
    subst(
        &HashMap::from([(
            "x".to_string(),
            F::Complex {
                constant: "T".to_string(),
                args: vec![],
            },
        )]),
        &mut f,
    );
    assert_eq!(
        f,
        F::Complex {
            constant: "T".to_string(),
            args: vec![],
        }
    );
}

fn mp() -> R {
    R {
        hyps: vec![
            A {
                set: "p".to_string(),
                f: F::Var {
                    name: "x".to_string(),
                    set: "wff".to_string(),
                },
            },
            A {
                set: "p".to_string(),
                f: F::Complex {
                    constant: "imp".to_string(),
                    args: vec![
                        F::Var {
                            name: "x".to_string(),
                            set: "wff".to_string(),
                        },
                        F::Var {
                            name: "y".to_string(),
                            set: "wff".to_string(),
                        },
                    ],
                },
            },
        ],
        head: A {
            set: "p".to_string(),
            f: F::Var {
                name: "y".to_string(),
                set: "p".to_string(),
            },
        },
    }
}

#[test]
fn process_t_1() {
    assert_eq!(
        process(
            (
                &[A {
                    set: "p".to_string(),
                    f: F::Complex {
                        constant: "T".to_string(),
                        args: vec![]
                    }
                }],
                &[]
            ),
            &[Step::A(0)]
        ),
        vec![A {
            set: "p".to_string(),
            f: F::Complex {
                constant: "T".to_string(),
                args: vec![]
            }
        }]
    );
}

#[test]
fn process_t_2() {
    assert_eq!(
        process(
            (
                &[
                    A {
                        set: "p".to_string(),
                        f: F::Complex {
                            constant: "F".to_string(),
                            args: vec![]
                        }
                    },
                    A {
                        set: "p".to_string(),
                        f: F::Complex {
                            constant: "imp".to_string(),
                            args: vec![
                                F::Complex {
                                    constant: "F".to_string(),
                                    args: vec![]
                                },
                                F::Complex {
                                    constant: "T".to_string(),
                                    args: vec![]
                                }
                            ]
                        }
                    },
                    A {
                        set: "wff".to_string(),
                        f: F::Complex {
                            constant: "F".to_string(),
                            args: vec![]
                        }
                    },
                    A {
                        set: "wff".to_string(),
                        f: F::Complex {
                            constant: "T".to_string(),
                            args: vec![]
                        }
                    }
                ],
                &[mp()]
            ),
            &[Step::A(0), Step::A(2), Step::A(3), Step::A(1), Step::R(0)]
        ),
        vec![A {
            set: "p".to_string(),
            f: F::Complex {
                constant: "T".to_string(),
                args: vec![]
            }
        }]
    );
}

#[test]
fn process_t_3() {
    assert_eq!(
        process(
            (
                &[
                    A {
                        set: "p".to_string(),
                        f: F::Var {
                            name: "z".to_string(),
                            set: "wff".to_string(),
                        },
                    },
                    A {
                        set: "p".to_string(),
                        f: F::Complex {
                            constant: "imp".to_string(),
                            args: vec![
                                F::Var {
                                    name: "z".to_string(),
                                    set: "wff".to_string(),
                                },
                                F::Var {
                                    name: "w".to_string(),
                                    set: "wff".to_string(),
                                },
                            ],
                        },
                    },
                ],
                &[mp()]
            ),
            &[Step::A(0), Step::A(1), Step::R(0)]
        ),
        vec![A {
            set: "p".to_string(),
            f: F::Var {
                name: "w".to_string(),
                set: "wff".to_string()
            }
        }],
    );
}

#[test]
fn process_t_4() {
    assert_eq!(
        process(
            (
                &[
                    A {
                        set: "p".to_string(),
                        f: F::Var {
                            name: "x".to_string(),
                            set: "wff".to_string(),
                        },
                    },
                    A {
                        set: "p".to_string(),
                        f: F::Complex {
                            constant: "imp".to_string(),
                            args: vec![
                                F::Var {
                                    name: "x".to_string(),
                                    set: "wff".to_string(),
                                },
                                F::Var {
                                    name: "y".to_string(),
                                    set: "wff".to_string(),
                                },
                            ],
                        },
                    },
                ],
                &[mp()]
            ),
            &[Step::A(0), Step::A(1), Step::R(0)]
        ),
        vec![A {
            set: "p".to_string(),
            f: F::Var {
                name: "y".to_string(),
                set: "wff".to_string()
            }
        }],
    );
}
