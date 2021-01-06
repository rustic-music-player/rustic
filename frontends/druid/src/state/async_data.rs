use druid::Data;

#[derive(Clone, Data, Debug)]
pub enum AsyncData<TData: Data, TPending: Data = (), TError: Data = String> {
    Empty,
    Pending(TPending),
    Resolved(TData),
    Rejected(TError),
}

impl<TData: Data, TPending: Data, TError: Data> Default for AsyncData<TData, TPending, TError> {
    fn default() -> Self {
        AsyncData::Empty
    }
}

impl<TData: Data, TPending: Data, TError: Data> AsyncData<TData, TPending, TError> {
    pub fn state(&self) -> AsyncDataState {
        match self {
            AsyncData::Empty => AsyncDataState::Empty,
            AsyncData::Pending(_) => AsyncDataState::Pending,
            AsyncData::Resolved(_) => AsyncDataState::Resolved,
            AsyncData::Rejected(_) => AsyncDataState::Rejected,
        }
    }
}

#[derive(Clone, Copy, Debug, Data, Eq, PartialEq)]
pub enum AsyncDataState {
    Empty,
    Pending,
    Resolved,
    Rejected,
}
