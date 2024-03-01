use super::*;

fn mp() -> R {
    R {
        hyps: vec![
            A {
                set: "p".to_string(),
                f: F::Var("x".to_string()),
            },
            A {
                set: "p".to_string(),
                f: F::Complex {
                    constant: "imp".to_string(),
                    args: vec![F::Var("x".to_string()), F::Var("y".to_string())],
                },
            },
        ],
        head: A {
            set: "p".to_string(),
            f: F::Var("y".to_string()),
        },
        vars: HashMap::from([
            ("x".to_string(), "wff".to_string()),
            ("y".to_string(), "wff".to_string()),
        ]),
        disjs: HashMap::new(),
    }
}

#[test]
fn process_t_1() {
    assert_eq!(
        process(
            (
                &[GA {
                    a: A {
                        set: "p".to_string(),
                        f: F::Complex {
                            constant: "T".to_string(),
                            args: vec![]
                        }
                    },
                    ..Default::default()
                }],
                &[]
            ),
            &HashMap::new(),
            &[Step::A {
                i: 0,
                substs: HashMap::new()
            }]
        )
        .unwrap(),
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
                    GA {
                        a: A {
                            set: "p".to_string(),
                            f: F::Complex {
                                constant: "F".to_string(),
                                args: vec![]
                            }
                        },
                        ..Default::default()
                    },
                    GA {
                        a: A {
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
                        ..Default::default()
                    },
                    GA {
                        a: A {
                            set: "wff".to_string(),
                            f: F::Complex {
                                constant: "F".to_string(),
                                args: vec![]
                            }
                        },
                        ..Default::default()
                    },
                    GA {
                        a: A {
                            set: "wff".to_string(),
                            f: F::Complex {
                                constant: "T".to_string(),
                                args: vec![]
                            }
                        },
                        ..Default::default()
                    }
                ],
                &[mp()]
            ),
            &HashMap::new(),
            &[
                Step::A {
                    i: 0,
                    substs: HashMap::new()
                },
                Step::A {
                    i: 1,
                    substs: HashMap::new()
                },
                Step::A {
                    i: 2,
                    substs: HashMap::new()
                },
                Step::A {
                    i: 3,
                    substs: HashMap::new()
                },
                Step::R {
                    i: 0,
                    substs: HashMap::from([
                        (
                            "x".to_string(),
                            F::Complex {
                                constant: "F".to_string(),
                                args: vec![]
                            }
                        ),
                        (
                            "y".to_string(),
                            F::Complex {
                                constant: "T".to_string(),
                                args: vec![]
                            }
                        )
                    ])
                }
            ]
        )
        .unwrap(),
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
                    GA {
                        a: A {
                            set: "p".to_string(),
                            f: F::Var("q".to_string()),
                        },
                        vars: HashMap::from([("q".to_string(), "wff".to_string())]),
                        ..Default::default()
                    },
                    GA {
                        a: A {
                            set: "p".to_string(),
                            f: F::Complex {
                                constant: "imp".to_string(),
                                args: vec![F::Var("x".to_string()), F::Var("y".to_string()),],
                            },
                        },
                        vars: HashMap::from([
                            ("x".to_string(), "wff".to_string()),
                            ("y".to_string(), "wff".to_string())
                        ]),
                        ..Default::default()
                    },
                ],
                &[mp()]
            ),
            &HashMap::from([
                ("z".to_string(), "wff".to_string()),
                ("w".to_string(), "wff".to_string())
            ]),
            &[
                Step::A {
                    i: 0,
                    substs: HashMap::from([("q".to_string(), F::Var("z".to_string()))])
                },
                Step::A {
                    i: 1,
                    substs: HashMap::from([
                        ("x".to_string(), F::Var("z".to_string())),
                        ("y".to_string(), F::Var("w".to_string()))
                    ])
                },
                Step::R {
                    i: 0,
                    substs: HashMap::from([
                        ("x".to_string(), F::Var("z".to_string())),
                        ("y".to_string(), F::Var("w".to_string()))
                    ])
                }
            ]
        )
        .unwrap(),
        vec![A {
            set: "p".to_string(),
            f: F::Var("w".to_string())
        }],
    );
}

#[test]
fn process_t_4() {
    process(
        (
            &[GA {
                a: A {
                    set: "p".to_string(),
                    f: F::Complex {
                        constant: "imp".to_string(),
                        args: vec![F::Var("x".to_string()), F::Var("y".to_string())],
                    },
                },
                vars: HashMap::from([
                    ("x".to_string(), "wff".to_string()),
                    ("y".to_string(), "wff".to_string()),
                ]),
                disjs: HashMap::from([("x".to_string(), HashSet::from(["y".to_string()]))]),
            }],
            &[],
        ),
        &HashMap::from([("x".to_string(), "wff".to_string())]),
        &[Step::A {
            i: 0,
            substs: HashMap::from([
                ("x".to_string(), F::Var("x".to_string())),
                ("y".to_string(), F::Var("x".to_string())),
            ]),
        }],
    )
    .unwrap_err();
}

#[test]
fn process_t_5() {
    process(
        (
            &[GA {
                a: A {
                    set: "p".to_string(),
                    f: F::Complex {
                        constant: "imp".to_string(),
                        args: vec![F::Var("x".to_string()), F::Var("y".to_string())],
                    },
                },
                vars: HashMap::from([
                    ("x".to_string(), "wff".to_string()),
                    ("y".to_string(), "wff".to_string()),
                ]),
                disjs: HashMap::from([("x".to_string(), HashSet::from(["y".to_string()]))]),
            }],
            &[],
        ),
        &HashMap::from([("x".to_string(), "wff".to_string())]),
        &[Step::A {
            i: 0,
            substs: HashMap::from([
                (
                    "x".to_string(),
                    F::Complex {
                        constant: "T".to_string(),
                        args: vec![F::Var("x".to_string())],
                    },
                ),
                (
                    "y".to_string(),
                    F::Complex {
                        constant: "T".to_string(),
                        args: vec![F::Var("x".to_string())],
                    },
                ),
            ]),
        }],
    )
    .unwrap_err();
}
