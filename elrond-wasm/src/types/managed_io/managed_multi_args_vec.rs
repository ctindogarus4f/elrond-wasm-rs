use core::marker::PhantomData;
use crate::{ArgId, DynArg, DynArgInput, api::ManagedTypeApi, types::ManagedVecItem};

pub struct ManagedMultiArgVec<M, T, I>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
    I: DynArgInput
{
    _phantom1: PhantomData<T>,
    _phantom2: PhantomData<M>,
    loader: I
}

// alias
pub type ManagedVarArgs<M, T, I> = ManagedMultiArgVec<M, T, I>;

impl<M, T, I> DynArg for ManagedMultiArgVec<M, T, I>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
    I: DynArgInput
{
    // #[inline(never)]
    fn dyn_load<DAI: DynArgInput>(loader: &mut DAI, arg_id: ArgId) -> Self {
        Self {
            _phantom1: PhantomData::default(),
            _phantom2: PhantomData::default(),
            loader: loader
        }
    }
}