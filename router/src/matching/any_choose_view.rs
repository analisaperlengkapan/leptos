use super::ChooseView;
use futures::FutureExt;
use std::{future::Future, pin::Pin};
use tachys::{erased::Erased, view::any_view::AnyView};

/// A type-erased [`ChooseView`].
pub struct AnyChooseView {
    value: Erased,
    clone: fn(&Erased) -> AnyChooseView,
    #[allow(clippy::type_complexity)]
    choose: fn(Erased) -> Pin<Box<dyn Future<Output = AnyView>>>,
    preload: fn(&Erased) -> Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Clone for AnyChooseView {
    fn clone(&self) -> Self {
        (self.clone)(&self.value)
    }
}

impl AnyChooseView {
    pub(crate) fn new<T: ChooseView>(value: T) -> Self {
        fn clone<T: ChooseView>(value: &Erased) -> AnyChooseView {
            AnyChooseView::new(value.get_ref::<T>().clone())
        }

        fn choose<T: ChooseView>(
            value: Erased,
        ) -> Pin<Box<dyn Future<Output = AnyView>>> {
            value.into_inner::<T>().choose().boxed_local()
        }

        fn preload<T: ChooseView>(
            value: &Erased,
        ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
            value.get_ref::<T>().preload()
        }

        Self {
            value: Erased::new(value),
            clone: clone::<T>,
            choose: choose::<T>,
            preload: preload::<T>,
        }
    }
}

impl ChooseView for AnyChooseView {
    async fn choose(self) -> AnyView {
        (self.choose)(self.value).await
    }

    fn preload(
        &self,
    ) -> ::std::pin::Pin<
        ::std::boxed::Box<dyn ::std::future::Future<Output = ()> + Send>,
    > {
        (self.preload)(&self.value)
    }
}
