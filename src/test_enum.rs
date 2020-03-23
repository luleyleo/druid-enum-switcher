use druid::Data;

#[derive(Data, Debug, Clone, PartialEq)]
pub enum TestEnum {
    First(f64),
    Second(f64),
    Third(u64),
}

impl TestEnum {
    #[allow(non_upper_case_globals)]
    pub const first: lenses::TestEnumLenseFirst = lenses::TestEnumLenseFirst;

    #[allow(non_upper_case_globals)]
    pub const second: lenses::TestEnumLenseSecond = lenses::TestEnumLenseSecond;

    #[allow(non_upper_case_globals)]
    pub const third: lenses::TestEnumLenseThird = lenses::TestEnumLenseThird;
}

mod lenses {
    use super::TestEnum;
    use druid::{Data, Lens};

    pub struct TestEnumLenseFirst;
    pub struct TestEnumLenseSecond;
    pub struct TestEnumLenseThird;

    impl Lens<TestEnum, Option<f64>> for TestEnumLenseFirst {
        fn with<V, F: FnOnce(&Option<f64>) -> V>(&self, data: &TestEnum, f: F) -> V {
            match data {
                TestEnum::First(value) => f(&Some(*value)),
                _ => f(&None),
            }
        }

        fn with_mut<V, F: FnOnce(&mut Option<f64>) -> V>(&self, data: &mut TestEnum, f: F) -> V {
            match data {
                TestEnum::First(value) => {
                    let mut opt = Some(*value);
                    let opt2 = opt;
                    let res = f(&mut opt);
                    if !opt.same(&opt2) {
                        match opt {
                            Some(val) => *value = val,
                            None => todo!("Decide how to handle Some -> None"),
                        }
                    }
                    res
                }
                _ => {
                    let mut opt = None;
                    let res = f(&mut opt);
                    if let Some(val) = opt {
                        *data = TestEnum::First(val);
                    }
                    res
                }
            }
        }
    }

    impl Lens<TestEnum, Option<f64>> for TestEnumLenseSecond {
        fn with<V, F: FnOnce(&Option<f64>) -> V>(&self, data: &TestEnum, f: F) -> V {
            match data {
                TestEnum::Second(value) => f(&Some(*value)),
                _ => f(&None),
            }
        }

        fn with_mut<V, F: FnOnce(&mut Option<f64>) -> V>(&self, data: &mut TestEnum, f: F) -> V {
            match data {
                TestEnum::Second(value) => {
                    let mut opt = Some(*value);
                    let opt2 = opt;
                    let res = f(&mut opt);
                    if !opt.same(&opt2) {
                        match opt {
                            Some(val) => *value = val,
                            None => todo!("Decide how to handle Some -> None"),
                        }
                    }
                    res
                }
                _ => {
                    let mut opt = None;
                    let res = f(&mut opt);
                    if let Some(val) = opt {
                        *data = TestEnum::Second(val);
                    }
                    res
                }
            }
        }
    }

    impl Lens<TestEnum, Option<u64>> for TestEnumLenseThird {
        fn with<V, F: FnOnce(&Option<u64>) -> V>(&self, data: &TestEnum, f: F) -> V {
            match data {
                TestEnum::Third(value) => f(&Some(*value)),
                _ => f(&None),
            }
        }

        fn with_mut<V, F: FnOnce(&mut Option<u64>) -> V>(&self, data: &mut TestEnum, f: F) -> V {
            match data {
                TestEnum::Third(value) => {
                    let mut opt = Some(*value);
                    let opt2 = opt;
                    let res = f(&mut opt);
                    if !opt.same(&opt2) {
                        match opt {
                            Some(val) => *value = val,
                            None => todo!("Decide how to handle Some -> None"),
                        }
                    }
                    res
                }
                _ => {
                    let mut opt = None;
                    let res = f(&mut opt);
                    if let Some(val) = opt {
                        *data = TestEnum::Third(val);
                    }
                    res
                }
            }
        }
    }
}
