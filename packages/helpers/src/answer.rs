use cosmwasm_std::{Attribute, Event, Response};

pub fn response<A: Into<Attribute>, T>(
    ty: &str,
    contract_name: &str,
    attrs: impl IntoIterator<Item = A>,
) -> Response<T> {
    Response::<T>::new()
        .add_event(Event::new(format!("{}-{}", contract_name, ty)).add_attributes(attrs))
}
