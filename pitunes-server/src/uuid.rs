use uuid::Uuid;

pub fn uuidv4() -> String {
    String::from(
        Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    )
}
