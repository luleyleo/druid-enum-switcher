use crate::test_enum::TestEnum;
use druid::{widget::prelude::*, WidgetPod};

pub struct TestEnumMatcher {
    content_first: Option<WidgetPod<f64, Box<dyn Widget<f64>>>>,
    first_added: bool,
    content_second: Option<WidgetPod<f64, Box<dyn Widget<f64>>>>,
    second_added: bool,
    content_third: Option<WidgetPod<u64, Box<dyn Widget<u64>>>>,
    third_added: bool,
}

impl TestEnumMatcher {
    pub fn new() -> Self {
        TestEnumMatcher {
            content_first: None,
            first_added: false,
            content_second: None,
            second_added: false,
            content_third: None,
            third_added: false,
        }
    }

    pub fn match_first(mut self, content_first: impl Widget<f64> + 'static) -> Self {
        self.content_first = Some(WidgetPod::new(Box::new(content_first)));
        self
    }

    pub fn match_second(mut self, content_second: impl Widget<f64> + 'static) -> Self {
        self.content_second = Some(WidgetPod::new(Box::new(content_second)));
        self
    }

    pub fn match_third(mut self, content_third: impl Widget<u64> + 'static) -> Self {
        self.content_third = Some(WidgetPod::new(Box::new(content_third)));
        self
    }
}

impl Widget<TestEnum> for TestEnumMatcher {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TestEnum, env: &Env) {
        match data {
            TestEnum::First(value) => {
                if let Some(content) = &mut self.content_first {
                    content.event(ctx, event, value, env);
                }
            }
            TestEnum::Second(value) => {
                if let Some(content) = &mut self.content_second {
                    content.event(ctx, event, value, env);
                }
            }
            TestEnum::Third(value) => {
                if let Some(content) = &mut self.content_third {
                    content.event(ctx, event, value, env);
                }
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &TestEnum, env: &Env) {
        match data {
            TestEnum::First(value) => {
                if let Some(content) = &mut self.content_first {
                    content.lifecycle(ctx, event, value, env);
                    self.first_added = true;
                }
            }
            TestEnum::Second(value) => {
                if let Some(content) = &mut self.content_second {
                    content.lifecycle(ctx, event, value, env);
                    self.second_added = true;
                }
            }
            TestEnum::Third(value) => {
                if let Some(content) = &mut self.content_third {
                    content.lifecycle(ctx, event, value, env);
                    self.third_added = true;
                }
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &TestEnum, data: &TestEnum, env: &Env) {
        match data {
            TestEnum::First(_) if !self.first_added => ctx.children_changed(),
            TestEnum::Second(_) if !self.second_added => ctx.children_changed(),
            TestEnum::Third(_) if !self.third_added => ctx.children_changed(),

            TestEnum::First(value) => {
                if let Some(content) = &mut self.content_first {
                    content.update(ctx, value, env);
                }
            }
            TestEnum::Second(value) => {
                if let Some(content) = &mut self.content_second {
                    content.update(ctx, value, env);
                }
            }
            TestEnum::Third(value) => {
                if let Some(content) = &mut self.content_third {
                    content.update(ctx, value, env);
                }
            }
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TestEnum,
        env: &Env,
    ) -> Size {
        match data {
            TestEnum::First(value) => {
                if let Some(content) = &mut self.content_first {
                    content.layout(ctx, bc, value, env)
                } else {
                    Size::ZERO
                }
            }
            TestEnum::Second(value) => {
                if let Some(content) = &mut self.content_second {
                    content.layout(ctx, bc, value, env)
                } else {
                    Size::ZERO
                }
            }
            TestEnum::Third(value) => {
                if let Some(content) = &mut self.content_third {
                    content.layout(ctx, bc, value, env)
                } else {
                    Size::ZERO
                }
            }
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &TestEnum, env: &Env) {
        match data {
            TestEnum::First(value) => {
                if let Some(content) = &mut self.content_first {
                    content.paint(ctx, value, env);
                }
            }
            TestEnum::Second(value) => {
                if let Some(content) = &mut self.content_second {
                    content.paint(ctx, value, env);
                }
            }
            TestEnum::Third(value) => {
                if let Some(content) = &mut self.content_third {
                    content.paint(ctx, value, env);
                }
            }
        }
    }
}
