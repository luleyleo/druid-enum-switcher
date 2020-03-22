use druid::{widget::prelude::*, Data, WidgetPod};

pub struct Optional<D> {
    some: WidgetPod<D, Box<dyn Widget<D>>>,
    initialized: bool,
}

impl<D: Data> Optional<D> {
    pub fn new(content: impl Widget<D> + 'static) -> Self {
        Optional {
            some: WidgetPod::new(Box::new(content)),
            initialized: false,
        }
    }
}

impl<D: Data> Widget<Option<D>> for Optional<D> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Option<D>, env: &Env) {
        if let Some(ref mut data) = data {
            self.some.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Option<D>,
        env: &Env,
    ) {
        if let Some(ref data) = data {
            self.some.lifecycle(ctx, event, data, env);
            self.initialized = true;
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Option<D>, data: &Option<D>, env: &Env) {
        match (old_data, data) {
            (None, Some(data)) => {
                if !self.initialized {
                    ctx.children_changed();
                } else {
                    self.some.update(ctx, data, env);
                    ctx.request_layout();
                }
            }
            (Some(_), Some(data)) => self.some.update(ctx, data, env),
            (Some(_), None) => ctx.request_layout(),
            (None, None) => (),
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Option<D>,
        env: &Env,
    ) -> Size {
        if let Some(ref data) = data {
            self.some.layout(ctx, bc, data, env)
        } else {
            Size::ZERO
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Option<D>, env: &Env) {
        if let Some(ref data) = data {
            self.some.paint(ctx, data, env);
        }
    }
}
