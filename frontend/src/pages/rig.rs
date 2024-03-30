use std::collections::BTreeMap;

use leptos::*;
use leptos_meta::Style;
use strum::VariantArray;
use thaw::*;

use crate::pages::component::ComponentsTable;
use crate::services::component::fetch_components;
use crate::services::rig::{create_rig, fetch_rigs};
use crate::types::rig::CreateRigData;
use crate::types::{component::{ComponentData, Slot}, rig::Rig};

#[component]
pub fn RigsPage() -> impl IntoView {
    let (rigs, set_rigs) = create_signal(Vec::<Rig>::new());

    let rig_loader = Resource::once(|| async {
        fetch_rigs().await
    });

    create_effect(move |_| {
        rig_loader.and_then(|rigs| {
            set_rigs.update(|r| r.extend(rigs.iter().cloned()))
        });
    });

    let (selected_rig, set_selected_rig) = create_signal(Option::<Rig>::None);
    let show_add_rig = create_rw_signal(false);
    let (read_rig, write_rig) = create_signal(Option::<Rig>::None);

    create_effect(move |_| {
        if let Some(rig) = read_rig() {
            set_rigs.update(|r| r.push(rig));
        }
    });

    view! {
        <RigsGrid rigs set_selected_rig set_add_rig=show_add_rig.write_only() />
        <RigDetailModal rig=selected_rig />
        <AddRig show=show_add_rig write_rig />
    }
}

#[component]
fn RigsGrid(
    #[prop(into)] rigs: ReadSignal<Vec<Rig>>,
    #[prop(into)] set_selected_rig: WriteSignal<Option<Rig>>,
    #[prop(into)] set_add_rig: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Grid cols=3 x_gap=8 y_gap=8>
            <For
                each=rigs
                key=|rig| rig.id.clone()
                let:rig
            >
                <GridItem>
                    <RigItem rig set_selected_rig />
                </GridItem>
            </For>
            <GridItem>
                <AddRigPlaceholder set_clicked=set_add_rig />
            </GridItem>
        </Grid>
    }
}

#[component]
fn RigItem(
    rig: Rig,
    #[prop(into)] set_selected_rig: WriteSignal<Option<Rig>>,
) -> impl IntoView {
    let on_click = {
        let rig = rig.clone();
        move |_| {
            set_selected_rig.set(Some(rig.clone()));
        }
    };

    view! {
        <div class="rig-parent" on:click=on_click>
            <Image
                src="https://i.ebayimg.com/images/g/MbUAAOSwI7ZimiI5/s-l1200.webp"
                width="200px"
                height="200px"
            />
            <div class="rig-desc">
                {rig.name}
            </div>
            <div class="rig-desc">
                R$ {rig.total_price},00
            </div>
        </div>
        <Style>
        ".rig-parent {
            border-color: #00FF00;
            border-style: solid;
            border-width: 3px;
            border-radius: 0.6rem;
            cursor: pointer;
        }

        .rig-desc {
            height: 60px;
            text-align: center;
            line-height: 60px;
        }
        "
        </Style>
    }
}

#[component]
pub fn AddRigPlaceholder(
    #[prop(into)] set_clicked: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="placeholder" on:click=move |_| set_clicked.set(true)>
            <Icon
                icon=icondata::IoAddCircleOutline
                width="100px"
                height="100px"
            />
        </div>
        <Style>
        ".placeholder {
            background-color: #3333CC;
            border-color: #0000FF;
            border-style: solid;
            border-width: 3px;
            border-radius: 0.6rem;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            height: 300px;
        }"
        </Style>
    }
}

#[component]
pub fn RigDetailModal(
    #[prop(into)] rig: Signal<Option<Rig>>,
) -> impl IntoView
{
    view! {
        {move || match rig() {
            None => ().into_view(),
            Some(rig) => {
                let name = rig.name.clone();
                view! {
                    <Modal title={name} show=true z_index=1999>
                        <ComponentsTable components=rig.components loaded=true />
                    </Modal>
                }
            }
        }}
    }
}

