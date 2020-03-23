use druid::{
    widget::{prelude::*, WidgetExt},
    Data, Point, Rect, WidgetPod,
};

pub struct Stack<D> {
    children: Vec<WidgetPod<D, Box<dyn Widget<D>>>>,
}

impl<D: Data> Stack<D> {
    pub fn new() -> Self {
        Stack {
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: impl Widget<D> + 'static) -> Self {
        self.add_child(child);
        self
    }

    pub fn add_child(&mut self, child: impl Widget<D> + 'static) {
        self.children.push(WidgetPod::new(child.boxed()));
    }
}

impl<D: Data> Widget<D> for Stack<D> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        for child in &mut self.children {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        for child in &mut self.children {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &D, data: &D, env: &Env) {
        for child in &mut self.children {
            child.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        let mut largest = Size::ZERO;
        for child in &mut self.children {
            let size = child.layout(ctx, bc, data, env);
            child.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
            if size.width > largest.width {
                largest.width = size.width;
            }
            if size.height > largest.height {
                largest.height = size.height;
            }
        }
        largest
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        for child in &mut self.children {
            child.paint_with_offset(ctx, data, env);
        }
    }
}
