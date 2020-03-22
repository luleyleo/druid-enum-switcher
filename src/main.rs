use druid::{
    widget::{Button, Flex, Label, WidgetExt},
    AppLauncher, Data, Lens, LensExt, Widget, WindowDesc,
};

mod optional;
use optional::Optional;

mod stack;
use stack::Stack;

mod matching;
use matching::TestEnumMatcher;

mod test_enum;
use test_enum::TestEnum;

#[derive(Debug, Clone, Data, Lens)]
struct TestState {
    some_int: u32,
    some_option: Option<u32>,
    #[druid(same_fn = "PartialEq::eq")]
    some_enum: TestEnum,
}

fn build_ui() -> impl Widget<TestState> {
    let button = Button::new("Next State", |_ctx, data: &mut TestState, _env| {
        println!("========= update =========");
        // A normal value
        data.some_int += 1;
        println!("some_int -> {:?}", data.some_int);

        // An optional value
        if data.some_option.is_some() {
            data.some_option = None;
        } else {
            data.some_option = Some(24);
        }
        println!("some_option -> {:?}", data.some_option);

        // An enumeration
        use TestEnum::*;
        data.some_enum = match data.some_enum {
            First(float) => Second(float + 0.5),
            Second(float) => Third(float.round() as u64),
            Third(int) => First(int as f64 / 2.0),
        };
        println!("some_enum -> {:?}", data.some_enum);

        println!();
    });

    let int_label = Label::dynamic(|data: &TestState, _| format!("some_int: {}", data.some_int));

    let optional_label = Optional::new(Label::dynamic(|data: &u32, _env| {
        format!("some_option: {}", data)
    }));

    let optional_second = Optional::new(Label::dynamic(|data: &f64, _env| {
        format!("The second was chosen! {}", data)
    }));


    let first_label = || Label::dynamic(|data: &f64, _env| {
        format!("Your Choice: First with {}", data)
    });
    let second_label = || Label::dynamic(|data: &f64, _env| {
        format!("Your Choice: Second with {}", data)
    });
    let third_label = || Label::dynamic(|data: &u64, _env| {
        format!("Your Choice: Third with {}", data)
    });

    let stack = Stack::new()
        .with_child(Optional::new(first_label()).lens(TestEnum::first))
        .with_child(Optional::new(second_label()).lens(TestEnum::second))
        .with_child(Optional::new(third_label()).lens(TestEnum::third));

    let matching = TestEnumMatcher::new()
        .match_first(first_label())
        .match_second(second_label())
        .match_third(third_label());

    Flex::column()
        .with_child(button)
        .with_spacer(10.0)
        .with_child(int_label)
        .with_child(optional_label.lens(TestState::some_option))
        .with_child(optional_second.lens(TestState::some_enum.then(TestEnum::second)))
        .with_child(stack.lens(TestState::some_enum))
        .with_child(matching.lens(TestState::some_enum))
        .padding(10.0)
}

fn main() {
    let main_window = WindowDesc::new(build_ui);

    let state = TestState {
        some_int: 42,
        some_option: None,
        some_enum: TestEnum::First(4.2),
    };

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}