#[component]
pub fn AddRig(
    #[prop(into)] show: RwSignal<bool>,
    #[prop(into)] write_rig: WriteSignal<Option<Rig>>,
) -> impl IntoView
{
    let name = create_rw_signal(String::new());
    let total_price = create_rw_signal(0);

    let name_invalid = Signal::derive(move || name().is_empty());

    let current_tab = create_rw_signal(Slot::Cpu.to_string());

    let (components, set_components) = create_signal(
        Slot::VARIANTS
            .iter()
            .map(|slot| (*slot, (Vec::<ComponentData>::new(), create_rw_signal(Option::<String>::None))))
            .collect::<BTreeMap<_, (_, _)>>()
    );

    let components_loader = Resource::once(|| async {
        fetch_components().await
    });

    create_effect(move |_| {
        components_loader.and_then(|comps| {
            comps
                .iter()
                .cloned()
                .for_each(|component| {
                    set_components.update(|components| {
                        components
                            .get_mut(&component.slot)
                            .unwrap()
                            .0
                            .push(component);
                    })
                })
        });
    });

    create_effect(move |_| {
        total_price.set(
            components()
                .values()
                .map(|(slot_components, selected_component)| {
                    if let Some(selected_component_id) = selected_component() {
                        slot_components
                            .iter()
                            .find(|component| component.id == selected_component_id)
                            .map(|component| component.price)
                            .unwrap_or_default()
                    } else {
                        0
                    }
                })
                .sum()
        );
    });

    let is_submitting = create_rw_signal(false);

    let submit_is_disabled = Signal::derive(move ||
        is_submitting()
        || name_invalid()
        || components()
            .values()
            .any(|(_, selected_component)| selected_component().is_none())
    );

    let on_click = move |_| {
        let rig = CreateRigData {
            name: name(),
            components: components()
                .values()
                .map(|(_, selected_component_id)| selected_component_id().unwrap())
                .collect()
        };

        is_submitting.set(true);

        spawn_local(async move {
            let req = create_rig(rig).await;
            match req {
                Ok(rig) => {
                    write_rig.set(Some(rig));
                    name.update(String::clear);
                    total_price.set(0);
                    is_submitting.set(false);
                    components()
                        .values()
                        .for_each(|(_, selected_component_id)| selected_component_id.set(None));

                    show.set(false);
                },
                Err(err) => logging::error!("Create Rig Error: {err}"),
            }
        });
    };

    view! {
        <Drawer title="Add new Rig" show placement=DrawerPlacement::Bottom height="80%">
            <Grid cols=2 x_gap=4 y_gap=4>
                <GridItem column=2>
                    <Input value=name placeholder="Name" invalid=name_invalid />
                </GridItem>
                <GridItem>
                    <div class="total">
                        Total: R$ {total_price}
                    </div>
                    <Style>
                    ".total {
                        border-radius: 3px;
                        border-style: solid;
                        border-width: 1px;
                        border-color: #e0e0e6;
                        height: 32px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    }"
                    </Style>
                </GridItem>
                <GridItem>
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
                </GridItem>
            </Grid>
            <Divider />
            <Tabs value=current_tab>
                {move || components().into_iter().map(|(slot, (slot_components, selected_component))| view! {
                    <Tab key={slot.to_string()}>
                        <TabLabel slot>
                            {slot}
                        </TabLabel>
                        <RadioGroup value=selected_component>
                            {slot_components.into_iter().map(|slot_component| {
                                view! {
                                    <RadioItem key={slot_component.id}>
                                        {slot_component.manufacturer} | {slot_component.model} - R$ {slot_component.price},00
                                    </RadioItem>
                                }
                            }).collect::<Vec<_>>()}
                        </RadioGroup>
                    </Tab>
                }).collect::<Vec<_>>()}
            </Tabs> 
        </Drawer>
    }
}