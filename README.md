# Display all the enums

This repo contains possible ways to display enums in druid using a data driven style.

# Implemented Ideas

See [main.rs] as an example

(Implemented is relative, I did not implement any proc macros)

## Using `Stack` + `Optional`

This idea adds two new widgets which probably should be available anyways and uses those two to solve this problem.

Requirements:
- `Stack` widget
- `Optional` widget
- Deriving `Lens` for enum

The `Stack` widget displays its children on top of each other

The `Optional` widget basically maps `Widget<Option<T>>` to `Widget<T>`

Lenses for enums `SomeEnum::any` map for example `SomeEnum::Any(u32)` to `Option<u32>`

This allows displaying any enum as
```rust
#[derive(Data, Lens)]
enum Test { A(u16), B(u32), C(u16) }

Stack::new()
    .with_child(Optional::new(WidgetForU16::new()).lens(Test::a))
    .with_child(WidgetForU32::new().optional().lens(Test::b))
    .with_child(WidgetForOtherU16::new().optional().lens(Test::c));
```

## A `SomeEnumMatcher` widget

Requirements:
- A proc macro that can generate all that boiler plate code in [matching.rs]

This would basically generate a widget to 'match' over an enums variants and assign some representation widget to each of them.

This allows displaying any enum as
```rust
#[derive[Data, Matchable]]
enum Test { A(u16), B(u32), C(u16) }

TestMatcher::new()
    .match_a(WidgetForU16::new())
    .match_b(WidgetForU32::new())
    .match_c(WidgetForOtherU16::new());
```

## The unholy `Immediate` widget

Requirements:
- `Immediate` widget

This blasphemous widget allows writing immediate-mode style code in druid. Thats it.

This allows displaying any enum as
```rust
#[derive[Data, Matchable]]
enum Test { A(u16), B(u32), C(u16) }

Immediate::new(|data: &A| {
    match data {
        Test::A(value) => Label::new(format!("A = {}", value)),
        Test::B(value) => widget_for_b(value),
        Test::C(value) => widget_for_c(value),
    }
});
```

# Unimplemented ideas

None, right now


[main.rs]: https://github.com/Finnerale/druid-enum-switcher/blob/master/src/main.rs
[matching.rs]: https://github.com/Finnerale/druid-enum-switcher/blob/master/src/matching.rs