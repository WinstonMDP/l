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
        ga: GA {
            a: A {
                set: "p".to_string(),
                f: F::Var("y".to_string()),
            },
            var_to_set: HashMap::from([
                ("x".to_string(), "wff".to_string()),
                ("y".to_string(), "wff".to_string()),
            ]),
            disjs: HashMap::new(),
        },
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
                &[],
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
                &[mp()],
                &[]
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
                        var_to_set: HashMap::from([("q".to_string(), "wff".to_string())]),
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
                        var_to_set: HashMap::from([
                            ("x".to_string(), "wff".to_string()),
                            ("y".to_string(), "wff".to_string())
                        ]),
                        ..Default::default()
                    },
                ],
                &[mp()],
                &[]
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
                var_to_set: HashMap::from([
                    ("x".to_string(), "wff".to_string()),
                    ("y".to_string(), "wff".to_string()),
                ]),
                disjs: HashMap::from([("x".to_string(), HashSet::from(["y".to_string()]))]),
            }],
            &[],
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
                var_to_set: HashMap::from([
                    ("x".to_string(), "wff".to_string()),
                    ("y".to_string(), "wff".to_string()),
                ]),
                disjs: HashMap::from([("x".to_string(), HashSet::from(["y".to_string()]))]),
            }],
            &[],
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

#[test]
fn process_t_6() {
    assert_eq!(
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
                    var_to_set: HashMap::from([
                        ("x".to_string(), "wff".to_string()),
                        ("y".to_string(), "wff".to_string()),
                    ]),
                    disjs: HashMap::new(),
                }],
                &[],
                &[D {
                    constant: "imp".to_string(),
                    t: T::Complex {
                        constant: "or".to_string(),
                        args: vec![
                            T::Complex {
                                constant: "not".to_string(),
                                args: vec![T::Var(0)],
                            },
                            T::Var(1),
                        ],
                    },
                }],
            ),
            &HashMap::from([
                ("x".to_string(), "wff".to_string()),
                ("y".to_string(), "wff".to_string())
            ]),
            &[
                Step::A {
                    i: 0,
                    substs: HashMap::from([
                        ("x".to_string(), F::Var("x".to_string())),
                        ("y".to_string(), F::Var("y".to_string())),
                    ]),
                },
                Step::D { i: 0 },
            ],
        )
        .unwrap(),
        vec![A {
            set: "p".to_string(),
            f: F::Complex {
                constant: "or".to_string(),
                args: vec![
                    F::Complex {
                        constant: "not".to_string(),
                        args: vec![F::Var("x".to_string())],
                    },
                    F::Var("y".to_string()),
                ]
            }
        }]
    );
}

#[test]
fn process_t_7() {
    assert_eq!(
        process(
            (
                &[
                    GA {
                        a: A {
                            set: "p".to_string(),
                            f: F::Complex {
                                constant: "imp".to_string(),
                                args: vec![F::Var("x".to_string()), F::Var("y".to_string())],
                            },
                        },
                        var_to_set: HashMap::from([
                            ("x".to_string(), "wff".to_string()),
                            ("y".to_string(), "wff".to_string()),
                            ("z".to_string(), "wff".to_string()),
                        ]),
                        disjs: HashMap::from([("x".to_string(), HashSet::from(["z".to_string()]))]),
                    },
                    GA {
                        a: A {
                            set: "wff".to_string(),
                            f: F::Complex {
                                constant: "T".to_string(),
                                args: vec![F::Var("x".to_string())],
                            }
                        },
                        var_to_set: HashMap::from([("x".to_string(), "wff".to_string())]),
                        disjs: HashMap::new()
                    }
                ],
                &[],
                &[],
            ),
            &HashMap::from([
                ("x".to_string(), "wff".to_string()),
                ("z".to_string(), "wff".to_string()),
            ]),
            &[
                Step::A {
                    i: 1,
                    substs: HashMap::from([("x".to_string(), F::Var("x".to_string()))]),
                },
                Step::A {
                    i: 1,
                    substs: HashMap::from([("x".to_string(), F::Var("x".to_string()))]),
                },
                Step::A {
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
                        ("z".to_string(), F::Var("z".to_string()))
                    ]),
                },
            ],
        )
        .unwrap(),
        vec![A {
            set: "p".to_string(),
            f: F::Complex {
                constant: "imp".to_string(),
                args: vec![
                    F::Complex {
                        constant: "T".to_string(),
                        args: vec![F::Var("x".to_string())],
                    },
                    F::Complex {
                        constant: "T".to_string(),
                        args: vec![F::Var("x".to_string())],
                    }
                ]
            }
        }]
    );
}

#[test]
fn process_t_8() {
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
                var_to_set: HashMap::from([
                    ("x".to_string(), "wff".to_string()),
                    ("y".to_string(), "wff".to_string()),
                    ("z".to_string(), "wff".to_string()),
                ]),
                disjs: HashMap::from([("x".to_string(), HashSet::from(["z".to_string()]))]),
            }],
            &[],
            &[],
        ),
        &HashMap::from([
            ("x".to_string(), "wff".to_string()),
            ("z".to_string(), "wff".to_string()),
        ]),
        &[Step::A {
            i: 0,
            substs: HashMap::from([
                ("x".to_string(), F::Var("x".to_string())),
                ("y".to_string(), F::Var("x".to_string())),
                ("z".to_string(), F::Var("z".to_string())),
            ]),
        }],
    )
    .unwrap();
}
