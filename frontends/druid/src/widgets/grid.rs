use druid::im::{vector, Vector};
use druid::widget::{List, ListIter};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, KeyOrValue, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget, WidgetPod,
};
use std::fmt::Debug;

type GridListInput<T> = Vector<T>;

pub struct GridList<TItem: Data + Debug> {
    tile_width: f64,
    inner: WidgetPod<GridData<TItem>, List<Vector<TItem>>>,
    tiles_per_line: u64,
    spacing: KeyOrValue<f64>,
}

impl<TItem: Data + Debug> GridList<TItem> {
    pub fn new<F, W>(tile_width: f64, widget: F, spacing: impl Into<KeyOrValue<f64>>) -> Self
    where
        W: Widget<TItem> + 'static,
        F: (Fn() -> W) + 'static + Clone,
    {
        let spacing = spacing.into();
        let outer_spacing = spacing.clone();

        GridList {
            spacing: spacing.clone(),
            tile_width,
            inner: WidgetPod::new(
                List::new(move || {
                    List::new(widget.clone())
                        .horizontal()
                        .with_spacing(spacing.clone())
                })
                .with_spacing(outer_spacing),
            ),
            tiles_per_line: 7, // TODO: fix auto layout
        }
    }

    fn get_grid_data(&self, data: &GridListInput<TItem>) -> GridData<TItem> {
        GridData {
            data: data.clone(),
            columns: self.tiles_per_line,
        }
    }
}

impl<TItem: Data + Debug> Widget<GridListInput<TItem>> for GridList<TItem> {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut GridListInput<TItem>,
        env: &Env,
    ) {
        let mut data = self.get_grid_data(data);
        self.inner.widget_mut().event(ctx, event, &mut data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &GridListInput<TItem>,
        env: &Env,
    ) {
        // log::trace!("lifecycle {:?}", event);
        // if let LifeCycle::Size(size) = event {
        //     log::debug!("update size {:?}", size);
        //     let spacing = self.spacing.resolve(env);
        //     let tile_bounds = self.tile_width + spacing;
        //     let tiles_per_line: f64 = size.width / tile_bounds;
        //     self.tiles_per_line = tiles_per_line.floor() as u64;
        // }
        let data = self.get_grid_data(data);
        self.inner.widget_mut().lifecycle(ctx, event, &data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &GridListInput<TItem>,
        data: &GridListInput<TItem>,
        env: &Env,
    ) {
        let old_data = self.get_grid_data(old_data);
        let data = self.get_grid_data(data);
        self.inner.widget_mut().update(ctx, &old_data, &data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &GridListInput<TItem>,
        env: &Env,
    ) -> Size {
        // log::debug!("layout {:?} - {:?}", bc.min(), bc.max());
        // let spacing = self.spacing.resolve(env);
        // let tile_bounds = self.tile_width + spacing;
        // let tiles_per_line: f64 = bc.max().width / tile_bounds;
        // let tiles_per_line = tiles_per_line.floor() as u64;
        // if tiles_per_line != self.tiles_per_line {
        //     log::debug!("tiles_per_line {:?}", tiles_per_line);
        //     self.tiles_per_line = tiles_per_line;
        // }
        let data = self.get_grid_data(data);
        self.inner.widget_mut().layout(ctx, bc, &data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &GridListInput<TItem>, env: &Env) {
        let data = self.get_grid_data(data);
        self.inner.widget_mut().paint(ctx, &data, env)
    }
}

#[derive(Debug, Clone, Data)]
struct GridData<TItem: Data + Debug> {
    data: GridListInput<TItem>,
    columns: u64,
}

impl<TItem: Data + Clone + Debug> GridData<TItem> {
    fn get_rows(&self) -> Vec<Vector<TItem>> {
        self.data
            .iter()
            .fold(Vec::<Vector<TItem>>::new(), |mut rows, item| {
                if let Some(last) = rows.last_mut() {
                    if last.len() < self.columns as usize {
                        last.push_back(item.clone());
                        return rows;
                    }
                }
                let row = vector![item.clone()];
                rows.push(row);
                rows
            })
    }
}

impl<TItem: Data + Clone + Debug> ListIter<Vector<TItem>> for GridData<TItem> {
    fn for_each(&self, mut cb: impl FnMut(&Vector<TItem>, usize)) {
        let rows = self.get_rows();
        log::trace!("grid rows: {}", rows.len());

        for (index, row) in rows.iter().enumerate() {
            log::trace!("grid columns: {}", row.len());
            cb(row, index);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Vector<TItem>, usize)) {
        let mut rows = self.get_rows();
        log::trace!("grid rows: {}", rows.len());

        for (index, row) in rows.iter_mut().enumerate() {
            log::trace!("grid columns: {}", row.len());
            cb(row, index);
        }
    }

    fn data_len(&self) -> usize {
        self.get_rows().len()
    }
}
