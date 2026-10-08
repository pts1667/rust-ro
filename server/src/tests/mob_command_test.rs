use crate::server::request_handler::atcommand_extra;

#[test]
fn mobsearch_reports_the_monsters_of_the_caller_map_and_rejects_unknown_names() {
    let (context, _repository, character) = super::native_payment_tests::fixture(false, false);
    let char_id = character.char_id;
    context.server.state_mut().insert_character(character);
    let mut state = context.server.state_mut();
    let unknown = atcommand_extra::handle(&context.server, &mut state, char_id, "mobsearch", &["not", "a", "monster"]).unwrap();
    assert!(unknown[0].contains("usage: @mobsearch"));
    let poring = atcommand_extra::handle(&context.server, &mut state, char_id, "mobsearch", &["Poring"]).unwrap();
    assert!(poring[0].starts_with("Mob Search... 'Poring'"));
    assert_eq!(poring.last().unwrap(), "Number of monsters: 0");
}
