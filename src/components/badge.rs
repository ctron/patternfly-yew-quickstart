use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct BadgeExample {}

impl Component for BadgeExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example2! ("Badge" => "badge.1.example");
        let example2 = example2! ("Badge (Read-only)" => "badge.2.example");

        html! {
            <>
                <ExamplePage title="Badge">
                    {example1}
                    {example2}
                </ExamplePage>
            </>
        }
    }
}
