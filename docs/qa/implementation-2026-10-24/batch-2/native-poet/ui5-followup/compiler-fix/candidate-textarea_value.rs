//! Dioxus controlled textarea values at the native DOM/Blitz layout boundary.
use blitz_dom::{BaseDocument, NodeId};

pub(super) fn rehydrate_clone(doc: &mut BaseDocument, cloned: NodeId) {
    // TextInputData::clone starts with an empty editor. Only explicit controlled
    // textarea values are rehydrated; other text editors keep existing behavior.
    doc.iter_subtree_mut(cloned, |id, doc| {
        let node = doc.get_node_mut(id).unwrap();
        let value = node
            .element_data()
            .filter(|el| el.name.local == blitz_dom::local_name!("textarea"))
            .and_then(|el| el.attr(blitz_dom::local_name!("value")))
            .map(str::to_owned);
        if let Some(value) = value {
            seed_editor(node, &value, true);
        }
    });
}
/// Dioxus's controlled `value` is a property, while Blitz initializes a
/// multiline editor from child text during its first layout. Seed that editor
/// before layout; subsequent updates use Blitz's normal value setter. Leave
/// uncontrolled textareas and all other element/editor types unchanged.
pub(super) fn seed_editor(node: &mut blitz_dom::Node, value: &str, cloned: bool) {
    let Some(element) = node.element_data_mut() else {
        return;
    };
    if element.name.local != blitz_dom::local_name!("textarea")
        || (!cloned && element.text_input_data().is_some())
    {
        return;
    }
    let mut input = blitz_dom::node::TextInputData::new(true);
    input.editor.set_text(value);
    element.special_data = blitz_dom::node::SpecialElementData::TextInput(input);
}

#[cfg(test)]
mod tests {
    use crate::{DioxusDocument, DocumentConfig};
    use dioxus::prelude::*;
    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc
    }
    fn editor_text(doc: &DioxusDocument, selector: &str) -> String {
        let inner = doc.inner.borrow();
        let id = inner.query_selector(selector).unwrap().unwrap();
        inner
            .get_node(id)
            .unwrap()
            .element_data()
            .unwrap()
            .text_input_data()
            .expect("textarea editor initialized")
            .editor
            .text()
            .to_string()
    }

    #[test]
    fn controlled_textarea_value_is_ready_before_first_layout_and_survives_layout() {
        fn app() -> Element {
            let value = "initial λ draft\nsecond line".to_string();
            rsx! { textarea { id: "draft", value: "{value}" } }
        }
        let doc = build(app);
        assert_eq!(editor_text(&doc, "#draft"), "initial λ draft\nsecond line");
        doc.inner.borrow_mut().resolve(0.0);
        assert_eq!(editor_text(&doc, "#draft"), "initial λ draft\nsecond line");
        let id = doc
            .inner
            .borrow()
            .query_selector("#draft")
            .unwrap()
            .unwrap();
        super::super::set_attribute_inner(
            &mut doc.inner.borrow_mut().mutate(),
            "value",
            None,
            Some("updated"),
            false,
            id,
        );
        assert_eq!(editor_text(&doc, "#draft"), "updated");
        super::super::set_attribute_inner(
            &mut doc.inner.borrow_mut().mutate(),
            "value",
            None,
            Some(""),
            true,
            id,
        );
        assert_eq!(editor_text(&doc, "#draft"), "");
    }

    #[test]
    fn controlled_empty_value_dominates_children_and_uncontrolled_uses_children() {
        fn app() -> Element {
            rsx! {
                textarea { id: "controlled", value: "", "child fallback" }
                textarea { id: "uncontrolled", "child fallback" }
                input { id: "ordinary", value: "ordinary input" }
            }
        }
        let doc = build(app);
        for selector in ["#uncontrolled", "#ordinary"] {
            let inner = doc.inner.borrow();
            let id = inner.query_selector(selector).unwrap().unwrap();
            assert!(
                inner
                    .get_node(id)
                    .unwrap()
                    .element_data()
                    .unwrap()
                    .text_input_data()
                    .is_none()
            );
        }
        doc.inner.borrow_mut().resolve(0.0);
        assert_eq!(editor_text(&doc, "#controlled"), "");
        assert_eq!(editor_text(&doc, "#uncontrolled"), "child fallback");
    }

    #[test]
    fn controlled_static_and_dynamic_template_clones_retain_values() {
        fn app() -> Element {
            let value = "dynamic draft".to_string();
            rsx! {
                textarea { id: "static", value: "template draft" }
                textarea { id: "dynamic", value: "{value}" }
            }
        }
        let doc = build(app);
        for (selector, expected) in [("#static", "template draft"), ("#dynamic", "dynamic draft")] {
            let id = doc
                .inner
                .borrow()
                .query_selector(selector)
                .unwrap()
                .unwrap();
            let mut inner = doc.inner.borrow_mut();
            let mut counts = [0; 64];
            let mut mounted = Vec::new();
            let mut backend = super::super::BlitzBackend {
                docm: inner.mutate(),
                event_handler_counts: &mut counts,
                queued_mounted_events: &mut mounted,
            };
            let cloned = super::super::RealDom::deep_clone(&mut backend, id);
            drop(backend);
            let text = inner
                .get_node(cloned)
                .unwrap()
                .element_data()
                .unwrap()
                .text_input_data()
                .unwrap()
                .editor
                .text();
            assert_eq!(text, expected);
        }
    }
}
