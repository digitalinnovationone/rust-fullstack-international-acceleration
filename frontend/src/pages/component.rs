use leptos::*;
use thaw::*;
use strum::VariantArray;

use crate::{services::component::{create_component, fetch_components}, types::component::{ComponentData, CreateComponentData, Slot}};

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let loaded = create_rw_signal(false);
    let (components, set_components) = create_signal(Vec::new());

    let components_loader = Resource::once(move || async {
        fetch_components().await
    });

    create_effect(move |_| {
        components_loader.and_then(|components_data| {
            logging::log!("{components_data:?}");
            set_components.update(|components| components.extend(components_data.iter().cloned()));
            loaded.set(true);
        })
    });

    let (read_component, write_component) = create_signal(Option::<ComponentData>::None);

    create_effect(move |_| {
        if let Some(component_data) = read_component() {
            set_components.update(|components| components.push(component_data));
        }
    });

    let show_add_modal = create_rw_signal(false);

    view! {
        <ComponentsTable components loaded/>
        <Button block=true on_click=move |_| show_add_modal.set(true) >Add</Button>
        <AddComponentModal show=show_add_modal write_component />
    }
}

#[component]
pub fn ComponentsTable(
    #[prop(into)] components: MaybeSignal<Vec<ComponentData>>,
    #[prop(into)] loaded: MaybeSignal<bool>,
) -> impl IntoView {
    let rows = Signal::derive(components);

    view! {
        <Table>
            <thead>
                <tr>
                    <th>"Slot"</th>
                    <th>"Manufacturer"</th>
                    <th>"Model"</th>
                    <th>"Price (R$)"</th>
                </tr>
            </thead>

            <tbody>
                <Show when=loaded
                    fallback=|| view! {
                        <tr>
                            <td><Skeleton text=true /></td>
                            <td><Skeleton text=true /></td>
                            <td><Skeleton text=true /></td>
                            <td><Skeleton text=true /></td>
                        </tr>
                    }
                >
                    <For
                        each=rows
                        key=|component| component.id.clone()
                        let:component
                    >
                        <tr>
                            <td>{component.slot}</td>
                            <td>{component.manufacturer}</td>
                            <td>{component.model}</td>
                            <td>{component.price}</td>
                        </tr>
                    </For>
                </Show>
            </tbody>
        </Table>
    }
}

#[component]
fn AddComponentModal(
    #[prop(into)] show: RwSignal<bool>,
    #[prop(into)] write_component: WriteSignal<Option<ComponentData>>,
) -> impl IntoView {
    let manufacturer = create_rw_signal(String::new());
    let model = create_rw_signal(String::new());
    let price = create_rw_signal(0);

    let slot_options: Vec<_> = Slot::VARIANTS
        .iter()
        .map(|slot| SelectOption {
            label: slot.to_string(),
            value: *slot,
        })
        .collect();

    let slot = create_rw_signal(Option::<Slot>::None);

    let manufacturer_invalid = Signal::derive(move || manufacturer().is_empty());
    let model_invalid = Signal::derive(move || model().is_empty());

    let is_submitting = create_rw_signal(false);

    let submit_is_disabled = Signal::derive(move ||
        is_submitting()
        || manufacturer_invalid()
        || model_invalid()
        || slot().is_none()
    );

    let on_click = move |_| {
        is_submitting.set(true);

        let component = CreateComponentData {
            manufacturer: manufacturer(),
            model: model(),
            slot: slot().unwrap(),
            price: price(),
        };

        spawn_local(async move {
            let req = create_component(component).await;
            match req {
                Ok(component_data) => {
                    write_component(Some(component_data));
                    slot.set(None);
                    manufacturer.update(String::clear);
                    model.update(String::clear);
                    price.set(0);
                    is_submitting.set(false);
                    show.set(false);
                },
                Err(err) => logging::error!("Create Component Error: {err}"),
            }
        });
    };

    view! {
        <Modal title="Add Component" show z_index=1999>
            <Grid cols=2>
                <GridItem column=2>
                    <Select value=slot options=slot_options />
                </GridItem>

                <GridItem>
                    <label for="manufacturer">"Manufacturer"</label>
                </GridItem>

                <GridItem>
                    <Input attr:id="manufacturer" value=manufacturer invalid=manufacturer_invalid />
                </GridItem>

                <GridItem>
                    <label for="model">"Model"</label>
                </GridItem>

                <GridItem>
                    <Input attr:id="model" value=model invalid=model_invalid/>
                </GridItem>

                <GridItem>
                    <label for="price">"Price (R$)"</label>
                </GridItem>

                <GridItem>
                    <InputNumber attr:id="price" value=price step=100 />
                </GridItem>
            </Grid>

            <Button
                block=true
                disabled=submit_is_disabled
                on_click
            >
                {move || if is_submitting() {
                    view! {
                        <Spinner size=SpinnerSize::Tiny/>
                    }
                } else {
                    "Submit".into_view()
                }}
            </Button>
        </Modal>
    }
}
