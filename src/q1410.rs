/*
    1410 - HTML Entity Parser
    Time: O(n)
    Space: O(n)
*/
pub fn entity_parser(text: String) -> String {
    let entities = [
        ("&quot;", r#"""#),
        ("&apos;", "'"),
        ("&gt;", ">"),
        ("&lt;", "<"),
        ("&frasl;", "/"),
        ("&amp;", "&"),
    ];
    entities
        .into_iter()
        .fold(text, |text, (entity, rep)| {
            text.replace(entity, rep)
        })
}
