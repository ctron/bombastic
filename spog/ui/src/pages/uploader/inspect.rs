use super::CommonHeader;
use patternfly_yew::prelude::*;
use reqwest::Body;
use spog_ui_backend::{use_backend, SBOMService};
use spog_ui_common::error::components::Error;
use spog_ui_navigation::AppRoute;
use std::rc::Rc;
use yew::prelude::*;
use yew_more_hooks::hooks::r#async::*;
use yew_nested_router::prelude::*;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Clone, PartialEq, Properties)]
pub struct InspectProperties {
    pub raw: Rc<String>,
    pub onreset: Callback<()>,
}

#[function_component(Inspect)]
pub fn inspect(props: &InspectProperties) -> Html {
    let backend = use_backend();
    let access_token = use_latest_access_token();

    let upload = {
        use_async_with_cloned_deps(
            move |raw| async move {
                let service = SBOMService::new(backend, access_token);
                service.upload(Body::from((*raw).clone())).await.map(Rc::new)
            },
            props.raw.clone(),
        )
    };

    html!(
        <>
            <CommonHeader onreset={props.onreset.clone()}/>
            {
                match &*upload {
                    UseAsyncState::Pending | UseAsyncState::Processing => html!(
                        <PageSection fill={PageSectionFill::Fill}>
                            <Spinner />
                        </PageSection>
                    ),
                    UseAsyncState::Ready(Ok(data)) => html!(
                        <Redirect sbom_id={data.clone()}/>
                    ),
                    UseAsyncState::Ready(Err(_)) => html!(
                        <Error title="Error" message="Error while uploading the file" />
                    ),
                }
            }
        </>
    )
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct RedirectProps {
    sbom_id: Rc<String>,
}

#[function_component(Redirect)]
pub fn redirect(props: &RedirectProps) -> Html {
    let router = use_router::<AppRoute>();

    use_effect_with(props.clone(), move |_props| {
        if let Some(router) = &router {
            router.push(AppRoute::Search { terms: "".to_string() });
        }
        || {}
    });

    Html::default()
}
